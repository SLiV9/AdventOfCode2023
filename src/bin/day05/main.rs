/**/

use aoc2023::run;
use parse_display::{Display, FromStr};
use smallvec::SmallVec;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> i64
{
	let mut lines = input.lines();
	let header = lines.next().unwrap();
	let (_, seeds) = header.split_once(':').unwrap();
	let seeds: SmallVec<[i64; 32]> = seeds
		.split(' ')
		.map(|word| word.trim())
		.filter(|word| !word.is_empty())
		.map(|word| word.parse().unwrap())
		.collect();
	let mut curr = seeds;
	let mut next: SmallVec<[i64; 32]> = SmallVec::default();
	for line in lines
	{
		if line
			.as_bytes()
			.get(0)
			.map_or(false, |&x| x.is_ascii_digit())
		{
			let transform: MapRow = line.parse().unwrap();
			let is_transformed = |&x: &i64| {
				x >= transform.source_start
					&& x < transform.source_start + transform.len
			};
			curr.sort_unstable_by_key(is_transformed);
			let drain_offset = curr.partition_point(|x| !is_transformed(x));
			let matched = curr.drain(drain_offset..);
			let mapped = matched.map(|x| {
				x - transform.source_start + transform.destination_start
			});
			next.extend(mapped);
		}
		else
		{
			curr.append(&mut next);
		}
	}
	curr.append(&mut next);
	curr.into_iter().min().unwrap()
}

#[derive(Display, FromStr)]
#[display("{destination_start} {source_start} {len}")]
struct MapRow
{
	destination_start: i64,
	source_start: i64,
	len: i64,
}

fn two(input: &str) -> i64
{
	input.len() as i64 * 0
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 35);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

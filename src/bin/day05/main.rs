/**/

use aoc2023::run;
use itertools::Itertools;
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
			let mapped = matched.map(|x| transform.apply(x));
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

impl MapRow
{
	fn apply(&self, x: i64) -> i64
	{
		x - self.source_start + self.destination_start
	}
}

fn two(input: &str) -> i64
{
	let mut lines = input.lines();
	let header = lines.next().unwrap();
	let (_, seeds) = header.split_once(':').unwrap();
	let seed_ranges: SmallVec<[SeedRange; 32]> = seeds
		.split(' ')
		.map(|word| word.trim())
		.filter(|word| !word.is_empty())
		.map(|word| word.parse().unwrap())
		.chunks(2)
		.into_iter()
		.map(|mut chunk| (chunk.next().unwrap(), chunk.next().unwrap()))
		.map(|(start, len)| SeedRange {
			start,
			end: start + len,
		})
		.collect();

	let mut curr = seed_ranges;
	let mut next: SmallVec<[SeedRange; 32]> = SmallVec::default();
	for line in lines
	{
		if line
			.as_bytes()
			.get(0)
			.map_or(false, |&x| x.is_ascii_digit())
		{
			let transform: MapRow = line.parse().unwrap();
			let transform_range = SeedRange::from(&transform);
			let is_touched = |&x: &SeedRange| transform_range.intersects(x);
			curr.sort_unstable_by_key(is_touched);
			let drain_offset = curr.partition_point(|x| !is_touched(x));
			let matched = curr.drain(drain_offset..);
			let mut untouched: SmallVec<[SeedRange; 32]> = SmallVec::default();
			for range in matched
			{
				let (before, inside, after) = range.cut(transform_range);
				if before.len() > 0
				{
					untouched.push(before);
				}
				if after.len() > 0
				{
					untouched.push(after);
				}
				let start = transform.apply(inside.start);
				next.push(SeedRange {
					start,
					end: start + inside.len(),
				});
			}
			curr.append(&mut untouched);
		}
		else
		{
			next.append(&mut curr);
			next.sort_unstable_by_key(|range| range.start);
			for range in next.drain(..)
			{
				if let Some(last) = curr.last_mut()
				{
					if last.intersects(range)
					{
						*last = last.union(range);
					}
					else
					{
						curr.push(range);
					}
				}
				else
				{
					curr.push(range);
				}
			}
		}
	}
	curr.append(&mut next);
	curr.into_iter().map(|range| range.start).min().unwrap()
}

#[derive(Debug, Clone, Copy)]
struct SeedRange
{
	start: i64,
	end: i64,
}

impl From<&MapRow> for SeedRange
{
	fn from(row: &MapRow) -> Self
	{
		SeedRange {
			start: row.source_start,
			end: row.source_start + row.len,
		}
	}
}

impl SeedRange
{
	fn len(&self) -> i64
	{
		self.end - self.start
	}

	fn intersects(self, other: SeedRange) -> bool
	{
		self.intersection(other).len() > 0
	}

	fn intersection(self, other: SeedRange) -> SeedRange
	{
		SeedRange {
			start: std::cmp::max(self.start, other.start),
			end: std::cmp::min(self.end, other.end),
		}
	}

	fn union(self, other: SeedRange) -> SeedRange
	{
		SeedRange {
			start: std::cmp::min(self.start, other.start),
			end: std::cmp::max(self.end, other.end),
		}
	}

	fn cut(self, other: SeedRange) -> (SeedRange, SeedRange, SeedRange)
	{
		let inside = self.intersection(other);
		let before = SeedRange {
			start: self.start,
			end: inside.start,
		};
		let after = SeedRange {
			start: inside.end,
			end: self.end,
		};
		(before, inside, after)
	}
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
		assert_eq!(two(PROVIDED), 46);
	}
}

/**/

use aoc2023::binary_search_range;
use aoc2023::run;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> i64
{
	let mut lines = input.lines();
	let (_, time_line) = lines.next().unwrap().split_once(':').unwrap();
	let (_, distance_line) = lines.next().unwrap().split_once(':').unwrap();
	let times = time_line
		.split(' ')
		.map(|x| x.trim())
		.filter(|x| !x.is_empty())
		.map(|x| x.parse().unwrap());
	let distances = distance_line
		.split(' ')
		.map(|x| x.trim())
		.filter(|x| !x.is_empty())
		.map(|x| x.parse().unwrap());
	let races = times.zip(distances);
	races.map(|(t, d)| win_race(t, d)).product()
}

fn win_race(time: i64, distance: i64) -> i64
{
	let mut num_possibilities = 0;
	let mut t = 1;
	while t + t < time
	{
		if t * (time - t) > distance
		{
			num_possibilities += 2;
		}
		t += 1;
	}
	if t + t == time && t * t > distance
	{
		num_possibilities += 1;
	}
	num_possibilities
}

fn two(input: &str) -> i64
{
	let mut lines = input.lines();
	let time = parse_badly_kerned_number(lines.next().unwrap());
	let distance = parse_badly_kerned_number(lines.next().unwrap());
	let root = (distance as f64).sqrt().ceil() as i64;
	let start = binary_search_range(0..root, |t| t * (time - t) > distance);
	let end = binary_search_range(root..time, |t| t * (time - t) <= distance);
	let start = start.unwrap();
	let end = end.unwrap();
	end - start
}

fn parse_badly_kerned_number(line: &str) -> i64
{
	let (_, line) = line.split_once(':').unwrap();
	let mut number = 0;
	for x in line.as_bytes()
	{
		if x.is_ascii_digit()
		{
			number *= 10;
			number += i64::from(x - b'0');
		}
	}
	number
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
		assert_eq!(one(PROVIDED), 288);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 71503);
	}
}

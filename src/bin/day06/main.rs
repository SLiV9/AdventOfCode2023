/**/

use aoc2023::run;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> i32
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

fn win_race(time: i32, distance: i32) -> i32
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

fn two(input: &str) -> i32
{
	input.len() as i32 * 0
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
		assert_eq!(two(PROVIDED), 0);
	}
}

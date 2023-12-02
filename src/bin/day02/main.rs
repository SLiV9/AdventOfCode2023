/**/

use aoc2023::run;

use parse_display::{Display, FromStr};
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> i32
{
	let max = Rgb {
		red: 12,
		green: 13,
		blue: 14,
	};
	input
		.lines()
		.filter(|line| !line.is_empty())
		.filter_map(|line| game_number_if_matches_max(line, max))
		.sum()
}

fn two(input: &str) -> i32
{
	input.len() as i32
}

#[derive(Debug, Clone, Copy)]
struct Rgb
{
	red: i32,
	green: i32,
	blue: i32,
}

#[derive(Debug, Clone, Copy, Display, FromStr)]
enum Sample
{
	#[display("{0} red")]
	Red(i32),
	#[display("{0} green")]
	Green(i32),
	#[display("{0} blue")]
	Blue(i32),
}

impl Sample
{
	fn is_allowed_by_max(self, max: Rgb) -> bool
	{
		match self
		{
			Sample::Red(x) => x <= max.red,
			Sample::Green(x) => x <= max.green,
			Sample::Blue(x) => x <= max.blue,
		}
	}
}

#[derive(Display, FromStr)]
#[display("Game {number}")]
struct Preamble
{
	number: i32,
}

fn game_number_if_matches_max(line: &str, max: Rgb) -> Option<i32>
{
	let (preamble, body) = line.split_once(':').unwrap();
	let preamble = Preamble::from_str(preamble).unwrap();
	let number = preamble.number;

	let is_possible = body
		.split(';')
		.flat_map(|grab| grab.trim().split(','))
		.map(|sample| Sample::from_str(sample.trim()).unwrap())
		.all(|sample| sample.is_allowed_by_max(max));

	if is_possible
	{
		Some(number)
	}
	else
	{
		None
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
		assert_eq!(one(PROVIDED), 8);
	}
}

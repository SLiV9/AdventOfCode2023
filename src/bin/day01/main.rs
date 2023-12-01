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
	input
		.lines()
		.filter(|line| !line.is_empty())
		.map(decode_line)
		.sum()
}

fn two(_input: &str) -> i32
{
	0
}

fn decode_line(line: &str) -> i32
{
	let mut digits =
		line.as_bytes().iter().filter_map(|&x| try_decode_digit(x));
	let head = digits.next().unwrap();
	let tail = digits.last().unwrap_or(head);
	head * 10 + tail
}

fn try_decode_digit(x: u8) -> Option<i32>
{
	match x
	{
		b'0'..=b'9' => Some(i32::from(x - b'0')),
		_ => None,
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
		assert_eq!(one(PROVIDED), 142);
	}
}

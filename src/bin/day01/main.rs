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
	input.len() as i32
}

fn two(_input: &str) -> i32
{
	0
}

#[cfg(test)]
mod tests
{
	use super::*;
	//use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 4);
	}
}

/**/

use aoc2023::run;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> u32
{
	let input = input.lines().next().unwrap();
	input.split(',').map(hash_word).map(u32::from).sum()
}

fn hash_word(word: &str) -> u8
{
	word.as_bytes().iter().cloned().fold(0, |acc: u8, x: u8| {
		acc.wrapping_mul(17).wrapping_add(x.wrapping_mul(17))
	})
}

fn two(input: &str) -> u32
{
	input.len() as u32 * 0
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
		assert_eq!(one(PROVIDED), 1320);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

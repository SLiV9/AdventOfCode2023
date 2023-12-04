/**/

use aoc2023::{ring_buffer::RingBuffer, run};
use smallvec::SmallVec;

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
		.map(decode_line_and_score_points)
		.sum()
}

fn decode_line_and_score_points(line: &str) -> i32
{
	let (_, data) = line.split_once(':').unwrap();
	let (windata, owndata) = data.split_once('|').unwrap();
	let winning: SmallVec<[u8; 10]> =
		parse_stream_of_numbers(windata).collect();
	let mut score = 0;
	for own in parse_stream_of_numbers(owndata)
	{
		if winning.contains(&own)
		{
			if score == 0
			{
				score = 1;
			}
			else
			{
				score *= 2;
			}
		}
	}
	score
}

fn parse_stream_of_numbers(data: &str) -> impl Iterator<Item = u8> + '_
{
	let mut current = 0;
	let chars = data.as_bytes().iter().chain(std::iter::once(&b'\0'));
	chars.filter_map(move |&x| match x
	{
		b'0'..=b'9' =>
		{
			let digit = x - b'0';
			current *= 10;
			current += digit;
			None
		}
		b' ' | b'\0' =>
		{
			let number = current;
			current = 0;
			Some(number).filter(|&x| x > 0)
		}
		_ => unreachable!(),
	})
}

fn two(input: &str) -> u32
{
	let lines = input.lines().filter(|line| !line.is_empty());
	let mut total_tickets = 0;
	let mut multipliers: RingBuffer<[u32; 12]> = RingBuffer::default();
	for line in lines
	{
		let own_multiplier = if let Some(&multiplier) = multipliers.head()
		{
			multipliers.drop_head();
			multiplier
		}
		else
		{
			1
		};
		total_tickets += own_multiplier;

		let (_, data) = line.split_once(':').unwrap();
		let (windata, owndata) = data.split_once('|').unwrap();
		let winning: SmallVec<[u8; 10]> =
			parse_stream_of_numbers(windata).collect();
		let num_matches = parse_stream_of_numbers(owndata)
			.filter(|own| winning.contains(own))
			.count();
		while multipliers.len() < num_matches
		{
			multipliers.push(1);
		}
		for multiplier in multipliers.iter_mut().take(num_matches)
		{
			*multiplier += own_multiplier;
		}
	}
	total_tickets
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
		assert_eq!(one(PROVIDED), 13);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 30);
	}
}

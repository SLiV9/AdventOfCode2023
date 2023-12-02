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
		.map(decode_line_using_only_digits)
		.sum()
}

fn two(input: &str) -> i32
{
	input
		.lines()
		.filter(|line| !line.is_empty())
		.map(decode_line_using_digits_and_words)
		.sum()
}

fn decode_line_using_only_digits(line: &str) -> i32
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

const UNMATCHABLE: &'static [u8] = b"\x01\x01\x01";

const WORDS: [&'static [u8]; 10] = [
	UNMATCHABLE,
	b"one",
	b"two",
	b"three",
	b"four",
	b"five",
	b"six",
	b"seven",
	b"eight",
	b"nine",
];

fn decode_line_using_digits_and_words(line: &str) -> i32
{
	let ascii_chars = line.as_bytes();
	let head = ForwardAutomaton::decode(ascii_chars);
	let tail = BackwardAutomaton::decode(ascii_chars);
	head * 10 + tail
}

type ForwardAutomaton = Automaton<1>;
type BackwardAutomaton = Automaton<-1>;

#[derive(Debug)]
struct Automaton<const SPEED: i32>
{
	buffer: [u8; 5],
}

impl<const SPEED: i32> Automaton<SPEED>
{
	fn decode(line: &[u8]) -> i32
	{
		let mut automaton: Automaton<SPEED> = Automaton { buffer: [0; 5] };
		if SPEED > 0
		{
			line.iter().find_map(|&x| automaton.step(x)).unwrap()
		}
		else
		{
			line.iter().rev().find_map(|&x| automaton.step(x)).unwrap()
		}
	}

	fn step(&mut self, x: u8) -> Option<i32>
	{
		match x
		{
			b'0'..=b'9' => return Some(i32::from(x - b'0')),
			b'a'..=b'z' =>
			{
				if SPEED > 0
				{
					self.buffer[0] = x;
					self.buffer[..].rotate_left(1);
				}
				else
				{
					self.buffer[..].rotate_right(1);
					self.buffer[0] = x;
				}

				(1..=9).find(|&number| {
					let word = WORDS[number as usize];
					let (start, end) = if SPEED > 0
					{
						(self.buffer.len() - word.len(), self.buffer.len())
					}
					else
					{
						(0, word.len())
					};
					word == &self.buffer[start..end]
				})
			}
			_ => panic!("Unexpected char: {x}"),
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const PROVIDED2: &str = include_str!("provided2.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 142);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED2), 281);
	}

	#[test]
	fn test_forward_automaton()
	{
		assert_eq!(ForwardAutomaton::decode(b"xfonexx"), 1);
	}

	#[test]
	fn test_backward_automaton()
	{
		assert_eq!(BackwardAutomaton::decode(b"xthreeeevenx"), 3);
	}

	#[test]
	fn test_decode_using_words()
	{
		assert_eq!(decode_line_using_digits_and_words("one84seven"), 17);
	}

	#[test]
	fn test_fix_bug()
	{
		assert_eq!(ForwardAutomaton::decode(b"abcone2threexyz"), 1);
	}
}

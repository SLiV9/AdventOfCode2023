/**/

use aoc2023::ring_buffer::RingBuffer;
use aoc2023::run;
use bitvec::array::BitArray;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> i32
{
	let lines = input.lines().filter(|line| !line.is_empty()).peekable();

	let mut resolved_sum = 0;
	let mut unresolved: RingBuffer<[UnresolvedNumber; 20]> =
		RingBuffer::default();
	let mut previous_symbols: BitArray<[u64; 4]> = BitArray::default();
	let mut current_symbols: BitArray<[u64; 4]> = BitArray::default();

	for (line_number, line) in lines.enumerate()
	{
		while let Some(head) = unresolved.head()
		{
			if head.line_number + 1 < line_number
			{
				unresolved.drop_head();
			}
			else
			{
				break;
			}
		}

		let mut active = UnresolvedNumber::default();
		let mut is_after_symbol = false;
		for (i, &x) in line.as_bytes().iter().enumerate()
		{
			match Grapheme::from_ascii(x)
			{
				Grapheme::Digit(number) =>
				{
					if active.number == 0
					{
						active.start = i.saturating_sub(1);
						active.line_number = line_number;
					}
					else
					{
						active.number *= 10;
					}
					active.number += number;
					active.end = (i + 2).min(line.len());
				}
				Grapheme::Whitespace =>
				{
					if active.number != 0
					{
						if is_after_symbol
							|| previous_symbols[active.range()].any()
						{
							resolved_sum += active.number;
						}
						else
						{
							unresolved.push(active);
						}
						active.number = 0;
					}
					is_after_symbol = false;
				}
				Grapheme::Symbol =>
				{
					while let Some(head) = unresolved.head()
					{
						if head.line_number < line_number && i + 1 >= head.start
						{
							resolved_sum += head.number;
							unresolved.drop_head();
						}
						else
						{
							break;
						}
					}

					if active.number != 0
					{
						resolved_sum += active.number;
						active.number = 0;
					}
					current_symbols.set(i, true);
					is_after_symbol = true;
				}
			}

			if let Some(head) = unresolved.head()
			{
				if head.line_number < line_number && head.end < i
				{
					unresolved.drop_head();
				}
			}
		}
		if active.number != 0
		{
			if is_after_symbol || previous_symbols[active.range()].any()
			{
				resolved_sum += active.number;
			}
			else
			{
				unresolved.push(active);
			}
		}

		previous_symbols = current_symbols;
		current_symbols = BitArray::default();
	}

	resolved_sum
}

fn two(input: &str) -> i32
{
	input.len() as i32 * 0
}

#[derive(Debug, Clone, Copy)]
enum Grapheme
{
	Digit(i32),
	Whitespace,
	Symbol,
}

#[derive(Debug, Clone, Copy, Default)]
struct UnresolvedNumber
{
	number: i32,
	line_number: usize,
	start: usize,
	end: usize,
}

impl UnresolvedNumber
{
	fn range(&self) -> std::ops::Range<usize>
	{
		self.start..self.end
	}
}

impl Grapheme
{
	fn from_ascii(x: u8) -> Self
	{
		match x
		{
			b'0'..=b'9' => Self::Digit((x - b'0') as i32),
			b'.' => Self::Whitespace,
			x if x.is_ascii_whitespace() => Self::Whitespace,
			_ => Self::Symbol,
		}
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
		assert_eq!(one(PROVIDED), 4361);
	}

	#[test]
	fn one_left()
	{
		assert_eq!(one("+123"), 123);
	}

	#[test]
	fn one_right()
	{
		assert_eq!(one("123*"), 123);
	}

	#[test]
	fn one_above()
	{
		assert_eq!(one("...+\n123."), 123);
		assert_eq!(one("+...\n.456"), 456);
	}

	#[test]
	fn one_below()
	{
		assert_eq!(one(".123\n$..."), 123);
		assert_eq!(one("456.\n...$"), 456);
		assert_eq!(one("9.10\n.$.."), 19);
	}

	#[test]
	fn one_none()
	{
		assert_eq!(one("......\n123..."), 0);
		assert_eq!(one(".....$\n456..."), 0);
		assert_eq!(one(".$....\n......\n789..."), 0);
	}
}

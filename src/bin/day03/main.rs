/**/

use aoc2023::ring_buffer::RingBuffer;
use aoc2023::run;
use bitvec::array::BitArray;
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> i32
{
	let lines = input.lines().filter(|line| !line.is_empty());

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
			if let Some(head) = unresolved.head()
			{
				if head.line_number < line_number && head.end <= i
				{
					unresolved.drop_head();
				}
			}

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
						if head.line_number < line_number && i >= head.start
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

fn two(input: &str) -> i32
{
	let mut gear_ratio_sum = 0;

	let width = input.lines().next().unwrap().len();
	let dots = [b'.'; 200];
	let empty = std::str::from_utf8(&dots[0..width]).unwrap();
	let lines = input.lines().filter(|line| !line.is_empty());
	let lines = std::iter::once(empty)
		.chain(lines)
		.chain(std::iter::once(empty));
	for (prev, curr, next) in lines.tuple_windows()
	{
		let gear_indices = curr
			.as_bytes()
			.iter()
			.enumerate()
			.filter(|(_, &x)| x == b'*')
			.map(|(i, _)| i);
		for i in gear_indices
		{
			if let Some(gear) = GearInfo::check(prev, curr, next, i)
			{
				gear_ratio_sum += gear.ratio()
			}
		}
	}
	gear_ratio_sum
}

fn try_left(line: &[u8], i: usize) -> Option<i32>
{
	let mut j = i;
	let mut part_number = 0;
	let mut multiplier = 1;
	while j > 0
	{
		j -= 1;
		match Grapheme::from_ascii(line[j])
		{
			Grapheme::Digit(number) =>
			{
				part_number += number * multiplier;
				multiplier *= 10;
			}
			Grapheme::Symbol => break,
			Grapheme::Whitespace => break,
		}
	}
	if part_number > 0
	{
		Some(part_number)
	}
	else
	{
		None
	}
}

fn try_right(line: &[u8], i: usize) -> Option<i32>
{
	read_right(line, i + 1)
}

fn read_right(line: &[u8], start: usize) -> Option<i32>
{
	let mut part_number = 0;
	for &x in &line[start..line.len()]
	{
		match Grapheme::from_ascii(x)
		{
			Grapheme::Digit(number) =>
			{
				part_number *= 10;
				part_number += number;
			}
			Grapheme::Symbol => break,
			Grapheme::Whitespace => break,
		}
	}
	if part_number > 0
	{
		Some(part_number)
	}
	else
	{
		None
	}
}

#[derive(Debug, Default)]
struct GearInfo
{
	part_numbers: [i32; 2],
	num_parts: usize,
}

impl GearInfo
{
	fn check(prev: &str, curr: &str, next: &str, i: usize) -> Option<Self>
	{
		let mut info = GearInfo::default();
		let curr = curr.as_bytes();
		let prev = prev.as_bytes();
		let next = next.as_bytes();
		info.check_part(try_left(curr, i)).unwrap();
		info.check_part(try_right(curr, i)).unwrap();
		info.check_line(prev, i)?;
		info.check_line(next, i)?;
		Some(info).filter(Self::is_complete)
	}

	fn check_line(&mut self, line: &[u8], i: usize) -> Option<()>
	{
		match Grapheme::from_ascii(line[i])
		{
			Grapheme::Digit(_) =>
			{
				if self.is_complete()
				{
					return None;
				}
				let mut start = i;
				while start > 0
				{
					match Grapheme::from_ascii(line[start - 1])
					{
						Grapheme::Digit(_) => start -= 1,
						_ => break,
					}
				}
				let part_number = read_right(line, start).unwrap();
				self.add_part(part_number);
				Some(())
			}
			_ =>
			{
				self.check_part(try_left(line, i))?;
				self.check_part(try_right(line, i))?;
				Some(())
			}
		}
	}

	fn check_part(&mut self, part_number: Option<i32>) -> Option<()>
	{
		if let Some(part_number) = part_number
		{
			if self.is_complete()
			{
				return None;
			}
			self.add_part(part_number);
		}
		Some(())
	}

	fn add_part(&mut self, part_number: i32)
	{
		self.part_numbers[self.num_parts] = part_number;
		self.num_parts += 1;
	}

	fn is_complete(&self) -> bool
	{
		self.num_parts == 2
	}

	fn ratio(&self) -> i32
	{
		self.part_numbers.iter().product()
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
		assert_eq!(one("1.2..4\n.$...."), 3);
	}

	#[test]
	fn one_none()
	{
		assert_eq!(one("......\n123..."), 0);
		assert_eq!(one(".....$\n456..."), 0);
		assert_eq!(one(".$....\n......\n789..."), 0);
		assert_eq!(one(".111\n"), 0);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 467835);
	}

	#[test]
	fn two_singlet()
	{
		assert_eq!(two("1*.\n..."), 0);
	}

	#[test]
	fn two_gears()
	{
		assert_eq!(two("10*20"), 200);
		assert_eq!(two("30.\n.*.\n.40"), 1200);
		assert_eq!(two("50...\n..*..\n...60"), 3000);
		assert_eq!(two("...70\n..*..\n80..."), 5600);
	}

	#[test]
	fn two_triplet()
	{
		assert_eq!(two("1*2\n.3."), 0);
	}
}

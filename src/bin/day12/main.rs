/**/

use aoc2023::run;
use smallvec::SmallVec;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> usize
{
	input.lines().filter(|x| !x.is_empty()).map(solve).sum()
}

fn solve(line: &str) -> usize
{
	let (symbols, numbers) = line.split_once(' ').unwrap();
	let numbers: SmallVec<[usize; 10]> = numbers
		.split(',')
		.map(|word| word.parse().unwrap())
		.collect();
	let symbols = symbols.as_bytes();

	let mut stack: SmallVec<[Probe; 128]> = SmallVec::new();
	stack.push(Probe::default());

	let mut num_possibilities = 0;
	'stack: while let Some(mut probe) = stack.pop()
	{
		// let mut minimal_width: usize =
		// 	numbers[probe.number_offset..].iter().map(|&x| x + 1).sum();
		// minimal_width -= 1;
		while probe.symbol_offset < symbols.len()
		{
			match symbols[probe.symbol_offset]
			{
				b'#' =>
				{
					if probe.gap_bits & 0b1 != 0
					{
						continue 'stack;
					}
					else if probe.set_bits & 0b1 != 0
					{
						// Nothing to do.
					}
					else if probe.number_offset < numbers.len()
					{
						let number = numbers[probe.number_offset];
						probe.gap_bits = 1 << number;
						probe.set_bits = probe.gap_bits - 1;
						let mask = u64::from(probe.set_bits);
						assert!(number < 32);
						probe.historic_bits |= mask << (32 - number);
						probe.number_offset += 1;
					}
					else
					{
						continue 'stack;
					}
				}
				b'.' =>
				{
					if probe.gap_bits & 0b1 != 0
					{
						assert_eq!(probe.gap_bits, 0b1);
						assert_eq!(probe.set_bits, 0);
						probe.gap_bits = 0;
					}
					else if probe.set_bits != 0
					{
						continue 'stack;
					}
					else
					{
						// Nothing to do.
					}
				}
				b'?' =>
				{
					if probe.gap_bits & 0b1 != 0
					{
						assert_eq!(probe.gap_bits, 0b1);
						assert_eq!(probe.set_bits, 0);
						probe.gap_bits = 0;
					}
					else if probe.set_bits & 0b1 != 0
					{
						// Nothing to do.
					}
					else if probe.number_offset < numbers.len()
					{
						let alt = Probe {
							symbol_offset: probe.symbol_offset + 1,
							number_offset: probe.number_offset,
							set_bits: 0,
							gap_bits: 0,
							historic_bits: (probe.historic_bits << 1),
						};
						stack.push(alt);

						let number = numbers[probe.number_offset];
						probe.gap_bits = 1 << number;
						probe.set_bits = probe.gap_bits - 1;
						let mask = u64::from(probe.set_bits);
						assert!(number < 32);
						probe.historic_bits |= mask << (32 - number);
						probe.number_offset += 1;
					}
					else
					{
						// Nothing to do.
					}
				}
				_ => unreachable!(),
			}
			probe.symbol_offset += 1;
			probe.set_bits >>= 1;
			probe.gap_bits >>= 1;
			probe.historic_bits <<= 1;
		}
		if probe.number_offset == numbers.len() && probe.set_bits == 0
		{
			// dbg!(format!("{:064b}", probe.historic_bits >> 31));
			// dbg!(probe);
			num_possibilities += 1;
		}
	}
	// dbg!(num_possibilities);
	num_possibilities
}

#[derive(Debug, Clone, Default)]
struct Probe
{
	symbol_offset: usize,
	number_offset: usize,
	set_bits: u32,
	gap_bits: u32,
	historic_bits: u64,
}

fn two(input: &str) -> usize
{
	input.len() * 0
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
		assert_eq!(one(PROVIDED), 21);
	}

	#[test]
	fn one_edge_case_end()
	{
		assert_eq!(solve("#..??## 1,4"), 1);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

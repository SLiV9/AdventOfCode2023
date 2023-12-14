/**/

use aoc2023::run;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut roll_point = [0u8; 128];
	let mut num_boulders_placed = 0;
	let mut total_load = 0;
	for (r, line) in input.lines().filter(|x| !x.is_empty()).enumerate()
	{
		for (c, x) in line.as_bytes().iter().enumerate()
		{
			match *x
			{
				b'#' => roll_point[c] = (r + 1) as u8,
				b'O' =>
				{
					num_boulders_placed += 1;
					if roll_point[c] < r as u8
					{
						total_load += r - roll_point[c] as usize;
						roll_point[c] += 1;
					}
					else
					{
						roll_point[c] = (r + 1) as u8;
					}
				}
				b'.' => (),
				_ => unreachable!(),
			}
		}
		total_load += num_boulders_placed;
	}
	total_load
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
		assert_eq!(one(PROVIDED), 136);
	}

	#[test]
	fn test_boulder_drop()
	{
		assert_eq!(one(".\n.\n.\nO"), 4);
		assert_eq!(one(".\n#\n.\nO"), 2);
		assert_eq!(one(".\n.\nO\nO"), 7);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

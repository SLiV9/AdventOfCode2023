/**/

use aoc2023::run;
use bitvec::array::BitArray;
use smallvec::SmallVec;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

#[derive(Debug, Clone, Copy)]
struct Galaxy
{
	row: i64,
	col: i64,
}

fn one(input: &str) -> i64
{
	solve(input, 2)
}

fn solve(input: &str, multiplier: i64) -> i64
{
	let mut galaxies: SmallVec<[Galaxy; 1024]> = SmallVec::new();
	let mut is_row_inhabited: BitArray<[u64; 3]> = BitArray::default();
	let mut is_col_inhabited: BitArray<[u64; 3]> = BitArray::default();
	let mut max_row = 0;
	let mut max_col = 0;
	for (r, line) in input.lines().filter(|x| !x.is_empty()).enumerate()
	{
		for (c, x) in line.as_bytes().iter().enumerate()
		{
			if *x == b'#'
			{
				galaxies.push(Galaxy {
					row: r as i64,
					col: c as i64,
				});
				is_row_inhabited.set(r, true);
				is_col_inhabited.set(c, true);
				max_row = max_row.max(r);
				max_col = max_col.max(c);
			}
		}
	}

	let addifier = multiplier - 1;
	for r in (0..max_row).rev()
	{
		if !is_row_inhabited[r]
		{
			for galaxy in &mut galaxies[..]
			{
				if galaxy.row as usize > r
				{
					galaxy.row += addifier;
				}
			}
		}
	}
	for c in (0..max_col).rev()
	{
		if !is_col_inhabited[c]
		{
			for galaxy in &mut galaxies[..]
			{
				if galaxy.col as usize > c
				{
					galaxy.col += addifier;
				}
			}
		}
	}

	let mut sum_of_distances = 0;
	for i in 0..galaxies.len()
	{
		let a = galaxies[i];
		for j in 0..i
		{
			let b = galaxies[j];
			let dr = (a.row as i64 - b.row as i64).abs();
			let dc = (a.col as i64 - b.col as i64).abs();
			let distance = dr + dc;
			// println!("From #{} to #{} is {distance}", j + 1, i + 1);
			sum_of_distances += distance;
		}
	}
	sum_of_distances
}

fn two(input: &str) -> i64
{
	solve(input, 1_000_000)
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
		assert_eq!(one(PROVIDED), 374);
	}
}

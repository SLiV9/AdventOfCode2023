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
	let lines = input.lines().chain(std::iter::once(""));
	let mut rows: SmallVec<[u32; 32]> = SmallVec::new();
	let mut cols: SmallVec<[u32; 32]> = SmallVec::new();
	let mut answer = 0;
	for line in lines
	{
		if line.is_empty()
		{
			// dbg!();
			// for row in &rows
			// {
			// 	dbg!(format!("{row:032b}"));
			// }
			// dbg!();
			// for col in &cols
			// {
			// 	dbg!(format!("{col:032b}"));
			// }

			let value = solve(&rows, &cols);
			// dbg!(value);
			answer += value;

			rows.clear();
			cols.clear();
		}
		else
		{
			let line = line.as_bytes();
			if cols.is_empty()
			{
				cols.resize(line.len(), 0);
			}
			let mut row = 0;
			for i in 0..cols.len()
			{
				let bit = match line[i]
				{
					b'#' => 1,
					b'.' => 0,
					_ => unreachable!(),
				};
				row <<= 1;
				row |= bit;
				cols[i] <<= 1;
				cols[i] |= bit;
			}
			rows.push(row);
		}
	}
	answer
}

fn solve(rows: &[u32], cols: &[u32]) -> usize
{
	for r in 1..rows.len()
	{
		let mut r0 = r - 1;
		let mut r1 = r;
		if rows[r0] != rows[r1]
		{
			continue;
		}
		let mut is_solution = true;
		while r0 > 0 && r1 + 1 < rows.len()
		{
			r0 -= 1;
			r1 += 1;
			if rows[r0] != rows[r1]
			{
				is_solution = false;
				break;
			}
		}
		if is_solution
		{
			return 100 * r;
		}
	}

	for c in 1..cols.len()
	{
		let mut c0 = c - 1;
		let mut c1 = c;
		if cols[c0] != cols[c1]
		{
			continue;
		}
		let mut is_solution = true;
		while c0 > 0 && c1 + 1 < cols.len()
		{
			c0 -= 1;
			c1 += 1;
			if cols[c0] != cols[c1]
			{
				is_solution = false;
				break;
			}
		}
		if is_solution
		{
			return c;
		}
	}

	panic!("No symmetry detected");
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
		assert_eq!(one(PROVIDED), 405);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

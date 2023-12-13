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
	let mirror_maker = || PerfectMirror { is_valid: true };
	solve(input, mirror_maker)
}

fn solve<F, M>(input: &str, mirror_maker: F) -> usize
where
	F: Fn() -> M,
	M: Mirror,
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

			let value = solve_grid(&rows, &cols, &mirror_maker);
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

trait Mirror
{
	fn reflect(&mut self, a: u32, b: u32) -> bool;
	fn resolve(self) -> bool;
}

struct PerfectMirror
{
	is_valid: bool,
}

impl Mirror for PerfectMirror
{
	fn reflect(&mut self, a: u32, b: u32) -> bool
	{
		self.is_valid = self.is_valid && a == b;
		self.is_valid
	}

	fn resolve(self) -> bool
	{
		self.is_valid
	}
}

struct SmudgedMirror
{
	num_smudges: u32,
}

impl Mirror for SmudgedMirror
{
	fn reflect(&mut self, a: u32, b: u32) -> bool
	{
		let differences = a ^ b;
		self.num_smudges += differences.count_ones();
		self.num_smudges <= 1
	}

	fn resolve(self) -> bool
	{
		self.num_smudges == 1
	}
}

fn solve_grid<F, M>(rows: &[u32], cols: &[u32], mirror_maker: F) -> usize
where
	F: Fn() -> M,
	M: Mirror,
{
	for r in 1..rows.len()
	{
		let mut r0 = r - 1;
		let mut r1 = r;
		let mut mirror = mirror_maker();
		if !mirror.reflect(rows[r0], rows[r1])
		{
			continue;
		}
		while r0 > 0 && r1 + 1 < rows.len()
		{
			r0 -= 1;
			r1 += 1;
			if !mirror.reflect(rows[r0], rows[r1])
			{
				break;
			}
		}
		if mirror.resolve()
		{
			return 100 * r;
		}
	}

	for c in 1..cols.len()
	{
		let mut c0 = c - 1;
		let mut c1 = c;
		let mut mirror = mirror_maker();
		if !mirror.reflect(cols[c0], cols[c1])
		{
			continue;
		}
		while c0 > 0 && c1 + 1 < cols.len()
		{
			c0 -= 1;
			c1 += 1;
			if !mirror.reflect(cols[c0], cols[c1])
			{
				break;
			}
		}
		if mirror.resolve()
		{
			return c;
		}
	}

	panic!("No symmetry detected");
}

fn two(input: &str) -> usize
{
	let mirror_maker = || SmudgedMirror { num_smudges: 0 };
	solve(input, mirror_maker)
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
		assert_eq!(two(PROVIDED), 400);
	}
}

/**/

use aoc2023::run;

const INPUT: &str = include_str!("input.txt");

const GRID_SIZE: usize = 192;

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut grid = [[0u8; GRID_SIZE]; GRID_SIZE];
	let mut start = Point::default();

	for (i, line) in input.lines().filter(|x| !x.is_empty()).enumerate()
	{
		let row = i + 1;
		let cols = 1..(line.as_bytes().len() + 1);
		grid[row][cols].copy_from_slice(line.as_bytes());
		if let Some(j) = line.as_bytes().iter().position(|&x| x == b'S')
		{
			start.row = row;
			start.col = j + 1;
		}
	}

	let mut probes = [start.up(), start.down(), start.left(), start.right()]
		.into_iter()
		.filter(|point| point.adjacents(&grid).contains(&start))
		.map(|point| Probe {
			curr: point,
			prev: start,
		});
	let mut a = probes.next().unwrap();
	let mut b = probes.next().unwrap();

	for round in 2..(GRID_SIZE * GRID_SIZE)
	{
		a = a.step(&grid);
		b = b.step(&grid);

		if a.curr == b.curr
		{
			return round;
		}
	}
	unreachable!()
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Point
{
	row: usize,
	col: usize,
}

impl Point
{
	fn up(self) -> Self
	{
		Self {
			row: self.row - 1,
			col: self.col,
		}
	}

	fn down(self) -> Self
	{
		Self {
			row: self.row + 1,
			col: self.col,
		}
	}

	fn left(self) -> Self
	{
		Self {
			row: self.row,
			col: self.col - 1,
		}
	}

	fn right(self) -> Self
	{
		Self {
			row: self.row,
			col: self.col + 1,
		}
	}

	fn adjacents(&self, grid: &[[u8; GRID_SIZE]; GRID_SIZE]) -> [Point; 2]
	{
		match grid[self.row][self.col]
		{
			b'|' => [self.up(), self.down()],
			b'-' => [self.left(), self.right()],
			b'L' => [self.up(), self.right()],
			b'J' => [self.up(), self.left()],
			b'7' => [self.down(), self.left()],
			b'F' => [self.down(), self.right()],
			_ => [*self, *self],
		}
	}
}

#[derive(Debug, Clone, Copy, Default)]
struct Probe
{
	curr: Point,
	prev: Point,
}

impl Probe
{
	fn step(&mut self, grid: &[[u8; GRID_SIZE]; GRID_SIZE]) -> Probe
	{
		let points = self.curr.adjacents(grid);
		let next = if points[0] != self.prev
		{
			points[0]
		}
		else
		{
			points[1]
		};
		Probe {
			curr: next,
			prev: self.curr,
		}
	}
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
	const PROVIDED_CLEAN: &str = include_str!("provided_clean.txt");
	const PROVIDED_ALT: &str = include_str!("provided_alt.txt");
	const PROVIDED_ALT_CLEAN: &str = include_str!("provided_alt_clean.txt");

	#[test]
	fn one_provided_clean()
	{
		assert_eq!(one(PROVIDED_CLEAN), 4);
	}

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 4);
	}

	#[test]
	fn one_provided_alt_clean()
	{
		assert_eq!(one(PROVIDED_ALT_CLEAN), 8);
	}

	#[test]
	fn one_provided_alt()
	{
		assert_eq!(one(PROVIDED_ALT), 8);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

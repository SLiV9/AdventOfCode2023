//!

use aoc2023::run;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

const INPUT: &str = include_str!("input.txt");

const GRID_SIZE: usize = 192;

const MAX_STRAIN: u8 = 2;

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut cost_grid = [[0; GRID_SIZE]; GRID_SIZE];
	let (num_rows, num_cols) = parse_grid(&mut cost_grid, input);

	let start = Explorer::default();
	let target = Point {
		row: (num_rows - 1) as u8,
		col: (num_cols - 1) as u8,
	};

	find_least_cost_with_a_star(&cost_grid, num_rows, num_cols, start, target)
}

fn parse_grid(
	grid: &mut [[u8; GRID_SIZE]; GRID_SIZE],
	input: &str,
) -> (usize, usize)
{
	let lines = input.lines().filter(|x| !x.is_empty()).enumerate();
	let mut num_rows = 0;
	let mut num_cols = 0;
	for (r, line) in lines
	{
		num_rows = num_rows.max(r + 1);
		num_cols = line.as_bytes().len();
		for (c, digit) in line.as_bytes().iter().enumerate()
		{
			grid[r][c] = *digit - b'0';
		}
	}
	(num_rows, num_cols)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point
{
	row: u8,
	col: u8,
}

impl Point
{
	fn step(
		self,
		direction: Direction,
		num_rows: usize,
		num_cols: usize,
	) -> Option<Point>
	{
		match direction
		{
			Direction::East if self.col as usize + 1 < num_cols =>
			{
				Some(Point {
					row: self.row,
					col: self.col + 1,
				})
			}
			Direction::South if self.row as usize + 1 < num_rows =>
			{
				Some(Point {
					row: self.row + 1,
					col: self.col,
				})
			}
			Direction::West if self.col > 0 => Some(Point {
				row: self.row,
				col: self.col - 1,
			}),
			Direction::North if self.row > 0 => Some(Point {
				row: self.row - 1,
				col: self.col,
			}),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction
{
	#[default]
	East,
	South,
	West,
	North,
}

impl Direction
{
	fn turn_left(self) -> Direction
	{
		match self
		{
			Direction::East => Direction::North,
			Direction::South => Direction::East,
			Direction::West => Direction::South,
			Direction::North => Direction::West,
		}
	}

	fn turn_right(self) -> Direction
	{
		match self
		{
			Direction::East => Direction::South,
			Direction::South => Direction::West,
			Direction::West => Direction::North,
			Direction::North => Direction::East,
		}
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Explorer
{
	at: Point,
	facing: Direction,
	strain: u8,
}

impl Explorer
{
	fn step_forward(self, num_rows: usize, num_cols: usize)
		-> Option<Explorer>
	{
		if self.strain == MAX_STRAIN
		{
			return None;
		}
		let at = self.at.step(self.facing, num_rows, num_cols)?;
		Some(Explorer {
			at,
			facing: self.facing,
			strain: self.strain + 1,
		})
	}

	fn turn_left(self, num_rows: usize, num_cols: usize) -> Option<Explorer>
	{
		let facing = self.facing.turn_left();
		let at = self.at.step(facing, num_rows, num_cols)?;
		Some(Explorer {
			at,
			facing,
			strain: 0,
		})
	}

	fn turn_right(self, num_rows: usize, num_cols: usize) -> Option<Explorer>
	{
		let facing = self.facing.turn_right();
		let at = self.at.step(facing, num_rows, num_cols)?;
		Some(Explorer {
			at,
			facing,
			strain: 0,
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Candidate
{
	f_score: usize,
	explorer: Explorer,
}

fn find_least_cost_with_a_star(
	cost_grid: &[[u8; GRID_SIZE]; GRID_SIZE],
	num_rows: usize,
	num_cols: usize,
	start: Explorer,
	target: Point,
) -> usize
{
	let manhattan_distance = |from: Point, to: Point| {
		let dr = (to.row as i32 - from.row as i32).abs() as usize;
		let dc = (to.col as i32 - from.col as i32).abs() as usize;
		dr + dc
	};
	let heuristic = |x: Explorer| manhattan_distance(x.at, target);
	let get_cost = |x: Explorer| {
		let r = x.at.row as usize;
		let c = x.at.col as usize;
		cost_grid[r][c]
	};
	// let mut upper_bound = manhattan_distance(start.at, target) * MAX_COST;

	let mut open_set = BinaryHeap::new();
	let mut g_score_map = HashMap::new();
	g_score_map.insert(start, 0);
	open_set.push(Reverse(Candidate {
		explorer: start,
		f_score: heuristic(start),
	}));

	while let Some(candidate) = open_set.pop()
	{
		let Reverse(Candidate {
			explorer,
			f_score: _,
		}) = candidate;
		let g_score = g_score_map[&explorer];
		let neighbors = [
			explorer.step_forward(num_rows, num_cols),
			explorer.turn_left(num_rows, num_cols),
			explorer.turn_right(num_rows, num_cols),
		];
		for next in neighbors.into_iter().filter_map(|x| x)
		{
			let cost = get_cost(next) as usize;
			let g_score = g_score + cost;
			if next.at == target
			{
				return g_score;
			}
			if g_score < g_score_map.get(&next).map_or(usize::MAX, |x| *x)
			{
				g_score_map.insert(next, g_score);
				open_set.push(Reverse(Candidate {
					explorer: next,
					f_score: g_score + heuristic(next),
				}));
			}
		}
	}
	unreachable!()
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
		assert_eq!(one(PROVIDED), 102);
	}

	#[test]
	fn size_of_structs()
	{
		assert_eq!(std::mem::size_of::<Explorer>(), 4);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

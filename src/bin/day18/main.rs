//!

use aoc2023::run;
use parse_display::{Display, FromStr};
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

const GRID_SIZE: usize = 1024;

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut grid = [[0u8; GRID_SIZE]; GRID_SIZE];
	let mut num_rows = 0;
	let mut num_cols = 0;
	let mut digger = Digger {
		row: GRID_SIZE / 2,
		col: GRID_SIZE / 2,
	};
	for line in input.lines().filter(|x| !x.is_empty())
	{
		let instruction = Instruction::from_str(line).unwrap();
		let Instruction {
			direction,
			distance,
			color: _,
		} = instruction;
		for _ in 0..distance
		{
			digger.step(direction);
			grid[digger.row][digger.col] = 1;
		}
		num_rows = num_rows.max(digger.row + 1);
		num_cols = num_cols.max(digger.col + 1);
	}
	debug_print_grid(&grid, num_rows, num_cols);
	for row in &mut grid
	{
		let mut state = 0;
		let mut last_wall_col = 0;
		for c in 0..num_cols
		{
			if row[c] > 0
			{
				if state == 0
				{
					state = 1;
					last_wall_col = c;
				}
				else if state == 2
				{
					for cc in (last_wall_col + 1)..c
					{
						row[cc] = 1;
					}
					state = 3;
				}
			}
			else
			{
				if state == 1
				{
					state = 2;
				}
				else if state == 3
				{
					state = 0;
				}
			}
		}
	}
	debug_print_grid(&grid, num_rows, num_cols);
	grid[0..num_rows]
		.iter()
		.map(|row| row[0..num_cols].iter().filter(|&&x| x > 0u8).count())
		.sum()
}

fn two(input: &str) -> usize
{
	input.len() * 0
}

#[derive(Debug, Clone, Copy, Default, Display, FromStr)]
enum Direction
{
	#[default]
	#[display("U")]
	Up,
	#[display("D")]
	Down,
	#[display("L")]
	Left,
	#[display("R")]
	Right,
}

#[derive(Debug, Clone, Default, Display, FromStr)]
#[display("{direction} {distance} (#{color})")]
struct Instruction
{
	direction: Direction,
	distance: i32,
	color: String,
}

#[derive(Debug)]
struct Digger
{
	row: usize,
	col: usize,
}

impl Digger
{
	fn step(&mut self, direction: Direction)
	{
		match direction
		{
			Direction::Up => self.row -= 1,
			Direction::Down => self.row += 1,
			Direction::Left => self.col -= 1,
			Direction::Right => self.col += 1,
		}
	}
}

fn debug_print_grid(
	grid: &[[u8; GRID_SIZE]; GRID_SIZE],
	num_rows: usize,
	num_cols: usize,
)
{
	println!();
	let mut buffer = String::new();
	for row in &grid[0..num_rows]
	{
		buffer.clear();
		for cell in &row[0..num_cols]
		{
			if *cell > 0
			{
				buffer.push('#');
			}
			else
			{
				buffer.push('.');
			}
		}
		println!("{buffer}");
	}
	println!();
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
		assert_eq!(one(PROVIDED), 62);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

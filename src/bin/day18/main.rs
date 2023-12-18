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
	let center = Point {
		row: GRID_SIZE / 2,
		col: GRID_SIZE / 2,
	};
	let mut digger = center;
	let mut topleft = center;
	let mut bottomright = center;
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
		topleft.row = topleft.row.min(digger.row);
		topleft.col = topleft.col.min(digger.col);
		bottomright.row = bottomright.row.max(digger.row);
		bottomright.col = bottomright.col.max(digger.col);
	}
	debug_print_grid(&grid, topleft, bottomright);

	grid[topleft.row - 1].fill(2);
	grid[bottomright.row + 1].fill(2);
	for r in topleft.row..=bottomright.row
	{
		grid[r][topleft.col - 1] = 2;
		grid[r][bottomright.col + 1] = 2;
	}
	let mut any_changes = true;
	while any_changes
	{
		any_changes = false;
		for r in topleft.row..=bottomright.row
		{
			for c in topleft.col..=bottomright.col
			{
				if grid[r][c] > 0
				{
					continue;
				}
				if grid[r - 1][c] == 2
					|| grid[r + 1][c] == 2
					|| grid[r][c - 1] == 2
					|| grid[r][c + 1] == 2
				{
					grid[r][c] = 2;
					any_changes = true;
				}
			}
		}
	}

	// for r in topleft.row..=bottomright.row
	// {
	// 	for c in topleft.col..=bottomright.col
	// 	{
	// 		if grid[r][c] == 0
	// 		{
	// 			grid[r][c] = 1;
	// 		}
	// 	}
	// }

	debug_print_grid(&grid, topleft, bottomright);
	grid[topleft.row..=bottomright.row]
		.iter()
		.map(|row| {
			row[topleft.col..=bottomright.col]
				.iter()
				.filter(|&&x| x < 2)
				.count()
		})
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

#[derive(Debug, Clone, Copy)]
struct Point
{
	row: usize,
	col: usize,
}

impl Point
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
	topleft: Point,
	bottomright: Point,
)
{
	println!();
	let mut buffer = String::new();
	for row in &grid[(topleft.row - 1)..(bottomright.row + 2)]
	{
		buffer.clear();
		for cell in &row[(topleft.col - 1)..(bottomright.col + 2)]
		{
			match *cell
			{
				0 => buffer.push('.'),
				1 => buffer.push('#'),
				2 => buffer.push('~'),
				_ => buffer.push('?'),
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

use std::{f32::consts::E, rc::Weak};

/**/

use aoc2023::run;
use smallvec::SmallVec;

const INPUT: &str = include_str!("input.txt");

const GRID_SIZE: usize = 128;

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut grid = [[0; GRID_SIZE]; GRID_SIZE];
	let lines = input.lines().filter(|x| !x.is_empty()).enumerate();
	let mut num_rows = 0;
	let mut num_cols = 0;
	for (r, line) in lines
	{
		num_rows = num_rows.max(r + 1);
		num_cols = line.as_bytes().len();
		for (c, symbol) in line.as_bytes().iter().enumerate()
		{
			grid[r][c] = Cell::parse(*symbol);
		}
	}

	if cfg!(debug_assertions)
	{
		print_contraptions(&grid, num_rows, num_cols);
	}

	let mut stack: SmallVec<[Head; 128]> = SmallVec::new();
	stack.push(Head {
		row: 0,
		col: 0,
		direction: EAST,
	});

	'withstack: while let Some(head) = stack.pop()
	{
		let mut head: Head = head;
		while is_empty(grid[head.row][head.col])
		{
			grid[head.row][head.col] |= head.direction;
			match head.direction
			{
				NORTH if head.row > 0 => head.row -= 1,
				SOUTH if head.row + 1 < num_rows => head.row += 1,
				WEST if head.col > 0 => head.col -= 1,
				EAST if head.col + 1 < num_cols => head.col += 1,
				_ => continue 'withstack,
			}
		}

		let cell = &mut grid[head.row][head.col];
		for direction in transmit(*cell, head.direction)
		{
			if *cell & direction == 0
			{
				*cell |= direction;
				let mut new_head = Head { direction, ..head };
				match direction
				{
					NORTH if new_head.row > 0 => new_head.row -= 1,
					SOUTH if new_head.row + 1 < num_rows => new_head.row += 1,
					WEST if new_head.col > 0 => new_head.col -= 1,
					EAST if new_head.col + 1 < num_cols => new_head.col += 1,
					_ => continue,
				}
				stack.push(new_head);
			}
		}
	}

	if cfg!(debug_assertions)
	{
		print_energized(&grid, num_rows, num_cols);
	}
	count_energized(&grid, num_rows, num_cols)
}

struct Head
{
	row: usize,
	col: usize,
	direction: u8,
}

const MIRROR_BLTR: u8 = 0b0010;
const MIRROR_TLBR: u8 = 0b0001;
const SPLITTER_H: u8 = 0b0100;
const SPLITTER_V: u8 = 0b1000;
const CONTRAPTION_BITS: u8 =
	MIRROR_BLTR | MIRROR_TLBR | SPLITTER_H | SPLITTER_V;

const NORTH: u8 = 0b1000_0000;
const SOUTH: u8 = 0b0100_0000;
const WEST: u8 = 0b0010_0000;
const EAST: u8 = 0b0001_0000;
const ENERGIZED_BITS: u8 = NORTH | SOUTH | WEST | EAST;

fn transmit(cell: u8, direction: u8) -> [u8; 2]
{
	match cell & CONTRAPTION_BITS
	{
		MIRROR_BLTR => match direction
		{
			NORTH => [EAST, EAST],
			SOUTH => [WEST, WEST],
			WEST => [SOUTH, SOUTH],
			EAST => [NORTH, NORTH],
			_ => unreachable!(),
		},
		MIRROR_TLBR => match direction
		{
			NORTH => [WEST, WEST],
			SOUTH => [EAST, EAST],
			WEST => [NORTH, NORTH],
			EAST => [SOUTH, SOUTH],
			_ => unreachable!(),
		},
		SPLITTER_H => match direction
		{
			NORTH => [WEST, EAST],
			SOUTH => [WEST, EAST],
			WEST => [WEST, WEST],
			EAST => [EAST, EAST],
			_ => unreachable!(),
		},
		SPLITTER_V => match direction
		{
			NORTH => [NORTH, NORTH],
			SOUTH => [SOUTH, SOUTH],
			WEST => [NORTH, SOUTH],
			EAST => [NORTH, SOUTH],
			_ => unreachable!(),
		},
		_ => unreachable!(),
	}
}

struct Cell;

impl Cell
{
	fn parse(symbol: u8) -> u8
	{
		match symbol
		{
			b'.' => 0,
			b'/' => MIRROR_BLTR,
			b'\\' => MIRROR_TLBR,
			b'-' => SPLITTER_H,
			b'|' => SPLITTER_V,
			_ => unreachable!(),
		}
	}
}

fn is_empty(cell: u8) -> bool
{
	cell & CONTRAPTION_BITS == 0
}

fn is_energized(cell: u8) -> bool
{
	cell & ENERGIZED_BITS != 0
}

fn print_contraptions(
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
			match cell & CONTRAPTION_BITS
			{
				0 => buffer.push('.'),
				MIRROR_BLTR => buffer.push('/'),
				MIRROR_TLBR => buffer.push('\\'),
				SPLITTER_H => buffer.push('-'),
				SPLITTER_V => buffer.push('|'),
				_ => unreachable!(),
			}
		}
		println!("{buffer}");
	}
	println!();
}

fn print_energized(
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
			if is_energized(*cell)
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

fn count_energized(
	grid: &[[u8; GRID_SIZE]; GRID_SIZE],
	num_rows: usize,
	num_cols: usize,
) -> usize
{
	grid[0..num_rows]
		.iter()
		.map(|row| {
			row[0..num_cols]
				.iter()
				.filter(|x| is_energized(**x))
				.count()
		})
		.sum()
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
		assert_eq!(one(PROVIDED), 46);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

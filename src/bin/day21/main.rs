//!

use aoc2023::run;

const INPUT: &str = include_str!("input.txt");

const CHUNK_SIZE: usize = 64;
const NUM_CHUNKS: usize = 3;
const GRID_SIZE: usize = NUM_CHUNKS * CHUNK_SIZE;

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> usize
{
	solve_one(input, 64)
}

fn two(input: &str) -> usize
{
	input.len() * 0
}

fn solve_one(input: &str, num_steps: usize) -> usize
{
	let mut walls = [[0u64; NUM_CHUNKS]; GRID_SIZE];
	let (r, c) = load_walls(&mut walls, input);
	count_accessible(&walls, r, c, num_steps)
}

fn load_walls(
	walls: &mut [[u64; NUM_CHUNKS]; GRID_SIZE],
	input: &str,
) -> (usize, usize)
{
	let mut starting_row = 0;
	let mut starting_col = 0;
	let mut max_row = 0;
	for (r, line) in input.lines().enumerate()
	{
		max_row = r;
		let width = line.as_bytes().len();
		for (i, chunk_xs) in line.as_bytes().chunks(CHUNK_SIZE).enumerate()
		{
			let mut chunk = 0u64;
			for (c, x) in chunk_xs.iter().enumerate()
			{
				match *x
				{
					b'#' => chunk |= 1 << c,
					b'S' =>
					{
						starting_row = r;
						starting_col = c;
					}
					b'.' => (),
					_ => unreachable!(),
				}
			}
			walls[r][i] = chunk;
		}
		set_bit(walls, r, width);
	}

	walls[max_row + 1] = [u64::MAX; NUM_CHUNKS];

	(starting_row, starting_col)
}

fn set_bit(grid: &mut [[u64; NUM_CHUNKS]; GRID_SIZE], r: usize, c: usize)
{
	grid[r][c / CHUNK_SIZE] |= 1 << (c % CHUNK_SIZE);
}

fn count_accessible(
	walls: &[[u64; NUM_CHUNKS]; GRID_SIZE],
	starting_row: usize,
	starting_col: usize,
	num_steps: usize,
) -> usize
{
	let mut ghosts = [[[0u64; NUM_CHUNKS]; GRID_SIZE]; 2];
	set_bit(&mut ghosts[0], starting_row, starting_col);

	let mut old = 0;
	let mut new = 1;
	for _ in 0..num_steps
	{
		if cfg!(debug_assertions)
		{
			debug_print_grid(walls, &ghosts[old]);
		}

		ghosts[new] = [[0u64; NUM_CHUNKS]; GRID_SIZE];
		for r in 0..GRID_SIZE
		{
			for i in 0..NUM_CHUNKS
			{
				// dbg!(new, r, i, format!("{:064b}", ghosts[old][r][i]));
				ghosts[new][r][i] |= ghosts[old][r][i] << 1;
				ghosts[new][r][i] |= ghosts[old][r][i] >> 1;
				if i > 0
				{
					ghosts[new][r][i] |= ghosts[old][r][i - 1] >> 63;
				}
				if i + 1 < NUM_CHUNKS
				{
					ghosts[new][r][i] |= ghosts[old][r][i + 1] << 63;
				}
				if r > 0
				{
					ghosts[new][r][i] |= ghosts[old][r - 1][i];
				}
				if r + 1 < GRID_SIZE
				{
					ghosts[new][r][i] |= ghosts[old][r + 1][i];
				}
				// dbg!(format!("{:064b}", ghosts[new][r][i]));
				ghosts[new][r][i] &= !walls[r][i];
				// dbg!(format!("{:064b}", ghosts[new][r][i]));
			}
		}
		std::mem::swap(&mut new, &mut old);
	}

	let num_ghosts: u32 = ghosts[old]
		.iter()
		.flat_map(|row| row.iter().map(|chunk| chunk.count_ones()))
		.sum();
	num_ghosts as usize
}

#[allow(unused)]
fn debug_print_grid(
	walls: &[[u64; NUM_CHUNKS]; GRID_SIZE],
	ghosts: &[[u64; NUM_CHUNKS]; GRID_SIZE],
)
{
	println!();
	let mut buffer = String::new();
	for r in 0..16
	{
		buffer.clear();
		for i in 0..1
		{
			for j in 0..16
			{
				let is_wall = walls[r][i] & (1 << j) != 0;
				let is_ghost = ghosts[r][i] & (1 << j) != 0;
				let symbol = match (is_wall, is_ghost)
				{
					(false, false) => '.',
					(false, true) => 'O',
					(true, false) => '#',
					(true, true) => '%',
				};
				buffer.push(symbol);
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
		assert_eq!(solve_one(PROVIDED, 6), 16);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

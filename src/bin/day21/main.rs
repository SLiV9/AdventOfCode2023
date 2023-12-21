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
	solve_one(input, 26501365)
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
			for (d, x) in chunk_xs.iter().enumerate()
			{
				match *x
				{
					b'#' => chunk |= 1 << d,
					b'S' =>
					{
						starting_row = r;
						starting_col = i * CHUNK_SIZE + d;
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
	dbg!(starting_row);
	dbg!(starting_col);
	dbg!(num_steps);

	let mut ghosts = [[[0u64; NUM_CHUNKS]; GRID_SIZE]; 2];
	set_bit(&mut ghosts[0], starting_row, starting_col);

	let mut old = 0;
	let mut new = 1;
	for i in 0..num_steps
	{
		if cfg!(debug_assertions)
		{
			dbg!(i);
			debug_print_grid(walls, &ghosts[old], &ghosts[new]);
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
	lingers: &[[u64; NUM_CHUNKS]; GRID_SIZE],
)
{
	println!();
	let mut buffer = String::new();
	for r in 0..GRID_SIZE
	{
		buffer.clear();
		for i in 0..NUM_CHUNKS
		{
			for j in 0..CHUNK_SIZE
			{
				let is_wall = walls[r][i] & (1 << j) != 0;
				let is_ghost = ghosts[r][i] & (1 << j) != 0;
				let symbol = match (is_wall, is_ghost)
				{
					(false, false) if lingers[r][i] & (1 << j) != 0 => 'o',
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
		assert_eq!(solve_one(PROVIDED, 6), 16);
		assert_eq!(solve_one(PROVIDED, 10), 50);
		assert_eq!(solve_one(PROVIDED, 50), 1594);
		assert_eq!(solve_one(PROVIDED, 100), 6536);
		assert_eq!(solve_one(PROVIDED, 500), 167004);
		assert_eq!(solve_one(PROVIDED, 1000), 668697);
		assert_eq!(solve_one(PROVIDED, 5000), 16733044);
	}
}

//
// The normal garden fills up after a certain number of iterations, and then oscillates between "even" and "odd"
// squares being reachable. Once I reach the same number of reachables twice, I can stop running that plot and
// just calculate whether the final state is even or odd.
// If I make a 5x5 grid of plots, I can update the plots one at a time (see below for 3x3) because each plot only
// inherits from two parents and never contributes back upstream (because there are no walls on the edges).
//
//  5 < 1 > 6
//  ^   ^   ^
//  2 < S > 3
//  v   v   v
//  7 < 4 > 8
//
// Hopefully the center 3x3 of plots reach oscillation before the outer ring of cells of the 5x5 is breached,
// so that I can collapse it into itself. Hmm but I have 16 plots in the outer ring and 8 in the middle ring.
// Maybe I should just start outputting an 11x11 of plots of the original in order to visually see the repetition.
//

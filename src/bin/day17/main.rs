//!

use aoc2023::run;

const INPUT: &str = include_str!("input.txt");

const GRID_SIZE: usize = 192;
const MAX_COST: u16 = 9;
const BUFFER_SIZE: usize = GRID_SIZE * 50;

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> u16
{
	let mut cost_grid = [[0; GRID_SIZE]; GRID_SIZE];
	let (num_rows, num_cols) = parse_grid(&mut cost_grid, input);

	find_least_cost::<0, 2>(&cost_grid, num_rows, num_cols)
}

fn two(input: &str) -> u16
{
	let mut cost_grid = [[0; GRID_SIZE]; GRID_SIZE];
	let (num_rows, num_cols) = parse_grid(&mut cost_grid, input);

	find_least_cost::<3, 6>(&cost_grid, num_rows, num_cols)
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
	fn get<T: Copy>(self, grid: &[[T; GRID_SIZE]; GRID_SIZE]) -> T
	{
		let r = self.row as usize;
		let c = self.col as usize;
		grid[r][c]
	}

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

const DIRECTION_LEN: usize = 2;

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
}

impl Explorer
{
	fn get<T: Copy>(
		self,
		grid: &[[[T; DIRECTION_LEN]; GRID_SIZE]; GRID_SIZE],
	) -> T
	{
		let r = self.at.row as usize;
		let c = self.at.col as usize;
		let d = (self.facing as usize) % 2;
		grid[r][c][d]
	}

	fn set<T: Copy>(
		self,
		grid: &mut [[[T; DIRECTION_LEN]; GRID_SIZE]; GRID_SIZE],
		value: T,
	)
	{
		let r = self.at.row as usize;
		let c = self.at.col as usize;
		let d = (self.facing as usize) % 2;
		grid[r][c][d] = value;
	}
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Candidate
{
	rank: u16,
	explorer: Explorer,
}

const MAX_INITIAL_SHORTLIST_LEN: usize = 16;

fn find_least_cost<const MIN_STRAIN: usize, const MAX_EXTRA_STRAIN: usize>(
	cost_grid: &[[u8; GRID_SIZE]; GRID_SIZE],
	num_rows: usize,
	num_cols: usize,
) -> u16
{
	let start = Point { row: 0, col: 0 };
	let target = Point {
		row: (num_rows - 1) as u8,
		col: (num_cols - 1) as u8,
	};
	let manhattan_distance = |from: Point, to: Point| {
		let dr = (to.row as i32 - from.row as i32).abs() as u16;
		let dc = (to.col as i32 - from.col as i32).abs() as u16;
		dr + dc
	};
	let upper_bound = manhattan_distance(start, target) * MAX_COST;
	let mut dist = [[[upper_bound; DIRECTION_LEN]; GRID_SIZE]; GRID_SIZE];
	let mut buffer = [Candidate::default(); BUFFER_SIZE];
	let mut shortlist_start = 0;
	let mut shortlist_end = 0;
	let mut shortlist_rank_threshold = 0;
	let mut len = 0;

	for facing in [Direction::East, Direction::South]
	{
		let explorer = Explorer { at: start, facing };
		explorer.set(&mut dist, 0);
		buffer[len] = Candidate { explorer, rank: 0 };
		len += 1;
	}

	while len > 0
	{
		debug_assert!(shortlist_start <= shortlist_end);
		debug_assert!(shortlist_end <= len);
		if shortlist_start == shortlist_end
		{
			if shortlist_end * 2 < len
			{
				buffer.copy_within(shortlist_end..len, 0);
			}
			else
			{
				buffer[0..len].rotate_left(shortlist_end);
			}
			len -= shortlist_end;
			if len == 0
			{
				break;
			}
			shortlist_start = 0;
			shortlist_end = len.min(MAX_INITIAL_SHORTLIST_LEN);
			buffer[0..len].select_nth_unstable(shortlist_end - 1);
			buffer[0..shortlist_end].sort_unstable();
			shortlist_rank_threshold = buffer[shortlist_end - 1].rank;
		}

		let Candidate {
			explorer: curr,
			rank,
		} = buffer[shortlist_start];
		shortlist_start += 1;

		let current_dist = curr.get(&dist);
		if rank != current_dist
		{
			continue;
		}

		'withfacings: for facing in
			[curr.facing.turn_left(), curr.facing.turn_right()]
		{
			let mut next = Explorer {
				at: curr.at,
				facing,
			};
			let mut cost = current_dist;
			for _ in 0..MIN_STRAIN
			{
				let Some(at) = next.at.step(facing, num_rows, num_cols)
				else
				{
					continue 'withfacings;
				};
				next.at = at;
				cost += at.get(cost_grid) as u16;
			}
			for _ in 0..=MAX_EXTRA_STRAIN
			{
				let Some(at) = next.at.step(facing, num_rows, num_cols)
				else
				{
					continue 'withfacings;
				};
				next.at = at;
				cost += at.get(cost_grid) as u16;

				if cost < next.get(&dist)
				{
					next.set(&mut dist, cost);

					let rank = cost;
					if rank < shortlist_rank_threshold
					{
						buffer[len] = buffer[shortlist_end];
						buffer[shortlist_end] = Candidate {
							explorer: next,
							rank,
						};
						let i = buffer[shortlist_start..shortlist_end]
							.partition_point(|&c| c.rank < rank);
						shortlist_end += 1;
						buffer[i..shortlist_end].rotate_right(1);
						len += 1;
					}
					else
					{
						buffer[len] = Candidate {
							explorer: next,
							rank,
						};
						len += 1;
					}
				}
			}
		}
	}

	dist[target.row as usize][target.col as usize]
		.into_iter()
		.min()
		.unwrap()
}

#[allow(unused)]
#[cfg(debug_assertions)]
fn print_dist(
	grid: &[[[u16; DIRECTION_LEN]; GRID_SIZE]; GRID_SIZE],
	num_rows: usize,
	num_cols: usize,
)
{
	println!();
	for row in &grid[0..num_rows]
	{
		for cell in &row[0..num_cols]
		{
			print!("H{:04}", cell[0]);
			print!("V{:04}", cell[1]);
			print!(" ");
		}
		println!();
	}
	println!();
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const PROVIDED2: &str = include_str!("provided2.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 102);
	}

	#[test]
	fn one_horizontal()
	{
		assert_eq!(one("0123"), 6);
	}

	#[test]
	fn one_vertical()
	{
		assert_eq!(one("07\n18\n19\n11"), 4);
	}

	#[test]
	fn size_of_structs()
	{
		assert!(std::mem::size_of::<Explorer>() <= 4);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 94);
	}

	#[test]
	fn two_provided2()
	{
		assert_eq!(two(PROVIDED2), 71);
	}
}

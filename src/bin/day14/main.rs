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
	let mut roll_point = [0u8; 128];
	let mut num_boulders_placed = 0;
	let mut total_load = 0;
	for (r, line) in input.lines().filter(|x| !x.is_empty()).enumerate()
	{
		for (c, x) in line.as_bytes().iter().enumerate()
		{
			match *x
			{
				b'#' => roll_point[c] = (r + 1) as u8,
				b'O' =>
				{
					num_boulders_placed += 1;
					if roll_point[c] < r as u8
					{
						total_load += r - roll_point[c] as usize;
						roll_point[c] += 1;
					}
					else
					{
						roll_point[c] = (r + 1) as u8;
					}
				}
				b'.' => (),
				_ => unreachable!(),
			}
		}
		total_load += num_boulders_placed;
	}
	total_load
}

#[derive(Debug)]
struct Grid
{
	data: [u128; 128],
}

impl Grid
{
	fn empty() -> Grid
	{
		Grid { data: [0u128; 128] }
	}

	fn set(&mut self, r: usize, c: usize)
	{
		self.data[r] |= 1 << c;
	}

	fn clear(&mut self, r: usize, c: usize)
	{
		self.data[r] &= !(1 << c);
	}

	fn get(&self, r: usize, c: usize) -> bool
	{
		(self.data[r] & (1 << c)) != 0
	}

	fn roll_north(&mut self, pillars: &Grid, num_rows: usize, num_cols: usize)
	{
		let mut roll_point = [0usize; 128];
		for r in 0..num_rows
		{
			for c in 0..num_cols
			{
				if pillars.get(r, c)
				{
					roll_point[c] = r + 1;
				}
				else if self.get(r, c)
				{
					if roll_point[c] < r
					{
						self.clear(r, c);
						self.set(roll_point[c], c);
						roll_point[c] += 1;
					}
					else
					{
						roll_point[c] = r + 1;
					}
				}
			}
		}
	}

	fn roll_south(&mut self, pillars: &Grid, num_rows: usize, num_cols: usize)
	{
		let mut roll_point = [num_rows - 1; 128];
		for r in (0..num_rows).rev()
		{
			for c in 0..num_cols
			{
				if pillars.get(r, c)
				{
					roll_point[c] = r.saturating_sub(1);
				}
				else if self.get(r, c)
				{
					if roll_point[c] > r
					{
						self.clear(r, c);
						self.set(roll_point[c], c);
						roll_point[c] -= 1;
					}
					else
					{
						roll_point[c] = r.saturating_sub(1)
					}
				}
			}
		}
	}

	fn roll_west(&mut self, pillars: &Grid, num_rows: usize, num_cols: usize)
	{
		let mut roll_point = [0usize; 128];
		for c in 0..num_cols
		{
			for r in 0..num_rows
			{
				if pillars.get(r, c)
				{
					roll_point[r] = c + 1;
				}
				else if self.get(r, c)
				{
					if roll_point[r] < c
					{
						self.clear(r, c);
						self.set(r, roll_point[r]);
						roll_point[r] += 1;
					}
					else
					{
						roll_point[r] = c + 1;
					}
				}
			}
		}
	}

	fn roll_east(&mut self, pillars: &Grid, num_rows: usize, num_cols: usize)
	{
		let mut roll_point = [num_cols - 1; 128];
		for c in (0..num_cols).rev()
		{
			for r in 0..num_rows
			{
				if pillars.get(r, c)
				{
					roll_point[r] = c.saturating_sub(1);
				}
				else if self.get(r, c)
				{
					if roll_point[r] > c
					{
						self.clear(r, c);
						self.set(r, roll_point[r]);
						roll_point[r] -= 1;
					}
					else
					{
						roll_point[r] = c.saturating_sub(1)
					}
				}
			}
		}
	}

	fn load_on_north_pillar(&self, num_rows: usize) -> u32
	{
		let mut num_boulders = 0;
		let mut total_load = 0;
		for row in &self.data[0..num_rows]
		{
			num_boulders += row.count_ones();
			total_load += num_boulders;
		}
		total_load
	}
}

const NUM_CYCLES: usize = 1000000000;
const NUM_ITERATIONS: usize = 4 * NUM_CYCLES;

fn two(input: &str) -> u32
{
	run_simulation(input, NUM_ITERATIONS)
}

fn run_simulation(input: &str, num_iterations: usize) -> u32
{
	let mut pillar_grid = Grid::empty();
	let mut boulder_grid = Grid::empty();
	let mut num_rows = 0;
	let mut num_cols = 0;
	for (r, line) in input.lines().filter(|x| !x.is_empty()).enumerate()
	{
		for (c, x) in line.as_bytes().iter().enumerate()
		{
			match *x
			{
				b'#' => pillar_grid.set(r, c),
				b'O' => boulder_grid.set(r, c),
				b'.' => (),
				_ => unreachable!(),
			}
		}
		num_rows = r + 1;
		num_cols = line.as_bytes().len();
	}

	let mut historic_loads: SmallVec<[u32; 1024]> = SmallVec::new();
	for t in 0..num_iterations
	{
		match t % 4
		{
			0 => boulder_grid.roll_north(&pillar_grid, num_rows, num_cols),
			1 => boulder_grid.roll_west(&pillar_grid, num_rows, num_cols),
			2 => boulder_grid.roll_south(&pillar_grid, num_rows, num_cols),
			3 => boulder_grid.roll_east(&pillar_grid, num_rows, num_cols),
			_ => unreachable!(),
		}

		if cfg!(test)
		{
			for r in 0..num_rows
			{
				for c in 0..num_cols
				{
					if pillar_grid.get(r, c)
					{
						print!("#");
					}
					else if boulder_grid.get(r, c)
					{
						print!("O");
					}
					else
					{
						print!(".");
					}
				}
				println!();
			}
		}

		if t % 4 == 3
		{
			// dbg!(t);
			let load = boulder_grid.load_on_north_pillar(num_rows);
			// dbg!(load);
			historic_loads.push(load);

			if t > 400
			{
				for k in 1..80
				{
					if (num_iterations - (t + 1)) % k != 0
					{
						continue;
					}
					let i = historic_loads.len() - 1;
					if (1..5)
						.map(|j| historic_loads[i - k * j])
						.all(|x| x == load)
					{
						return load;
					}
				}
			}
		}
	}
	match historic_loads.last()
	{
		Some(final_load) => *final_load,
		None => boulder_grid.load_on_north_pillar(num_rows),
	}
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
		assert_eq!(one(PROVIDED), 136);
	}

	#[test]
	fn one_simulation_provided()
	{
		assert_eq!(run_simulation(PROVIDED, 1), 136);
	}

	#[test]
	fn one_simulation_12_provided()
	{
		run_simulation(PROVIDED, 12);
	}

	#[test]
	fn test_boulder_drop()
	{
		assert_eq!(one(".\n.\n.\nO"), 4);
		assert_eq!(one(".\n#\n.\nO"), 2);
		assert_eq!(one(".\n.\nO\nO"), 7);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 64);
	}

	#[test]
	fn two_simulation_provided()
	{
		run_simulation(PROVIDED, 1000);
	}
}

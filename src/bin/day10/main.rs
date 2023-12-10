/**/

use aoc2023::run;

const INPUT: &str = include_str!("input.txt");

const GRID_SIZE: usize = 192;

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn parse_input(
	input: &str,
	grid: &mut [[u8; GRID_SIZE]; GRID_SIZE],
	start: &mut Point,
)
{
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
}

fn get_probes(
	start: Point,
	grid: &[[u8; GRID_SIZE]; GRID_SIZE],
) -> (Probe, Probe)
{
	let mut probes = [start.right(), start.up(), start.left(), start.down()]
		.into_iter()
		.filter(|point| point.adjacents(&grid).contains(&start))
		.map(|point| Probe {
			curr: point,
			prev: start,
		});
	let a = probes.next().unwrap();
	let b = probes.next().unwrap();
	(a, b)
}

fn one(input: &str) -> usize
{
	let mut grid = [[0u8; GRID_SIZE]; GRID_SIZE];
	let mut start = Point::default();
	parse_input(input, &mut grid, &mut start);

	let (mut a, mut b) = get_probes(start, &grid);
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

	fn pipe(&self, grid: &[[u8; GRID_SIZE]; GRID_SIZE]) -> u8
	{
		grid[self.row][self.col]
	}

	fn adjacents(&self, grid: &[[u8; GRID_SIZE]; GRID_SIZE]) -> [Point; 2]
	{
		self.adjacents_for_pipe(self.pipe(grid))
	}

	fn adjacents_for_pipe(&self, pipe: u8) -> [Point; 2]
	{
		match pipe
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
	fn step(&self, grid: &[[u8; GRID_SIZE]; GRID_SIZE]) -> Probe
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

#[derive(Debug, Clone, Copy, Default)]
struct Painter
{
	curr: Point,
	prev: Point,
	is_reversed: bool,
}

impl From<Probe> for Painter
{
	fn from(probe: Probe) -> Painter
	{
		Painter {
			curr: probe.curr,
			prev: probe.prev,
			is_reversed: false,
		}
	}
}

impl Painter
{
	fn step(
		&self,
		grid: &[[u8; GRID_SIZE]; GRID_SIZE],
		wall: &mut [[u8; GRID_SIZE]; GRID_SIZE],
		num_twos: &mut usize,
		num_threes: &mut usize,
	) -> Painter
	{
		let curr = self.curr;
		let pipe = curr.pipe(grid);

		match wall[curr.row][curr.col]
		{
			2 => *num_twos -= 1,
			3 => *num_threes -= 1,
			0 => (),
			6 => (),
			_ => unreachable!(),
		}
		wall[curr.row][curr.col] = 1;

		let exits = match pipe
		{
			b'|' => [curr.up(), curr.down()],
			b'-' => [curr.right(), curr.left()],
			b'L' => [curr.right(), curr.up()],
			b'J' => [curr.up(), curr.left()],
			b'7' => [curr.down(), curr.left()],
			b'F' => [curr.right(), curr.down()],
			_ => return *self,
		};

		let sides = match pipe
		{
			b'|' => [curr.left(), curr.right(), curr, curr],
			b'-' => [curr.up(), curr.down(), curr, curr],
			b'L' => [
				curr.down().left(),
				curr.up().right(),
				curr.down(),
				curr.left(),
			],
			b'J' => [
				curr.down().right(),
				curr.up().left(),
				curr.down(),
				curr.right(),
			],
			b'7' => [
				curr.up().right(),
				curr.down().left(),
				curr.up(),
				curr.right(),
			],
			b'F' => [
				curr.up().left(),
				curr.down().right(),
				curr.up(),
				curr.left(),
			],
			_ => return *self,
		};
		let mut colors = [2, 3];
		if (pipe == b'L' || pipe == b'7') && exits[0] == self.prev
		{
			colors.swap(0, 1);
		}
		else if pipe == b'J'
		{
			colors.swap(0, 1);
		}
		if self.is_reversed
		{
			colors.swap(0, 1);
		}
		for (i, side) in sides.into_iter().enumerate()
		{
			let color_offset = if i == 1 { 1 } else { 0 };
			let color = colors[color_offset];
			match wall[side.row][side.col]
			{
				0 =>
				{
					wall[side.row][side.col] = color;
					if color == 2
					{
						*num_twos += 1;
					}
					else
					{
						*num_threes += 1;
					}
				}
				1 => (),
				2 if color == 2 => (),
				3 if color == 3 => (),
				2 =>
				{
					*num_twos -= 1;
					wall[side.row][side.col] = 6;
				}
				3 =>
				{
					*num_threes -= 1;
					wall[side.row][side.col] = 6;
				}
				6 => (),
				_ => unreachable!(),
			}
		}

		let next = if exits[0] != self.prev
		{
			exits[0]
		}
		else
		{
			exits[1]
		};

		let should_flip = match pipe
		{
			b'|' => false,
			b'-' => false,
			b'L' => true,
			b'J' => false,
			b'7' => true,
			b'F' => false,
			_ => unreachable!(),
		};

		Painter {
			curr: next,
			prev: self.curr,
			is_reversed: self.is_reversed != should_flip,
		}
	}
}

fn two(input: &str) -> usize
{
	let mut grid = [[0u8; GRID_SIZE]; GRID_SIZE];
	let mut start = Point::default();
	parse_input(input, &mut grid, &mut start);

	let mut wall = [[0u8; GRID_SIZE]; GRID_SIZE];
	let mut num_twos = 0;
	let mut num_threes = 0;
	wall[start.row][start.col] = 1;

	let (a, b) = get_probes(start, &grid);
	let mut a = Painter::from(a);
	let mut b = Painter::from(b);
	{
		let dr = b.curr.row as i32 - a.curr.row as i32;
		let dc = b.curr.col as i32 - a.curr.col as i32;
		if (dr * dc) > 0
		{
			b.is_reversed = true;
		}
	}

	while a.curr != b.curr
	{
		a = a.step(&grid, &mut wall, &mut num_twos, &mut num_threes);
		b = b.step(&grid, &mut wall, &mut num_twos, &mut num_threes);
	}

	match wall[a.curr.row][a.curr.col]
	{
		2 => num_twos -= 1,
		3 => num_threes -= 1,
		0 => (),
		6 => (),
		_ => unreachable!(),
	}
	wall[a.curr.row][a.curr.col] = 1;

	// println!("{}", debug_grid_wall(&grid, &wall));

	let inside_color: u8 = if num_twos < num_threes { 2 } else { 3 };
	let mut num_inside = std::cmp::min(num_twos, num_threes);

	let mut old_num_inside = 0;
	while old_num_inside < num_inside
	{
		old_num_inside = num_inside;

		for r in 1..(GRID_SIZE - 1)
		{
			for c in 1..(GRID_SIZE - 1)
			{
				if wall[r][c] != 0
				{
					continue;
				}
				if wall[r - 1][c] == inside_color
					|| wall[r + 1][c] == inside_color
					|| wall[r][c - 1] == inside_color
					|| wall[r][c + 1] == inside_color
				{
					wall[r][c] = inside_color;
					num_inside += 1;
				}
				else if wall[r - 1][c] == 1
					&& wall[r + 1][c] == 1
					&& wall[r][c - 1] == 1
					&& wall[r][c + 1] == 1
				{
					unreachable!();
				}
			}
		}
	}

	// println!("{}", debug_grid_wall(&grid, &wall));

	num_inside
}

#[allow(unused)]
fn debug_grid_wall(
	grid: &[[u8; GRID_SIZE]; GRID_SIZE],
	wall: &[[u8; GRID_SIZE]; GRID_SIZE],
) -> String
{
	use std::fmt::Write;
	let mut output = String::new();
	writeln!(&mut output);
	for r in 0..GRID_SIZE
	{
		for c in 0..GRID_SIZE
		{
			let color = wall[r][c];
			let x = match color
			{
				0 => ' ',
				1 => char::from(grid[r][c]),
				2 => '2',
				3 => '3',
				_ => '?',
			};
			output.push(x);
		}
		writeln!(&mut output);
	}
	writeln!(&mut output);
	output
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
	const PROVIDED_TWO: &str = include_str!("provided_two.txt");
	const PROVIDED_TWO_N: &str = include_str!("provided_two_n.txt");
	const PROVIDED_TWO_TIGHT: &str = include_str!("provided_two_tight.txt");

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
		assert_eq!(two(PROVIDED), 1);
	}

	#[test]
	fn two_provided_alt()
	{
		assert_eq!(two(PROVIDED_ALT), 1);
	}

	#[test]
	fn two_provided_two_n()
	{
		assert_eq!(two(PROVIDED_TWO_N), 4);
	}

	#[test]
	fn two_provided_two_tight()
	{
		assert_eq!(two(PROVIDED_TWO_TIGHT), 4);
	}

	#[test]
	fn two_provided_two_full()
	{
		assert_eq!(two(PROVIDED_TWO), 10);
	}
}

//!

use aoc2023::{ring_buffer::RingBuffer, run};
use parse_display::{Display, FromStr};
use smallvec::SmallVec;

const INPUT: &str = include_str!("input.txt");

const GRID_SIZE: usize = 192;
const MAX_NUM_VERTICES: usize = 64;

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut grid = [[0; GRID_SIZE]; GRID_SIZE];
	let (num_rows, num_cols) = parse_grid(&mut grid, input);
	debug_print_grid(&grid, num_rows, num_cols);

	let mut graph = Graph::default();
	graph_grid(&mut grid, &mut graph, num_rows, num_cols);
	debug_print_grid(&grid, num_rows, num_cols);
	debug_print_graph(&graph);

	length_of_longest_route_part_one(&graph)
}

fn two(input: &str) -> usize
{
	let mut grid = [[0; GRID_SIZE]; GRID_SIZE];
	let (num_rows, num_cols) = parse_grid(&mut grid, input);
	let mut graph = Graph::default();
	graph_grid(&mut grid, &mut graph, num_rows, num_cols);
	length_of_longest_route_part_two(&graph)
}

fn parse_grid(
	grid: &mut [[u8; GRID_SIZE]; GRID_SIZE],
	input: &str,
) -> (usize, usize)
{
	let mut num_rows = 0;
	let mut num_cols = 0;
	for (r, line) in input.lines().enumerate()
	{
		let bytes = line.as_bytes();
		num_rows = r + 1;
		num_cols = bytes.len();
		for (c, x) in bytes.iter().enumerate()
		{
			grid[r][c] = *x;
		}
	}
	(num_rows, num_cols)
}

#[allow(unused)]
fn debug_print_grid(
	grid: &[[u8; GRID_SIZE]; GRID_SIZE],
	num_rows: usize,
	num_cols: usize,
)
{
	if !cfg!(debug_assertions)
	{
		return;
	}

	println!();
	for row in &grid[..num_rows]
	{
		println!("{}", std::str::from_utf8(&row[..num_cols]).unwrap());
	}
	println!();
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Display, FromStr)]
enum Direction
{
	#[default]
	#[display("^")]
	Up,
	#[display("v")]
	Down,
	#[display("<")]
	Left,
	#[display(">")]
	Right,
}

const ALL_DIRECTIONS: [Direction; 4] = [
	Direction::Right,
	Direction::Left,
	Direction::Down,
	Direction::Up,
];

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Point
{
	row: u16,
	col: u16,
}

impl Point
{
	fn step(&mut self, direction: Direction) -> &mut Self
	{
		match direction
		{
			Direction::Up => self.row -= 1,
			Direction::Down => self.row += 1,
			Direction::Left => self.col -= 1,
			Direction::Right => self.col += 1,
		}
		self
	}
}

#[derive(Debug, Clone)]
struct Vertex
{
	edges: SmallVec<[Edge; 4]>,
	point: Point,
	color: u8,
}

#[derive(Debug, Clone, Copy, Default)]
struct Edge
{
	length: u16,
	color: u8,
	vertex_offset: u8,
}

#[derive(Debug, Default)]
struct Graph
{
	vertices: SmallVec<[Vertex; MAX_NUM_VERTICES]>,
}

#[derive(Debug, Clone, Copy, Default)]
struct Entry
{
	from_vertex_offset: u8,
	from_vertex_edge_offset: u8,
	first_step: Point,
}

fn graph_grid(
	grid: &mut [[u8; GRID_SIZE]; GRID_SIZE],
	graph: &mut Graph,
	num_rows: usize,
	num_cols: usize,
)
{
	let start = Point { row: 0, col: 1 };
	add_vertex(grid, graph, start, b'S');

	let finish_line = (num_rows - 1) as u16;

	let mut queue: RingBuffer<[Entry; 128]> = RingBuffer::default();
	graph.vertices[0].edges.push(Edge::default());
	graph.vertices[0].edges[0].color = b'O';
	graph.vertices[0].edges[0].length = 1;
	queue.push(Entry {
		from_vertex_offset: 0,
		from_vertex_edge_offset: 0,
		first_step: Point { row: 1, col: 1 },
	});

	let mut next_vertex_color = b'1';
	let mut next_edge_color = b'a';

	while let Some(entry) = queue.pop_head()
	{
		let from_offset = entry.from_vertex_offset as usize;
		let edge_offset = entry.from_vertex_edge_offset as usize;
		let edge_color = graph.vertices[from_offset].edges[edge_offset].color;
		let mut cursor = entry.first_step;

		'find_intersection: loop
		{
			grid[cursor.row as usize][cursor.col as usize] = edge_color;

			let direction = ALL_DIRECTIONS.into_iter().find(|&direction| {
				let mut next = cursor;
				next.step(direction);
				let x = grid[next.row as usize][next.col as usize];
				x == b'.'
					|| (cursor != entry.first_step
						&& [b'<', b'>', b'^', b'v'].contains(&x))
			});
			cursor.step(direction.unwrap());
			graph.vertices[from_offset].edges[edge_offset].length += 1;

			if cursor.row == finish_line
			{
				debug_assert_eq!(cursor.col as usize, num_cols - 2);
				graph.vertices[from_offset].edges[edge_offset].vertex_offset =
					graph.vertices.len() as u8;
				add_vertex(grid, graph, cursor, b'F');
				return;
			}
			else if let Some(direction) = inspect_slope(grid, cursor)
			{
				cursor.step(direction);
				graph.vertices[from_offset].edges[edge_offset].length += 1;
				break 'find_intersection;
			}
		}

		if let Some(to_offset) = graph
			.vertices
			.iter()
			.position(|vertex| vertex.point == cursor)
		{
			graph.vertices[from_offset].edges[edge_offset].vertex_offset =
				to_offset as u8;
			continue;
		}

		let new_vertex_offset = graph.vertices.len() as u8;
		graph.vertices[from_offset].edges[edge_offset].vertex_offset =
			new_vertex_offset;
		add_vertex(grid, graph, cursor, next_vertex_color);
		next_vertex_color += 1;
		if next_vertex_color > b'9'
		{
			next_vertex_color = b'0';
		}

		let new_directions = ALL_DIRECTIONS.into_iter().filter(|&direction| {
			let mut next = cursor;
			next.step(direction);
			inspect_slope(grid, next) == Some(direction)
		});
		for (new_edge_offset, direction) in new_directions.enumerate()
		{
			let mut next = cursor;
			next.step(direction);
			next.step(direction);
			graph.vertices.last_mut().unwrap().edges.push(Edge {
				length: 2,
				color: next_edge_color,
				vertex_offset: 0,
			});
			next_edge_color += 1;
			if next_edge_color == b'u'
			{
				next_edge_color = b'a';
			}
			queue.push(Entry {
				from_vertex_offset: new_vertex_offset,
				from_vertex_edge_offset: new_edge_offset as u8,
				first_step: next,
			});
		}
	}
}

fn add_vertex(
	grid: &mut [[u8; GRID_SIZE]; GRID_SIZE],
	graph: &mut Graph,
	point: Point,
	color: u8,
)
{
	grid[point.row as usize][point.col as usize] = color;
	graph.vertices.push(Vertex {
		edges: SmallVec::default(),
		point,
		color,
	});
}

fn inspect_slope(
	grid: &[[u8; GRID_SIZE]; GRID_SIZE],
	cursor: Point,
) -> Option<Direction>
{
	match grid[cursor.row as usize][cursor.col as usize]
	{
		b'.' => None,
		b'^' => Some(Direction::Up),
		b'v' => Some(Direction::Down),
		b'<' => Some(Direction::Left),
		b'>' => Some(Direction::Right),
		b'#' => None,
		_ => unreachable!(),
	}
}

#[allow(unused)]
fn debug_print_graph(graph: &Graph)
{
	if !cfg!(debug_assertions)
	{
		return;
	}

	println!();
	for (i, vertex) in graph.vertices.iter().enumerate()
	{
		let color = char::from(vertex.color);
		let r = vertex.point.row;
		let c = vertex.point.col;
		println!("{i} ('{color}') @ r{r}c{c}");
		for edge in &vertex.edges
		{
			let color = char::from(edge.color);
			let to = edge.vertex_offset;
			let steps = edge.length;
			println!("----{color}---> {to} ({steps} steps)");
		}
	}
	println!();
}

fn length_of_longest_route_part_one(graph: &Graph) -> usize
{
	let n = graph.vertices.len();
	let mut grid = [[0; MAX_NUM_VERTICES]; MAX_NUM_VERTICES];
	for i in 0..n
	{
		for edge in &graph.vertices[i].edges
		{
			let j = edge.vertex_offset as usize;
			let len = edge.length as usize;
			if len > grid[i][j]
			{
				grid[i][j] = len;
			}
			for k in 0..n
			{
				// i -> j -> k
				if grid[j][k] > 0
				{
					if grid[i][k] < grid[i][j] + grid[j][k]
					{
						grid[i][k] = grid[i][j] + grid[j][k];
					}
				}
				// k -> i -> j
				if grid[k][i] > 0
				{
					if grid[k][j] < grid[k][i] + grid[i][j]
					{
						grid[k][j] = grid[k][i] + grid[i][j];
					}
				}
				// h -> i -> j -> k
				for h in 0..n
				{
					if grid[h][i] > 0 && grid[j][k] > 0
					{
						if grid[h][k] < grid[h][i] + grid[i][j] + grid[j][k]
						{
							grid[h][k] = grid[h][i] + grid[i][j] + grid[j][k];
						}
					}
				}
			}
		}
		debug_print_max_walk_grid(&grid, n);
	}

	grid[0][n - 1]
}

#[allow(unused)]
fn debug_print_max_walk_grid(
	grid: &[[usize; MAX_NUM_VERTICES]; MAX_NUM_VERTICES],
	n: usize,
)
{
	if !cfg!(debug_assertions)
	{
		return;
	}

	println!();
	for i in 0..n
	{
		for j in 0..n
		{
			print!("\t{}", grid[i][j]);
		}
		println!();
	}
	println!();
}

#[derive(Debug, Clone, Default)]
struct Route
{
	vertices: SmallVec<[u8; MAX_NUM_VERTICES]>,
	length: usize,
}

fn length_of_longest_route_part_two(graph: &Graph) -> usize
{
	let n = graph.vertices.len();
	let finish = (n - 1) as u8;
	let mut answer = 0;
	let mut stack: SmallVec<[Route; 1024]> = SmallVec::new();
	stack.push(Route::default());
	stack[0].vertices.push(0);
	'withstack: while let Some(route) = stack.pop()
	{
		dbg!(&route);
		dbg!(stack.len());

		let i = *route.vertices.last().unwrap() as usize;
		for edge in &graph.vertices[i].edges
		{
			if edge.vertex_offset == finish
			{
				let total_length = route.length + edge.length as usize;
				if total_length > answer
				{
					answer = total_length;
				}
			}
			else if route.vertices.len() + 1 == n
			{
				continue 'withstack;
			}
			else if !route.vertices.contains(&edge.vertex_offset)
			{
				let mut next = route.clone();
				next.vertices.push(edge.vertex_offset);
				next.length += edge.length as usize;
				stack.push(next);
			}
		}
		for j in 0..n
		{
			if route.vertices.contains(&(j as u8))
			{
				continue;
			}
			for edge in &graph.vertices[j].edges
			{
				if edge.vertex_offset as usize == i
				{
					let mut next = route.clone();
					next.vertices.push(j as u8);
					next.length += edge.length as usize;
					stack.push(next);
				}
			}
		}
	}
	answer
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
		assert_eq!(one(PROVIDED), 94);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 154);
	}
}

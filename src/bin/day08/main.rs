/**/

use aoc2023::run;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> usize
{
	let (instructions, rest) = input.split_once('\n').unwrap();
	let graph = Graph::from_input(rest);
	loop
	{
		todo!()
	}
}

#[derive(Debug)]
struct Graph
{
	names: [[u8; 3]; 1024],
	lefts: [usize; 1024],
	rights: [usize; 1024],
}

impl Graph
{
	fn from_input(input: &str) -> Graph
	{
		let lines = input.lines().filter(|x| !x.is_empty());
		let mut graph = Graph {
			names: [[0; 3]; 1024],
			lefts: [0; 1024],
			rights: [0; 1024],
		};
		for (i, line) in lines.enumerate()
		{
			let line = line.as_bytes();
			graph.names[i] = line[0..3].try_into().unwrap();
			graph.lefts[i] = graph.find_fast(&line[8..11], i);
			graph.rights[i] = graph.find_fast(&line[13..16], i);
		}
		graph
	}

	fn find_fast(&self, name: &[u8], len: usize) -> usize
	{
		self.names.iter().take(len).position(|x| x == name).unwrap()
	}
}

fn two(input: &str) -> i32
{
	input.len() as i32 * 0
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const PROVIDED_SHUFFLED: &str = include_str!("provided_shuffled.txt");
	const PROVIDED2: &str = include_str!("provided2.txt");

	#[test]
	fn one_provided()
	{
		assert_eq!(one(PROVIDED), 2);
	}

	#[test]
	fn one_provided_shuffled()
	{
		assert_eq!(one(PROVIDED_SHUFFLED), 2);
	}

	#[test]
	fn one_provided2()
	{
		assert_eq!(one(PROVIDED2), 6);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

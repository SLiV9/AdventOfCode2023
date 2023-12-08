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
	let mut i = 0;
	let mut instructions = instructions.as_bytes().iter().cycle();
	let mut num_steps_taken = 0;
	while i < NUM_NAMES - 1
	{
		if instructions.next().unwrap() == &b'L'
		{
			i = graph.lefts[i] as usize;
		}
		else
		{
			i = graph.rights[i] as usize;
		}
		num_steps_taken += 1;
	}
	num_steps_taken
}

const NUM_NAMES: usize = 26 * 26 * 26;

fn encode_name(name: &[u8]) -> u16
{
	name.iter()
		.rev()
		.fold(0, |x, c| x * 26 + u16::from(c - b'A'))
}

#[derive(Debug)]
struct Graph
{
	lefts: [u16; NUM_NAMES],
	rights: [u16; NUM_NAMES],
}

impl Graph
{
	fn from_input(input: &str) -> Graph
	{
		let lines = input.lines().filter(|x| !x.is_empty());
		let mut graph = Graph {
			lefts: [0; NUM_NAMES],
			rights: [0; NUM_NAMES],
		};
		for i in 0..MAX_NUM_GHOSTS
		{
			graph.lefts[i] = i as u16;
			graph.rights[i] = i as u16;
		}
		for line in lines
		{
			let line = line.as_bytes();
			let cur = encode_name(&line[0..3]);
			let left = encode_name(&line[7..10]);
			let right = encode_name(&line[12..15]);
			let i = cur as usize;
			graph.lefts[i] = left;
			graph.rights[i] = right;
		}
		graph
	}
}

const MAX_NUM_GHOSTS: usize = 26 * 26;

fn two(input: &str) -> usize
{
	let (instructions, rest) = input.split_once('\n').unwrap();
	let graph = Graph::from_input(rest);
	let mut instructions = instructions.as_bytes().iter().cycle();
	let mut num_steps_taken = 0;

	let first_end = (NUM_NAMES - MAX_NUM_GHOSTS) as u16;
	let is_end = |x: u16| x >= first_end;

	dbg!(graph.lefts.iter().filter(|&&x| is_end(x)).count());
	dbg!(graph.rights.iter().filter(|&&x| is_end(x)).count());

	let mut ixs = [0u16; MAX_NUM_GHOSTS];
	let num_ghosts = {
		let mut num_ghosts = 0;
		for i in 0..MAX_NUM_GHOSTS
		{
			if graph.lefts[i] != i as u16 || graph.rights[i] != i as u16
			{
				ixs[num_ghosts] = i as u16;
				num_ghosts += 1;
			}
		}
		num_ghosts
	};

	for _ in 0..u64::from(u32::MAX)
	{
		if ixs[0..num_ghosts].iter().cloned().all(is_end)
		{
			break;
		}

		if instructions.next().unwrap() == &b'L'
		{
			for i in 0..num_ghosts
			{
				ixs[i] = graph.lefts[ixs[i] as usize];
			}
		}
		else
		{
			for i in 0..num_ghosts
			{
				ixs[i] = graph.rights[ixs[i] as usize];
			}
		}
		num_steps_taken += 1;

		if num_steps_taken % usize::from(u16::MAX) == 0
		{
			dbg!(num_steps_taken);
		}
	}
	num_steps_taken
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED: &str = include_str!("provided.txt");
	const PROVIDED_SHUFFLED: &str = include_str!("provided_shuffled.txt");
	const PROVIDED2: &str = include_str!("provided2.txt");
	const PROVIDED_TWO: &str = include_str!("provided_two.txt");

	#[test]
	fn sanity()
	{
		assert!(NUM_NAMES < usize::from(u16::MAX));
	}

	#[test]
	fn test_encode_name()
	{
		assert_eq!(encode_name("AAA".as_bytes()), 0);
		assert_eq!(usize::from(encode_name("ZZZ".as_bytes())), NUM_NAMES - 1);
		assert_eq!(
			usize::from(encode_name("ZZA".as_bytes())),
			MAX_NUM_GHOSTS - 1
		);
		assert_eq!(
			usize::from(encode_name("AAZ".as_bytes())),
			NUM_NAMES - MAX_NUM_GHOSTS
		);
	}

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
	fn two_provided_two()
	{
		assert_eq!(two(PROVIDED_TWO), 6);
	}
}

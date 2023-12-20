//!

use aoc2023::ring_buffer::RingBuffer;
use aoc2023::run;
use smallvec::SmallVec;

const INPUT: &str = include_str!("input.txt");

const MAX_NUM_NODES: usize = 64;
const MAX_NUM_SUCCESSORS: usize = 16;
const MAX_MEMORY: usize = 128;

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut nodes: [Node; MAX_NUM_NODES] =
		std::array::from_fn(|_i| Node::default());
	let mut names = [""; MAX_NUM_NODES];
	load_nodes(&mut nodes, &mut names, input);
	let mut memory = 0;
	let mut num_lo = 0;
	let mut num_hi = 0;
	for _ in 0..1000
	{
		let [l, h] = press_button(&nodes, &mut memory);
		num_lo += l;
		num_hi += h;

		if cfg!(debug_assertions)
		{
			// dbg!([l, h], memory, format!("{memory:032b}"));
		}
	}
	num_lo * num_hi
}

fn two(input: &str) -> usize
{
	input.len() * 0
}

fn insert<'a: 'b, 'b>(
	name: &'a str,
	names: &'b mut [&'a str],
	num_names: &'b mut usize,
) -> usize
{
	let i = *num_names;
	names[i] = name;
	*num_names += 1;
	i
}

fn find_or_insert<'a: 'b, 'b>(
	name: &'a str,
	names: &'b mut [&'a str],
	num_names: &'b mut usize,
) -> usize
{
	match names[0..*num_names].iter().position(|&n| n == name)
	{
		Some(i) => i,
		None => insert(name, names, num_names),
	}
}

fn load_nodes<'a: 'b, 'b>(
	nodes: &'b mut [Node; MAX_NUM_NODES],
	names: &'b mut [&'a str; MAX_NUM_NODES],
	input: &'a str,
)
{
	let mut lines = input.lines().filter(|x| !x.is_empty());

	let mut num_names = 0;

	find_or_insert("button", names, &mut num_names);
	find_or_insert("broadcaster", names, &mut num_names);
	nodes[0].kind = NodeKind::Button;
	nodes[0].successors.push(1);
	nodes[1].predecessors.push(0);

	while let Some(line) = lines.next()
	{
		if line.is_empty()
		{
			break;
		}
		let node_kind = match line.as_bytes()[0]
		{
			b'%' => NodeKind::FlipFlop,
			b'&' => NodeKind::Conjunction,
			_ => NodeKind::Broadcast,
		};
		let line = line.trim_start_matches(|x: char| ['%', '&'].contains(&x));
		let (name, rest) = line.split_once(" -> ").unwrap();
		let i = find_or_insert(name, names, &mut num_names);
		nodes[i].kind = node_kind;
		for next in rest.split(", ")
		{
			let j = find_or_insert(next, names, &mut num_names);
			nodes[i].successors.push(j as u8);
			debug_assert!(nodes[i].successors.len() <= MAX_NUM_SUCCESSORS);
			nodes[j].predecessors.push(i as u8);
			debug_assert!(nodes[j].predecessors.len() <= MAX_NUM_SUCCESSORS);
		}
	}

	if cfg!(debug_assertions)
	{
		if let Some(i) = names[0..num_names]
			.iter()
			.position(|&name| name == "output")
		{
			nodes[i].kind = NodeKind::Output;
		}
	}

	let mut memory_width = 0;
	for i in 0..num_names
	{
		let node = &mut nodes[i];
		node.memory_shr = memory_width;
		match node.kind
		{
			NodeKind::Broadcast => (),
			NodeKind::Button => (),
			NodeKind::Output =>
			{
				memory_width += 1;
			}
			NodeKind::FlipFlop =>
			{
				memory_width += 1;
			}
			NodeKind::Conjunction =>
			{
				memory_width += node.predecessors.len() as u8;
				debug_assert!(memory_width as usize <= MAX_MEMORY);
			}
		}
	}

	if cfg!(debug_assertions)
	{
		dbg!(num_names);
		// for (name, node) in
		// 	names[0..num_names].iter().zip(nodes[0..num_names].iter())
		// {
		// 	dbg!(name, node);
		// }
		dbg!(memory_width);
	}
}

const SIGNAL_MASK: u16 = 0b01111111;

fn press_button(nodes: &[Node; MAX_NUM_NODES], memory: &mut u128)
	-> [usize; 2]
{
	let mut signals: RingBuffer<[u16; MAX_NUM_NODES]> = RingBuffer::default();
	signals.push(1);
	let mut nums_lo_hi = [1, 0];

	while let Some(signal) = signals.pop_head()
	{
		let input = (signal >> 15) as u8;
		let source = ((signal >> 8) & SIGNAL_MASK) as u8;
		let i = (signal & SIGNAL_MASK) as usize;
		let node = &nodes[i];
		// dbg!(signal, input, source, i, node);
		let Some(output) = node.process(input, source, memory)
		else
		{
			continue;
		};
		nums_lo_hi[output as usize] += node.successors.len();
		let source = i as u16;
		let high_bits = (output as u16) << 15 | (source << 8);
		for j in &node.successors
		{
			let recipient = *j as u16;
			signals.push(high_bits | recipient);
		}
	}

	nums_lo_hi
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum NodeKind
{
	#[default]
	Broadcast,
	Button,
	FlipFlop,
	Conjunction,
	Output,
}

#[derive(Debug, Clone, Default)]
struct Node
{
	predecessors: SmallVec<[u8; MAX_NUM_SUCCESSORS]>,
	successors: SmallVec<[u8; MAX_NUM_SUCCESSORS]>,
	kind: NodeKind,
	memory_shr: u8,
}

impl Node
{
	fn process(&self, input: u8, source: u8, memory: &mut u128) -> Option<u8>
	{
		match self.kind
		{
			NodeKind::Broadcast => Some(input),
			NodeKind::FlipFlop if input > 0 => None,
			NodeKind::FlipFlop =>
			{
				*memory ^= 1 << self.memory_shr;
				let memory_bit = (*memory >> self.memory_shr) & 1;
				Some(memory_bit as u8)
			}
			NodeKind::Conjunction =>
			{
				let j = self
					.predecessors
					.iter()
					.position(|&j| j == source)
					.unwrap();
				let n = self.memory_shr + (j as u8);
				*memory &= !(1 << n);
				*memory |= (input as u128) << n;
				let mask = ((1 << self.predecessors.len()) - 1) as u8;
				let memory_bits = ((*memory >> self.memory_shr) as u8) & mask;
				let is_full = memory_bits == mask;
				Some(!is_full as u8)
			}
			NodeKind::Output =>
			{
				*memory &= !(1 << self.memory_shr);
				*memory |= (input as u128) << self.memory_shr;
				None
			}
			NodeKind::Button => None,
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	const PROVIDED1: &str = include_str!("provided1.txt");
	const PROVIDED2: &str = include_str!("provided2.txt");

	#[test]
	fn one_provided1()
	{
		assert_eq!(one(PROVIDED1), 32000000);
	}

	#[test]
	fn one_provided2()
	{
		assert_eq!(one(PROVIDED2), 11687500);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED1), 0);
	}
}

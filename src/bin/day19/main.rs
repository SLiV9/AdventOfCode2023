//!

use aoc2023::run;
use parse_display::{Display, FromStr};
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

const MAX_NUM_NODES: usize = 2 * 1024;
const INNER_INDEX: usize = MAX_NUM_NODES / 2;
const ACCEPTANCE_INDEX: usize = MAX_NUM_NODES;
const REJECTANCE_INDEX: usize = ACCEPTANCE_INDEX + 1;

const MIN_VALUE: u16 = 1;
const MAX_VALUE: u16 = 4000;

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> u32
{
	let mut lines = input.lines();
	let mut names = [""; MAX_NUM_NODES / 2];
	let mut nodes = [Node::default(); MAX_NUM_NODES];
	load_nodes(&mut nodes, &mut names, &mut lines);
	solve_parts(&nodes, lines)
}

fn two(input: &str) -> usize
{
	let lines = input.lines();
	let mut names = [""; MAX_NUM_NODES / 2];
	let mut nodes = [Node::default(); MAX_NUM_NODES];
	load_nodes(&mut nodes, &mut names, lines);
	solve_everything(&nodes)
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
	if name == "A"
	{
		ACCEPTANCE_INDEX
	}
	else if name == "R"
	{
		REJECTANCE_INDEX
	}
	else
	{
		match names[0..*num_names].iter().position(|&n| n == name)
		{
			Some(i) => i,
			None => insert(name, names, num_names),
		}
	}
}

fn load_nodes<'a: 'b, 'b>(
	nodes: &'b mut [Node; MAX_NUM_NODES],
	names: &'b mut [&'a str; MAX_NUM_NODES / 2],
	mut lines: impl Iterator<Item = &'a str>,
)
{
	let mut num_names = 0;
	let mut inner_ix = INNER_INDEX;

	find_or_insert("in", names, &mut num_names);

	while let Some(line) = lines.next()
	{
		if line.is_empty()
		{
			break;
		}
		let (name, rest) = line.split_once('{').unwrap();
		let rest = rest.trim_end_matches('}');
		let mut i = find_or_insert(name, names, &mut num_names);
		let (rules, last) = rest.rsplit_once(',').unwrap();
		let last_ix = find_or_insert(last, names, &mut num_names);
		let mut rules = rules.split(',').peekable();
		while let Some(rule) = rules.next()
		{
			let (condition, then_name) = rule.split_once(':').unwrap();
			let cond = Condition::from_str(condition).unwrap();
			let then_ix = find_or_insert(then_name, names, &mut num_names);
			let else_ix = if rules.peek().is_some()
			{
				let else_ix = inner_ix;
				inner_ix += 1;
				else_ix
			}
			else
			{
				last_ix
			};
			let (less, more) = cond.less_and_more(then_ix, else_ix);
			nodes[i] = Node {
				shr: cond.shr(),
				threshold: cond.threshold(),
				less: less as u16,
				more: more as u16,
			};
			i = else_ix;
		}
	}

	let num_inner = inner_ix - INNER_INDEX;

	if cfg!(debug_assertions)
	{
		dbg!(num_names);
		dbg!(num_inner);
	}

	let mut num_nodes = num_names + num_inner;
	let mut selves = [0; MAX_NUM_NODES + 2];
	for i in (0..num_names).chain(INNER_INDEX..inner_ix)
	{
		selves[i] = i as u16;
	}
	selves[ACCEPTANCE_INDEX] = ACCEPTANCE_INDEX as u16;
	selves[REJECTANCE_INDEX] = REJECTANCE_INDEX as u16;

	let mut any_change = true;
	while any_change
	{
		any_change = false;

		for i in (0..num_names).chain(INNER_INDEX..inner_ix).rev()
		{
			if selves[i] as usize != i
			{
				continue;
			}
			let node = &mut nodes[i];
			node.less = selves[node.less as usize];
			node.more = selves[node.more as usize];
			if node.less == node.more
			{
				selves[i] = node.less;
				num_nodes -= 1;
				any_change = true;
			}
		}
	}

	if cfg!(debug_assertions)
	{
		dbg!(num_nodes);
	}
}

fn solve_parts<'a>(
	nodes: &[Node; MAX_NUM_NODES],
	mut lines: impl Iterator<Item = &'a str>,
) -> u32
{
	let mut answer = 0;
	while let Some(line) = lines.next()
	{
		if line.is_empty()
		{
			break;
		}
		let part = Part::from_str(line).unwrap();
		let part_as_u64 = part.as_u64();

		let mut i = 0;
		while i < MAX_NUM_NODES
		{
			i = nodes[i].step(part_as_u64);
		}
		if i == ACCEPTANCE_INDEX
		{
			answer += part.xmas_sum()
		}
	}
	answer
}

fn solve_everything(nodes: &[Node; MAX_NUM_NODES]) -> usize
{
	0
}

#[derive(Debug, Clone, Copy, Default)]
struct Node
{
	shr: u16,
	threshold: u16,
	less: u16,
	more: u16,
}

impl Node
{
	fn step(self, part: u64) -> usize
	{
		let value = ((part >> self.shr) & 0xFFFF) as u16;
		if value < self.threshold
		{
			self.less as usize
		}
		else
		{
			self.more as usize
		}
	}
}

#[derive(Debug, Clone, Copy, Display, FromStr)]
enum ComparisonOperator
{
	#[display("<")]
	Less,
	#[display(">")]
	More,
}

#[derive(Debug, Clone, Copy, Display, FromStr)]
#[display(style = "lowercase")]
enum Field
{
	X,
	M,
	A,
	S,
}

#[derive(Debug, Clone, Copy, Display)]
#[display("{field}{op}{value}")]
struct Condition
{
	field: Field,
	op: ComparisonOperator,
	value: u16,
}

impl FromStr for Condition
{
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		let (head, value) = s.split_at(2);
		let (field, op) = head.split_at(1);
		Ok(Condition {
			field: field.parse().unwrap(),
			op: op.parse().unwrap(),
			value: value.parse().unwrap(),
		})
	}
}

impl Condition
{
	fn shr(&self) -> u16
	{
		match self.field
		{
			Field::X => 48,
			Field::M => 32,
			Field::A => 16,
			Field::S => 0,
		}
	}

	fn threshold(&self) -> u16
	{
		match self.op
		{
			ComparisonOperator::Less => self.value,
			ComparisonOperator::More => self.value + 1,
		}
	}

	fn less_and_more(&self, then_ix: usize, else_ix: usize) -> (usize, usize)
	{
		match self.op
		{
			ComparisonOperator::Less => (then_ix, else_ix),
			ComparisonOperator::More => (else_ix, then_ix),
		}
	}
}

#[derive(Debug, Clone, Copy, Default, Display, FromStr)]
#[display("{{x={x},m={m},a={a},s={s}}}")]
struct Part
{
	x: u16,
	m: u16,
	a: u16,
	s: u16,
}

impl Part
{
	fn as_u64(self) -> u64
	{
		u64::from(self.x) << 48
			| u64::from(self.m) << 32
			| u64::from(self.a) << 16
			| u64::from(self.s)
	}

	fn xmas_sum(self) -> u32
	{
		u32::from(self.x)
			+ u32::from(self.m)
			+ u32::from(self.a)
			+ u32::from(self.s)
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
		assert_eq!(one(PROVIDED), 19114);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 167409079868000);
	}
}

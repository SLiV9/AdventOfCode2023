//!

use aoc2023::run;
use parse_display::{Display, FromStr};
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

const MAX_NUM_NODES: usize = 8 * 1024;
const ACCEPTANCE_INDEX: usize = MAX_NUM_NODES;
const REJECTANCE_INDEX: usize = ACCEPTANCE_INDEX + 1;

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> u32
{
	let mut lines = input.lines();
	let mut names = [""; MAX_NUM_NODES];
	let mut nodes = [Node::default(); MAX_NUM_NODES];
	let mut len = 0;

	let insert = |x, xs: &mut [_], n: &mut usize| {
		let i = *n;
		xs[i] = x;
		*n += 1;
		i
	};
	let find_or_insert = |x, xs: &mut [_], n: &mut usize| {
		if x == "A"
		{
			ACCEPTANCE_INDEX
		}
		else if x == "R"
		{
			REJECTANCE_INDEX
		}
		else
		{
			match xs.iter().position(|&n| n == x)
			{
				Some(i) => i,
				None => insert(x, xs, n),
			}
		}
	};

	let start = find_or_insert("in", &mut names, &mut len);

	while let Some(line) = lines.next()
	{
		if line.is_empty()
		{
			break;
		}
		let (name, rest) = line.split_once('{').unwrap();
		let rest = rest.trim_end_matches('}');
		let mut i = find_or_insert(name, &mut names, &mut len);
		let (rules, last) = rest.rsplit_once(',').unwrap();
		let last_ix = find_or_insert(last, &mut names, &mut len);
		let mut rules = rules.split(',').peekable();
		while let Some(rule) = rules.next()
		{
			let (condition, then_name) = rule.split_once(':').unwrap();
			let cond = Condition::from_str(condition).unwrap();
			let then_ix = find_or_insert(then_name, &mut names, &mut len);
			let else_ix = if rules.peek().is_some()
			{
				insert("", &mut names, &mut len)
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

	let mut answer = 0;
	while let Some(line) = lines.next()
	{
		if line.is_empty()
		{
			break;
		}
		let part = Part::from_str(line).unwrap();
		let part_as_u64 = part.as_u64();

		let mut i = start;
		while i < len
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

fn two(input: &str) -> u32
{
	input.len() as u32 * 0
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
		assert_eq!(two(PROVIDED), 0);
	}
}

//!

use aoc2023::{ring_buffer::RingBuffer, run};
use parse_display::{Display, FromStr};
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> usize
{
	let mut bricks: Vec<Brick> = input
		.lines()
		.filter(|x| !x.is_empty())
		.map(|line| Brick::from_str(line).unwrap())
		.collect();
	bricks.sort_unstable();
	drop_bricks(&mut bricks);
	count_candidates(&bricks)
}

fn two(input: &str) -> usize
{
	input.len() * 0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, FromStr)]
#[display("{x},{y},{z}")]
struct Point
{
	z: u16,
	x: u16,
	y: u16,
}

fn do_ranges_intersect(a0: u16, a1: u16, b0: u16, b1: u16) -> bool
{
	(a0..=a1).contains(&b0)
		|| (a0..=b1).contains(&b1)
		|| (b0..=b1).contains(&a0)
		|| (b0..=b1).contains(&a1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, FromStr)]
#[display("{start}~{end}")]
struct Brick
{
	start: Point,
	end: Point,
}

impl Brick
{
	fn is_pillar(&self) -> bool
	{
		self.start.z != self.end.z
	}

	fn overlaps(&self, other: &Brick) -> bool
	{
		do_ranges_intersect(
			self.start.x,
			self.end.x,
			other.start.x,
			other.end.x,
		) && do_ranges_intersect(
			self.start.y,
			self.end.y,
			other.start.y,
			other.end.y,
		)
	}

	fn drop(&mut self, settled_bricks: &[Brick])
	{
		let floor = settled_bricks
			.iter()
			.rev()
			.take_while(|other| other.end.z < self.start.z)
			.find(|other| self.overlaps(other))
			.map_or(0, |other| other.end.z);
		let dz = self.start.z - floor - 1;
		self.start.z -= dz;
		self.end.z -= dz;
	}
}

fn drop_bricks(bricks: &mut [Brick])
{
	for i in 0..bricks.len()
	{
		let (settled, floating) = bricks.split_at_mut(i);
		floating[0].drop(settled);
	}
}

fn count_candidates(bricks: &[Brick]) -> usize
{
	let len = bricks.len();

	let mut num_candidates = 0;
	let mut only_supported_by: [usize; 128];
	let mut pillar_indices: RingBuffer<[usize; 32]> = RingBuffer::default();
	let mut head = 0;
	let mut mid = 0;
	let mut tail;

	let mut floor = 1;
	while mid < len && bricks[mid].start.z == floor
	{
		mid += 1;
	}

	while mid < len
	{
		debug_assert!(head < mid);
		debug_assert!(mid <= head + 128);
		only_supported_by = [0; 128];
		while pillar_indices
			.head()
			.map_or(false, |i| bricks[*i].end.z < floor)
		{
			if cfg!(debug_assertions)
			{
				dbg!(pillar_indices.head().unwrap());
			}
			num_candidates += 1;
			pillar_indices.drop_head();
		}

		floor = bricks[mid].start.z;

		tail = mid;
		debug_assert!(tail <= mid + 128);
		while tail < len && bricks[tail].start.z == floor
		{
			let mut supporters =
				(head..mid).filter(|&i| bricks[tail].overlaps(&bricks[i]));
			let main_supporter = supporters.next();
			let pillar = pillar_indices
				.remove_where(|&i| bricks[tail].overlaps(&bricks[i]));
			match (main_supporter, pillar)
			{
				(Some(_support), Some(pillar)) =>
				{
					if cfg!(debug_assertions)
					{
						dbg!(pillar);
					}
					num_candidates += 1;
				}
				(Some(support), None) =>
				{
					if supporters.next().is_none()
					{
						only_supported_by[tail - mid] = support;
					}
				}
				(None, Some(_pillar)) => (),
				(None, None) => (),
			}
			tail += 1;
		}

		for i in head..mid
		{
			let responsibility =
				only_supported_by.iter().filter(|&&j| j == i).count();
			if responsibility == 0
			{
				if bricks[i].is_pillar()
				{
					pillar_indices.push(i);
				}
				else
				{
					if cfg!(debug_assertions)
					{
						dbg!(i);
					}
					num_candidates += 1;
				}
			}
		}

		head = mid;
		mid = tail;
	}

	let num_on_top = mid - head;
	if cfg!(debug_assertions)
	{
		dbg!(num_on_top);
	}
	num_candidates += num_on_top;

	num_candidates
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
		assert_eq!(one(PROVIDED), 5);
	}

	#[test]
	fn one_tower()
	{
		const TOWER: &str =
			"2,2,2~2,2,5\n1,2,10~3,2,10\n1,1,20~1,3,20\n3,1,30~3,3,30";
		assert_eq!(one(TOWER), 2);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

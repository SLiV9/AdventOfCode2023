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
	bricks.sort_unstable();
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
		|| (a0..=a1).contains(&b1)
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
			.filter(|other| self.overlaps(other))
			.map(|other| other.end.z)
			.max()
			.unwrap_or(0);
		let dz = self.start.z - floor - 1;
		self.start.z -= dz;
		self.end.z -= dz;
	}
}

fn drop_bricks(bricks: &mut [Brick])
{
	if cfg!(debug_assertions)
	{
		dbg!(&bricks);
	}

	for i in 0..bricks.len()
	{
		let (settled, floating) = bricks.split_at_mut(i);
		floating[0].drop(settled);
	}
}

fn count_candidates(bricks: &[Brick]) -> usize
{
	let len = bricks.len();

	if cfg!(debug_assertions)
	{
		dbg!(&bricks);
	}

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
		floor = bricks[mid].start.z;

		debug_assert!(head < mid);
		debug_assert!(mid <= head + 128);
		only_supported_by = [usize::MAX; 128];
		while let Some(pillar) =
			pillar_indices.remove_where(|i| bricks[*i].end.z + 1 < floor)
		{
			if cfg!(debug_assertions)
			{
				dbg!("candidate", pillar);
			}
			num_candidates += 1;
		}

		tail = mid;
		debug_assert!(tail <= mid + 128);
		while tail < len && bricks[tail].start.z == floor
		{
			let mut supporters = (head..mid)
				.filter(|&i| bricks[i].end.z + 1 == floor)
				.filter(|&i| bricks[tail].overlaps(&bricks[i]));
			let main_supporter = supporters.next();
			let pillar = pillar_indices
				.remove_where(|&i| bricks[tail].overlaps(&bricks[i]));
			match (main_supporter, pillar)
			{
				(Some(_support), Some(pillar)) =>
				{
					if cfg!(debug_assertions)
					{
						dbg!("candidate", pillar);
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

		// dbg!(head, mid, tail, only_supported_by);

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
						dbg!("candidate", i);
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
		dbg!(pillar_indices.len());
	}
	num_candidates += num_on_top + pillar_indices.len();

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
	fn one_reddit_ric2b()
	{
		const RIC2B: &str =
			"0,0,1~0,1,1\n1,1,1~1,1,1\n0,0,2~0,0,2\n0,1,2~1,1,2";
		assert_eq!(one(RIC2B), 3);
	}

	#[test]
	fn one_reddit_kullu00()
	{
		const KULLU00: &str =
			"0,0,1~1,0,1\n0,1,1~0,1,2\n0,0,5~0,0,5\n0,0,4~0,1,4";
		assert_eq!(one(KULLU00), 2);
	}

	#[test]
	fn one_reddit_barracuda()
	{
		const BARRACUDA: &str =
			"0,0,1~0,0,2\n1,0,1~2,0,1\n1,0,2~1,0,2\n0,0,3~1,0,3";
		assert_eq!(one(BARRACUDA), 3);
	}

	#[test]
	fn one_reddit_falling()
	{
		const FALLING: &str = "2,8,48~2,8,49\n1,8,2~3,8,2";
		assert_eq!(one(FALLING), 1);
	}

	#[test]
	fn one_reddit_cruficer()
	{
		const CRUCIFER: &str =
			"0,0,2~0,0,4\n1,0,3~2,0,3\n1,0,4~1,0,5\n0,0,6~1,0,6";
		assert_eq!(one(CRUCIFER), 3);
	}

	#[test]
	fn one_reddit_lefty()
	{
		const LEFTY: &str =
			"0,0,1~0,5,1\n0,6,1~0,9,1\n0,0,2~0,0,2\n0,3,2~0,8,2";
		assert_eq!(one(LEFTY), 3);
	}

	#[test]
	fn one_reddit_exile()
	{
		const EXILE: &str = "1,0,1~1,2,1\n0,0,2~2,0,2\n0,2,3~2,2,3\n0,0,4~0,2,\
		                     4\n2,0,5~2,2,5\n0,1,6~2,1,6\n1,1,8~1,1,9\n0,1,\
		                     3~0,1,3";
		assert_eq!(one(EXILE), 6);
	}

	#[test]
	fn one_rev_find()
	{
		const REV: &str = "0,0,100~9,0,100\n1,0,10~1,0,11\n2,0,20~2,0,22\n3,0,\
		                   30~3,0,30\n0,1,150~0,1,153\n0,0,500~0,2,500";
		assert_eq!(one(REV), 5);
	}

	#[test]
	fn test_overlap()
	{
		let a = Brick::from_str("0,0,1~0,1,1").unwrap();
		let b = Brick::from_str("1,1,1~1,1,1").unwrap();
		let c = Brick::from_str("0,0,2~0,0,2").unwrap();
		assert!(!a.overlaps(&b));
		assert!(!b.overlaps(&a));
		assert!(a.overlaps(&c));
		assert!(c.overlaps(&a));
		assert!(!b.overlaps(&c));
		assert!(!c.overlaps(&b));
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 0);
	}
}

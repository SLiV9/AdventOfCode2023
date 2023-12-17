/**/

use aoc2023::run;
use parse_display::FromStr;
use smallvec::SmallVec;
use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> u32
{
	let input = input.lines().next().unwrap();
	input.split(',').map(hash_word).map(u32::from).sum()
}

fn hash_word(word: &str) -> u8
{
	word.as_bytes().iter().cloned().fold(0, |acc: u8, x: u8| {
		acc.wrapping_mul(17).wrapping_add(x.wrapping_mul(17))
	})
}

fn safe_hash_word(word: &str) -> u32
{
	// It is safe and 99% of the time, it works every time.
	let mut bytes = [0; 4];
	let len = word.as_bytes().len().min(4);
	bytes[0..len].copy_from_slice(&word.as_bytes()[0..len]);
	u32::from_le_bytes(bytes)
}

#[derive(Debug, FromStr)]
enum Instruction
{
	#[display("{word}={focal_length}")]
	Insert
	{
		word: String, focal_length: u32
	},
	#[display("{word}-")]
	Delete
	{
		word: String
	},
}

#[derive(Debug, Clone, Copy, Default)]
struct Entry
{
	safe_hash: u32,
	focal_length: u32,
}

#[derive(Debug)]
struct HashMap
{
	data: [SmallVec<[Entry; 16]>; 256],
}

impl HashMap
{
	fn new() -> HashMap
	{
		HashMap {
			data: core::array::from_fn(|_i| SmallVec::new()),
		}
	}

	fn insert(&mut self, word: &str, focal_length: u32)
	{
		let hash = hash_word(word);
		let safe_hash = safe_hash_word(word);
		let entries = &mut self.data[hash as usize];
		let entry = entries
			.iter_mut()
			.find(|entry| entry.safe_hash == safe_hash);
		if let Some(entry) = entry
		{
			entry.focal_length = focal_length;
		}
		else
		{
			entries.push(Entry {
				safe_hash,
				focal_length,
			});
		}
	}

	fn delete(&mut self, word: &str)
	{
		let hash = hash_word(word);
		let safe_hash = safe_hash_word(word);
		let entries = &mut self.data[hash as usize];
		entries.retain(|entry| entry.safe_hash != safe_hash);
	}

	fn total_focusing_power(&self) -> u32
	{
		self.data
			.iter()
			.enumerate()
			.flat_map(|(i, entries)| {
				let box_number = (i + 1) as u32;
				entries.iter().enumerate().map(move |(i, entry)| {
					let slot_number = (i + 1) as u32;
					box_number * slot_number * entry.focal_length
				})
			})
			.sum()
	}
}

fn two(input: &str) -> u32
{
	let input = input.lines().next().unwrap();
	let mut hashmap = HashMap::new();
	for instruction in input.split(',')
	{
		let instruction = Instruction::from_str(instruction).unwrap();
		match instruction
		{
			Instruction::Insert { word, focal_length } =>
			{
				hashmap.insert(&word, focal_length);
			}
			Instruction::Delete { word } => hashmap.delete(&word),
		}
	}
	hashmap.total_focusing_power()
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
		assert_eq!(one(PROVIDED), 1320);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 145);
	}
}

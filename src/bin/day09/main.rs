/**/

use aoc2023::run;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> i32
{
	input
		.lines()
		.filter(|x| !x.is_empty())
		.map(solve_naive)
		.sum()
}

fn solve_naive(line: &str) -> i32
{
	let numbers = line.split(' ').map(|x| x.parse().unwrap());

	let mut data = [[0i32; 32]; 32];
	let mut len = 0;
	for number in numbers
	{
		data[0][len] = number;
		len += 1;
	}
	let mut depth = data.len();
	for d in 1..data.len()
	{
		for i in 0..(len - d)
		{
			data[d][i] = data[d - 1][i + 1] - data[d - 1][i];
		}

		if data[d].iter().all(|&x| x == 0)
		{
			depth = d;
			break;
		}
	}
	for d in (0..depth).rev()
	{
		let i = len - d;
		data[d][i] = data[d][i - 1] + data[d + 1][i - 1];
	}
	// print_data(&data, depth, len);
	data[0][len]
}

#[allow(unused)]
fn print_data(data: &[[i32; 32]; 32], depth: usize, len: usize)
{
	println!();
	for d in 0..(depth + 1)
	{
		let row = data[d];
		for _ in 0..d
		{
			print!("\t");
		}
		for x in &row[0..(len + 1 - d)]
		{
			print!("\t{x}\t");
		}
		println!();
	}
	println!();
}

fn two(input: &str) -> i32
{
	input
		.lines()
		.filter(|x| !x.is_empty())
		.map(solve_naive_rev)
		.sum()
}

fn solve_naive_rev(line: &str) -> i32
{
	let numbers = line.split(' ').map(|x| x.parse().unwrap());

	let mut data = [[0i32; 32]; 32];
	let mut len = 0;
	for number in numbers
	{
		len += 1;
		data[0][len] = number;
	}
	let mut depth = data.len();
	for d in 1..data.len()
	{
		for i in 1..(len + 1 - d)
		{
			data[d][i] = data[d - 1][i + 1] - data[d - 1][i];
		}

		if data[d].iter().all(|&x| x == 0)
		{
			depth = d;
			break;
		}
	}
	for d in (0..depth).rev()
	{
		data[d][0] = data[d][1] - data[d + 1][0];
	}
	// print_data(&data, depth, len);
	data[0][0]
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
		assert_eq!(one(PROVIDED), 114);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 2);
	}
}

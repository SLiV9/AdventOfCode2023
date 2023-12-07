/**/

use aoc2023::run;
use smallvec::SmallVec;

const INPUT: &str = include_str!("input.txt");

pub fn main()
{
	run!(one(INPUT));
	run!(two(INPUT));
}

fn one(input: &str) -> i64
{
	solve(input, parse_hand_v1)
}

fn solve(input: &str, parse_hand: impl Fn(&str) -> u64) -> i64
{
	let lines = input.lines().filter(|x| !x.is_empty());
	let mut hands: SmallVec<[u64; 1024]> = lines.map(parse_hand).collect();
	debug_assert!(hands.len() < 1024);
	hands.sort_unstable();
	hands
		.into_iter()
		.enumerate()
		.map(|(i, hand)| bid_from_hand(hand) * (i as i64 + 1))
		.sum()
}

fn parse_hand_v1(input: &str) -> u64
{
	parse_hand(input, card_from_ascii_v1)
}

fn parse_hand_v2(input: &str) -> u64
{
	parse_hand(input, card_from_ascii_v2)
}

fn parse_hand(input: &str, card_from_ascii: impl Fn(u8) -> u8) -> u64
{
	let bytes = input.as_bytes();
	let mut cards = [0u8; 5];
	for i in 0..5
	{
		cards[i] = card_from_ascii(bytes[i]);
	}
	let hand_kind: u8 = determine_hand(&cards);
	let bid = parse_bid(&bytes[6..]);
	hand_from_parts(cards, hand_kind, bid)
}

fn hand_from_parts(cards: [u8; 5], hand_kind: u8, bid: u16) -> u64
{
	let mut hand = 0;
	for card in cards
	{
		debug_assert!(card <= 0xF);
		hand <<= 4;
		hand |= u64::from(card) << 32;
	}
	debug_assert!(hand_kind <= 0xF);
	hand |= u64::from(hand_kind) << 60;
	hand |= u64::from(bid);
	hand
}

fn parse_bid(bytes: &[u8]) -> u16
{
	let mut bid: u16 = 0;
	for &x in bytes
	{
		if x.is_ascii_digit()
		{
			bid *= 10;
			bid += u16::from(x - b'0');
		}
	}
	bid
}

fn card_from_ascii_v1(x: u8) -> u8
{
	match x
	{
		b'2'..=b'9' => x - b'2' + 0x2,
		b'T' => 0xA,
		b'J' => 0xB,
		b'Q' => 0xC,
		b'K' => 0xD,
		b'A' => 0xE,
		_ => panic!("Unexpected character token: '{x}'"),
	}
}

fn card_from_ascii_v2(x: u8) -> u8
{
	match x
	{
		b'J' => 0x1,
		b'2'..=b'9' => x - b'2' + 0x2,
		b'T' => 0xA,
		b'Q' => 0xC,
		b'K' => 0xD,
		b'A' => 0xE,
		_ => panic!("Unexpected character token: '{x}'"),
	}
}

fn determine_hand(cards: &[u8; 5]) -> u8
{
	let mut data: [u8; 5] = *cards;
	let mut len = 5;
	let num_jokers = {
		let mut num_jokers = 0;
		let mut i = 0;
		while i < len
		{
			if data[i] == 0x1
			{
				data.swap(i, len - 1);
				len -= 1;
				num_jokers += 1;
			}
			else
			{
				i += 1;
			}
		}
		num_jokers
	};
	let mut nums = [0usize; 2];
	let mut offset_into_nums = 0;
	for _ in 0..4
	{
		if len < 2
		{
			break;
		}
		let key = data[len - 1];
		len -= 1;
		let mut i = 0;
		let mut num_matches = 0;
		while i < len
		{
			if data[i] == key
			{
				data.swap(i, len - 1);
				len -= 1;
				num_matches += 1;
			}
			else
			{
				i += 1;
			}
		}
		if num_matches > 0
		{
			nums[offset_into_nums] = num_matches + 1;
			offset_into_nums += 1;
		}
	}
	if nums[1] > nums[0]
	{
		nums.swap(0, 1);
	}
	if num_jokers > 0
	{
		if nums[0] == 0
		{
			nums[0] = std::cmp::min(num_jokers + 1, 5);
		}
		else
		{
			nums[0] += num_jokers;
		}
	}
	match nums
	{
		[5, 0] => 0xF,
		[4, 0] => 0xD,
		[3, 2] => 0x6,
		[3, 0] => 0x3,
		[2, 2] => 0x2,
		[2, 0] => 0x1,
		[0, 0] => 0x0,
		_ => panic!(
			"Impossible hand: {nums:?} with data {data:?} from cards {cards:?}"
		),
	}
}

fn bid_from_hand(hand: u64) -> i64
{
	let bid = (hand & 0xFFFF) as u16;
	i64::from(bid)
}

fn two(input: &str) -> i64
{
	solve(input, parse_hand_v2)
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
		assert_eq!(one(PROVIDED), 6440);
	}

	#[test]
	fn test_hands()
	{
		assert_eq!(determine_hand(&[2, 3, 4, 5, 6]), 0x0);
		assert_eq!(determine_hand(&[2, 2, 4, 5, 6]), 0x1);
		assert_eq!(determine_hand(&[2, 8, 4, 2, 6]), 0x1);
		assert_eq!(determine_hand(&[2, 2, 4, 4, 6]), 0x2);
		assert_eq!(determine_hand(&[2, 4, 6, 4, 2]), 0x2);
		assert_eq!(determine_hand(&[2, 2, 6, 4, 2]), 0x3);
		assert_eq!(determine_hand(&[2, 3, 2, 3, 3]), 0x6);
		assert_eq!(determine_hand(&[2, 2, 3, 2, 3]), 0x6);
		assert_eq!(determine_hand(&[3, 3, 2, 3, 3]), 0xD);
		assert_eq!(determine_hand(&[3, 3, 3, 3, 3]), 0xF);
		// Jokers
		assert_eq!(determine_hand(&[1, 2, 4, 5, 6]), 0x1);
		assert_eq!(determine_hand(&[2, 8, 4, 1, 6]), 0x1);
		assert_eq!(determine_hand(&[2, 2, 4, 1, 6]), 0x3);
		assert_eq!(determine_hand(&[2, 1, 6, 4, 2]), 0x3);
		assert_eq!(determine_hand(&[2, 1, 2, 3, 3]), 0x6);
		assert_eq!(determine_hand(&[2, 3, 2, 3, 1]), 0x6);
		assert_eq!(determine_hand(&[1, 1, 2, 3, 3]), 0xD);
		assert_eq!(determine_hand(&[3, 3, 3, 3, 1]), 0xF);
		assert_eq!(determine_hand(&[1, 1, 1, 1, 1]), 0xF);
	}

	#[test]
	fn two_provided()
	{
		assert_eq!(two(PROVIDED), 5905);
	}
}

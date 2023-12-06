/**/

use std::ops::Range;

pub fn binary_search_range(
	range: Range<i64>,
	predicate: impl Fn(i64) -> bool,
) -> Option<i64>
{
	let mut start = range.start;
	let mut end = range.end;
	if end <= start
	{
		return None;
	}
	else if predicate(start)
	{
		return Some(start);
	}
	else if !predicate(end - 1)
	{
		return None;
	}

	let mut mid = (start + end) / 2;

	loop
	{
		debug_assert!(start <= mid);
		debug_assert!(mid < end);
		if start == mid && mid + 1 == end
		{
			if predicate(mid)
			{
				return Some(mid);
			}
			else
			{
				return Some(mid + 1);
			}
		}

		if predicate(mid)
		{
			if start + 1 == mid
			{
				return Some(mid);
			}
			end = mid;
			mid = (start + end) / 2;
		}
		else
		{
			if mid + 1 == end
			{
				return Some(end);
			}
			start = mid;
			mid = (start + end) / 2;
		}
	}
}

#[cfg(test)]
mod tests
{
	use super::*;

	#[test]
	fn test_binary_search_range()
	{
		let p = |x| x >= 37;
		assert_eq!(binary_search_range(0..0, p), None);
		assert_eq!(binary_search_range(37..37, p), None);
		assert_eq!(binary_search_range(100..100, p), None);
		assert_eq!(binary_search_range(0..100, p), Some(37));
		assert_eq!(binary_search_range(1..100, p), Some(37));
		assert_eq!(binary_search_range(2..100, p), Some(37));
		assert_eq!(binary_search_range(3..100, p), Some(37));
		assert_eq!(binary_search_range(4..100, p), Some(37));
		assert_eq!(binary_search_range(22..54, p), Some(37));
		assert_eq!(binary_search_range(-120..4838, p), Some(37));
		assert_eq!(binary_search_range(37..100, p), Some(37));
		assert_eq!(binary_search_range(0..38, p), Some(37));
		assert_eq!(binary_search_range(37..38, p), Some(37));
		assert_eq!(binary_search_range(0..37, p), None);
		assert_eq!(binary_search_range(38..100, p), Some(38));
	}
}

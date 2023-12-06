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
				return Some(end);
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

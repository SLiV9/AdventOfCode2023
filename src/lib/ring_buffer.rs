/**/

#[derive(Debug)]
pub struct RingBuffer<A: smallvec::Array>
{
	buffer: A,
	head: usize,
	tail: usize,
}

impl<T: Copy + Default, const N: usize> Default for RingBuffer<[T; N]>
where
	[T; N]: smallvec::Array,
{
	fn default() -> Self
	{
		Self {
			buffer: [T::default(); N],
			head: usize::default(),
			tail: usize::default(),
		}
	}
}

impl<T: Copy + Default, const N: usize> RingBuffer<[T; N]>
where
	[T; N]: smallvec::Array,
{
	pub fn push(&mut self, x: T)
	{
		self.buffer[self.tail] = x;
		self.tail = (self.tail + 1) % N;
		debug_assert_ne!(self.head, self.tail);
	}

	pub fn head(&self) -> Option<&T>
	{
		if self.head != self.tail
		{
			Some(&self.buffer[self.head])
		}
		else
		{
			None
		}
	}

	pub fn drop_head(&mut self)
	{
		debug_assert_ne!(self.head, self.tail);
		self.head = (self.head + 1) % N;
	}

	pub fn pop_head(&mut self) -> Option<T>
	{
		if self.head != self.tail
		{
			let value = self.buffer[self.head];
			self.drop_head();
			Some(value)
		}
		else
		{
			None
		}
	}

	pub fn len(&self) -> usize
	{
		(self.tail + N - self.head) % N
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T>
	{
		let len = self.len();
		let (wrapped, afterhead) = self.buffer.split_at_mut(self.head);
		afterhead.iter_mut().chain(wrapped.iter_mut()).take(len)
	}
}

#[cfg(test)]
mod tests
{
	use super::*;
	use pretty_assertions::assert_eq;

	#[test]
	fn test_ring_buffer_10()
	{
		let mut ring_buffer: RingBuffer<[i32; 10]> = RingBuffer::default();

		ring_buffer.push(10);
		ring_buffer.push(20);
		assert_eq!(ring_buffer.head(), Some(&10));
		ring_buffer.drop_head();
		ring_buffer.push(30);
		assert_eq!(ring_buffer.head(), Some(&20));
		ring_buffer.drop_head();
		assert_eq!(ring_buffer.head(), Some(&30));
		ring_buffer.drop_head();

		let mut do_test_run = |num| {
			for x in 0..num
			{
				ring_buffer.push(x);
			}
			for x in 0..num
			{
				assert_eq!(ring_buffer.head(), Some(&x));
				ring_buffer.drop_head();
			}
		};

		do_test_run(7);
		do_test_run(8);
		do_test_run(9);
		assert_eq!(ring_buffer.head(), None);
	}
}

/**/

#[derive(Debug, Default)]
pub struct RingBuffer<A: smallvec::Array + Default>
{
	buffer: A,
	head: usize,
	tail: usize,
}

impl<T: Copy + Default, const N: usize> RingBuffer<[T; N]>
where
	[T; N]: smallvec::Array + Default,
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
}

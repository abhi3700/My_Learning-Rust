//! Understand Cell pointer
use std::cell::UnsafeCell;

struct Cell<T> {
	value: UnsafeCell<T>,
}

impl<T> Cell<T>
where
	T: Copy,
{
	fn new(val: T) -> Self {
		Self { value: UnsafeCell::new(val) }
	}

	fn set(&self, val: T) {
		unsafe { *self.value.get() = val }
	}

	fn get(&self) -> T {
		unsafe { *self.value.get() }
	}
}

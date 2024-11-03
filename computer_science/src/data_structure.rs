use std::mem::MaybeUninit;

// TODO: add unit test
pub struct CircularArray<T, const N: usize> {
    front: usize,
    size: usize,
    items: [MaybeUninit<T>; N],
}

impl<T, const N: usize> CircularArray<T, N> {
    pub fn new() -> Self {
        Self {
            front: 0,
            size: 0,
            items: [const { MaybeUninit::uninit() }; N],
        }
    }

    // O(1) instead of O(n) with regular fixed size array
    pub fn push_front(&mut self, value: T) -> Result<(), ()> {
        if self.size == N {
            return Err(());
        }

        self.decrement_front();
        self.size += 1;
        self.items[self.front].write(value);
        Ok(())
    }

    pub fn push_back(&mut self, value: T) -> Result<(), ()> {
        if self.size == N {
            return Err(());
        }

        self.items[self.back()].write(value);
        self.size += 1;

        Ok(())
    }

    // O(1) instead of O(n) with regular fixed size array
    pub fn pop_front(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let value = unsafe { self.items[self.front].assume_init_read() };
        self.increment_front();
        self.size -= 1;
        Some(value)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        let value = unsafe { self.items[self.back() - 1].assume_init_read() };
        self.size -= 1;
        Some(value)
    }

    pub fn len(&self) -> usize {
        return self.size;
    }

    fn back(&self) -> usize {
        return (self.front + self.size) % N;
    }

    fn increment_front(&mut self) {
        self.front = Self::get_next_index(self.front);
    }

    fn decrement_front(&mut self) {
        self.front = match self.front == 0 {
            true => N - 1,
            false => self.front - 1,
        };
    }

    fn get_next_index(index: usize) -> usize {
        match index == N - 1 {
            true => 0,
            false => index + 1,
        }
    }
}

impl<T, const N: usize> IntoIterator for CircularArray<T, N> {
    type Item = T;
    type IntoIter = CircularArrayIterator<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        let item_index = self.front;
        CircularArrayIterator::<T, N> {
            array: self,
            item_index,
            iteration_index: 0,
        }
    }
}

pub struct CircularArrayIterator<T, const N: usize> {
    array: CircularArray<T, N>,
    item_index: usize,
    iteration_index: usize,
}

impl<T, const N: usize> Iterator for CircularArrayIterator<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration_index == self.array.size {
            return None;
        }

        let item = unsafe { self.array.items[self.item_index].assume_init_read() };

        self.item_index = CircularArray::<T, N>::get_next_index(self.item_index);
        self.iteration_index += 1;

        Some(item)
    }
}

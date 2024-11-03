use std::mem::MaybeUninit;

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

        let value = unsafe { self.items[Self::get_prev_index(self.back())].assume_init_read() };
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
        self.front = Self::get_prev_index(self.front);
    }

    fn get_next_index(index: usize) -> usize {
        match index == N - 1 {
            true => 0,
            false => index + 1,
        }
    }

    fn get_prev_index(index: usize) -> usize {
        match index == 0 {
            true => N - 1,
            false => index - 1,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_push() -> Result<(), ()> {
        let mut array = CircularArray::<i32, 4>::new();
        array.push_back(5)?;
        array.push_back(8)?;
        array.push_front(4)?;
        array.push_front(2)?;

        let should_be_array = [2, 4, 5, 8];
        for (i, item) in array.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }

        Ok(())
    }

    #[test]
    fn can_pop() -> Result<(), ()> {
        let mut array = CircularArray::<i32, 4>::new();
        array.push_back(1)?;
        array.push_back(4)?;
        array.push_back(6)?;
        array.push_back(7)?;
        array.pop_front();
        array.pop_back();

        let should_be_array = [4, 6];
        for (i, item) in array.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }

        Ok(())
    }

    #[test]
    fn handles_overflow() -> Result<(), ()> {
        let mut array = CircularArray::<i32, 1>::new();
        array.push_back(1)?;
        let result = array.push_back(2);
        assert_eq!(result, Err(()));

        let result = array.push_front(2);
        assert_eq!(result, Err(()));

        Ok(())
    }

    #[test]
    fn gets_correct_size() -> Result<(), ()> {
        let mut array = CircularArray::<i32, 2>::new();
        assert_eq!(array.len(), 0);
        array.push_back(1)?;
        assert_eq!(array.len(), 1);
        array.push_front(2)?;
        assert_eq!(array.len(), 2);
        array.pop_back();
        assert_eq!(array.len(), 1);
        array.pop_front();
        assert_eq!(array.len(), 0);

        Ok(())
    }
}

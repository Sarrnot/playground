use std::{
    alloc::{alloc, handle_alloc_error, realloc, Layout},
    ptr::NonNull,
};

/// A naive version of dynamic array (does not guarantee memory safety, correct overflow handling, write/delete con/destructing etc.).
/// Inspirated by https://doc.rust-lang.org/nomicon/vec/vec.html.
pub struct DynamicArray<T> {
    pointer: NonNull<T>,
    items_count: usize,
    items_cap: usize,
}

impl<T> DynamicArray<T> {
    pub fn new() -> Self {
        Self {
            pointer: NonNull::dangling(),
            items_count: 0,
            items_cap: 0,
        }
    }

    /// Based on https://doc.rust-lang.org/nomicon/vec/vec-alloc.html
    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.items_cap == 0 {
            let new_layout = Layout::array::<T>(1).unwrap();
            (1, new_layout)
        } else {
            let new_cap = self.items_cap * 2;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // TODO: assert taken from docs, try to better understand: "Ensure that the new allocation doesn't exceed `isize::MAX` bytes."
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_pointer = if self.items_cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_pointer = self.pointer.as_ptr() as *mut u8;
            let old_layout = Layout::array::<T>(self.items_cap).unwrap();
            unsafe { realloc(old_pointer, old_layout, new_layout.size()) }
        };

        self.pointer = match NonNull::new(new_pointer as *mut T) {
            Some(pointer) => pointer,
            None => handle_alloc_error(new_layout),
        };
        self.items_cap = new_cap;
    }

    pub fn push(&mut self, item: T) {
        if self.items_count == self.items_cap {
            self.grow();
        }

        unsafe {
            self.pointer.add(self.items_count).write(item);
        };
        self.items_count += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.items_count == 0 {
            return None;
        }

        let value = unsafe { self.pointer.add(self.items_count - 1).read() };
        self.items_count -= 1;
        Some(value)
    }

    pub fn insert(&mut self, item: T, index: usize) {
        assert!(index <= self.items_count, "Out of bounds index.");

        if self.items_count == self.items_cap {
            self.grow();
        }

        unsafe {
            let index_pointer = self.pointer.add(index);
            index_pointer.copy_to(index_pointer.add(1), self.items_count - index);
            index_pointer.write(item);
        };

        self.items_count += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.items_count, "Out of bounds index.");

        let index_pointer = unsafe { self.pointer.add(index) };
        let value = unsafe { index_pointer.read() };

        unsafe {
            index_pointer
                .add(1)
                .copy_to(index_pointer, self.items_count - index - 1);
        }

        self.items_count -= 1;

        value
    }
}

impl<T> IntoIterator for DynamicArray<T> {
    type Item = T;
    type IntoIter = DynamicArrayIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        DynamicArrayIterator::<T> {
            array: self,
            index: 0,
        }
    }
}

pub struct DynamicArrayIterator<T> {
    array: DynamicArray<T>,
    index: usize,
}

impl<T> Iterator for DynamicArrayIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.array.items_count {
            return None;
        }

        let item = unsafe { self.array.pointer.add(self.index).read() };

        self.index += 1;

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_push_pop() {
        let mut array = DynamicArray::<i32>::new();
        array.push(12);
        array.push(4);
        array.push(5);
        array.push(33);
        let pop_result = array.pop();

        let should_be_array = [12, 4, 5];
        for (i, item) in array.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }

        assert_eq!(pop_result.unwrap(), 33);
    }

    #[test]
    fn pop_empty_returns_none() {
        let mut array = DynamicArray::<i32>::new();

        let result = array.pop();
        assert_eq!(result, None);
    }

    #[test]
    fn grows_correctly() {
        let mut array = DynamicArray::<i32>::new();

        assert_eq!(array.items_cap, 0);
        array.push(0);
        assert_eq!(array.items_cap, 1);
        array.push(0);
        assert_eq!(array.items_cap, 2);
        array.push(0);
        assert_eq!(array.items_cap, 4);
        for _ in 1..=2 {
            array.push(0);
        }
        assert_eq!(array.items_cap, 8);
    }

    #[test]
    fn can_insert_remove() {
        let mut array = DynamicArray::<i32>::new();

        array.push(1);
        array.push(3);
        array.push(4);

        array.insert(5, 3);
        let remove_result = array.remove(2);
        array.insert(2, 1);

        let should_be_array = [1, 2, 3, 5];
        for (i, item) in array.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }

        assert_eq!(remove_result, 4);
    }

    #[test]
    fn insert_remove_out_of_bounds_panics() {
        let result = std::panic::catch_unwind(|| {
            let mut array = DynamicArray::<i32>::new();
            array.insert(0, 1);
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            let mut array = DynamicArray::<i32>::new();
            array.remove(0);
        });
        assert!(result.is_err());
    }
}

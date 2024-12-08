use std::collections::LinkedList;

pub struct Stack<T> {
    items: LinkedList<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            items: LinkedList::new(),
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push_front(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop_front()
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = StackIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        StackIterator::<T> {
            iterator: self.items.into_iter(),
        }
    }
}

pub struct StackIterator<T> {
    iterator: std::collections::linked_list::IntoIter<T>,
}

impl<T> Iterator for StackIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_push() {
        let mut stack = Stack::<i32>::new();
        stack.push(4);
        stack.push(2);

        let should_be_array = [2, 4];
        for (i, item) in stack.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }
    }

    #[test]
    fn can_pop() {
        let mut stack = Stack::<i32>::new();
        stack.push(4);
        stack.push(2);

        assert_eq!(stack.items.len(), 2);
        assert_eq!(stack.pop().unwrap(), 2);
        assert_eq!(stack.pop().unwrap(), 4);
        assert_eq!(stack.items.len(), 0);
    }
}

use std::collections::LinkedList;

// Can be implemented by a singly-linked list (with tail pointer) or a dynamic circular array -> different performance in different scenarios.
// Linked list - O(1) push/pop but requires a new allocation with each push (can be expensive)
// Dynamic circular array - better overall performance (amortized O(1) push/pop with no allocation) but O(n) when growing
pub struct Queue<T> {
    items: LinkedList<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push_back(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop_front()
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = QueueIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        QueueIterator::<T> {
            iterator: self.items.into_iter(),
        }
    }
}

pub struct QueueIterator<T> {
    iterator: std::collections::linked_list::IntoIter<T>,
}

impl<T> Iterator for QueueIterator<T> {
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
        let mut queue = Queue::<i32>::new();
        queue.enqueue(4);
        queue.enqueue(7);

        let should_be_array = [4, 7];
        for (i, item) in queue.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }
    }

    #[test]
    fn can_pop() {
        let mut queue = Queue::<i32>::new();
        queue.enqueue(3);
        queue.enqueue(4);

        assert_eq!(queue.items.len(), 2);
        assert_eq!(queue.dequeue().unwrap(), 3);
        assert_eq!(queue.dequeue().unwrap(), 4);
        assert_eq!(queue.items.len(), 0);
    }
}

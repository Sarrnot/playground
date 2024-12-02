use std::{
    alloc::{alloc, handle_alloc_error, Layout},
    ptr::NonNull,
};

struct Node<T> {
    value: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, item: T) {
        self.insert(item, 0);
    }

    pub fn push_back(&mut self, item: T) {
        self.insert(item, self.len);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        Some(self.remove(0))
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        Some(self.remove(self.len - 1))
    }

    pub fn insert(&mut self, item: T, index: usize) {
        assert!(index <= self.len, "Out of bounds index.");

        // Get node before and after.
        let (node_before, node_after) = if index == self.len {
            (self.tail, None)
        } else {
            let node_after = self.node_at(index);
            let node_before = unsafe { node_after.read().prev };
            (node_before, Some(node_after))
        };

        // Create new node.
        let new_node = Node {
            value: item,
            prev: node_before,
            next: node_after,
        };

        let layout = Layout::new::<Node<T>>();
        let new_pointer = match NonNull::new(unsafe { alloc(layout) } as *mut Node<T>) {
            Some(pointer) => pointer,
            None => handle_alloc_error(layout),
        };

        unsafe { new_pointer.write(new_node) };

        // Update pointers of surrounding nodes.
        match node_before {
            Some(pointer) => {
                let mut node = unsafe { pointer.read() };
                node.next = Some(new_pointer);
                unsafe { pointer.write(node) };
            }
            None => {
                self.head = Some(new_pointer);
            }
        }

        match node_after {
            Some(pointer) => {
                let mut node = unsafe { pointer.read() };
                node.prev = Some(new_pointer);
                unsafe { pointer.write(node) };
            }
            None => {
                self.tail = Some(new_pointer);
            }
        }

        // Update length.
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        // Find node
        let node = unsafe { self.node_at(index).read() };

        // Update pointers of surrounding nodes (or head/tail).
        match node.prev {
            Some(pointer) => {
                let mut prev_node = unsafe { pointer.read() };
                prev_node.next = node.next;
                unsafe { pointer.write(prev_node) };
            }
            None => {
                self.head = node.next;
            }
        }

        match node.next {
            Some(pointer) => {
                let mut next_node = unsafe { pointer.read() };
                next_node.prev = node.prev;
                unsafe { pointer.write(next_node) };
            }
            None => {
                self.tail = node.prev;
            }
        };

        // Update length.
        self.len -= 1;

        // Return removed value
        node.value
    }

    pub fn at(&self, index: usize) -> T {
        let node = self.node_at(index);
        unsafe { node.read().value }
    }

    fn node_at(&self, index: usize) -> NonNull<Node<T>> {
        assert!(index < self.len, "Out of bounds index.");

        let search_from_tail = index >= (self.len / 2);

        let mut current_index = match search_from_tail {
            true => self.len - 1,
            false => 0,
        };
        let mut current_node = match search_from_tail {
            true => self.tail,
            false => self.head,
        };

        loop {
            if current_index == index {
                return current_node.unwrap();
            }
            current_node = match search_from_tail {
                true => unsafe { current_node.unwrap().read().prev },
                false => unsafe { current_node.unwrap().read().next },
            };
            current_index = match search_from_tail {
                true => current_index - 1,
                false => current_index + 1,
            };
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator::<T> {
            node_pointer: self.head,
        }
    }
}

pub struct LinkedListIterator<T> {
    node_pointer: Option<NonNull<Node<T>>>,
}

impl<T> Iterator for LinkedListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node_pointer {
            Some(node_pointer) => {
                let node = unsafe { node_pointer.read() };
                self.node_pointer = node.next;
                Some(node.value)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_push_front() {
        let mut list = LinkedList::<i32>::new();
        list.push_front(4);
        list.push_front(2);

        let should_be_array = [2, 4];
        for (i, item) in list.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }
    }

    #[test]
    fn can_push_back() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(3);
        list.push_back(5);

        let should_be_array = [3, 5];
        for (i, item) in list.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }
    }

    #[test]
    fn can_insert() {
        let mut list = LinkedList::<i32>::new();
        // Insert first/last
        list.insert(2, 0);
        // Insert last
        list.insert(4, 1);
        // Insert middle
        list.insert(3, 1);
        // Insert first
        list.insert(1, 0);

        let should_be_array = [1, 2, 3, 4];
        for (i, item) in list.into_iter().enumerate() {
            assert_eq!(item, should_be_array[i]);
        }
    }

    #[test]
    fn can_read_value_at() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        // Read first/middle/last
        assert_eq!(list.at(0), 2);
        assert_eq!(list.at(1), 3);
        assert_eq!(list.at(2), 4);
    }

    #[test]
    fn can_remove() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(list.len, 4);

        // Remove middle
        list.remove(1);

        // Remove first
        list.remove(0);

        // Remove last
        list.remove(1);

        // Check remaining value
        assert_eq!(list.at(0), 3);
        assert_eq!(list.len, 1);

        // Remove first/last
        list.remove(0);

        // Check empty
        assert_eq!(list.len, 0);
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn can_pop_front() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);

        // Pop last
        assert_eq!(list.pop_back(), Some(2));

        // Pop first/last
        assert_eq!(list.pop_back(), Some(1));

        // Pop empty
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn can_pop_back() {
        let mut list = LinkedList::<i32>::new();
        list.push_back(1);
        list.push_back(2);

        // Pop first
        assert_eq!(list.pop_front(), Some(1));

        // Pop first/last
        assert_eq!(list.pop_front(), Some(2));

        // Pop empty
        assert_eq!(list.pop_front(), None);
    }
}

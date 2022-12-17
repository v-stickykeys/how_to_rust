/// Implementation of a queue

use std::borrow::BorrowMut;
use std::mem;
use std::ops::{DerefMut, Deref};

pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<'a, T> List<'a, T> {
    pub fn new() -> List<'a, T> {
        List {
            head: None,
            tail: None
        }
    }

    /// This demonstrates how ownership works: After `tail` is moved to a position
    /// where it is owned, it can not be borrowed, because it has moved out of the scope
    /// of the function we are writing. However, we can get access to it from its
    /// new location in order to have a reference to it stored or used elsewhere.
    pub fn push(&'a mut self, elem: T) {
        let mut next_tail = Box::new(Node::new(elem));
        let tail = match self.tail.take() {
            Some(prev_tail) => {
                prev_tail.next = Some(next_tail);
                prev_tail.next.as_deref_mut()
            },
            None => {
                self.head = Some(next_tail);
                self.head.as_deref_mut()
            }
        };
        self.tail = tail;
    }

    pub fn pop(&'a mut self) -> Option<T> {
        self.head.take().map(|prev_head| {
            let head = *prev_head;
            self.head = head.next;
            if self.head.is_none() {
                self.tail = None;
            }
            head.elem
        })
        // match self.head.take() {
        //     Some(prev_head) => {
        //         self.head = prev_head.next;
        //         if self.head.is_none() {
        //             self.tail = None;
        //         }
        //         Some(prev_head.elem)
        //     },
        //     None => None
        // }
    }
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Node<T> {
        Node {
            elem,
            next: None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::linked_lists::fifth::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(0);
        assert_eq!(list.pop(), Some(0));
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }
}

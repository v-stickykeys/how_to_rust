#![allow(unused)]
use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    /// Returns a mutable reference of the head of the list.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list: List<i32> = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list: List<i32> = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(9);
        list.push(8);
        list.push(7);

        assert_eq!(list.peek(), Some(&7));
        assert_eq!(list.peek(), Some(&7));

        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.peek(), Some(&8));

        list.push(6);
        assert_eq!(list.peek(), Some(&6));
    }

    #[test]
    fn peek_mut() {
        let mut list: List<String> = List::new();
        list.push("good morning".to_string());
        list.push("good afternoon".to_string());

        assert_eq!(list.peek_mut(), Some(&mut "good afternoon".to_string()));
        assert_eq!(list.peek_mut(), Some(&mut "good afternoon".to_string()));

        assert_eq!(list.pop(), Some("good afternoon".to_string()));
        list.push("good night".to_string());
        list.peek_mut().map(|message| {
            *message = "good evening".to_string()
        });

        assert_eq!(list.peek(), Some(&"good evening".to_string()));
        assert_eq!(list.pop(), Some("good evening".to_string()));
        assert_eq!(list.pop(), Some("good morning".to_string()));
    }
}

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

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
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

    /// Moves self into an IntoIter.
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    /// Shares a reference to self in an Iter.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter { next: self.head.as_deref().map(|node| { node }) }
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

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref().map(|node| node);
            &node.elem
        })
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

    #[test]
    fn into_iter() {
        let mut list: List<i32> = List::new();
        let mut list = list.into_iter();
        assert_eq!(list.next(), None);

        let mut list: List<i32> = List::new();
        list.push(0); list.push(1); list.push(2);
        let mut list = list.into_iter();
        assert_eq!(list.next(), Some(2));
        assert_eq!(list.next(), Some(1));
        assert_eq!(list.next(), Some(0));
        assert_eq!(list.next(), None);
    }

    #[test]
    fn iter() {
        let mut list: List<i32> = List::new();
        let mut list = list.iter();
        assert_eq!(list.next(), None);

        let mut list: List<i32> = List::new();
        list.push(0); list.push(1); list.push(2);
        let mut list = list.iter();
        assert_eq!(list.next(), Some(&2));
        assert_eq!(list.next(), Some(&1));
        assert_eq!(list.next(), Some(&0));
        assert_eq!(list.next(), None);
    }
}

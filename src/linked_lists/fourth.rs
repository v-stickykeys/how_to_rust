use std::cell::{RefCell, Ref};
use std::rc::Rc;
use std::borrow::Borrow;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.borrow().as_ref().map(|node| Ref::map(node.as_ref().borrow(), |node| &node.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.borrow().as_ref().map(|node| Ref::map(node.as_ref().borrow(), |node| &node.elem))
    }

    /// Add node with `elem` to the head of the list
    pub fn push_front(&mut self, elem: T) {
        let node = Node::new(elem);
        node.borrow_mut().next = None;
        match &self.head {
            Some(head) => {
                head.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::clone(&head));
                self.head = Some(node);
            }
            None => {
                self.tail = Some(Rc::clone(&node));
                self.head = Some(Rc::clone(&node));
                node.borrow_mut().prev = None;
            }
        }
    }

    /// Add node with `elem` to the tail of the list
    pub fn push_back(&mut self, elem: T) {
        let mut node = Node::new(elem);
        match &self.tail {
            Some(tail) => {
                tail.borrow_mut().prev = Some(Rc::clone(&node));
                node.borrow_mut().next = Some(Rc::clone(tail));
            },
            None => {
                self.head = Some(Rc::clone(&node));
            }
        }
        self.tail = Some(Rc::clone(&node));
    }

    /// Return the node value at the head of the list and remove it
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|curr_head| {
            match curr_head.borrow_mut().prev.take() {
                Some(new_head) => {
                    new_head.borrow_mut().next = None;
                    self.head = Some(new_head);
                },
                None => {
                    self.tail.take();
                }
            };
            Rc::try_unwrap(curr_head).ok().unwrap().into_inner().elem
        })
    }

    /// Return the node value at the tail of the list and remove it
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|curr_tail| {
            match curr_tail.borrow_mut().next.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().prev = None;
                    self.tail = Some(new_tail);
                },
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(curr_tail).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        list.push_front(5);
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        list.push_front("one");
        assert_eq!(*list.peek_front().unwrap(), "one");
        assert_eq!(*list.peek_front().unwrap(), "one");
        list.push_front("two");
        list.push_front("three");
        assert_eq!(*list.peek_front().unwrap(), "three");
    }

    #[test]
    fn pop_back() {
        let mut list = List::new();
        assert_eq!(list.pop_back(), None);
        list.push_front(1);
        assert_eq!(list.pop_back(), Some(1));
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn push_back() {
        let mut list = List::new();
        list.push_back(1);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);
        assert_eq!(&*list.peek_front().unwrap(), &2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(4));
    }

    #[test]
    fn peek_back() {
        let mut list = List::new();
        assert!(list.peek_back().is_none());
        list.push_back(1);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(&*list.peek_back().unwrap(), &3);
    }
}

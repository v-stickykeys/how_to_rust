use std::cell::RefCell;
use std::rc::Rc;

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

    /// Return the node at the head of the list and remove it
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|curr_head| {
            match curr_head.borrow_mut().prev.take() {
                Some(new_head) => {
                    new_head.borrow_mut().next = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            };
            Rc::try_unwrap(curr_head).ok().unwrap().into_inner().elem
        })
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
}

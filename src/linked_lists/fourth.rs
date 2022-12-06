use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None
        }))
    }
}

impl<T> List<T> {
    fn new() -> Self {
        List {
            head: None,
            tail: None
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let node = Node::new(elem);
        node.borrow_mut().next = None;
        match &self.head {
            Some(head) => {
                head.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(Rc::clone(&head));
                self.head = Some(node);
            },
            None => {
                self.tail = Some(Rc::clone(&node));
                self.head = Some(Rc::clone(&node));
                node.borrow_mut().prev = None;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_front() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
    }
}
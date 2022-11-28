use std::mem;

pub struct List {
    head: Link,
}

#[derive(Clone)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Clone)]
enum Link {
    Empty,
    More(Box<Node>),
}

// impl Drop for List {
//     fn drop(&mut self) {
//         self.head.drop();
//     }
// }
// 
// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {}
//             Link::More(ref mut boxed_node) => {
//                 boxed_node.drop();
//             }
//         }
//     }
// }
// 
// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop();
//         deallocate(self.ptr);
//     }
// }
// 
// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }
// We can't drop the contents of Box after deallocating. Why? Do the contents get stuck? Why is
// dropping the node just dropping next? How can we drop contents if we don't call it in the Box
// node implementation? 

impl Drop for List {
    // fn drop(&mut self) {
    //     let mut cur_link = mem::replace(&mut self.head, Link::Empty);
    //     while let Link::More(mut boxed_node) = cur_link {
    //         cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
    //     }
    // }

    fn drop(&mut self) {
        while let Link::More(_) = self.pop_node() {
            self.pop_node();
        }
    }
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    // pub fn push(&mut self, elem: i32) {
    //     let node = Node {
    //         elem,
    //         next: self.head.clone()
    //     };
    //     self.head = Link::More(Box::new(node))
    // }
    pub fn push(&mut self, elem: i32) {
        let node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty)
        };
        self.head = Link::More(Box::new(node))
    }
    pub fn pop(&mut self) -> Option<i32> {
        match self.pop_node() {
            Link::Empty => None,
            Link::More(n) => {
                Some(n.elem)
            },
        }
        // unimplemented!()
    }
    fn pop_node(&mut self) -> Link {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => Link::Empty,
            Link::More(mut n) => {
                self.head = mem::replace(&mut n.next, Link::Empty);
                Link::More(n)
            }
        }
    }
}

#[cfg(test)]
mod should {
    // use super::*;
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}

use std::rc::Rc;

pub struct List<G> {
    head: Link<G>
}

type Link<G> = Option<Rc<Node<G>>>;

struct Node<G> {
    elem: G,
    next: Link<G>
}

struct Iter<'a, G> {
    next: Option<&'a Rc<Node<G>>>
}

impl<G> List<G> {
    pub fn new() -> Self {
        List {head: None}
    }

    pub fn head(&self) -> Option<&G> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn prepend(&self, elem: G) -> List<G> {
        let node = Node {
            elem,
            next: self.head.as_ref().map(|rc| Rc::clone(&rc))
        };
        List { head: Some(Rc::new(node)) }
    }

    pub fn behead(&self) -> List<G> {
        let next = self.head.as_ref().and_then(|node| node.next.clone());
        List { head: next }
    }

    pub fn iter(&self) -> Iter<G> {
        Iter { next: self.head.as_ref() }
    }
}

impl<G> Drop for List<G> {
    fn drop(&mut self) {
        let mut curr_node = self.head.take();
        while let Some(mut node) = curr_node {
            match Rc::try_unwrap(node) {
                Ok(mut unwrapped) => { curr_node = unwrapped.next.take(); }
                Err(_) => break
            }
        }
    }
}

impl<'a, G> Iterator for Iter<'a, G> {
    type Item = &'a G;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref();
            &node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let list: List<i32> = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1);
        assert_eq!(list.head(), Some(&1));

        let list = list.behead();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.behead();
        assert_eq!(list.head(), Some(&2));

        let list = list.behead();
        assert_eq!(list.head(), Some(&1));

        let list = list.behead();
        assert_eq!(list.head(), None);

        let list = list.behead();
        assert_eq!(list.head(), None);
    }

    fn iter() {
        let list: List<i32> = List::new();
        let mut list = list.iter();
        assert_eq!(list.next(), None);

        let list: List<i32> = List::new();
        list.prepend(1);
        list.prepend(2);
        list.prepend(3);
        let mut list = list.iter();
        assert_eq!(list.next(), Some(&3));
        assert_eq!(list.next(), Some(&2));
        assert_eq!(list.next(), Some(&1));
        assert_eq!(list.next(), None);
        assert_eq!(list.next(), None);
    }
}


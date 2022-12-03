use std::rc::Rc;

pub struct List<G> {
    head: Link<G>
}

type Link<G> = Option<Rc<Node<G>>>;

struct Node<G> {
    elem: G,
    next: Link<G>
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
}


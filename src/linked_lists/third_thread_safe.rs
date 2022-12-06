use std::sync::Arc;

pub struct ThreadSafeList<G> {
    head: ThreadSafeLink<G>
}

type ThreadSafeLink<G> = Option<Arc<ThreadSafeNode<G>>>;

struct ThreadSafeNode<G> {
    elem: G,
    next: ThreadSafeLink<G>
}

struct ThreadSafeIter<'a, G> {
    next: Option<&'a Arc<ThreadSafeNode<G>>>
}

impl<G> ThreadSafeList<G> {
    pub fn new() -> Self {
        ThreadSafeList {head: None}
    }

    pub fn head(&self) -> Option<&G> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn prepend(&self, elem: G) -> ThreadSafeList<G> {
        let node = ThreadSafeNode {
            elem,
            next: self.head.as_ref().map(|arc| Arc::clone(&arc))
        };
        ThreadSafeList { head: Some(Arc::new(node)) }
    }

    pub fn behead(&self) -> ThreadSafeList<G> {
        let next = self.head.as_ref().and_then(|node| node.next.clone());
        ThreadSafeList { head: next }
    }

    pub fn iter(&self) -> ThreadSafeIter<G> {
        ThreadSafeIter { next: self.head.as_ref() }
    }
}

impl<G> Drop for ThreadSafeList<G> {
    fn drop(&mut self) {
        let mut curr_node = self.head.take();
        while let Some(mut node) = curr_node {
            match Arc::try_unwrap(node) {
                Ok(mut unwrapped) => { curr_node = unwrapped.next.take(); }
                Err(_) => break
            }
        }
    }
}

impl<'a, G> Iterator for ThreadSafeIter<'a, G> {
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
    use super::ThreadSafeList;

    #[test]
    fn basics() {
        let list: ThreadSafeList<i32> = ThreadSafeList::new();
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
        let list: ThreadSafeList<i32> = ThreadSafeList::new();
        let mut list = list.iter();
        assert_eq!(list.next(), None);

        let list: ThreadSafeList<i32> = ThreadSafeList::new();
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


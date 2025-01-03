use std::rc::Rc;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

impl<T> Node<T> {
    fn new(elem: T) -> Node<T> {
        Node { elem, next: None, prev: None }
    }
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { head: None, tail: None }
    }

    fn add_front(&mut self, elem: T) {
        let mut new_node = Box::new(Node::new(elem));
        let new_head = match self.head.take() {
            Some(mut old_head) => {
                old_head.prev = Some(new_node);
                old_head.prev.as_deref_mut()

            }
            None => {
                self.tail = Some(new_node)
                self.tail.as_deref_mut()
            }
        };
        self.head = new_head;
    }
}
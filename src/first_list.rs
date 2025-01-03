use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    More(Box<Node>),
    Empty
}

pub struct Node {
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if let Link::More(node) = mem::replace(&mut self.head, Link::Empty) {
            self.head = node.next;
            return Some(node.elem);
        } else {
            return None;
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut node) = cur_link {
            cur_link = mem::replace(&mut node.next, Link::Empty);
        }
        self.head = Link::Empty;
    }
}

fn main() {
    println!("yo")
}



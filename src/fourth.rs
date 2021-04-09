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
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        return Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }));
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        return List {
            head: None,
            tail: None,
        };
    }
    pub fn push_front(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node)
            }
            Node => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }
}

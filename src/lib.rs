use std::cell::RefCell;
use std::rc::Rc;
pub struct Node<T>
where
    T: Copy + PartialEq + PartialOrd + Eq,
{
    pub val: T,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
where
    T: Copy + PartialEq + PartialOrd + Eq,
{
    pub fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Bst<T>
where
    T: Copy + PartialEq + PartialOrd + Eq,
{
    root: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Bst<T>
where
    T: Copy + PartialEq + PartialOrd + Eq,
{
    pub fn new(val: T) -> Self {
        Self {
            root: Some(Rc::new(RefCell::new(Node::new(val)))),
        }
    }
}

impl<T> Bst<T>
where
    T: Copy + PartialEq + PartialOrd + Eq,
{
    pub fn insert(&mut self, val: T) {
        if let None = self.root {
            let node = Node::new(val);
            self.root = Some(Rc::new(RefCell::new(node)));
            return;
        }

        let mut node = Rc::clone(self.root.as_ref().unwrap());
        loop {
            let curr_val = node.borrow().val;
            if val <= curr_val {
                if let None = &node.as_ref().borrow().left {
                    let new_node = Node::new(val);
                    node.borrow_mut().left = Some(Rc::new(RefCell::new(new_node)));
                    return;
                } else {
                    node = Rc::clone(node.as_ref().borrow().left.as_ref().unwrap());
                }
            } else {
            }
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
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

#[derive(Debug)]
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
    pub fn iter(&self) -> BstIterator<T> {
        let node = if let Some(ref n) = self.root {
            Some(Rc::clone(n))
        } else {
            None
        };
        BstIterator {
            stack: Vec::new(),
            node,
        }
    }
    pub fn head(&self) -> Option<T> {
        let rc = self.root.as_ref()?;
        Some(rc.borrow().val)
    }
    pub fn insert(&mut self, val: T) {
        if let None = self.root {
            let node = Node::new(val);
            self.root = Some(Rc::new(RefCell::new(node)));
            return;
        }

        let mut curr = Rc::clone(self.root.as_ref().unwrap());
        loop {
            let curr_val = curr.borrow().val;
            if val <= curr_val {
                let is_left_tree_available;
                {
                    //We need this scope so that curr.borrow() below and curr.borrow_mut()[used outside this scope] both can be used
                    is_left_tree_available = if let Some(_) = curr.borrow().left {
                        true
                    } else {
                        false
                    }
                }
                if !is_left_tree_available {
                    let new_node = Node::new(val);
                    curr.borrow_mut().left = Some(Rc::new(RefCell::new(new_node)));
                    return;
                } else {
                    let new_curr = Rc::clone(&curr);
                    curr = Rc::clone(new_curr.borrow().left.as_ref().unwrap());
                }
            } else {
                let is_right_tree_available;
                {
                    if let Some(_) = curr.borrow().right {
                        is_right_tree_available = true;
                    } else {
                        is_right_tree_available = false;
                    }
                }
                if !is_right_tree_available {
                    let new_node = Node::new(val);
                    curr.borrow_mut().right = Some(Rc::new(RefCell::new(new_node)));
                    return;
                } else {
                    let new_curr = Rc::clone(&curr);
                    curr = Rc::clone(new_curr.borrow().right.as_ref().unwrap());
                }
            }
        }
    }
}

pub struct BstIterator<T>
where
    T: Copy + PartialEq + PartialOrd + Eq,
{
    node: Option<Rc<RefCell<Node<T>>>>,
    stack: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Iterator for BstIterator<T>
where
    T: Copy + PartialEq + PartialOrd + Eq,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let node_clone = self.node.clone();
        match node_clone {
            Some(mut curr) => loop {
                {
                    if let None = curr.borrow().left {
                        let to_return = curr.borrow().val;
                        self.node = if let Some(next) = curr.borrow().right.as_ref() {
                            Some(Rc::clone(next))
                        } else {
                            None
                        };
                        return Some(to_return);
                    }
                }
                self.stack.push(Rc::clone(&curr));
                let new_curr = Rc::clone(curr.borrow().left.as_ref().unwrap());
                curr = new_curr;
            },
            None => {
                if self.stack.is_empty() {
                    return None;
                }
                let curr = self.stack.pop().unwrap();
                let to_return = curr.borrow().val;
                self.node = if let Some(next) = curr.borrow().right.as_ref() {
                    // self.stack.push(Rc::clone(next));
                    Some(Rc::clone(next))
                } else {
                    None
                };
                Some(to_return)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        let bst = Bst::new(10);
        assert_eq!(bst.head().unwrap(), 10);
    }
    #[test]
    fn second() {
        let mut bst = Bst::new(10);
        bst.insert(5);
        bst.insert(15);
        bst.insert(20);
        bst.insert(13);
        let mut iterator = bst.iter();
        assert_eq!(Some(5), iterator.next());
        assert_eq!(Some(10), iterator.next());
        assert_eq!(Some(13), iterator.next());
        assert_eq!(Some(15), iterator.next());
        assert_eq!(Some(20), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn left_skewed_tree() {
        let mut bst = Bst::new(20);
        bst.insert(15);
        bst.insert(10);
        bst.insert(5);
        bst.insert(0);
        bst.insert(-1);
        let mut iterator = bst.iter();
        assert_eq!(Some(-1), iterator.next());
        assert_eq!(Some(0), iterator.next());
        assert_eq!(Some(5), iterator.next());
        assert_eq!(Some(10), iterator.next());
        assert_eq!(Some(15), iterator.next());
        assert_eq!(Some(20), iterator.next());
        assert_eq!(None, iterator.next());
    }
}

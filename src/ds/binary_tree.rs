use std::collections::LinkedList;
use std::rc::Rc;
use std::cell::{RefCell, Cell};

struct TreeNode<T> {
    data: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>
}

impl<T> TreeNode<T> {
    fn new(data: T) -> Self {
        TreeNode{
            data: data,
            left: None,
            right: None
        }
    }

}

impl<T> TreeNode<T> {
    fn leftIter(&self) -> LeftIterator<T> {
        LeftIterator{root: Some(&self)}
    }
}

struct LeftIterator<'a, T> {
    root: Option<&'a TreeNode<T>>
}



impl<'a, T> Iterator for LeftIterator<'a, T> {
    type Item = &'a TreeNode<T>;

    fn next(&mut self) -> Option<&'a TreeNode<T>> {
        None
    }
}

struct InorderState<'a, T> {
    stack: LinkedList<&'a TreeNode<T>>,
    root: &'a TreeNode<T>
}

impl<'a, T> InorderState<'a, T> {
    fn new(root: &'a TreeNode<T>) -> Self {
        InorderState {stack: LinkedList::new(), root: root}
    }
}

impl<'a, T> Iterator for InorderState<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        while let Some(ref current) = self.root.leftIter().next() {
            self.stack.push_front(current);
            unimplemented!();
        }
        None
    } 
}

fn main() {
    let mut root = TreeNode::<i32>::new(1);
    let mut list = LinkedList::<i32>::new();
    list.push_front(1);
}
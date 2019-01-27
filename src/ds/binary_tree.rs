use std::collections::LinkedList;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::mem;
use std::marker::PhantomData;

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
    fn left_iter(&self) -> LeftIterator<T> {
        LeftIterator::new(self.left.clone())
    }
}

struct LeftIterator<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>
}

impl<T> LeftIterator<T> {
    fn new(root: Option<Rc<RefCell<TreeNode<T>>>>) -> Self {
        LeftIterator{root: root}
    }
}

impl<T> Iterator for LeftIterator<T> {
    type Item = Rc<RefCell<TreeNode<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.root.is_some() {
            let current: Option<Rc<RefCell<TreeNode<T>>>> = self.root.clone();
            let left: Option<Rc<RefCell<TreeNode<T>>>> = self.root.clone().unwrap().borrow().left.clone();
            self.root = left;
            return current;
        }
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
        None
    } 
}

fn main() {
    let mut root = TreeNode::<i32>::new(1);
    let mut list = LinkedList::<i32>::new();
    list.push_front(1);
}
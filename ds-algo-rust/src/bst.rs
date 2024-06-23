use std::cmp::Ordering;
use std::fmt;

use crate::vector::Vector;

pub struct Node<T> {
    data: T,
    left: Link<T>,
    right: Link<T>,
}

pub struct BST<T> {
    root: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

impl<T: Ord> Node<T> {
    pub fn new(data: T) -> Box<Self> {
        Box::new(Node {
            data,
            left: None,
            right: None,
        })
    }
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        BST { root: None }
    }

    pub fn insert(&mut self, data: T) {
        let new_node: Box<Node<T>> = Node::new(data);

        match self.root {
            None => self.root = Some(new_node),
            Some(ref mut node) => Self::insert_recursive(node, new_node),
        }
    }

    fn insert_recursive(curr_node: &mut Box<Node<T>>, new_node: Box<Node<T>>) {
        match new_node.data.cmp(&curr_node.data) {
            Ordering::Less => match curr_node.left {
                Some(ref mut node) => Self::insert_recursive(node, new_node),
                None => curr_node.left = Some(new_node),
            },
            Ordering::Greater => match curr_node.right {
                Some(ref mut node) => Self::insert_recursive(node, new_node),
                None => curr_node.right = Some(new_node),
            },
            Ordering::Equal => {}
        }
    }

    pub fn in_order_traversal(&self)
    where
        T: fmt::Display,
    {
        Self::in_order_traversal_recursive(&self.root);
    }

    fn in_order_traversal_recursive(node: &Link<T>)
    where
        T: fmt::Display,
    {
        match node {
            None => {}
            Some(node) => {
                Self::in_order_traversal_recursive(&node.left);
                print!("{} ", node.data);
                Self::in_order_traversal_recursive(&node.right);
            }
        }
    }

    pub fn pre_order_traversal(&self)
    where
        T: fmt::Display,
    {
        Self::pre_order_traversal_recursive(&self.root);
    }

    fn pre_order_traversal_recursive(node: &Link<T>)
    where
        T: fmt::Display,
    {
        match node {
            None => {}
            Some(node) => {
                print!("{} ", node.data);
                Self::pre_order_traversal_recursive(&node.left);
                Self::pre_order_traversal_recursive(&node.right);
            }
        }
    }

    pub fn post_order_traversal(&self)
    where
        T: fmt::Display,
    {
        Self::post_order_traversal_recursive(&self.root);
    }

    fn post_order_traversal_recursive(node: &Link<T>)
    where
        T: fmt::Display,
    {
        match node {
            None => {}
            Some(node) => {
                Self::post_order_traversal_recursive(&node.left);
                Self::post_order_traversal_recursive(&node.right);
                print!("{} ", node.data);
            }
        }
    }

    pub fn level_order_traversal(&self)
    where
        T: fmt::Display,
    {
        let mut queue: Vec<&Link<T>> = Vec::new();
        queue.push(&self.root);

        while !queue.is_empty() {
            let curr_node = queue.remove(0);

            match curr_node {
                None => {}
                Some(node) => {
                    print!("{} ", node.data);
                    queue.push(&node.left);
                    queue.push(&node.right);
                }
            }
        }
    }

    pub fn remove(&mut self, data: T) {
        self.root = Self::remove_recursive(self.root.take(), data);
    }

    fn remove_recursive(node: Link<T>, data: T) -> Link<T> {
        match node {
            None => None,
            Some(mut node) => match data.cmp(&node.data) {
                Ordering::Less => {
                    node.left = Self::remove_recursive(node.left.take(), data);
                    Some(node)
                }
                Ordering::Greater => {
                    node.right = Self::remove_recursive(node.right.take(), data);
                    Some(node)
                }
                Ordering::Equal => {
                    if node.left.is_none() {
                        return node.right.take();
                    } else if node.right.is_none() {
                        return node.left.take();
                    }

                    let (min_node, new_right) = Self::remove_min(node.right.take().unwrap());
                    node.data = min_node.data;
                    node.right = new_right;
                    Some(node)
                }
            },
        }
    }
    fn remove_min(mut node: Box<Node<T>>) -> (Box<Node<T>>, Link<T>) {
        match node.left.take() {
            Some(left) => {
                let (min_node, new_left) = Self::remove_min(left);
                node.left = new_left;
                (min_node, Some(node))
            }
            None => {
                let right = node.right.take();
                (node, right)
            }
        }
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

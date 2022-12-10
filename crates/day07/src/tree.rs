use std::collections::VecDeque;

#[derive(Debug)]
pub struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

struct LevelTraversal<'a> {
    node: Option<&'a Box<Node>>,
    queue: VecDeque<&'a Box<Node>>,
}

impl<'a> LevelTraversal<'a> {
    fn new(node: Option<&'a Box<Node>>) -> Self {
        LevelTraversal {
            node,
            queue: VecDeque::new(),
        }
    }
}

struct InorderTraversal<'a> {
    node: Option<&'a Box<Node>>,
    queue: Vec<&'a Box<Node>>,
}

impl<'a> InorderTraversal<'a> {
    fn new(node: Option<&'a Box<Node>>) -> Self {
        InorderTraversal {
            node,
            queue: Vec::new(),
        }
    }
}

impl<'a> Iterator for LevelTraversal<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.node, &mut self.queue) {
            (None, q) if q.is_empty() => None,
            (None, q) => {
                self.node = q.pop_front();
                self.next()
            }
            (Some(node), q) => {
                if let Some(ref left) = node.left {
                    q.push_back(left);
                }
                if let Some(ref right) = node.right {
                    q.push_back(right);
                }
                self.node = None;
                Some(node.value)
            }
        }
    }
}

impl<'a> Iterator for InorderTraversal<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.node, &mut self.queue) {
            (None, q) if q.is_empty() => None,
            (None, q) => {
                let node = q.pop().unwrap();
                self.node = node.right.as_ref();

                Some(node.value)
            }
            (Some(node), q) => {
                q.push(node);
                self.node = node.left.as_ref();
                self.next()
            }
        }
    }
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn traverse_level(&self) -> Vec<i32> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut results: Vec<i32> = Vec::new();
        let mut q: VecDeque<&Box<Node>> = VecDeque::new();
        let root = self.root.as_ref().unwrap();
        results.push(root.value);
        q.push_back(root);

        while !q.is_empty() {
            for _ in 0..q.len() {
                if let Some(node) = q.pop_front() {
                    if let Some(ref left) = node.left {
                        results.push(left.value);
                        q.push_back(left);
                    }
                    if let Some(ref right) = node.right {
                        results.push(right.value);
                        q.push_back(right);
                    }
                }
            }
        }
        results
    }

    fn traverse_inorder_recursive(values: &mut Vec<i32>, node: &Box<Node>) {
        if let Some(ref left) = node.left {
            Tree::traverse_inorder_recursive(values, left);
        }
        values.push(node.value);
        if let Some(ref right) = node.right {
            Tree::traverse_inorder_recursive(values, right);
        }
    }

    fn level_iter(&self) -> LevelTraversal {
        LevelTraversal::new(self.root.as_ref())
    }

    fn inorder_iter(&self) -> InorderTraversal {
        InorderTraversal::new(self.root.as_ref())
    }

    fn traverse_inorder_iterative(&self) -> Vec<i32> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut result: Vec<i32> = Vec::new();
        let mut q: Vec<&Box<Node>> = Vec::new();
        let mut current = self.root.as_ref();
        while !q.is_empty() || current.is_some() {
            while let Some(node) = current {
                q.push(node);
                current = node.left.as_ref();
            }
            if let Some(node) = q.pop() {
                result.push(node.value);
                current = node.right.as_ref();
            }
        }
        result
    }

    fn inorder(&self) -> Vec<i32> {
        if self.root.is_none() {
            return Vec::new();
        }

        let mut results: Vec<i32> = Vec::new();

        if let Some(ref root) = self.root {
            Tree::traverse_inorder_recursive(&mut results, root);
        }

        results
    }

    fn insert(&mut self, value: i32) {
        // match &mut self.root {
        //     None => {
        //         self.root = Node::new(value).into();
        //     }
        //     Some(node) => {
        //         Tree::insert_recursive(node, value);
        //     }
        // }
        self.insert_iterative(value);
    }

    fn insert_iterative(&mut self, value: i32) {
        if self.root.is_none() {
            self.root = Node::new(value).into();
            return;
        }

        let mut q: Vec<&mut Box<Node>> = Vec::new();
        let root = self.root.as_mut().unwrap();
        q.push(root);

        while let Some(node) = q.pop() {
            if value > node.value {
                let right = &mut node.right;
                match right {
                    None => {
                        *right = Node::new(value).into();
                    }
                    Some(n) => {
                        q.push(n);
                    }
                }
            } else if value < node.value {
                let left = &mut node.left;
                match left {
                    None => {
                        *left = Node::new(value).into();
                    }
                    Some(n) => {
                        q.push(n);
                    }
                }
            }
        }
    }

    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value > node.value {
            match &mut node.right {
                None => {
                    node.right = Node::new(value).into();
                }
                Some(node) => {
                    Tree::insert_recursive(node, value);
                }
            }
        } else if value < node.value {
            match &mut node.left {
                None => {
                    node.left = Node::new(value).into();
                }
                Some(node) => {
                    Tree::insert_recursive(node, value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);
        println!("{tree:?}");
        assert!(tree.root.is_some());
    }

    #[test]
    fn works_builds_level_traversal_tree() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);
        tree.insert(7);
        tree.insert(14);
        tree.insert(13);

        assert_eq!(tree.traverse_level(), vec![8, 3, 10, 1, 6, 14, 4, 7, 13]);
    }

    #[test]
    fn works_builds_tree_inorder() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);
        tree.insert(7);
        tree.insert(14);
        tree.insert(13);

        assert_eq!(tree.inorder(), vec![1, 3, 4, 6, 7, 8, 10, 13, 14]);
    }

    #[test]
    fn works_builds_tree_inorder_iterative() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);
        tree.insert(7);
        tree.insert(14);
        tree.insert(13);

        assert_eq!(
            tree.traverse_inorder_iterative(),
            vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
        );
    }

    #[test]
    fn works_builds_level_traversal_iter_tree() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);
        tree.insert(7);
        tree.insert(14);
        tree.insert(13);

        assert_eq!(
            tree.level_iter().collect::<Vec<i32>>(),
            vec![8, 3, 10, 1, 6, 14, 4, 7, 13]
        );
    }

    #[test]
    fn works_builds_tree_inorder_iterator() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(1);
        tree.insert(6);
        tree.insert(4);
        tree.insert(7);
        tree.insert(14);
        tree.insert(13);

        assert_eq!(
            tree.inorder_iter().collect::<Vec<i32>>(),
            vec![1, 3, 4, 6, 7, 8, 10, 13, 14]
        );
    }
}

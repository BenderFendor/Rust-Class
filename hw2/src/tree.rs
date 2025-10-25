use std::clone::Clone;
#[allow(unused_imports)]
use std::{cmp::Ord, mem};

#[derive(Clone, Debug)]
pub enum TreeNode<T: Ord> {
    Leaf,
    Node(T, Box<TreeNode<T>>, Box<TreeNode<T>>),
}

// Provided functions
impl<T: Ord> TreeNode<T> {
    pub fn height(&self) -> usize {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => 1 + std::cmp::max(left.height(), right.height()),
        }
    }

    /// Verifies that the tree is a binary search tree
    fn is_bst(&self) -> bool {
        fn is_bst_helper<T: Ord>(tree: &TreeNode<T>, min: Option<&T>, max: Option<&T>) -> bool {
            match tree {
                TreeNode::Leaf => true,
                TreeNode::Node(value, left, right) => {
                    match min {
                        Some(min) => {
                            if value <= min {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    match max {
                        Some(max) => {
                            if value >= max {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    is_bst_helper(left, min, Some(value)) && is_bst_helper(right, Some(value), max)
                }
            }
        }
        is_bst_helper(self, None, None)
    }

    /// Verifies that the tree is balanced
    pub fn is_balanced(&self) -> bool {
        match self {
            TreeNode::Leaf => true,
            TreeNode::Node(_, left, right) => {
                let left_height = left.height();
                let right_height = right.height();
                let diff = (left_height as i32 - right_height as i32).abs();
                diff <= 1 && left.is_balanced() && right.is_balanced()
            }
        }
    }

    /// Verifies that the tree is a valid balanced binary search tree
    pub fn validate(&self) -> bool {
        self.is_bst() && self.is_balanced()
    }
}

// Required functions
impl<T: Ord> TreeNode<T> {
    /// Creates a new `TreeNode<T>` with value `value` and children `left` and `right`
    pub fn node(value: T, left: TreeNode<T>, right: TreeNode<T>) -> TreeNode<T> {
        TreeNode::Node(value, Box::new(left), Box::new(right))
    }

    /// Creates a new `TreeNode<T>` with no children
    pub fn new() -> TreeNode<T> {
        TreeNode::Leaf
    }

    /// Inserts a new node with value `value` into the tree. If the value already exists in the tree,
    /// the function does nothing.
    ///
    /// After insertion, the tree is rebalanced if necessary
    pub fn insert(&mut self, value: T) {
        match self {
            TreeNode::Leaf => {
                *self = TreeNode::Node(value, Box::new(TreeNode::Leaf), Box::new(TreeNode::Leaf));
            }
            TreeNode::Node(ref mut current_value, ref mut left, ref mut right) => {
                if value < *current_value {
                    left.insert(value);
                } else if value > *current_value {
                    right.insert(value);
                }
            }
        }
        self.rebalance();
    }

    /// Computes the balance factor of the tree (the difference between the height of the left and right subtrees)
    fn balance_factor(&self) -> i32 {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => left.height() as i32 - right.height() as i32,
        }
    }

    /// Performs a left rotation on the tree
    pub fn left_rotate(&mut self) {
        let current_node = std::mem::take(self);

        match current_node {
            TreeNode::Leaf => {
                *self = TreeNode::Leaf;
            }
            TreeNode::Node(value, left, right) => match *right {
                TreeNode::Leaf => {
                    *self = TreeNode::Node(value, left, Box::new(TreeNode::Leaf));
                }
                TreeNode::Node(right_value, right_left, right_right) => {
                    let new_left = TreeNode::Node(value, left, right_left);
                    *self = TreeNode::Node(right_value, Box::new(new_left), right_right);
                }
            },
        }
    }

    /// Performs a right rotation on the tree
    pub fn right_rotate(&mut self) {
        let current_node = std::mem::take(self);

        match current_node {
            TreeNode::Leaf => {
                *self = TreeNode::Leaf;
            }
            TreeNode::Node(value, left, right) => match *left {
                TreeNode::Leaf => {
                    *self = TreeNode::Node(value, Box::new(TreeNode::Leaf), right);
                }
                TreeNode::Node(left_value, left_left, left_right) => {
                    let new_right = TreeNode::Node(value, left_right, right);
                    *self = TreeNode::Node(left_value, left_left, Box::new(new_right));
                }
            },
        }
    }

    /// Rebalances the tree using either a single or double rotation, as specified in the AVL tree
    /// rebalancing algorithm.
    fn rebalance(&mut self) {
        let balance = self.balance_factor();
        if balance > 1 {
            // Left heavy
            if let TreeNode::Node(_, left, _) = self {
                if left.balance_factor() < 0 {
                    // Left-Right case
                    left.left_rotate();
                }
            }
            // Left-Left case
            self.right_rotate();
        } else if balance < -1 {
            // Right heavy
            if let TreeNode::Node(_, _, right) = self {
                if right.balance_factor() > 0 {
                    // Right-Left case
                    right.right_rotate();
                }
            }
            // Right-Right case
            self.left_rotate();
        }
    }
}

// Implement `Default` for `TreeNode<T>`
impl<T: Ord> Default for TreeNode<T> {
    fn default() -> Self {
        TreeNode::Leaf
    }
}

// Implement `PartialEq` for `TreeNode<T>`
// TODO:
impl<T: Ord> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TreeNode::Leaf, TreeNode::Leaf) => true,
            (TreeNode::Node(n1, l1, r1), TreeNode::Node(n2, l2, r2)) => {
                n1 == n2 && l1 == l2 && r1 == r2
            }
            _ => false,
        }
    }
}
// Implement `Eq` for `TreeNode<T>`
// TODO:
impl<T: Ord> Eq for TreeNode<T> {}

// Implement `From<Vec<T>>` for `TreeNode<T>`
// TODO:
impl<T: Ord> From<Vec<T>> for TreeNode<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut tree = TreeNode::new();
        for value in vec {
            tree.insert(value);
        }
        tree
    }
}
// Implement `From<TreeNode<T>>` for `Vec<T>`
// TODO:
impl<T: Ord + Clone> From<TreeNode<T>> for Vec<T> {
    fn from(tree: TreeNode<T>) -> Self {
        let mut vec = Vec::new();
        fn inorder_traversal<T: Ord + Clone>(node: &TreeNode<T>, vec: &mut Vec<T>) {
            match node {
                TreeNode::Leaf => {}
                TreeNode::Node(value, left, right) => {
                    inorder_traversal(left, vec);
                    vec.push(value.clone());
                    inorder_traversal(right, vec);
                }
            }
        }
        inorder_traversal(&tree, &mut vec);
        vec
    }
}

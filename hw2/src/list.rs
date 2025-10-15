use std::collections::hash_map::Values;
#[allow(unused_imports)]
use std::{fmt::Display, mem};
use std::fmt;

#[derive(Debug)]
pub enum ListNode<T> {
    Nil,
    Cons(T, Box<ListNode<T>>),
}

impl<T> ListNode<T> {
  // Use the implementation of this method to guide your implementation of
  // `insert` and `reverse`
  /// Deletes a node from the list
  pub fn delete(&mut self) {
    // Temporarily replaces the current node with default value (Nil).
    // In exchange, we get to take ownership of the current node instead of just
    // having it by mutable reference.
    let as_owned: ListNode<T> = mem::take(self);
    match as_owned {
      ListNode::Nil => {}
      ListNode::Cons(_, next) => {
        // Write the next node to the current node
        *self = *next;
      }
    }
  }
}

// Required methods for `ListNode<T>`
impl<T> ListNode<T> {
    /// Creates a new empty list
    pub fn new() -> Self {
        ListNode::Nil
    }
    pub fn insert(&mut self, value: T) -> &mut Self {
        match self {
            ListNode::Nil => {
                *self = ListNode::Cons(value, Box::new(ListNode::Nil));
                self
            }
            ListNode::Cons(_, next) => {
                let original_next = mem::take(next);

                let new_node = ListNode::Cons(value, original_next);

                *next = Box::new(new_node);

                next.as_mut()
            }
        }
    }


    /// Reverses the list in place.
    pub fn reverse(&mut self) {
        let mut prev = ListNode::Nil;

        let mut current = mem::take(self);

        while let ListNode::Cons(value,next) = current {
            prev = ListNode::Cons(value, (Box::new(prev)));
            current = *next;
        }
        *self = prev;
    }
}

// Implement `Default` for `ListNode<T>`
impl<T> Default for ListNode<T> {
    fn default() -> Self {
        ListNode::Nil
    }
}

// Implement `PartialEq` for `ListNode<T>`
// TODO:
impl<T: PartialEq> PartialEq for ListNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ListNode::Nil, ListNode::Nil) => true,
            (ListNode::Nil, ListNode::Cons(_, _)) => false,
            (ListNode::Cons(_, _), ListNode::Nil) => false,
            (ListNode::Cons(a, next_a), ListNode::Cons(b, next_b)) => {
                a == b && next_a == next_b
            }
        }
    }
}
// Implement `Eq` for `ListNode<T>`
impl<T: PartialEq> Eq for ListNode<T> {}

impl<T: Display> Display for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ListNode::Nil => write!(f, "Nil"),
            ListNode::Cons(val, next) => {
                // Write the current value, then recursively call on the next node
                write!(f, "{} -> {}", val, next)
            }
        }
    }
}

// Implement `From<Vec<T>>` for `ListNode<T>`
impl<T> From<Vec<T>> for ListNode<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut list = ListNode::new();
        for item in vec.into_iter().rev() {
            let new_head = ListNode::Cons(item, Box::new(list));
            list = new_head;
        }
        list
    }
}
// Implement `From<ListNode<T>>` for `Vec<T>`
impl<T> From<ListNode<T>> for Vec<T> {
    fn from(mut list: ListNode<T>) -> Self {
        let mut vec = Vec::new();
        while let ListNode::Cons(val, next) = list {
            vec.push(val); 
            list = *next;
        }
        vec
    }
}

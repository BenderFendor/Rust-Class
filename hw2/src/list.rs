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
    /// Inserts a new list node with value `value` after `self` and returns a reference to the new
    /// node
    pub fn insert(&mut self, value: T) -> &mut Self {
        &mut ListNode::Cons(value, (Box::<ListNode<self>>))    
    }
    /// Reverses the list in place.
    pub fn reverse(&mut self) {
        todo!()
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

// Implement `Eq` for `ListNode<T>`
// TODO:

// Implement `Display` for `ListNode<T>`
// TODO:
impl <T> Display for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result 
    {
        write!(f, "{} -> {}", self.value,self.next)
    }
}
// Implement `From<Vec<T>>` for `ListNode<T>`
// TODO:

// Implement `From<ListNode<T>>` for `Vec<T>`
// TODO:

use std::cell::RefCell;
use std::num::IntErrorKind;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{clone, thread};

// Node for Questions 3-5.
// It uses Rc<RefCell<...>> for shared, optional, and mutable pointers.
struct Node {
    pub value: i32,
    pub next: Option<Rc<RefCell<Node>>>,
}

fn new_shared_node(value: i32) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node { value, next: None }))
}

/// Q1: Shared Ownership
fn q1_shared_rc_count() -> usize {
    // TODO:
    // 1. Create a `shared_value` usize (e.g., 42).
    // 2. Wrap it in Rc.
    // 3. Clone the Rc (creating a second owner).

    
    let shared_value = Rc::new(RefCell::new(20));

    let copy_value = shared_value.clone();

    // 4. Uncomment this (Return the number of things that own your shared value)
    Rc::strong_count(&shared_value)
}

#[cfg(test)]
#[test]
fn test_q1_shared_rc_count() {
    assert_eq!(
        q1_shared_rc_count(),
        2,
        "Q1 failed: Should have 2 strong owners."
    );
}

/// Q2: Interior Mutability
fn q2_interior_mutability() -> i32 {
    // 1. Create RefCell<i32> with initial value 5.
    // 2. Use `borrow_mut()` to get a mutable reference.
    // 3. Change the value to 100.
    // 4. Return the new value (by using `borrow()` (borrowing immutably) after the mutable borrow drops).
    
    let interior_mut = Rc::new(RefCell<i32>::new(5));

    interior_mut.borrow_mut() = 100;

}

#[cfg(test)]
#[test]
fn test_q2_interior_mutability() {
    assert_eq!(
        q2_interior_mutability(),
        100,
        "Q2 failed: Inner value was not mutated correctly."
    );
}

/// Q3: Shared Node Linkage
fn q3_node_linkage() -> usize {
    let c = new_shared_node(30); // Rc count 1
    let b = new_shared_node(20); // Rc count 1
    let a = new_shared_node(10); // Rc count 1

    // TODO: Set A's next pointer to B, and B's next pointer to C.
    todo!()

    // Return the owners of Node B.
    //Rc::strong_count(&b)
}

#[cfg(test)]
#[test]
fn test_q3_node_linkage() {
    assert_eq!(
        q3_node_linkage(),
        2,
        "Q3 failed: Node B should have 2 strong owners."
    );
}

/// Q4: Memory Leak (Reference Cycle)
fn q4_reference_cycle() -> usize {
    let a = new_shared_node(10); // Rc count 1
    let b = new_shared_node(20); // Rc count 1

    // TODO: Create a cycle: Node A points to B, and Node B points back to A.
    todo!()

    // Return owners of Node a
    //Rc::strong_count(&a)
}

#[cfg(test)]
#[test]
fn test_q4_reference_cycle() {
    assert_eq!(
        q4_reference_cycle(),
        2,
        "Q4 failed: Node A should have 2 strong owners due to the cycle."
    );
}

/// Q5: Mutating Linked Node Value
fn q5_mutate_linked_node_value() -> i32 {
    let b = new_shared_node(20);
    let a = new_shared_node(10);

    todo!()
    // TODO: Using only the handle to Node A, mutate Node B's value from 20 to 999.
    // 1. Link A -> B

    // 2. Get the Rc<RefCell<Node>> for B using A's link.
    // Hint: you will need to `borrow` a, get it's `next` field, and `unwrap` it

    // 3. Mutate B's value via the acquired handle.

    // 4. Uncomment to return B's final value using the original 'b' variable.
    // b.borrow().value
}

#[cfg(test)]
#[test]
fn test_q5_mutate_linked_node_value() {
    assert_eq!(
        q5_mutate_linked_node_value(),
        999,
        "Q5 failed: Node B's value should be 999 after mutation via Node A."
    );
}

/// Q6: Don't Forget about Plain Mutable References
fn q6_plain_mutable_ref() -> i32 {
    // Helper function that takes a plain mutable reference.
    fn increment_by_one(n: &mut i32) {
        *n += 1;
    }

    todo!()
    // TODO:
    // 1. Create a normal mutable value (no Rc/RefCell). Set it to be 10.

    // 2. Pass a mutable reference to the helper function twice.

    // 3. Return the final value.
}

#[cfg(test)]
#[test]
fn test_q6_plain_mutable_ref() {
    assert_eq!(
        q6_plain_mutable_ref(),
        12,
        "Q6 failed: The value should be 12 after two increments."
    );
}

/// Q7: Mutation via Function Taking Immutable Rc Handle
fn q7_mutate_via_immutable_handle() -> i32 {
    // Helper function demonstrating interior mutability using an immutable handle.
    fn mutate_value(data: &Rc<RefCell<i32>>) {
        // We can call borrow_mut() on the RefCell despite only having an immutable reference to the Rc.
        *data.borrow_mut() = 404;
    }

    todo!();
    // 1. Create the shared, mutable data: Wrap any value you want (not 404) in a Rc<RefCell<...>>

    // 2. Pass an immutable reference of the handle to the mutating function.

    // 3. Return the mutated value (Use `borrow`!).
}

#[cfg(test)]
#[test]
fn test_q7_mutate_via_immutable_handle() {
    assert_eq!(
        q7_mutate_via_immutable_handle(),
        404,
        "Q7 failed: Inner value should be 404 after mutation."
    );
}

/// Q8: Thread-Safe Mutability
fn q8_thread_safe_counter() -> i32 {
    const NUM_THREADS: usize = 4;

    // Create the counter wrapped in Mutex for mutability and Arc for thread-safety.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..NUM_THREADS {
        // TODO: Clone the Arc for each thread.
        let counter_clone: Arc<Mutex<i32>> = todo!();

        let handle = thread::spawn(move || {
            for _ in 0..10 {
                // Lock the Mutex to get mutable access to the inner value.
                let mut num = counter_clone.lock().unwrap();

                // Mutate the inner value.
                *num += 1;

                // The lock is automatically released when 'num' goes out of scope. Yay Rust!
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }

    // Lock the final counter and return the value.
    *counter.lock().unwrap()
}

// CHALLENGE 1: When you are done with this question and your test passes, try changing the function
// so that one of the four threads runs 200 times, while the other three just run 10. You already
// have all the tools you need to do this!

// CHALLENGE 2: Put a struct inside the Arc<Mutex<...>> instead of a value.

#[cfg(test)]
#[test]
fn test_q8_thread_safe_counter() {
    let expected = 4 * 10; // 4 threads * 10 increments
    let actual = q8_thread_safe_counter();
    assert_eq!(
        actual, expected,
        "Q8 failed: Final counter value should be 40."
    );
}

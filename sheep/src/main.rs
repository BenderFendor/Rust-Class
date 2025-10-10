// 🐑 Rust Ranch Lab
//
// INSTRUCTIONS:
// 1. Fill in the required Rust code under the "// Implement your solution here" comment for each question.
// 2. Ensure all provided tests pass when running the file.
//    (Compile and run with: rustc --test ranch_lab.rs && ./ranch_lab)
//

// Question 1: Shearing Sheep
// -----------------------------------------------------------------------------
enum ShearStatus {
    Ready,
    RecentlySheared,
    NeedsAttention,
}

// TODO: Complete this function using a match statement
// return a String based on the ShearStatus variant.
fn assess_sheep(status: ShearStatus) -> String {
    match status {
        ShearStatus::Ready => String::from("Time to shear this one!"),
        ShearStatus::NeedsAttention => String::from("Check for minor injuries"),
        ShearStatus::RecentlySheared => String::from("Leave it be for now."),
    }
}

#[test]
fn test_q1_basic_enum() {
    assert_eq!(assess_sheep(ShearStatus::Ready), "Time to shear this one!");
    assert_eq!(
        assess_sheep(ShearStatus::RecentlySheared),
        "Leave it be for now."
    );
    assert_eq!(
        assess_sheep(ShearStatus::NeedsAttention),
        "Check for minor injuries."
    );
}

// Question 2: Herding Sheep
// -----------------------------------------------------------------------------
enum HerderCommand {
    Sleep,
    Whistle,
    Move(i32, i32), // Holds (dx, dy)
}

// TODO: Complete this function. Use `if let` to check for the `Move(dx, dy)` variant.
// If it's a Move, return the sum of the absolute changes (|dx| + |dy|). Otherwise, return 0.
fn apply_move(cmd: HerderCommand) -> i32 {
    if let HerderCommand::Move(dx, dy) = cmd {
        dx.abs() + dy.abs()
    } else {
        0
    }
}

#[test]
fn test_q2_enum_data_if_let() {
    assert_eq!(apply_move(HerderCommand::Move(5, -2)), 7); // 5 + 2 = 7
    assert_eq!(apply_move(HerderCommand::Whistle), 0);
}

// Question 3: Getting the Front Sheep
// -----------------------------------------------------------------------------

// TODO: Make this function generic over type T, and return a reference to the first element.

fn get_first_sheep <T>(herd: &[T]) -> &T {
    &herd[0]
}


// TODO: Uncomment the test

#[test]
fn test_q3_basic_generics() {
    let sheep_ids = vec![1001, 1002, 1003];
    assert_eq!(*get_first_sheep(&sheep_ids), 1001);

    let names = vec!["Baa", "Lamby", "Ram"];
    assert_eq!(*get_first_sheep(&names), "Baa");
}


// Question 4: Finding Twin Sheep
// -----------------------------------------------------------------------------

// TODO: Make this function generic over T. T must implement `PartialEq` to allow comparison.

fn is_twin<T: PartialEq>(tag_a: &T, tag_b: &T) -> bool {
    tag_a == tag_b
}


// TODO: Uncomment the test

#[test]
fn test_q4_generic_trait_bound() {
    assert_eq!(is_twin(&101, &101), true);
    assert_eq!(is_twin(&101, &102), false);
    assert_eq!(is_twin(&"ID-A", &"ID-A"), true);
}

// Question 5: Backup Tags for Sheep
// -----------------------------------------------------------------------------

// TODO: Make this function generic over T. T must implement `Copy` to allow dereferencing/copying.

fn backup_tag<T: Copy>(original_tag_ref: &T) -> T {
    *original_tag_ref
}

#[test]
fn test_q5_copy_trait_bound() {
    let original_id = 42;
    let backup_id = backup_tag(&original_id);
    assert_eq!(backup_id, 42);

    // The original is still usable because it was copied, not moved.
    let _ = original_id + 1;
}


// Question 6
// -----------------------------------------------------------------------------
use std::fmt::Display;

struct SheepCrate<T> {
    contents: T,
}

impl<T> SheepCrate<T> {
    fn load(item: T) -> Self {
        SheepCrate { contents: item }
    }

    fn unload(&self) -> &T {
        &self.contents
    }
}

// Trait to implement
trait WoolCountable {
    fn report_wool_weight(&self) -> String;
}

impl <T: Display> WoolCountable for SheepCrate<T> {
    fn report_wool_weight(&self) -> String
    {
        format!("Weight of crate is {}",self.contents)
    }
}

// TODO: Implement WoolCountable for SheepCrate<T> only when T implements the Display trait.

// Question 7: Notice Q6 has no test!
// -----------------------------------------------------------------------------

#[test]
fn test_q7_conditional_impl() {
    // TODO: make a SheepCrate with a type inside that impls Display
    // (Hint: f64 impls Display. A Vector of usize also impls Display)

    // TODO: load an item into your sheep crate

    // TODO: assert that calling report_wool_weight on your sheep crate returns
    // "Crate weight recorded: <item>"

    // TODO: remove panic
    panic!()
}

fn main() {}

use std::f64::consts::PI;

pub trait Area {
    fn calculate_area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

// TODO: Implement the `Area` trait for both structs.
// For Circle, use the formula PI * r^2.
// For Rectangle, use the formula width * height.
impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        PI * (self.radius * self.radius)
    }
}

impl Area for Rectangle {
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }
}

// TODO: Create a generic function `print_area` that prints the area.
pub fn print_area<T: Area>(shape: &T) {
  println!("The area of this shape is {}",shape.calculate_area()) 
}

fn main() {
    let my_circle = Circle { radius: 5.0 };
    let my_rectangle = Rectangle {
        width: 10.0,
        height: 7.5,
    };

    println!("Calculating area for the circle...");
    print_area(&my_circle);

    println!("\nCalculating area for the rectangle...");
    print_area(&my_rectangle);
}

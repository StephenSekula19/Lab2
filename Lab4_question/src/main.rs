// Defining a struct named Rectangle with two fields, width and height
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Constructor to calculate a new Rectangle
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // Method to calculate the area of the rectangle
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Method to calculate the perimeter of the rectangle
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    // Method to check if the rectangle is a square
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// Defining a struct named Circle with a single field, radius
struct Circle {
    radius: f64,
}

impl Circle {
    // Constructor to create a new Circle
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    // Method to calculate the area of the circle
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }

    // Method to calculate the circumference of the circle
    fn circumference(&self) -> f64 {
        2.0 * 3.14 * self.radius
    }
}

fn main() {
    // Create a rectangle with width 10 and height 5
    let rect = Rectangle::new(10.0, 5.0);

    // Print area of the rectangle
    println!("Area: {}", rect.area());

    // Print perimeter of the rectangle
    println!("Perimeter: {}", rect.perimeter());

    // Print whether the rectangle is a square
    println!("Is square? {}", rect.is_square());

    // Test cases:
    // A rectangle with equal sides should be a square
    assert!(Rectangle::new(5.0, 5.0).is_square());

    // A rectangle with unequal sides should not be a square
    assert!(!Rectangle::new(5.0, 6.0).is_square());

    // Create a Circle with radius 4.0
    let circle = Circle::new(4.0);

    // Print area of the circle
    println!("Circle Area: {}", circle.area());

    // Print circumference of the circle
    println!("Circle Circumference: {}", circle.circumference());
}
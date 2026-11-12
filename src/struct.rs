// Working with structs in Rust is a fundamental part of the language,
// allowing you to create complex data types that can hold multiple values. 
// Structs are defined using the `struct` keyword, followed by the name of the struct and its fields.
// Each field has a name and a type.
// solve the problem of rectangle area and perimeter using struct in rust 
// solve the problem of the circle area and circumference using struct in rust

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}
impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

pub struct Circle {
    pub radius: f64,
}
impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}
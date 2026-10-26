pub fn variable() {
    mutable_variable(); // we can have mutable variables using the mut keyword 
    immutable_variable(); // we can have immutable variables by default in rust, we can not change the value of an immutable variablea
}
fn mutable_variable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
fn immutable_variable() {
    let x = 5;
    println!("The value of x is: {}", x);
}
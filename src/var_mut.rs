pub fn variable() {
    mutable_variable(); // we can have mutable variables using the mut keyword 
    immutable_variable(); // we can have immutable variables by default in rust, we can not change the value of an immutable variable
    Const_variable(); // we can have constant variables using the const keyword, we can not change the value of a constant variable
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
fn Const_variable() { // we can not use mut with the const keyword 
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
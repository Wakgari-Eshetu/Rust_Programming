pub fn operation() {
    addition();
    subtraction();
    multiplication();
    division();
    remainder();
}

fn addition() {
    let x: i32 = 5;
    let y: i32 = 10;
    let z: i32 = x + y;
    println!("The value of z is: {}", z);
}
fn subtraction() {
    let x: i32 = 10;
    let y: i32 = 5;
    let z: i32 = x - y;
    println!("The value of z is: {}", z);
}
fn multiplication() {
    let x: i32 = 5;
    let y: i32 = 10;
    let z: i32 = x * y;
    println!("The value of z is: {}", z);
}
fn division() {
    let x: i32 = 10;
    let y: i32 = 5;
    let z: i32 = x / y;
    println!("The value of z is: {}", z);
}
fn remainder() {
    let x: i32 = 10;
    let y: i32 = 3;
    let z: i32 = x % y;
    println!("The value of z is: {}", z);
}
pub fn function(){
    one();
    two( 4 , 5);
    three(10, 20);
}

fn one() {
    println!("This is function one");
}

fn two(x: i32, y: i32) {
    let z = x + y;
    println!("The value of z is: {}", z);
}   

fn three(x: i32, y: i32) -> i32 {
    return (x + y) as i32;
}
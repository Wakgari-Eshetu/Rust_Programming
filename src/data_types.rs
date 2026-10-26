pub fn data_types() {
    integer_data_type();
    float_data_type();
    boolean_data_type();
    character_data_type();
    string_data_type();
}


fn integer_data_type() { // 'i' is for the signed integer type (zero and positive) and 'u' is for the unsigned integer type(negative , zero, positive), we can use the 'i' and 'u' keywords to specify the type of the integer variable
    let x: i32 = 5; // i32 is the default integer type in rust
    println!("The value of x is: {}", x);
}

fn float_data_type() { // 'f' is for the floating point type, we can use the 'f' keyword to specify the type of the floating point variable
    let x: f32 = 5.0; // f32 is the default floating point type in rust
    println!("The value of x is: {}", x);
}
fn boolean_data_type() { // 'bool' is for the boolean type, we can use the 'bool' keyword to specify the type of the boolean variable
    let x: bool = true; // bool is the default boolean type in rust ethier True or false 
    println!("The value of x is: {}", x);
}
fn character_data_type() { // 'char' is for the character type, we can use the 'char' keyword to specify the type of the character variable
    let x: char = 'a'; // char is the default character type in rust
    println!("The value of x is: {}", x);
}
fn string_data_type() { // 'String' is for the string type, we can use the 'String' keyword to specify the type of the string variable
    let x: String = "Hello, Rust!".into(); // String is the default string type in rust
    println!("The value of x is: {}", x);
}
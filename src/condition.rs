/* Conditionals in Rust 
there are two main types of conditionals in Rust: if expressions and match expressions 
if expressions are used to execute code based on whether a condition is true or false 
and thier syntax is as follows: */
pub fn condition() {
    let x = 12;
    if x < 10 {
        println!("x is less than 10");
    } else if x == 10 {
        println!("x is equal to 10");
    }else{
        println!("x is greater than 10");
    }
}
// In the conditions rust expects a boolean value, so we can use the comparison operators to compare values and return a boolean value.

// lets look at the match expression, which is used to compare a value against a series of patterns and execute code based on which pattern matches. 
// The syntax for a match expression is as follows:

pub fn match_expression() {
    let x = 12;
    match x {
        1 => println!("x is equal to 1"),
        2 => println!("x is equal to 2"),
        3 => println!("x is equal to 3"),
        _ => println!("x is not equal to 1, 2, or 3"),
    }
}
pub fn refrence(){
    refrence_1();
}
fn calculate_length(s: &String) -> usize { // taking refrence of s as parameter
    s.len() // returning length of s
}
fn refrence_1(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // passing refrence of s1 to calculate_length function
    println!("The length of '{}' is {}.", s1, len);
} 
// when having the reference we can not chnage the value of the variable but we can read it and use it in our program
fn print_value(s: &mut String) -> String{ // taking mutable refrence of s as parameter
    s.push_str(", world"); // changing the value of s
    return s.to_string(); // returning the value of s
}
fn muatable_refrence(){
    let mut s1 = String::from("hello");
    let len = print_value(&mut s1); // passing mutable refrence of s1 to calculate_length function
    return len;
}

// Now using muatable refrence we can change the value of the variable and use it in our program.
// but not more than one mutable refrence to a variable in a particular scope.
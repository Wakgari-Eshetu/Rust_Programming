pub fn ownership() {
    let s1 = 12; // since int , bool , float , char ,  tuples and arrays having int , float and bool , char
    let s2 = s1; // can support coping so s1 is still valid after this line 
    
    let s3 = String::from("hello"); // String is a heap allocated data type so it does not support coping
    let s4 = s3; // s3 is moved to s4 and s3 is no longer valid after this line

    let s4 = s4.clone(); // cloning s4 to s4 so both are valid after this line
    let s5 = function_ownership(s4); // s4 is moved to function_ownership and is no longer valid after this line
    let s3 = function_ownership_1(s3);
    }

fn function_ownership(s: String){
    println!("The value of s is: {}", s); // this function takes ownership of s and prints it so s4 is nolonger valid after this line
 }

fn function_ownership_1(s: i32)-> i32{
    return s // this function takes ownership of s3  and returns it so s3 is nolonger valid after this line
 }

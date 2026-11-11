pub fn ownership() {
    let s1 = 12; // since int , bool , float , char ,  tuples and arrays having int , float and bool , char
    let s2 = s1; // can support coping so s1 is still valid after this line 
    
    let s3 = String::from("hello"); // String is a heap allocated data type so it does not support coping
    let s4 = s3; // s3 is moved to s4 and s3 is no longer valid after this line
    }


pub fn handling_overflow_intergers() {
    wrapping();
    staturated();
    overflow();
    checked();
}
/*  
The formula behind the wrapping method is: 
    let z = x + y;
    if z > max_value {
        z = min_value + (z - max_value - 1);

    }
    or z = (x + y) % (max_value + 1);   also the formula behind the method.
 */
fn wrapping() { 
    let x: u8 = 255;
    let y: u8 = 10;
    let z: u8 = x.wrapping_add(y);
    println!("The value of z is: {}", z);
}
/* The saturating method returns the saturated value if the operation would have overflowed. 
what is saturated? value means the maximum or minimum value that can be represented by the data type
when will be the maximum or minimum value? min value when it underflows, max value when it overflows */
fn staturated() { 
    let x: u8 = 255;
    let y: u8 = 10;
    let z: u8 = x.saturating_add(y);
    println!("The value of z is: {}", z);
}
/* The overflowing method returns a tuple (z, overflowed) where overflowed is a boolean indicating whether the operation overflowed. */
fn overflow() { 
    let x: u8 = 255;
    let y: u8 = 10;
    let z: u8 = x.overflowing_add(y).0;
    let overflowed: bool = x.overflowing_add(y).1;
    println!("The value of z is: {}", z);
    println!("Did overflow occur? {}", overflowed);
}
/* The checked method returns an Option<u8> which is Some(z) if the operation succeeded or None if it overflowed. */
fn checked() { 
    let x: u8 = 255;
    let y: u8 = 10;
    let z: Option<u8> = x.checked_add(y);
    match z {
        Some(z) => println!("The value of z is: {}", z),
        None => println!("Overflow occurred"),
    }
}


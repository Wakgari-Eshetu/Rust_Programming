/* Write a function that takes a string of words separated by spaces and returns the first
 word it finds in that string. If the function doesn't find a space in the string,
 the whole string must be one word, so the entire string should be returned. */
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // converting string to bytes
    for (i, &item) in bytes.iter().enumerate() {
        // iterating through the bytes
        if item == b' ' {
            // splitting on an ASCII space is safe: it never lands inside a UTF-8 character
            return &s[0..i]; // returning the first word
        }
    }
    s
}

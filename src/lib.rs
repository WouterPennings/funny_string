/// Returns the character at the index of the given string.
/// The first character of **string** is at index 0
///
/// # Example:
/// ```rust
///  let input: &str = "Hello, World!";
///  let result: char = funny_string::index_string(input, 4);
///  // result now contains the char 'o'
/// ```
pub fn index_string(string: &str, index: i32) -> char {
    return string.chars().nth(index as usize).unwrap();
}

/// Returns the substring between the given **begin** index and **end** index.
/// The first character of **string** is at index 0
///  - **begin** --> inclusive
///  - **end** --> exclusive
///
/// # Example:
/// ```rust
///  let input: &str = "Hello, World!";
///  let result: String = funny_string::substring(input, 2, 6);
///  // result now contains the string "llo,"
/// ```
pub fn substring(string: &str, begin: i32, end: i32) -> String {
    return string.chars().skip(begin as usize).take((end - begin) as usize).collect();
}

/// Replaces the character at the given index and returns the string.
///
/// # Example:
/// ```rust
///  let input: &str = "Hello, World!";
///  let result: String = funny_string::replace_char(input, 3, 'X');
///  // result now contains the string "HelXo, World!"
/// ```
pub fn replace_char(string: &str, index: i32, replacement: char) -> String {
    let index: usize = index as usize;
    let first = &string[..index];
    let second = &string[index + 1..];
    return format!("{}{}{}", first, replacement, second);
}

/// Replaces a range of characters in **string** with the given **replacement**.
/// The range is **begin** to **end**.
///
///  - **begin** --> inclusive
///  - **end** --> exclusive
///
/// The string with the replaced substring gets returned.
///
/// # Example:
/// ```rust
///  let input: &str = "Hello, World!";
///  let result: String = funny_string::replace_substring(input, 1, 5, 'X');
///  // result now contains the string "HXXXX, World!"
/// ```
pub fn replace_substring(string: &str, begin: i32, end: i32, replacement: char) -> String {
    let begin: usize = begin as usize;
    let end: usize = end as usize;
    let mut replacement_substring = "".to_string();
    for _ in 0..end-begin {
        replacement_substring.push(replacement);
    }
    let first = &string[..begin];
    let second = &string[end..];
    return format!("{}{}{}", first, replacement_substring, second);
}

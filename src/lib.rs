pub fn index_string(string: &str, index: i32) -> char {
    return string.chars().nth(index as usize).unwrap();
}

pub fn substring(string: &str, begin: i32, end: i32) -> String {
    return string.chars().skip(begin as usize).take((end) as usize).collect();
}

pub fn replace_char(string: &str, index: i32, replacement: char) -> String {
    let x: usize = index as usize;
    let first = &string[..x];
    let second = &string[x + 1..];
    return format!("{}{}{}", first, replacement, second);
}

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

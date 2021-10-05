fn main() {

}

fn get_char_from_string(string: &str, index: i32) -> char {
    return string.chars().nth(index as usize).unwrap();
}

fn get_substring(mut text: &str, begin: i32, end: i32) -> String {
    return text.chars().skip(begin as usize).take((end) as usize).collect();
}
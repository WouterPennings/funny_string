extern crate funny_string as helper;

#[test]
fn index_string_1() {
    let input = "hello";
    let result = helper::index_string(input, 2);
    assert_eq!('l', result)
}

#[test]
fn index_string_2() {
    let input = "hello";
    let result = helper::index_string(input, 4);
    assert_eq!('o', result)
}

#[test]
fn substring_1() {
    let input = "Hello, World!";
    let result = helper::substring(input, 2, 6);
    println!("{}", result);
    assert_eq!("llo,", result)
}

#[test]
fn substring_2() {
    let input = "hello";
    let result = helper::substring(input, 0, 1);
    assert_eq!("h", result)
}

#[test]
fn substring_3() {
    let input = "hello";
    let result = helper::substring(input, 0, 0);
    assert_eq!("", result)
}

#[test]
fn replace_char_1() {
    let input = "hello";
    let result = helper::replace_char(input, 0, 'x');
    assert_eq!("xello", result);
}

#[test]
fn replace_char_2() {
    let input = "hello";
    let result = helper::replace_char(input, 0, ' ');
    assert_eq!(" ello", result);
}

#[test]
fn replace_char_3() {
    let input = "hello";
    let result = helper::replace_char(input, 0, '\\');
    assert_eq!("\\ello", result);
}

#[test]
fn replace_substring_1() {
    let input = "hello";
    let result = helper::replace_substring(input, 1, 3, ' ');
    assert_eq!("h  lo", result);
}

#[test]
fn replace_substring_2() {
    let input = "hello";
    let result = helper::replace_substring(input, 0, 5, ' ');
    assert_eq!("     ", result);
}

#[test]
fn replace_substring_3() {
    let input = "hello";
    let result = helper::replace_substring(input, 0, 0, ' ');
    assert_eq!("hello", result);
}
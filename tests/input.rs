extern crate aoc_util;

use aoc_util::input::{FileReader, FromFile};

#[test]
fn to_string() {
    let input: String = FileReader::new()
        .read_from_file("tests/inputs/string_input.txt")
        .unwrap();
    assert_eq!("This is a string.", input);
}

#[test]
fn to_string_trim() {
    let input: String = FileReader::new()
        .trim()
        .read_from_file("tests/inputs/string_input_trim.txt")
        .unwrap();
    assert_eq!("This is a string.", input);
}

#[test]
fn to_string_no_trim() {
    let input: String = FileReader::new()
        .read_from_file("tests/inputs/string_input_trim.txt")
        .unwrap();
    assert_eq!("  This is a string.\n", input);
}

#[test]
fn newline_delimited_numbers() {
    let input: Vec<u32> = FileReader::new()
        .split_lines()
        .read_from_file("tests/inputs/newline_delimited.txt")
        .unwrap();
    assert_eq!(vec![4, 8, 15, 16, 23, 42], input);
}

#[test]
fn newline_delimited_strings() {
    let input: Vec<String> = FileReader::new()
        .split_lines()
        .read_from_file("tests/inputs/newline_delimited_string_trim.txt")
        .unwrap();
    assert_eq!(vec!["A", " BC", "D  "], input);
}

#[test]
fn newline_delimited_strings_trim() {
    let input: Vec<String> = FileReader::new()
        .split_lines()
        .trim()
        .read_from_file("tests/inputs/newline_delimited_string_trim.txt")
        .unwrap();
    assert_eq!(vec!["A", "BC", "D"], input);
}

#[test]
fn whitespace_delimited_numbers() {
    let input: Vec<u32> = FileReader::new()
        .split_whitespace()
        .read_from_file("tests/inputs/whitespace_delimited.txt")
        .unwrap();
    assert_eq!(vec![4, 8, 15, 16, 23, 42], input);
}

use std::path::PathBuf;

use fake::{
    faker::{lorem::en::Paragraph, name::raw::Name},
    locales::EN,
    Fake,
};
use wc_rust::{count, entity::CountFlags};

struct TestData {
    file_name: PathBuf,
    content: String,
    count_flags: CountFlags,
    expected_output: Vec<u8>,
}

fn generate_test_data(
    count_byte: bool,
    word_count: bool,
    line_count: bool,
    character_count: bool,
) -> TestData {
    let file_name = PathBuf::from(Name(EN).fake::<String>());
    let content: String = Paragraph(10..100).fake();

    let all_flags: bool = !(count_byte || line_count || word_count || character_count);

    let mut count_str = String::new();

    if count_byte || all_flags {
        count_str += &format!("{} Bytes ", &content.len());
    }
    if line_count || all_flags {
        count_str += &format!("{} Lines ", &content.lines().count());
    }
    if word_count || all_flags {
        count_str += &format!(
            "{} Words ",
            &content
                .lines()
                .map(|line| line.split_whitespace().count())
                .count()
        );
    }
    if character_count || all_flags {
        count_str += &format!("{} Characters ", &content.chars().count());
    }

    count_str += "\n";

    TestData {
        file_name: file_name.clone(),
        content,
        count_flags: CountFlags {
            count_byte,
            word_count,
            line_count,
            character_count,
        },
        expected_output: format!("File '{}': {}", &file_name.display(), count_str).into_bytes(),
    }
}

#[test]
fn test_count_all_flags() {
    let test_data: TestData = generate_test_data(true, true, true, true);

    let mut actual_result: Vec<u8> = Vec::new();

    count(
        &test_data.file_name,
        &test_data.content,
        &test_data.count_flags,
        &mut actual_result,
    );

    assert_eq!(actual_result, test_data.expected_output);
}

#[test]
fn test_count_byte_flag() {
    let test_data: TestData = generate_test_data(true, false, false, false);

    let mut actual_result: Vec<u8> = Vec::new();

    count(
        &test_data.file_name,
        &test_data.content,
        &test_data.count_flags,
        &mut actual_result,
    );

    assert_eq!(actual_result, test_data.expected_output);
}

#[test]
fn test_count_word_flag() {
    let test_data: TestData = generate_test_data(false, true, false, false);

    let mut actual_result: Vec<u8> = Vec::new();

    count(
        &test_data.file_name,
        &test_data.content,
        &test_data.count_flags,
        &mut actual_result,
    );

    assert_eq!(actual_result, test_data.expected_output);
}

#[test]
fn test_count_line_flag() {
    let test_data: TestData = generate_test_data(false, false, true, false);

    let mut actual_result: Vec<u8> = Vec::new();

    count(
        &test_data.file_name,
        &test_data.content,
        &test_data.count_flags,
        &mut actual_result,
    );

    assert_eq!(actual_result, test_data.expected_output);
}

#[test]
fn test_count_character_flag() {
    let test_data: TestData = generate_test_data(false, false, false, true);

    let mut actual_result: Vec<u8> = Vec::new();

    count(
        &test_data.file_name,
        &test_data.content,
        &test_data.count_flags,
        &mut actual_result,
    );

    assert_eq!(actual_result, test_data.expected_output);
}

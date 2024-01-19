/// Implements the logic for Counting

use std::path::PathBuf;

use entity::CountFlags;

pub mod entity;

pub struct Count {
    pub byte: usize,
    pub line: usize,
    pub word: usize,
    pub char: usize,
}

impl Count {
    pub fn print(
        &self,
        file_name: &PathBuf,
        count_flags: &CountFlags,
        mut output_file: impl std::io::Write,
    ) {
        let mut output_str = String::new();

        let all_flags: bool = !(count_flags.count_byte || count_flags.line_count || count_flags.word_count || count_flags.character_count);

        if count_flags.count_byte || all_flags{
            output_str += &format!("{} Bytes ", self.byte);
        }
        if count_flags.line_count || all_flags {
            output_str += &format!("{} Lines ", self.line);
        }
        if count_flags.word_count  || all_flags{
            output_str += &format!("{} Words ", self.word);
        }
        if count_flags.character_count || all_flags{
            output_str += &format!("{} Characters ", self.char);
        }

        writeln!(
            output_file,
            "File '{}': {}",
            file_name.display(),
            output_str
        )
        .unwrap();
    }
}

pub fn count(
    file_name: &PathBuf,
    content: &String,
    count_flags: &entity::CountFlags,
    output_file: impl std::io::Write,
) {
    let count = Count {
        byte: content.len(),
        line: content.lines().count(),
        word: content
            .lines()
            .map(|line| line.split_whitespace().count())
            .count(),
        char: content.chars().count(),
    };

    count.print(file_name, count_flags, output_file)
}

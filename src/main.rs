/// Command Line Tool to Count Bytes, Words, Lines and Characters in file(s)


use std::{
    fs::File,
    io::{stdin, BufReader},
    path::PathBuf,
};

use clap::Parser;

mod entity;
use crate::entity::InputArgs;

use wc_rust::entity::CountFlags;

mod file_process;
use file_process::process_file;

fn process_stdin(count_flags: &CountFlags) {
    process_file(
        &PathBuf::from(""),
        BufReader::new(stdin()),
        &count_flags,
        &None,
    )
}

fn process_input_files(
    files: Vec<PathBuf>,
    count_flags: &CountFlags,
    output_file: &Option<PathBuf>,
) {
    for file_name in files {
        if file_name == PathBuf::from("-") {
            process_stdin(&count_flags);
        } else {
            match File::open(&file_name) {
                Ok(file) => process_file(
                    &file_name,
                    BufReader::new(&file),
                    &count_flags,
                    &output_file,
                ),
                Err(e) => {
                    eprintln!("Error While Opening File '{}': {}", file_name.display(), e)
                }
            }
        }
    }
}

fn main() {
    let args = InputArgs::parse();

    let count_flags = CountFlags {
        count_byte: args.count_byte,
        word_count: args.word_count,
        line_count: args.line_count,
        character_count: args.character_count,
    };

    match args.files {
        Some(files) => process_input_files(files, &count_flags, &args.output),
        None => process_stdin(&count_flags),
    };
}

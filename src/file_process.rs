use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;

use wc_rust::count;

use wc_rust::entity::CountFlags;

pub fn process_file<R: BufRead>(
    file_name: &PathBuf,
    mut buf_reader: R,
    count_flags: &CountFlags,
    output_file: &Option<PathBuf>,
) {
    let mut content = String::new();

    if let Err(e) = buf_reader.read_to_string(&mut content) {
        eprintln!("Error While Processing {}: {}", file_name.display(), e);
    } else {
        match output_file {
            Some(path) => count(
                &file_name,
                &content,
                &count_flags,
                File::create(path).unwrap(),
            ),
            None => count(&file_name, &content, &count_flags, io::stdout()),
        }
    }
}

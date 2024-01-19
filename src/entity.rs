use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct InputArgs {
    /// Count Number of Bytes
    #[arg(short = 'c')]
    pub count_byte: bool,

    /// Count Number of Words
    #[arg(short = 'w')]
    pub word_count: bool,

    /// Count Number of Lines
    #[arg(short = 'l')]
    pub line_count: bool,

    /// Count Number of Characters
    #[arg(short = 'm')]
    pub character_count: bool,

    /// Input FIle(s)
    pub files: Option<Vec<PathBuf>>,

    /// Output the Result to File (STDOUT If Unspecified)
    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

#[derive(Debug)]
pub struct CountFlags {
    pub count_byte: bool,

    pub word_count: bool,

    pub line_count: bool,

    pub character_count: bool,
}

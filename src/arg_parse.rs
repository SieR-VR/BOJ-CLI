use std::path::PathBuf;
use clap::{Parser, ArgEnum};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// The problem number.
    #[clap(value_parser)]
    pub problem_number: String,

    /// The kind of language.
    #[clap(short, long, value_parser)]
    pub lang: Lang,

    /// Output directory or file path.
    #[clap(parse(from_os_str), default_value = ".")]
    pub path: PathBuf,
}

#[derive(Parser, Debug, ArgEnum, Clone, Copy)]
pub enum Lang {
    Cpp11,
    Cpp17,
    Cpp,
    C99,
    C11,
    C,
    Python3,
    Typescript,
    Nodejs,
    Rust2018,
    Rust,
    Java8,
    Java11,
    Java,
}
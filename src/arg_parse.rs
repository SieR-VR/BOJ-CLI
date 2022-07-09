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

impl std::fmt::Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Lang::Cpp11 => write!(f, "cpp"),
            Lang::Cpp17 => write!(f, "cpp"),
            Lang::Cpp => write!(f, "cpp"),
            Lang::C99 => write!(f, "c"),
            Lang::C11 => write!(f, "c"),
            Lang::C => write!(f, "c"),
            Lang::Python3 => write!(f, "py"),
            Lang::Typescript => write!(f, "ts"),
            Lang::Nodejs => write!(f, "js"),
            Lang::Rust2018 => write!(f, "rs"),
            Lang::Rust => write!(f, "rs"),
            Lang::Java8 => write!(f, "java"),
            Lang::Java11 => write!(f, "java"),
            Lang::Java => write!(f, "java"),
        }
    }
}
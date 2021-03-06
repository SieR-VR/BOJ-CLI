mod arg_parse;
mod codegen;
mod problem_parse;

use arg_parse::{Args, Lang};
use problem_parse::Problem;

#[tokio::main]
async fn main() {
    let args = <Args as clap::Parser>::parse();
    let body = reqwest::get(format!(
        "https://raw.githubusercontent.com/SieR-VR/BOJ-CLI-Database/master/{}.json",
        args.problem_number
    ))
    .await
    .expect("Failed to get problem data.")
    .text()
    .await
    .expect("Failed to get problem data.");

    let problem: Problem = serde_json::from_str(&body).unwrap();

    let result = match args.lang {
        Lang::Cpp11 => codegen::cpp::codegen(&problem),
        Lang::Cpp17 => codegen::cpp::codegen(&problem),
        Lang::Cpp => codegen::cpp::codegen(&problem),
        _ => panic!("Not implemented yet."),
    };

    if args.path.is_dir() {
        let path = args.path.join(format!("{}.{}", args.problem_number, args.lang));
        std::fs::write(path, result).expect("Failed to write code.");
    } else {
        std::fs::write(args.path, result).expect("Failed to write code.");
    }
}

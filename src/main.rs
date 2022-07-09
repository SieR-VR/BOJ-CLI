mod arg_parse;
mod codegen;
mod problem_parse;

use arg_parse::{Args, Lang};
use problem_parse::Problem;

#[tokio::main]
async fn main() {
    let args = <Args as clap::Parser>::parse();
    println!("{:?}", args);

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

    match args.lang {
        Lang::Cpp11 => codegen::cpp::codegen(&problem),
        Lang::Cpp17 => codegen::cpp::codegen(&problem),
        Lang::Cpp => codegen::cpp::codegen(&problem),
        _ => panic!("Not implemented yet."),
    }
}

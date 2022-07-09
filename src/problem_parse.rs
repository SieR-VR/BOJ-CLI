use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Problem {
    pub problem: String,
    pub problem_title: String,
    pub problem_number: String,
    pub input: Vec<Input>,
    pub output: Vec<Output>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub rank: usize,
    pub length: Vec<String>,
    pub format: InputFormat,
    pub parameter: String,
    pub parameter_description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum InputFormat {
    #[serde(rename = "integer")]
    Integer,

    #[serde(rename = "float")]
    Float,

    #[serde(rename = "string")]
    String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub rank: usize,
    pub length: Vec<String>,
    pub format: OutputFormat,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OutputFormat {
    #[serde(rename = "integer")]
    Integer,

    #[serde(rename = "float")]
    Float,

    #[serde(rename = "string")]
    String,

    Literal(String),
}
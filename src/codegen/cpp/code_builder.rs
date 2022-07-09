use crate::problem_parse::InputFormat;

#[derive(Debug, Clone)]
pub struct CodeBuilder<'a> {
    pub code: Vec<Code<'a>>,
}

impl CodeBuilder<'_> {
    pub fn build(&self, indent: usize) -> String {
        self.code
            .iter()
            .map(|code| code.build(indent))
            .collect::<Vec<_>>()
            .join("")
    }
}

#[derive(Debug, Clone)]
pub enum Code<'a> {
    Line(String),
    Function(Function<'a>),
    MultiLineComment(&'a str),
}

impl Code<'_> {
    pub fn build(&self, indent: usize) -> String {
        match self {
            Code::Line(line) => format!("{}{}\n", " ".repeat(indent), line),  
            Code::Function(function) => function.build(indent),
            Code::MultiLineComment(comment) => format!("{}/*{}*/\n", " ".repeat(indent), comment),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function<'a> {
    pub name: &'a str,
    pub args: Vec<String>,
    pub return_type: &'a str,
    pub code: CodeBuilder<'a>,
}

impl Function<'_> {
    fn build(&self, indent: usize) -> String {
        match self.code.code.len() {
            0 => format!(
                "{}{} {}({});\n",
                " ".repeat(indent),
                self.return_type,
                self.name,
                self.args.join(", ")
            ),
            _ => format!(
                "{}{} {}({}) {{\n{}}}\n",
                " ".repeat(indent),
                self.return_type,
                self.name,
                self.args.join(", "),
                self.code.build(indent + 2),
            ),
        }
    }
}

pub fn get_type(rank: usize, format: &InputFormat) -> String {
    match format {
        InputFormat::Integer => get_vector_with_type(rank, "long long"),
        InputFormat::Float => get_vector_with_type(rank, "double"),
        InputFormat::String => get_vector_with_type(rank, "std::string"),
    }
}

pub fn get_initializer(length: &Vec<String>, format: &InputFormat) -> String {
    match format {
        InputFormat::Integer => get_vector_with_initializer(length, "0", "long long"),
        InputFormat::Float => get_vector_with_initializer(length, "0.0", "double"),
        InputFormat::String => get_vector_with_initializer(length, "\"\"", "std::string"),
    }
}

pub fn get_vector_with_type(rank: usize, inner_type: &str) -> String {
    match rank {
        0 => format!("{}", inner_type),
        _ => format!(
            "std::vector<{}>",
            get_vector_with_type(rank - 1, inner_type)
        ),
    }
}

pub fn get_vector_with_initializer(
    length: &[String],
    inner_initializer: &str,
    inner_type: &str,
) -> String {
    match length.len() {
        0 => format!("{}", inner_initializer),
        _ => format!(
            "std::vector<{}>({}, {})",
            get_vector_with_type(length.len() - 1, inner_type),
            length[0].as_str(),
            get_vector_with_initializer(&length[1..], inner_initializer, inner_type),
        ),
    }
}

pub fn function_call(name: &str, args: &Vec<String>) -> String {
    format!("{}({})", name, args.join(", "))
}
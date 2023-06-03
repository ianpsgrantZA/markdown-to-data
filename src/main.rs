#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{path::PathBuf, error::Error, env::{args, Args, self}, string};

// Input ./markdown-to-data [file-path] [-o output-path] [-e output type]
fn main() {
    let file_data = FileData::from_input(env::args().collect());

    println!("FileData Created!");
    println!("path: {}", file_data.path.to_str().expect("Bad input path"));
    println!("path: {}", file_data.output_path.to_str().expect("Bad output path"));
    println!("path: {}", file_data.output_type.to_string());


    let file_content = std::fs::read_to_string(file_data.path).expect("could not read file");
    
    println!("file: {}", file_content);
}

enum OutputType {
    Json,
    Markdown,
    Xml,
    Html,
}

impl OutputType {
    fn from_string(s: &str) -> Result<OutputType, &str> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputType::Json),
            "js" => Ok(OutputType::Json),
            "markdown" => Ok(OutputType::Markdown),
            "md" => Ok(OutputType::Markdown),
            "xml" => Ok(OutputType::Xml),
            "html" => Ok(OutputType::Html),
            _ => Err("Bad things happened"),
        }
    }

    fn to_string(&self) -> String {
        match self {
            OutputType::Json => String::from("json"),
            OutputType::Markdown => String::from("markdown"),
            OutputType::Xml => String::from("xml"),
            OutputType::Html => String::from("html"),
        }
    }
}

struct FileData {
    path: PathBuf,
    output_path: PathBuf,
    output_type: OutputType
}

impl FileData {
    fn from_input(args: Vec<String>) -> FileData {
        let path = PathBuf::from(&args[1]);
        let output_path = PathBuf::from(&args[2]);
        let output_type = OutputType::from_string(&args[3]).expect("Invalid output type");
        FileData {path, output_path, output_type}
    }
}
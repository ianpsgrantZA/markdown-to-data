#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{path::PathBuf, error::Error, env::{args, Args}};

// Input ./markdown-to-data [file-path] [-o output-path] [-e output type]
fn main() {
    let file_path = std::env::args().nth(1).expect("");
    let output_path = std::env::args().nth(2).expect(".");
    let output_type = std::env::args().nth(3).expect("json");

    println!("FileData Created!");
    println!("path: {}", file_path);
    println!("path: {}", output_path);
    println!("path: {}", output_type);

    let file_data = FileData::from_input(args());




// let file_content = std::fs::read_to_string(&path).expect("could not read file");
    

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
}

struct FileData {
    path: PathBuf,
    output_path: PathBuf,
    output_type: OutputType
}



impl FileData {
    fn from_input(mut args: Args) -> FileData {
        let path = PathBuf::from(args.nth(1).expect(""));
        let output_path = PathBuf::from(args.nth(2).expect("."));
        let output_type = OutputType::from_string(&args.nth(3).expect("json")).unwrap();

        FileData {path, output_path, output_type}
    }
}
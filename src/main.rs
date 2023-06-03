#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    println!("HStart");

    let path_name = std::env::args().nth(1).expect("");

    println!("{}", path_name);
    let path = std::path::PathBuf::from(path_name);

    let file_content = std::fs::read_to_string(&path).expect("could not read file");

    


    

}

struct FileData {
    file_path: String,
}

impl FileData {
    fn new(args: &[String]) -> FileData {
        let file_path = args[1].clone();

        FileData {file_path}
    }
}
fn main() {
    println!("Hello, world!");

    let path_name = std::env::args().nth(1).expect("");
    let path = std::path::PathBuf::from(path_name);
    

}

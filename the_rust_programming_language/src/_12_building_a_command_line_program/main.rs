#[allow(dead_code)]
pub fn test() {
    let args: Vec<String> = std::env::args().collect();

    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("{:?}", args);
    println!("Query: \"{query}\"");
    println!("File path: \"{file_path}\"");

    println!("\n");

    let contents: String = std::fs::read_to_string(&file_path).expect("Should have been able to read the file");

    println!("File content: \"{contents}\"");
}

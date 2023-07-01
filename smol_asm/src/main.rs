use std::{ fs, process::exit};

mod ast;
mod compiler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Give file as an argument");
        exit(1);
    }

    let file_contents = fs::read_to_string(&args[1]).unwrap();
    let tree = ast::parse_source(&file_contents).unwrap();
    let binary = compiler::compile_ast(tree);
    binary.save(&format!("{}.obj", &args[1]));
}

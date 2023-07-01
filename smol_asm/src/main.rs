use std::{fmt::format, fs, process::exit};

mod ast;
mod compiler;

const FILE_DATA: &str = "

    ADD r0 r1
    ADDI r3 123
";

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
    let bin2 = smol_file::SmolFile::load(&format!("{}.obj", &args[1]));
}

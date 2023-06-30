use std::{fs, process::exit};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Give file as an argument");
        exit(1);
    }

    let file_contents = fs::read(&args[1]).unwrap();

    let mut vm = smol_vm::Vm::default();
    vm.instructions.instructions = file_contents;
    vm.run();

    println!("{:#?}", vm.registers);
}

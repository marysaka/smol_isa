use std::{fs, process::exit};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Give file as an argument");
        exit(1);
    }

    let file_contents = fs::read(&args[1]).unwrap();

    let mut vm = smol_vm::Vm::default();
    /*     vm.registers.r0 = 1;
    vm.registers.r1 = 2; */
    vm.instructions.instructions = file_contents; /* vec![
                                                      // ALU Add from Register
                                                      #[allow(clippy::unusual_byte_groupings)]
                                                      0b00_000_0_0_0,
                                                      // Registers r0 and r1
                                                      0b0001_0000,
                                                  ]; */
    vm.run();

    println!("{:#?}", vm.registers);
}

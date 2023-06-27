use smol_isa;

fn main() {
    let mut vm = smol_isa::Vm::default();
    vm.registers.r0 = 1;
    vm.registers.r1 = 2;
    vm.instructions.instructions = vec![
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r0 and r1
        0b0001_0000,
    ];
    vm.run();

    assert!(vm.registers.r0 == 3);
}

use smol_isa;

#[test]
pub fn it_adds_r0_r1() {
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

    assert_eq!(vm.registers.r0, 3);
}

#[test]
pub fn it_adds_r2_r3() {
    let mut vm = smol_isa::Vm::default();
    vm.registers.r2 = 2;
    vm.registers.r3 = 6;
    vm.instructions.instructions = vec![
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r2 and r3
        0b0011_0010,
    ];
    vm.run();

    assert_eq!(vm.registers.r2, 8);
}

#[test]
pub fn it_adds_r7_r4() {
    let mut vm = smol_isa::Vm::default();
    vm.registers.r4 = 11;
    vm.registers.r7 = 100;
    vm.instructions.instructions = vec![
        // ALU Add from Register
        0b00_000_0_0_0,
        // Registers r7 and r4
        0b0100_0111,
    ];
    vm.run();

    assert_eq!(vm.registers.r7, 111);
}

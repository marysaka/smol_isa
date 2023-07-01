use smol_vm::Vm;

#[test]
pub fn it_loads_immediate_variable_address() {
    let mut vm = Vm::default();
    vm.instructions.instructions = vec![
        // Stack load variable immediate 8bit
        0b10_10_1_0_00,
        // Value of 10
        10,
    ];
    vm.run();

    assert_eq!(vm.registers.sp, (u16::MAX / 2) + 10);
}

#[test]
pub fn it_loads_immediate_variable_address_16bit() {
    let mut vm = Vm::default();
    vm.instructions.instructions = vec![
        // Stack load variable immediate 16 bit
        0b10_10_1_1_00,
        // Value of 256 in 16 bit little endian
        0b00000000,
        0b00000001,
    ];
    vm.run();

    assert_eq!(vm.registers.sp, (u16::MAX / 2) + 256);
}

#[test]
pub fn it_loads_register_variable_address() {
    let mut vm = Vm::default();
    vm.registers.r6 = 5;
    vm.instructions.instructions = vec![
        // Stack load variable regsiter
        0b10_10_0_0_00,
        // Register r6
        0b0000_0110,
    ];
    vm.run();

    assert_eq!(vm.registers.sp, (u16::MAX / 2) + 5);
}

#[test]
pub fn it_loads_16b_register_variable_address() {
    let mut vm = Vm::default();
    let mut vm = Vm::default();
    vm.registers.l1 = 700;
    vm.instructions.instructions = vec![
        // Stack load variable register 16 bit
        0b10_10_0_1_00,
        // Register l1
        0b0000_1010,
    ];
    vm.run();
    assert_eq!(vm.registers.sp, (u16::MAX / 2) + 700);
}

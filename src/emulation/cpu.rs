struct CPU {
    cycles: usize,
    program_counter: u16,
    stack_pointer: u8,
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    carry_flag: u8,
    zero_flag: u8,
    interrupt_flag: u8,
    decimal_mode: u8,
    break_command: u8,
    unused: u8,
    overflow: u8,
    negative: u8,
    interrupt_type: u8,
    stall: isize 
}

struct Instruction {
    opcode: u8,
    name: &str,
    mode: u8,
    size: u8,
    cycles: u8,
    page_cycles: u8,
}

const cpu_frequency: u16 = 1789773;
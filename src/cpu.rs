use crate::{
    flags::FlagsRegister,
    clock::Clock,
};


#[allow(non_snake_case)]
struct ControlUnit {
    REQ: u8,
    ACK: u8,
}

impl ControlUnit {
    fn new() -> Self {
        Self {
            REQ: 0,
            ACK: 0,
        }
    }
}

struct ALU {
    input0: u8,
    input1: u8,
    output: u8,
}

impl ALU {
    fn new() -> Self {
        Self {
            input0: 0,
            input1: 0,
            output: 0,
        }
    }
}

impl ALU {
    fn add(&mut self, cpu: &mut CPU) {
        let (output, did_overflow) = self.input0.overflowing_add(self.input1);
        // cpu.register_file.F.zero = output == 0;
        // cpu.register_file.F.subtract = false;
        // cpu.register_file.F.carry = did_overflow;
        //
        self.output = output;
    }

    fn push_to_data_bus(&self, cpu: &mut CPU) {
       cpu.DataBus = self.output; 
    }
}

#[allow(non_snake_case)]
pub struct CPU {
    control_unit: ControlUnit,
    register_file: RegisterFile,
    ALU: ALU, // Arithmetic Logic Unit
    AddressBus: u16,
    DataBus: u8,
    Clock: Clock,
}

#[allow(non_snake_case)]
impl CPU {
    // this function encapsulates the IDU increment functionality
    fn IDU_inc(&mut self) {
        self.AddressBus = self.AddressBus.wrapping_add(0x01);
    }
    // this function encapsulates the IDU decrement functionality
    fn IDU_dec(&mut self) {
        self.AddressBus = self.AddressBus.wrapping_sub(0x01);
    }
}

#[allow(non_snake_case)]
struct RegisterFile {
    PC: u16, // program counter
    SP: u16, // stack pointer
    ACC: u8, // accumulator
    F: FlagsRegister,
    BC: u16, // general purpose, two 8 bit halves
    DE: u16, // general purpose, two 8 bit halves
    HL: u16, // general purpose, two 8 bit halves
    IR: u8, // Instruction registers
    IE: u8, // Interrupt enable
}

impl RegisterFile {
    fn new() -> Self {
        Self {
            PC: 0,
            SP: 0,
            ACC: 0,
            F: FlagsRegister::init(),
            BC: 0,
            DE: 0,
            HL: 0,
            IR: 0,
            IE: 0,
        }
    }
    
}

impl CPU {
    fn new() -> Self {
        Self {
            control_unit: ControlUnit::new(),
            ALU: ALU::new(),
            register_file: RegisterFile::new(),
            AddressBus: 0,
            DataBus: 0,
            Clock: Clock::new(),

        }
    }

    fn step(&self, ) {
    }
}

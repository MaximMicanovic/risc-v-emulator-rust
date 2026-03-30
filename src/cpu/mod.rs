mod instructions;
mod mmu;

use crate::bus::Bus;
use crate::bus::ram::{RAM_BASE, RAM_SIZE};

#[repr(u64)]
pub enum CpuPriv {
    U = 0,
    S = 1,
    M = 3,
}

pub enum CpuStatus {
    Ok = 0,
    Halt,
    Trap,
    ErrFetchFault,
    ErrIllegalInsn,
    ErrMisaligned,
}

pub struct Cpu{
    pub gpr: [u64; 32],
    pub csr: [u64; 4096],
    pub pc: u64,
    pub privilege: u64,
    pub satp: u64,
}

impl Cpu{
    pub fn new() -> Self {
        let mut cpu = Cpu{
            gpr: [0; 32],
            csr: [0; 4096],
            pc: 0,
            privilege: 0,
            satp: 0,
        };
        cpu.gpr[2] = (RAM_SIZE + RAM_BASE) as u64;
        cpu.pc = 0x8000_0000;
        cpu
    }
    pub fn execute(&mut self, bus: &mut Bus, instruction: u32){
        let opcode = (instruction & 0x7F) as u8;
        
        // ALL THE CURRENTLY IMPLEMENTED INSTRUCTIONS
        match opcode {
            0b0110011 => self.ins_r(instruction),
            0b0010011 | 0b0000011 | 0b1100111 => self.ins_i(instruction, bus),
            0b0100011 => self.ins_s(instruction, bus),
            0b1100011 => self.ins_b(instruction),
            0b1101111 => self.ins_j(instruction),
            0b0110111 | 0b0010111 => self.ins_u(instruction),
            _ => panic!("unimplemented opcode: {:#09b} (instruction: {:#010x})", opcode, instruction),
        }
    }
}


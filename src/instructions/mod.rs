// THESE ARE THE BASE INSTRUCTIONS FOR THE RISC-V INSTRUCTIONS SET
// RV32I ONLY FOR INTEGERS

use crate::cpu::CpuData;
use crate::memory::{load_byte, load_h, load_ubyte, load_uh, load_w, store_byte, store_h, store_w};

fn sign_extend(value: u32, bits: u32) -> i32 {
    let mask = 1u32 << (bits - 1);
    ((value ^ mask).wrapping_sub(mask)) as i32
}

// This function needs some sort of understanding of the change the PC causes
fn ins_b(instruction: u32, cpu: &mut CpuData) {
    let funct3 = ((instruction >> 12) & 0x7) as u8;
    let rs1 = ((instruction >> 15) & 0x1F) as usize;
    let rs2 = ((instruction >> 20) & 0x1F) as usize;

    // Reading the imm
    // As it's divided into parts and saved separately
    let imm12 = (instruction >> 31) & 0x1;
    let imm10_5 = (instruction >> 25) & 0x3F;
    let imm11 = (instruction >> 7) & 0x1;
    let imm4_1 = (instruction >> 8) & 0xF;

    let imm = sign_extend(
        (imm12 << 12) | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1),
        13,
    );

    // BASIC INTEGER INSTRUCTIONS IMPLEMENTED
    match funct3 {
        0x0 => {
            if cpu.gpr[rs1] == cpu.gpr[rs2] {
                cpu.pc = ((cpu.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        0x1 => {
            if cpu.gpr[rs1] != cpu.gpr[rs2] {
                cpu.pc = ((cpu.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        0x4 => {
            if (cpu.gpr[rs1] as i32) < (cpu.gpr[rs2] as i32) {
                cpu.pc = ((cpu.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        0x5 => {
            if (cpu.gpr[rs1] as i32) >= (cpu.gpr[rs2] as i32) {
                cpu.pc = ((cpu.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        0x6 => {
            if cpu.gpr[rs1] < cpu.gpr[rs2] {
                cpu.pc = ((cpu.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        0x7 => {
            if cpu.gpr[rs1] >= cpu.gpr[rs2] {
                cpu.pc = ((cpu.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        _ => panic!("unimplemented branch funct3: {:#05b} (instruction: {:#010x})", funct3, instruction),
    }
    cpu.pc += 4;
}

// THIS HAS:
// EXPANDED MULTI
// BASE INTEGER
fn ins_r(instruction: u32, cpu: &mut CpuData) {
    let rd = ((instruction >> 7) & 0x1F) as usize;
    let funct3 = ((instruction >> 12) & 0x7) as u8;
    let rs1 = ((instruction >> 15) & 0x1F) as usize;
    let rs2 = ((instruction >> 20) & 0x1F) as usize;
    let funct7 = ((instruction >> 25) & 0x7F) as u8;

    if rd == 0 {
        cpu.pc += 4;
        return;
    }

    match funct7 {
        // THE BASIC INTEGER INSTRUCTIONS
        0x0 | 0x20 => match funct3 {
            0x0 => {
                if funct7 == 0x20 {
                    cpu.gpr[rd] = cpu.gpr[rs1].wrapping_sub(cpu.gpr[rs2]);
                } else {
                    cpu.gpr[rd] = cpu.gpr[rs1].wrapping_add(cpu.gpr[rs2]);
                }
            }
            0x4 => cpu.gpr[rd] = cpu.gpr[rs1] ^ cpu.gpr[rs2],
            0x6 => cpu.gpr[rd] = cpu.gpr[rs1] | cpu.gpr[rs2],
            0x7 => cpu.gpr[rd] = cpu.gpr[rs1] & cpu.gpr[rs2],
            0x1 => cpu.gpr[rd] = cpu.gpr[rs1] << (cpu.gpr[rs2] & 0x1F),
            0x5 => {
                if funct7 == 0x20 {
                    cpu.gpr[rd] = ((cpu.gpr[rs1] as i32) >> (cpu.gpr[rs2] & 0x1F)) as u64;
                } else {
                    cpu.gpr[rd] = ((cpu.gpr[rs1] as u32) >> (cpu.gpr[rs2] & 0x1F)) as u64;
                }
            }
            0x2 => cpu.gpr[rd] = if (cpu.gpr[rs1] as i32) < (cpu.gpr[rs2] as i32) { 1 } else { 0 },
            0x3 => cpu.gpr[rd] = if cpu.gpr[rs1] < cpu.gpr[rs2] { 1 } else { 0 },
            _ => panic!("unimplemented R-type funct3: {:#05b} (instruction: {:#010x})", funct3, instruction),
        },
        // MULTIPLY EXTENSION
        0x01 => match funct3 {
            0x0 => {
                // LOWER 32 BITS
                cpu.gpr[rd] = cpu.gpr[rs1].wrapping_mul(cpu.gpr[rs2]);
            }
            0x1 => {
                // UPPER 32 BITS
                let prod = (cpu.gpr[rs1] as i64).wrapping_mul(cpu.gpr[rs2] as i64);
                cpu.gpr[rd] = (prod >> 32) as i32 as u64;
            }
            0x2 => {
                let prod = (cpu.gpr[rs1] as i64).wrapping_mul(cpu.gpr[rs2] as u64 as i64);
                cpu.gpr[rd] = (prod >> 32) as i32 as u64;
            }
            0x3 => {
                let prod = (cpu.gpr[rs1] as u64).wrapping_mul(cpu.gpr[rs2] as u64);
                cpu.gpr[rd] = (prod >> 32) as u32 as u64;
            }
            0x4 => {
                cpu.gpr[rd] = ((cpu.gpr[rs1] as i32) / (cpu.gpr[rs2] as i32)) as u64;
            }
            0x5 => {
                cpu.gpr[rd] = cpu.gpr[rs1] / cpu.gpr[rs2];
            }
            0x6 => {
                cpu.gpr[rd] = ((cpu.gpr[rs1] as i32) % (cpu.gpr[rs2] as i32)) as u64;
            }
            0x7 => {
                cpu.gpr[rd] = cpu.gpr[rs1] % cpu.gpr[rs2];
            }
            _ => panic!("unimplemented M-ext funct3: {:#05b} (instruction: {:#010x})", funct3, instruction),
        },
        _ => panic!("unimplemented R-type funct7: {:#09b} (instruction: {:#010x})", funct7, instruction),
    }
    cpu.pc += 4;
}

fn ins_i_load(funct3: u8, mem: &[u8], addr: u32) -> u32 {
    match funct3 {
        0x0 => load_byte(mem, addr),
        0x1 => load_h(mem, addr),
        0x2 => load_w(mem, addr),
        0x3 => load_ubyte(mem, addr),
        0x4 => load_uh(mem, addr),
        _ => panic!("unimplemented load funct3: {:#05b} (addr: {:#010x})", funct3, addr),
    }
}

// BASIC INTEGER AND LOAD MEMORY INSTRUCTIONS
fn ins_i(instruction: u32, cpu: &mut CpuData, mem: &mut [u8]) {
    let opcode = (instruction & 0x7F) as u8;
    let rd = ((instruction >> 7) & 0x1F) as usize;
    let funct3 = ((instruction >> 12) & 0x7) as u8;
    let rs1 = ((instruction >> 15) & 0x1F) as usize;
    let imm = sign_extend((instruction >> 20) & 0xFFF, 12);

    match opcode {
        // BASIC INTEGER INSTRUCTIONS IMPLEMENTED
        0b0010011 => match funct3 {
            0x0 => cpu.gpr[rd] = cpu.gpr[rs1].wrapping_add(imm as u64),
            0x4 => cpu.gpr[rd] = cpu.gpr[rs1] ^ (imm as u64),
            0x6 => cpu.gpr[rd] = cpu.gpr[rs1] | (imm as u64),
            0x7 => cpu.gpr[rd] = cpu.gpr[rs1] & (imm as u64),
            0x1 => cpu.gpr[rd] = cpu.gpr[rs1] << ((imm & 0x1F) as u32),
            0x5 => {
                if ((imm >> 5) & 0x7F) == 0x00 {
                    cpu.gpr[rd] = cpu.gpr[rs1] >> ((imm & 0x1F) as u32);
                } else if ((imm >> 5) & 0x7F) == 0x20 {
                    cpu.gpr[rd] = ((cpu.gpr[rs1] as i32) >> ((imm & 0x1F) as u32)) as u64;
                }
            }
            0x2 => cpu.gpr[rd] = if (cpu.gpr[rs1] as i32) < imm { 1 } else { 0 },
            0x3 => cpu.gpr[rd] = if cpu.gpr[rs1] < (imm as u64) { 1 } else { 0 },
            _ => panic!("unimplemented I-type funct3: {:#05b} (instruction: {:#010x})", funct3, instruction),
        },
        // LOAD MEMORY INSTRUCTIONS
        0b0000011 => {
            let addr = ((cpu.gpr[rs1] as i64) + (imm as i64)) as u32;
            let loaded = ins_i_load(funct3, mem, addr);
            if funct3 <= 0x2 {
                // Signed loads (LB/LH/LW): sign-extend to XLEN (64 here).
                cpu.gpr[rd] = loaded as i32 as i64 as u64;
            } else {
                // Unsigned loads (LBU/LHU): zero-extend.
                cpu.gpr[rd] = loaded as u64;
            }
        }
        // JUMP AND LINK REGISTER INSTRUCTION
        0b1100111 => {
            cpu.gpr[rd] = cpu.pc + 4;
            cpu.pc = ((cpu.gpr[rs1] as i64) + (imm as i64)) as u64;
            return;
        }
        _ => {}
    }
    cpu.pc += 4;
}

fn ins_s(instruction: u32, cpu: &mut CpuData, mem: &mut [u8]) {
    let imm4_0 = (instruction >> 7) & 0x1F;
    let funct3 = ((instruction >> 12) & 0x7) as u8;
    let rs1 = ((instruction >> 15) & 0x1F) as usize;
    let rs2 = ((instruction >> 20) & 0x1F) as usize;
    let imm11_5 = (instruction >> 25) & 0x7F;

    let imm = sign_extend((imm11_5 << 5) | imm4_0, 12);

    let rs1_val = cpu.gpr[rs1];
    let rs2_val = cpu.gpr[rs2] as u32;
    let addr = ((rs1_val as i64) + (imm as i64)) as u32;

    match funct3 {
        0x0 => store_byte(mem, addr, rs2_val),
        0x1 => store_h(mem, addr, rs2_val),
        0x2 => store_w(mem, addr, rs2_val),
        _ => panic!("unimplemented store funct3: {:#05b} (instruction: {:#010x})", funct3, instruction),
    }

    cpu.pc += 4;
}

// ONE J-INSTRUCTION
fn ins_j(instruction: u32, cpu: &mut CpuData) {
    let rd = ((instruction >> 7) & 0x1F) as usize;

    let imm19_12 = (instruction >> 12) & 0xFF;
    let imm11 = (instruction >> 20) & 1;
    let imm10_1 = (instruction >> 21) & 0x3FF;
    let sign = (instruction >> 31) & 1;

    let imm = sign_extend(
        (imm10_1 << 1) | (imm11 << 11) | (imm19_12 << 12) | (sign << 20),
        21,
    );

    cpu.gpr[rd] = cpu.pc + 4;
    cpu.pc = ((cpu.pc as i64) + (imm as i64)) as u64;
}

// U INSTRUCTIONS
fn ins_u(instruction: u32, cpu: &mut CpuData) {
    let opcode = (instruction & 0x7F) as u8;
    let rd = ((instruction >> 7) & 0x1F) as usize;
    let imm = instruction >> 12;

    match opcode {
        0b0110111 => cpu.gpr[rd] = (imm << 12) as u64,
        0b0010111 => cpu.gpr[rd] = cpu.pc + ((imm << 12) as u64),
        _ => {}
    }

    cpu.pc += 4;
}

pub fn read_ins(instruction: u32, cpu: &mut CpuData, mem: &mut [u8]) {
    let opcode = (instruction & 0x7F) as u8;

    // ALL THE CURRENTLY IMPLEMENTED INSTRUCTIONS
    match opcode {
        0b0110011 => ins_r(instruction, cpu),
        0b0010011 | 0b0000011 | 0b1100111 => ins_i(instruction, cpu, mem),
        0b0100011 => ins_s(instruction, cpu, mem),
        0b1100011 => ins_b(instruction, cpu),
        0b1101111 => ins_j(instruction, cpu),
        0b0110111 | 0b0010111 => ins_u(instruction, cpu),
        _ => panic!("unimplemented opcode: {:#09b} (instruction: {:#010x})", opcode, instruction),
    }
}

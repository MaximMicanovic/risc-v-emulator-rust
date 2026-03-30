// THESE ARE THE BASE INSTRUCTIONS FOR THE RISC-V INSTRUCTIONS SET
// RV32I ONLY FOR INTEGERS

use crate::bus::Bus;

impl super::Cpu {
fn sign_extend(value: u32, bits: u32) -> i32 {
    let mask = 1u32 << (bits - 1);
    ((value ^ mask).wrapping_sub(mask)) as i32
}

// This function needs some sort of understanding of the change the PC causes
pub(super) fn ins_b(&mut self, instruction: u32) {
    let funct3 = ((instruction >> 12) & 0x7) as u8;
    let rs1 = ((instruction >> 15) & 0x1F) as usize;
    let rs2 = ((instruction >> 20) & 0x1F) as usize;

    // Reading the imm
    // As it's divided into parts and saved separately
    let imm12 = (instruction >> 31) & 0x1;
    let imm10_5 = (instruction >> 25) & 0x3F;
    let imm11 = (instruction >> 7) & 0x1;
    let imm4_1 = (instruction >> 8) & 0xF;

    let imm = Self::sign_extend(
        (imm12 << 12) | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1),
        13,
    );

    // BASIC INTEGER INSTRUCTIONS IMPLEMENTED
    match funct3 {
        0x0 => {
            if self.gpr[rs1] == self.gpr[rs2] {
                self.pc = ((self.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        0x1 => {
            if self.gpr[rs1] != self.gpr[rs2] {
                self.pc = ((self.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        0x4 => {
            if (self.gpr[rs1] as i32) < (self.gpr[rs2] as i32) {
                self.pc = ((self.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        0x5 => {
            if (self.gpr[rs1] as i32) >= (self.gpr[rs2] as i32) {
                self.pc = ((self.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        0x6 => {
            if self.gpr[rs1] < self.gpr[rs2] {
                self.pc = ((self.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        0x7 => {
            if self.gpr[rs1] >= self.gpr[rs2] {
                self.pc = ((self.pc as i64) + (imm as i64)) as u64;
                return;
            }
        }
        _ => panic!("unimplemented branch funct3: {:#05b} (instruction: {:#010x})", funct3, instruction),
    }
    self.pc += 4;
}

// THIS HAS:
// EXPANDED MULTI
// BASE INTEGER
pub(super) fn ins_r(&mut self, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1F) as usize;
    let funct3 = ((instruction >> 12) & 0x7) as u8;
    let rs1 = ((instruction >> 15) & 0x1F) as usize;
    let rs2 = ((instruction >> 20) & 0x1F) as usize;
    let funct7 = ((instruction >> 25) & 0x7F) as u8;

    if rd == 0 {
        self.pc += 4;
        return;
    }

    match funct7 {
        // THE BASIC INTEGER INSTRUCTIONS
        0x0 | 0x20 => match funct3 {
            0x0 => {
                if funct7 == 0x20 {
                    self.gpr[rd] = self.gpr[rs1].wrapping_sub(self.gpr[rs2]);
                } else {
                    self.gpr[rd] = self.gpr[rs1].wrapping_add(self.gpr[rs2]);
                }
            }
            0x4 => self.gpr[rd] = self.gpr[rs1] ^ self.gpr[rs2],
            0x6 => self.gpr[rd] = self.gpr[rs1] | self.gpr[rs2],
            0x7 => self.gpr[rd] = self.gpr[rs1] & self.gpr[rs2],
            0x1 => self.gpr[rd] = self.gpr[rs1] << (self.gpr[rs2] & 0x1F),
            0x5 => {
                if funct7 == 0x20 {
                    self.gpr[rd] = ((self.gpr[rs1] as i32) >> (self.gpr[rs2] & 0x1F)) as u64;
                } else {
                    self.gpr[rd] = ((self.gpr[rs1] as u32) >> (self.gpr[rs2] & 0x1F)) as u64;
                }
            }
            0x2 => self.gpr[rd] = if (self.gpr[rs1] as i32) < (self.gpr[rs2] as i32) { 1 } else { 0 },
            0x3 => self.gpr[rd] = if self.gpr[rs1] < self.gpr[rs2] { 1 } else { 0 },
            _ => panic!("unimplemented R-type funct3: {:#05b} (instruction: {:#010x})", funct3, instruction),
        },
        // MULTIPLY EXTENSION
        0x01 => match funct3 {
            0x0 => {
                // LOWER 32 BITS
                self.gpr[rd] = self.gpr[rs1].wrapping_mul(self.gpr[rs2]);
            }
            0x1 => {
                // UPPER 32 BITS
                let prod = (self.gpr[rs1] as i64).wrapping_mul(self.gpr[rs2] as i64);
                self.gpr[rd] = (prod >> 32) as i32 as u64;
            }
            0x2 => {
                let prod = (self.gpr[rs1] as i64).wrapping_mul(self.gpr[rs2] as u64 as i64);
                self.gpr[rd] = (prod >> 32) as i32 as u64;
            }
            0x3 => {
                let prod = (self.gpr[rs1] as u64).wrapping_mul(self.gpr[rs2] as u64);
                self.gpr[rd] = (prod >> 32) as u32 as u64;
            }
            0x4 => {
                self.gpr[rd] = ((self.gpr[rs1] as i32) / (self.gpr[rs2] as i32)) as u64;
            }
            0x5 => {
                self.gpr[rd] = self.gpr[rs1] / self.gpr[rs2];
            }
            0x6 => {
                self.gpr[rd] = ((self.gpr[rs1] as i32) % (self.gpr[rs2] as i32)) as u64;
            }
            0x7 => {
                self.gpr[rd] = self.gpr[rs1] % self.gpr[rs2];
            }
            _ => panic!("unimplemented M-ext funct3: {:#05b} (instruction: {:#010x})", funct3, instruction),
        },
        _ => panic!("unimplemented R-type funct7: {:#09b} (instruction: {:#010x})", funct7, instruction),
    }
    self.pc += 4;
}

fn ins_i_load(funct3: u8, bus: &mut Bus, addr: u32) -> u32 {
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
pub(super) fn ins_i(&mut self, instruction: u32, bus: &mut Bus) {
    let opcode = (instruction & 0x7F) as u8;
    let rd = ((instruction >> 7) & 0x1F) as usize;
    let funct3 = ((instruction >> 12) & 0x7) as u8;
    let rs1 = ((instruction >> 15) & 0x1F) as usize;
    let imm = Self::sign_extend((instruction >> 20) & 0xFFF, 12);

    match opcode {
        // BASIC INTEGER INSTRUCTIONS IMPLEMENTED
        0b0010011 => match funct3 {
            0x0 => self.gpr[rd] = self.gpr[rs1].wrapping_add(imm as u64),
            0x4 => self.gpr[rd] = self.gpr[rs1] ^ (imm as u64),
            0x6 => self.gpr[rd] = self.gpr[rs1] | (imm as u64),
            0x7 => self.gpr[rd] = self.gpr[rs1] & (imm as u64),
            0x1 => self.gpr[rd] = self.gpr[rs1] << ((imm & 0x1F) as u32),
            0x5 => {
                if ((imm >> 5) & 0x7F) == 0x00 {
                    self.gpr[rd] = self.gpr[rs1] >> ((imm & 0x1F) as u32);
                } else if ((imm >> 5) & 0x7F) == 0x20 {
                    self.gpr[rd] = ((self.gpr[rs1] as i32) >> ((imm & 0x1F) as u32)) as u64;
                }
            }
            0x2 => self.gpr[rd] = if (self.gpr[rs1] as i32) < imm { 1 } else { 0 },
            0x3 => self.gpr[rd] = if self.gpr[rs1] < (imm as u64) { 1 } else { 0 },
            _ => panic!("unimplemented I-type funct3: {:#05b} (instruction: {:#010x})", funct3, instruction),
        },
        // LOAD MEMORY INSTRUCTIONS
        0b0000011 => {
            let addr = ((self.gpr[rs1] as i64) + (imm as i64)) as u32;
            let loaded = Self::ins_i_load(funct3, bus, addr);
            if funct3 <= 0x2 {
                // Signed loads (LB/LH/LW): sign-extend to XLEN (64 here).
                self.gpr[rd] = loaded as i32 as i64 as u64;
            } else {
                // Unsigned loads (LBU/LHU): zero-extend.
                self.gpr[rd] = loaded as u64;
            }
        }
        // JUMP AND LINK REGISTER INSTRUCTION
        0b1100111 => {
            self.gpr[rd] = self.pc + 4;
            self.pc = ((self.gpr[rs1] as i64) + (imm as i64)) as u64;
            return;
        }
        _ => panic!("unimplemented I-type funct3: {:#05b} (instruction: {:#010x})", funct3, instruction),
    }
    self.pc += 4;
}

pub(super) fn ins_s(&mut self, instruction: u32, bus: &mut Bus) {
    let imm4_0 = (instruction >> 7) & 0x1F;
    let funct3 = ((instruction >> 12) & 0x7) as u8;
    let rs1 = ((instruction >> 15) & 0x1F) as usize;
    let rs2 = ((instruction >> 20) & 0x1F) as usize;
    let imm11_5 = (instruction >> 25) & 0x7F;

    let imm = Self::sign_extend((imm11_5 << 5) | imm4_0, 12);

    let rs1_val = self.gpr[rs1];
    let rs2_val: u32 = self.gpr[rs2] as u32;
    let addr: u32 = ((rs1_val as i64) + (imm as i64)) as u32;

    match funct3 {
        0x0 => bus.write8(addr, rs2_val),
        0x1 => bus.write16(addr, rs2_val),
        0x2 => bus.write32(addr, rs2_val),
        _ => panic!("unimplemented store funct3: {:#05b} (instruction: {:#010x})", funct3, instruction),
    }

    self.pc += 4;
}

// ONE J-INSTRUCTION
pub(super) fn ins_j(&mut self, instruction: u32) {
    let rd = ((instruction >> 7) & 0x1F) as usize;

    let imm19_12 = (instruction >> 12) & 0xFF;
    let imm11 = (instruction >> 20) & 1;
    let imm10_1 = (instruction >> 21) & 0x3FF;
    let sign = (instruction >> 31) & 1;

    let imm = Self::sign_extend(
        (imm10_1 << 1) | (imm11 << 11) | (imm19_12 << 12) | (sign << 20),
        21,
    );

    self.gpr[rd] = self.pc + 4;
    self.pc = ((self.pc as i64) + (imm as i64)) as u64;
}

// U INSTRUCTIONS
pub(super) fn ins_u(&mut self, instruction: u32) {
    let opcode = (instruction & 0x7F) as u8;
    let rd = ((instruction >> 7) & 0x1F) as usize;
    let imm = instruction >> 12;

    match opcode {
        0b0110111 => self.gpr[rd] = (imm << 12) as u64,
        0b0010111 => self.gpr[rd] = self.pc + ((imm << 12) as u64),
        _ => {}
    }

    self.pc += 4;
}
}

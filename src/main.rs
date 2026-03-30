// Note: main.h in the original C project contained a partial/old ins_B implementation.
// It is not part of the active build and is preserved here as a comment for reference.
//
// ins_B (old version from main.h):
// fn ins_b_old(instruction: u32, cpu: &mut CpuData) {
//     let funct3 = ((instruction >> 12) & 0x7) as u8;
//     let rs1 = ((instruction >> 15) & 0x1F) as u8;
//     let rs2 = ((instruction >> 20) & 0x1F) as u8;
//     let imm12 = ((instruction >> 31) & 0x1) as u8;
//     let imm10_5 = ((instruction >> 25) & 0x3F) as u8;
//     let imm11 = ((instruction >> 7) & 0x1) as u8;
//     let imm4_1 = ((instruction >> 8) & 0xF) as u8;
//     let imm = (imm12 << 12) | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1);
//     match funct3 {
//         0x0 => if rs1 == rs2 { cpu.pc += imm as u64; }
//         0x1 => if rs1 != rs2 { cpu.pc += imm as u64; }
//         0x4 => if rs1 < rs2 { cpu.pc += imm as u64; }
//         _ => {}
//     }
// }

use risc_v_emulator_rust::bus;
use risc_v_emulator_rust::emulator::Emulator;
use risc_v_emulator_rust::mmu;
use std::fs::File;
use std::io::Read;

fn print_instruction(instruction: u32) {
    println!("0x{:08X}", instruction);
}

fn main() {
    let mut emu = Emulator::new();

    let mut f = File::open("program.bin").expect("fopen");
    let mut buf = [0u8; 4];

    while f.read_exact(&mut buf).is_ok() {
        let instruction = (buf[0] as u32)
            | ((buf[1] as u32) << 8)
            | ((buf[2] as u32) << 16)
            | ((buf[3] as u32) << 24);

        print_instruction(instruction);
        instructions::read_ins(instruction, &mut emu.cpu, &mut emu.mem);

        for i in 0..32 {
            println!("{}", emu.cpu.gpr[i]);
        }
    }
}

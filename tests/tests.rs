use risc_v_emulator_rust::cpu::CpuData;
use risc_v_emulator_rust::instructions::read_ins;

#[test]
fn test_all() {
    let mut cpu = CpuData::new();

    // ============================
    // I-TYPE INSTRUCTIONS
    // ============================
    // ADDI x1, x0, 5
    read_ins(5 << 20 | 0 << 15 | 0 << 12 | 1 << 7 | 0b0010011, &mut cpu);
    assert_eq!(cpu.gpr[1], 5);

    // XORI x2, x1, 3
    read_ins(3 << 20 | 1 << 15 | 0x4 << 12 | 2 << 7 | 0b0010011, &mut cpu);
    assert_eq!(cpu.gpr[2], (5 ^ 3));

    // ORI x3, x1, 2
    read_ins(2 << 20 | 1 << 15 | 0x6 << 12 | 3 << 7 | 0b0010011, &mut cpu);
    assert_eq!(cpu.gpr[3], (5 | 2));

    // ANDI x4, x2, 7
    read_ins(7 << 20 | 2 << 15 | 0x7 << 12 | 4 << 7 | 0b0010011, &mut cpu);
    assert_eq!(cpu.gpr[4], ((5 ^ 3) & 7));

    // SLLI x5, x1, 1
    read_ins(0x0 << 25 | 1 << 20 | 1 << 15 | 0x1 << 12 | 5 << 7 | 0b0010011, &mut cpu);
    assert_eq!(cpu.gpr[5], (5u64 << 1));

    // SRLI x6, x1, 1
    read_ins(0x0 << 25 | 1 << 20 | 1 << 15 | 0x5 << 12 | 6 << 7 | 0b0010011, &mut cpu);
    assert_eq!(cpu.gpr[6], (5u64 >> 1));

    // SRAI x7, x1, 1
    read_ins(0x20 << 25 | 1 << 20 | 1 << 15 | 0x5 << 12 | 7 << 7 | 0b0010011, &mut cpu);
    assert_eq!(cpu.gpr[7], ((5i32 >> 1) as u64));

    // SLTI x8, x1, 10
    read_ins(10 << 20 | 1 << 15 | 0x2 << 12 | 8 << 7 | 0b0010011, &mut cpu);
    assert_eq!(cpu.gpr[8], 1);

    // SLTIU x9, x1, 10
    read_ins(10 << 20 | 1 << 15 | 0x3 << 12 | 9 << 7 | 0b0010011, &mut cpu);
    assert_eq!(cpu.gpr[9], 1);

    // ============================
    // R-TYPE INSTRUCTIONS
    // ============================
    // ADD x10, x1, x2
    read_ins(0x0 << 25 | 2 << 20 | 1 << 15 | 0x0 << 12 | 10 << 7 | 0b0110011, &mut cpu);
    assert_eq!(cpu.gpr[10], (5 + (5 ^ 3)));

    // SUB x11, x2, x1
    read_ins(0x20 << 25 | 1 << 20 | 2 << 15 | 0x0 << 12 | 11 << 7 | 0b0110011, &mut cpu);
    assert_eq!(cpu.gpr[11], ((5u64 ^ 3) - 5));

    // XOR x12, x1, x2
    read_ins(0x0 << 25 | 2 << 20 | 1 << 15 | 0x4 << 12 | 12 << 7 | 0b0110011, &mut cpu);
    assert_eq!(cpu.gpr[12], (5 ^ (5 ^ 3)));

    // OR x13, x1, x2
    read_ins(0x0 << 25 | 2 << 20 | 1 << 15 | 0x6 << 12 | 13 << 7 | 0b0110011, &mut cpu);
    assert_eq!(cpu.gpr[13], (5 | (5 ^ 3)));

    // AND x14, x1, x2
    read_ins(0x0 << 25 | 2 << 20 | 1 << 15 | 0x7 << 12 | 14 << 7 | 0b0110011, &mut cpu);
    assert_eq!(cpu.gpr[14], (5 & (5 ^ 3)));

    // SLL x15, x1, x2
    read_ins(0x0 << 25 | 2 << 20 | 1 << 15 | 0x1 << 12 | 15 << 7 | 0b0110011, &mut cpu);
    assert_eq!(cpu.gpr[15], (5u64 << ((5u64 ^ 3) & 0x1F)));

    // SLT x16, x1, x2
    read_ins(0x0 << 25 | 2 << 20 | 1 << 15 | 0x2 << 12 | 16 << 7 | 0b0110011, &mut cpu);
    assert_eq!(cpu.gpr[16], if 5 < (5 ^ 3) { 1 } else { 0 });

    // SLTU x17, x1, x2
    read_ins(0x0 << 25 | 2 << 20 | 1 << 15 | 0x3 << 12 | 17 << 7 | 0b0110011, &mut cpu);
    assert_eq!(cpu.gpr[17], if 5u64 < (5 ^ 3) { 1 } else { 0 });

    // SRL x18, x1, x2
    read_ins(0x0 << 25 | 2 << 20 | 1 << 15 | 0x5 << 12 | 18 << 7 | 0b0110011, &mut cpu);
    assert_eq!(cpu.gpr[18], ((5u32 >> ((5u32 ^ 3) & 0x1F)) as u64));

    // SRA x19, x1, x2
    read_ins(0x20 << 25 | 2 << 20 | 1 << 15 | 0x5 << 12 | 19 << 7 | 0b0110011, &mut cpu);
    assert_eq!(cpu.gpr[19], ((5i32 >> ((5u32 ^ 3) & 0x1F)) as u64));

    // ============================
    // RV32M MULTIPLY INSTRUCTIONS
    // ============================
    // MUL x20, x1, x2
    read_ins(0x01 << 25 | 2 << 20 | 1 << 15 | 0x0 << 12 | 20 << 7 | 0b0110011, &mut cpu);
    assert_eq!(cpu.gpr[20], (5i32 * (5i32 ^ 3)) as u64);

    // MULH x21, x1, x2
    read_ins(0x01 << 25 | 2 << 20 | 1 << 15 | 0x1 << 12 | 21 << 7 | 0b0110011, &mut cpu);
    let prod: i64 = (5i64) * ((5i64) ^ 3);
    assert_eq!(cpu.gpr[21], (prod >> 32) as i32 as u64);

    // MULHSU x22, x1, x2
    read_ins(0x01 << 25 | 2 << 20 | 1 << 15 | 0x2 << 12 | 22 << 7 | 0b0110011, &mut cpu);
    let prod: i64 = (5i64).wrapping_mul((5u64 ^ 3) as i64);
    assert_eq!(cpu.gpr[22], (prod >> 32) as i32 as u64);

    // MULHU x23, x1, x2
    read_ins(0x01 << 25 | 2 << 20 | 1 << 15 | 0x3 << 12 | 23 << 7 | 0b0110011, &mut cpu);
    let uprod: u64 = (5u64) * (5u64 ^ 3);
    assert_eq!(cpu.gpr[23], (uprod >> 32) as u32 as u64);

    println!("[ALL RV32I + RV32M TESTS PASSED]");

    // ----------------------------
    // S-TYPE STORE INSTRUCTIONS
    // ----------------------------
    cpu.gpr[1] = 100; // base
    cpu.gpr[2] = 0xAB; // value

    // SB x2, 0(x1)
    read_ins(0 << 25 | 2 << 20 | 1 << 15 | 0 << 12 | 0 << 7 | 0x23, &mut cpu);
    assert_eq!(cpu.mem[100], 0xAB);

    // SH x2, 2(x1)
    cpu.gpr[2] = 0xCDEF;
    read_ins(0 << 25 | 2 << 20 | 1 << 15 | 1 << 12 | 2 << 7 | 0x23, &mut cpu);
    assert_eq!(cpu.mem[102], 0xEF);
    assert_eq!(cpu.mem[103], 0xCD);

    // SW x2, 4(x1)
    cpu.gpr[2] = 0x12345678;
    read_ins(0 << 25 | 2 << 20 | 1 << 15 | 2 << 12 | 4 << 7 | 0x23, &mut cpu);
    assert_eq!(cpu.mem[104], 0x78);
    assert_eq!(cpu.mem[105], 0x56);
    assert_eq!(cpu.mem[106], 0x34);
    assert_eq!(cpu.mem[107], 0x12);

    println!("[TEST] S-TYPE PASSED");

    // ----------------------------
    // B-TYPE BRANCH INSTRUCTIONS
    // ----------------------------
    cpu.pc = 0;
    cpu.gpr[1] = 5;
    cpu.gpr[2] = 5;

    // BEQ x1, x2, +8
    let imm: u32 = 8;
    let imm12 = (imm >> 12) & 0x1;
    let imm11 = (imm >> 11) & 0x1;
    let imm10_5 = (imm >> 5) & 0x3F;
    let imm4_1 = (imm >> 1) & 0xF;
    let beq_instr = (imm12 << 31)
        | (imm10_5 << 25)
        | (2 << 20)
        | (1 << 15)
        | (0 << 12)
        | (imm4_1 << 8)
        | (imm11 << 7)
        | 0b1100011;
    read_ins(beq_instr, &mut cpu);
    assert_eq!(cpu.pc, 8);

    println!("[TEST] B-TYPE PASSED");

    // ----------------------------
    // JAL INSTRUCTION
    // ----------------------------
    cpu.pc = 0;
    let jal_imm: u32 = 16;
    let jal_instr = ((jal_imm & 0x1000) << 19)    // imm[20]
        | ((jal_imm & 0xFF0) << 20)               // imm[10:1]
        | ((jal_imm & 0x800) << 9)                // imm[11]
        | ((jal_imm & 0x7FE00000) >> 0)           // imm[19:12] not needed for small test
        | (1 << 7)
        | 0b1101111; // rd=1, opcode=JAL
    read_ins(jal_instr, &mut cpu);
    assert_eq!(cpu.gpr[1], 4); // PC+4
    assert_eq!(cpu.pc, 16);

    // ----------------------------
    // JALR INSTRUCTION
    // ----------------------------
    cpu.pc = 0;
    cpu.gpr[1] = 100;
    let jalr_imm: u32 = 12;
    let jalr_instr = (jalr_imm << 20) | (1 << 15) | (0x0 << 12) | (2 << 7) | 0b1100111;
    read_ins(jalr_instr, &mut cpu);
    assert_eq!(cpu.gpr[2], 4); // rd = PC+4
    assert_eq!(cpu.pc, 112); // PC = rs1 + imm

    // ----------------------------
    // LUI / AUIPC
    // ----------------------------
    // LUI x3, 0x12345
    let lui_instr: u32 = (0x12345 << 12) | (3 << 7) | 0b0110111;
    read_ins(lui_instr, &mut cpu);
    assert_eq!(cpu.gpr[3], 0x12345000);

    // AUIPC x4, 0x1
    cpu.pc = 0x100;
    let auipc_instr: u32 = (0x1 << 12) | (4 << 7) | 0b0010111;
    read_ins(auipc_instr, &mut cpu);
    assert_eq!(cpu.gpr[4], 0x100 + 0x1000); // PC + imm<<12

    println!("[TEST] J / LUI / AUIPC / ECALL");
    println!("[ALL EXTRA INSTRUCTIONS PASSED]");
}

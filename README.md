# RISC-V Emulator

A RISC-V emulator written in Rust.

## Implemented

- **RV32I** — base integer instruction set (all core instructions)
- **RV32M** — multiply/divide extension
- **Privilege levels** — U, S, M mode (CSR space allocated)
- **MMU** — Sv39 virtual address translation (in progress)
- **Display** — optional SDL2 window (feature-gated)

## Instruction Set

| Format | Instructions |
|--------|-------------|
| R | ADD, SUB, XOR, OR, AND, SLL, SRL, SRA, SLT, SLTU, MUL, MULH, MULHSU, MULHU, DIV, DIVU, REM, REMU |
| I | ADDI, XORI, ORI, ANDI, SLLI, SRLI, SRAI, SLTI, SLTIU, LB, LH, LW, LBU, LHU, JALR |
| S | SB, SH, SW |
| B | BEQ, BNE, BLT, BGE, BLTU, BGEU |
| J | JAL |
| U | LUI, AUIPC |

## Project Structure

```
src/
├── main.rs          # Entry point
├── lib.rs           # Module exports
├── emulator/        # Emulator struct (owns CPU + memory)
├── cpu/             # CpuData struct (registers, CSRs, PC)
├── instructions/    # Instruction decoding and execution
├── memory/          # RAM and load/store helpers
├── mmu/             # Virtual address translation (Sv39)
└── display/         # SDL2 display output (optional)
```

## Building

```sh
# Standard build
cargo build --release

# With display support (requires SDL2)
cargo build --release --features display
```

## Running

The emulator loads a raw binary (`program.bin`) from the working directory:

```sh
cargo run --release
```

## Memory

- RAM size: 4 MB
- Stack pointer initialized to top of RAM (`0x400000`)
- All unimplemented opcodes and funct codes panic with the raw instruction value

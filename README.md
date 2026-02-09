# CVM

A C runtime VM with garbage collection.

CVM compiles a subset of C to bytecode and runs it on a register-based virtual machine. The design is heavily inspired by Lua's VM architecture.

## Current Status

Working:
- Lexer, parser, semantic analyzer for C99 subset
- Bytecode generation with register allocation
- Arithmetic, comparisons, control flow (if/else, while loops)
- Constant pooling and register optimizations

In progress:
- For loops
- VM interpreter
- Garbage collector design

## Bytecode Architecture

**Instruction Format:**
- Fixed 32-bit instructions (Lua-style encoding)
- Register-based VM with 256 virtual registers per function
- Three instruction formats:
  - iABC: 3-operand instructions (arithmetic, comparisons, moves)
  - iABx: register + large immediate (constant loading)
  - iAsBx: signed offsets (jumps, control flow)

**Register Allocation:**
- Permanent registers: assigned to declared variables, never freed
- Temporary registers: used for intermediate values, freed after use
- Bitvec tracking for O(1) allocation (find first free register)

**Optimizations:**
- Constant deduplication: identical constants share table entries
- Dead register reuse: temps freed immediately after last use
- Result register reuse: operations write to operand registers when safe
- Target register forwarding: expressions compute directly into destination
- Special-case variable copies: `y = x` generates single MOV, not load+store

See `isa_spec.txt` for complete ISA specification and examples.

## Language Support

Supports: variables, arithmetic, comparisons, if/else, while loops, assignments

Not yet: functions, arrays, pointers, for loops, structs

Won't support: VLAs, preprocessor, volatile/restrict/inline, complex types

## Building
```sh
cargo build
cargo run <source.c>
```
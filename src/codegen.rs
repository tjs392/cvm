// see design.txt for isa specs

// I will be taking using Lua's bytecode format closely
// for iABC instruction:
// [opcode][dest][reg][reg]
// [  6   ][ 8  ][ 9 ][ 9 ]
// for the two registers at the end,
// use the first 8 bits for the reg number
// and then a flag bit for the constant table index

use std::collections::HashMap;

use bitvec::vec::BitVec;

use crate::ast::Program;

// 6 bit opcode
pub enum OpCode {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
}

// enforce 9 bit register for b and c
pub struct Instruction {
    opcode: OpCode,
    a: u8, // 8 bit destination register
    b: u16, // 9 bit source register
    c: u16, // another 9 it source register
}

pub struct CodeGenerator {
    // returned instructs
    instructions: Vec<Instruction>,

    // static 256 (8bit) registers, so using a bitvec to 
    // track state. 0 = not in use, 1 = in use
    register_state: BitVec,

    // var name -> register id
    sym_table: HashMap<String, u8>,

    // max register allocated so can 
    // allocate registers at compile time
    max_reg: u8,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            instructions: vec![],
            register_state: BitVec::repeat(false, 256),
            sym_table: HashMap::new(),
            max_reg: 0,
        }
    }

    // allocate mem to reg and return its id
    fn allocate_register(&mut self) -> u8 {
        todo!()
    }

    // free register
    fn free_register(&mut self,reg_id: u8) {
        todo!()
    }

    // emit instruction to instr vec
    fn emit(&mut self, instr: Instruction) {
        todo!()
    }

    pub fn gen_program(program: &Program) {
        todo!()
    }
}
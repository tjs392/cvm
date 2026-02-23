use std::collections::HashMap;

use crate::codegen::{FunctionChunk, Instruction, OpCode};

/* 
    The VM >:D

    Execution Loop:

    VM Has:
        stack:          Vec<i64>, 8192 slots, shared across all functions calls
        frames:         Vec<CallFrame>, each fram tracks one active func
        functions:      Vec<FunctionChunk>, the bytecode from codegen
        functionMap:    Hashmap<String, usize>, maps the function name to indices in Functions
    
    CallFrame has:
        function_idx:   the function chunk that's the vm is currently running
        pc:             program counter
        base:           offset into global stack, -- register window start 
    
    Initializing:
        - find "main" in function map and push a callframe to frames
        - will look like frames: [CallFrame {function_idx: 1, pc: 0, base: 0} ]
        - this is handling the call frame's "environment"
    
    Iteration Loop:
        - grab current frame (last elem on frames stack)
        - read the instruction at frame[top].pc from that func's bytecode
        - increment pc **BEFORE** executing (because of JUMP)
        - match on instruction and do the thing
        ** every register access is stack[base + register_number]
           so r0 = stack[base + 0], r3 = stack[base + 3]

    Regular Instructions (LOADK, MOV, ADD, SUB, etc.):
        - pretty straightforward, just read and write to the stack
        - LOADK r0, K1   -->  stack[base + 0] = constants[1]
        - MOV r1, r0     -->  stack[base + 1] = stack[base + 0]
        - ADD r2, r0, r1 -->  stack[base + 2] = stack[base + 0] + stack[base + 1]
        - all arithmetic/bitwise/comparison ops follow the same pattern
    
    TEST:
        - TEST rA checks if stack[base + a] == 0
        - if zero, skip the next instruction (pc += 1)
        - if not zero, do nothing
        - next instruction is always a JMP so test does are we jumping or not
    
    JMP:
        - JMP has signed offset
        - pc = (pc as i32 + offset) as usize
        - pc is incremented BEFORE execution so the offset is relative to the instruciton AFTER JMP
    
    CLOSURE:
        - CLOSE rA, Fx just stores the function index in the stack
        - stack[base + a] = function_index
        - func index is just a number and CALL reads it later
        - kinda like storing a func pointer but instead of mem address its an index into the funcitons array
    
    CALL:
        - CALL rA, B, C
            - step 1: read function index from stack[base + a] (CLOSURE put it there)
            - step 2: calc the new base = current_base + a + 1
            - step 3: push new CallFrame
    
    RETURN:
        -RETURN rA, B
            - step1: if B == 2, grab the return value
            - step2: popthe current frame off the frames stack
            - step 3: check if frames is empty, if empty program is done
                            if not empty, return to caller
            - step 4: find where to put the ret value, the callers frame is now on top stack
            - step 5 continue

*/

struct CallFrame {
    /// which func chunk is currently going
    function_idx: usize,

    /// program counter
    pc: usize,

    /// offset into the global register array
    base: usize,
}

pub struct VM {
    /// global register stack
    stack: Vec<i64>,

    /// global call stack
    frames: Vec<CallFrame>,

    /// all the functions from codegen
    functions: Vec<FunctionChunk>,

    /// 
    function_map: HashMap<String, usize>,
}

impl VM {
    pub fn new(functions: Vec<FunctionChunk>, function_map: HashMap<String, usize>) -> Self {
        VM {
            stack: vec![0i64; 8192],
            frames: vec![],
            functions,
            function_map,
        }
    }

    pub fn run(&mut self) -> i64 {
        let main_idx = *self.function_map.get("main").expect("No main function found");
        self.frames.push(CallFrame {
            function_idx: main_idx,
            pc: 0,
            base: 0,
        });

        loop {
            let frame = self.frames.last().unwrap();
            let func_idx = frame.function_idx;
            let pc = frame.pc;
            let base = frame.base;

            let func = &self.functions[func_idx];
            let instr = &func.instructions[pc];

            self.frames.last_mut().unwrap().pc += 1;

            match instr {
                Instruction::ABx { opcode, a, bx } => {
                    match opcode {
                        OpCode::LOADK => {
                            let constant = self.functions[func_idx].constants[*bx as usize];
                            self.stack[base + *a as usize] = constant;
                        }
                        OpCode::CLOSURE => {
                            self.stack[base + *a as usize] = *bx as i64;
                        }
                        _ => panic!("Unknown ABx opcode"),
                    }
                }

                Instruction::ABC { opcode, a, b, c } => {
                    match opcode {
                        OpCode::ADD => {
                            self.stack[base + *a as usize] = self.stack[base + *b as usize] + self.stack[base + *c as usize];
                        }
                        OpCode::MOV => {
                            self.stack[base + *a as usize] = self.stack[base + *b as usize];
                        }
                        OpCode::RETURN => {
                            let return_val = if *b == 2 {
                                self.stack[base + *a as usize]
                            } else {
                                0
                            };

                            self.frames.pop();

                            if self.frames.is_empty() {
                                return return_val;
                            }

                            let caller = self.frames.last().unwrap();
                            let caller_func = &self.functions[caller.function_idx];
                            let call_instr = &caller_func.instructions[caller.pc - 1];

                            if let Instruction::ABC { a: call_a, .. } = call_instr {
                                self.stack[caller.base + *call_a as usize] = return_val;
                            }
                        }

                        OpCode::CALL => {
                            let func_idx = self.stack[base + *a as usize] as usize;
                            let new_base = base + *a as usize + 1;
                            self.frames.push(CallFrame {
                                function_idx: func_idx,
                                pc: 0,
                                base: new_base,
                            });
                        }

                        OpCode::SUB => {
                            self.stack[base + *a as usize] = self.stack[base + *b as usize] - self.stack[base + *c as usize];
                        }

                        OpCode::MUL => {
                            self.stack[base + *a as usize] = self.stack[base + *b as usize] * self.stack[base + *c as usize];
                        }

                        OpCode::DIV => {
                            self.stack[base + *a as usize] = self.stack[base + *b as usize] / self.stack[base + *c as usize];
                        }

                        OpCode::MOD => {
                            self.stack[base + *a as usize] = self.stack[base + *b as usize] % self.stack[base + *c as usize];
                        }

                        OpCode::EQ => {
                            self.stack[base + *a as usize] = (self.stack[base + *b as usize] == self.stack[base + *c as usize]) as i64;
                        }

                        OpCode::NE => {
                            self.stack[base + *a as usize] = (self.stack[base + *b as usize] != self.stack[base + *c as usize]) as i64;
                        }

                        OpCode::LT => {
                            self.stack[base + *a as usize] = (self.stack[base + *b as usize] < self.stack[base + *c as usize]) as i64;
                        }
                        
                        OpCode::LE => {
                            self.stack[base + *a as usize] = (self.stack[base + *b as usize] <= self.stack[base + *c as usize]) as i64;
                        }

                        OpCode::GT => {
                            self.stack[base + *a as usize] = (self.stack[base + *b as usize] > self.stack[base + *c as usize]) as i64;
                        }

                        OpCode::GE => {
                            self.stack[base + *a as usize] = (self.stack[base + *b as usize] >= self.stack[base + *c as usize]) as i64;
                        }

                        OpCode::BAND => {
                            self.stack[base + *a as usize] = self.stack[base + *b as usize] & self.stack[base + *c as usize];
                        }
                        
                        OpCode::BOR => {
                            self.stack[base + *a as usize] = self.stack[base + *b as usize] | self.stack[base + *c as usize];
                        }

                        OpCode::BXOR => {
                            self.stack[base + *a as usize] = self.stack[base + *b as usize] ^ self.stack[base + *c as usize];
                        }

                        OpCode::SHL => {
                            self.stack[base + *a as usize] = self.stack[base + *b as usize] << self.stack[base + *c as usize];
                        }

                        OpCode::SHR => {
                            self.stack[base + *a as usize] = self.stack[base + *b as usize] >> self.stack[base + *c as usize];
                        }

                        OpCode::UNM => {
                            self.stack[base + *a as usize] = -self.stack[base + *b as usize];
                        }

                        OpCode::NOT => {
                            self.stack[base + *a as usize] = (self.stack[base + *b as usize] == 0) as i64;
                        }

                        OpCode::BNOT => {
                            self.stack[base + *a as usize] = !self.stack[base + *b as usize];
                        }

                        OpCode::TEST => {
                            if self.stack[base + *a as usize] != 0 {
                                self.frames.last_mut().unwrap().pc += 1;
                            }
                        }

                        _ => panic!("unknown iABC"),
                    }
                }

                Instruction::AsBx { opcode, offset } => {
                    match opcode {
                        OpCode::JMP => {
                            let current_pc = self.frames.last().unwrap().pc as i32;
                            self.frames.last_mut().unwrap().pc = (current_pc + offset) as usize;
                        }
                        _ => panic!("Unknown AsBx opcode"),
                    }
                }
            }
        }
    }
}
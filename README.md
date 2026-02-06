# CVM

C subset language with a VM and garbage collector. Written in Rust.

## What it is

CVM compiles a subset of C99 to bytecode and runs it on a register-based virtual machine with automatic garbage collection.

Status: lexer, parser, and semantic analyzer done. Currently building the bytecode compiler and VM.

## Done
- Lexer
- Parser  
- AST
- Semantic analysis
- Symbol table

## Working on
- Bytecode generation (register-based, Lua-style instruction format)
- VM interpreter
- Garbage collector

## End goal
Run a web server written in CVM that binds to a port and handles requests

## Language features

Types: int, char, short, long, float, double, void, pointers, arrays, structs, unions, enums, typedef, const

Control flow: if/else, while, for, do-while, switch, break, continue, goto, return

Operators: arithmetic, comparison, logical, bitwise, assignment variants, ternary, member access, pointers, array indexing, cast, sizeof

Storage classes: static, extern

Extensions: native bool type (true/false)

Not supported: VLAs, compound literals, preprocessor macros, volatile/restrict/inline, complex types, designated initializers

## Architecture

Bytecode: Fixed 32-bit instructions, register-based (see isa_spec.txt)

VM: 256 virtual registers per function, interpreter executes bytecode

GC: not sure yet
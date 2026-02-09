# CVM

CVM is a C subset language with a VM and garbage collector. Written in Rust.
It will compile a subset of C99 to bytecode and run it on a register-based virtual machine with automatic garbage collection.

Status: Currently building and designing bytecode ISA for da V

## Language features

C99 Subset Lang Right Now

Not supported: VLAs, compound literals, preprocessor macros, volatile/restrict/inline, complex types, designated initializers

## Architecture

Bytecode: Fixed 32-bit instructions, register-based (see isa_spec.txt)

VM: 256 virtual registers per function, interpreter executes bytecode, etc. Lua/JVM inspired

GC: not sure yet

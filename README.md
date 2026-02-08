# CVM

C subset language with a VM and garbage collector. Written in Rust.

## What it is

CVM will compile a subset of C99 to bytecode and run it on a register-based virtual machine with automatic garbage collection.

Status: lexer, parser, and semantic analyzer done. Currently building and designing bytecode ISA for da VM

## Language features

C99 Subset Lang Right Now

Not supported: VLAs, compound literals, preprocessor macros, volatile/restrict/inline, complex types, designated initializers

## Architecture

Bytecode: Fixed 32-bit instructions, register-based (see isa_spec.txt)

VM: 256 virtual registers per function, interpreter executes bytecode, etc. Lua/JVM inspired

GC: not sure yet
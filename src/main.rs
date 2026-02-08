mod ast;
mod lexer;
mod parser;
mod symbol_table;
mod semantic;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use ast::{Declaration, Program};
use codegen::CodeGenerator;
use std::env;
use std::fs;
use std::process;

use crate::semantic::SemanticAnalyzer;

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("error reading file '{}': {}", filename, e);
            process::exit(1);
        }
    }
}

fn lex(source: &str) -> Vec<lexer::Token> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}

fn parse(tokens: Vec<lexer::Token>) -> Program {
    let mut parser = Parser::new(tokens);
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        parser.parse_program()
    })) {
        Ok(ast) => ast,
        Err(_) => {
            eprintln!("\nParsing failed!");
            process::exit(1);
        }
    }
}

fn analyze(ast: &Program) -> Result<(), Vec<String>> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(ast)
}

fn compile(ast: &Program) -> CodeGenerator {
    let mut codegen = CodeGenerator::new();
    codegen.gen_program(ast);
    codegen
}

fn print_ast(ast: &Program) {
    println!("\n======== AST ========");
    println!("{:#?}", ast);
}

fn print_semantic_results(result: &Result<(), Vec<String>>) {
    println!("\n======== SEMANTIC ANALYSIS ========");
    match result {
        Ok(()) => println!("No semantic errors found"),
        Err(errors) => {
            println!("Found {} semantic error(s):\n", errors.len());
            for (i, err) in errors.iter().enumerate() {
                println!("  {}. {}", i + 1, err);
            }
        }
    }
}

fn print_codegen_results(codegen: &CodeGenerator) {
    println!("\n======== BYTECODE ========");
    codegen.print_instructions();
}

fn print_summary(ast: &Program) {
    println!("\n======== SUMMARY ========");
    println!("Total declarations: {}", ast.declarations.len());
    
    let mut func_count = 0;
    let mut var_count = 0;
    let mut struct_count = 0;
    let mut union_count = 0;
    let mut enum_count = 0;
    let mut typedef_count = 0;
    
    for decl in &ast.declarations {
        match decl {
            Declaration::Function(f) => {
                func_count += 1;
                let has_body = if f.body.is_some() { "definition" } else { "declaration" };
                println!("  [Function] {} ({} params, {})", 
                         f.name, f.params.len(), has_body);
            }
            Declaration::Variable(v) => {
                var_count += 1;
                let has_init = if v.init.is_some() { "initialized" } else { "uninitialized" };
                println!("  [Variable] {} ({})", v.name, has_init);
            }
            Declaration::Struct(s) => {
                struct_count += 1;
                let name = s.name.as_ref().map(|n| n.as_str()).unwrap_or("<anonymous>");
                println!("  [Struct] {} ({} fields)", name, s.fields.len());
            }
            Declaration::Union(u) => {
                union_count += 1;
                let name = u.name.as_ref().map(|n| n.as_str()).unwrap_or("<anonymous>");
                println!("  [Union] {} ({} fields)", name, u.fields.len());
            }
            Declaration::Enum(e) => {
                enum_count += 1;
                let name = e.name.as_ref().map(|n| n.as_str()).unwrap_or("<anonymous>");
                println!("  [Enum] {} ({} variants)", name, e.variants.len());
            }
            Declaration::Typedef(t) => {
                typedef_count += 1;
                println!("  [Typedef] {}", t.name);
            }
        }
    }
    
    println!("\n======== DECLARATION COUNTS ========");
    println!("Functions: {}", func_count);
    println!("Variables: {}", var_count);
    println!("Structs:   {}", struct_count);
    println!("Unions:    {}", union_count);
    println!("Enums:     {}", enum_count);
    println!("Typedefs:  {}", typedef_count);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file.c>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    let source = read_file(filename);
    let tokens = lex(&source);
    let ast = parse(tokens);
    let semantic_result = analyze(&ast);
    let codegen = compile(&ast);

    print_ast(&ast);
    print_semantic_results(&semantic_result);
    print_codegen_results(&codegen);
    print_summary(&ast);

    if semantic_result.is_err() {
        process::exit(1);
    }
}
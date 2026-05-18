use std::env;
use std::fs;
use std::process;

pub mod lexer;
pub mod parser;
pub mod semantic;

use crate::lexer::lexer::Lexer;
use crate::parser::parser::Parser;
use crate::semantic::analyzer;
use crate::semantic::analyzer::SemanticAnalyzer;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Use:{} <archive.c>", args[0]);
        process::exit(1);
    };
    let file_path = &args[1];
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("error reading file {}:{}", file_path, err);
        process::exit(1);
    });
    println!("{}", source_code);

    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();
    println!("{:#?}", ast);
    let mut analyzer = SemanticAnalyzer::new();
    match analyzer.analyzer_program(&ast) {
        Ok(()) => println!("Sucess"),
        Err(_) => {
            eprintln!("shit");
            process::exit(1);
        }
    }
}

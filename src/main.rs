mod ast;
mod lexer;
mod parser;
mod compiler;

use lexer::Lexer;
use parser::Parser;
use compiler::Compiler;

fn main() {
    let filename = "main.xi";
    let script = r#"
        import XiHash
        Console.Print("Hello, World!");
        using namespace Console
        Print("Hello, World!")
        string name = "Alice"
        Print("Hello, " + name + "!")
        Console.Print("My name is: XiHash!")
    "#;

    println!("--- 1. Lexing Source Code ({}) ---", filename);
    let lexer_result = Lexer::tokenize(script, filename);
    println!("Generated Include Enums Array: {:?}", lexer_result.required_includes);

    println!("\n--- 2. Parsing into AST (With Token Namespaces) ---");
    let statements = Parser::parse(lexer_result.tokens);
    println!("Successfully parsed {} statements.", statements.len());

    println!("\n--- 3. Compiling to C++ ---");
    let output_cpp_code = Compiler::compile(&statements, &lexer_result.required_includes);

    println!("\nGenerated C++ Output:\n{}", output_cpp_code);
}
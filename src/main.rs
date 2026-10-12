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
        string name = "Alice"
        int age = 30
        uint score = 9500
        ufloat temperature = 98.6
        double pi = 3.14159

        using namespace Console
        Print("Hello, " + name + "!")
    "#;

    let lexer_result = Lexer::tokenize(script, filename);
    let statements = Parser::parse(lexer_result.tokens);
    let output_cpp_code = Compiler::compile(&statements, &lexer_result.required_includes);

    println!("\nGenerated C++ Output:\n{}", output_cpp_code);
}
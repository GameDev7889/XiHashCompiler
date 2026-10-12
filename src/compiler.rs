use crate::ast::{Statement, RequiredInclude};

pub struct Compiler;

impl Compiler {
    pub fn compile(statements: &[Statement], required_includes: &[RequiredInclude]) -> String {
        let mut globals: Vec<String> = Vec::new();
        let mut main_body: Vec<String> = Vec::new();

        // Translate the array of RequiredInclude enums directly into C++ #include directives
        for inc in required_includes {
            match inc {
                RequiredInclude::Iostream => globals.push("#include <iostream>".to_string()),
                RequiredInclude::String => globals.push("#include <string>".to_string()),
                RequiredInclude::XiHash => globals.push("#include <XiHash.h>".to_string()),
                RequiredInclude::Custom(m) => globals.push(format!("#include <{}.h>", m)),
            }
        }

        // Global mock namespace definition
        globals.push(r#"namespace Console {
    void Print(const std::string& msg) { std::cout << msg << std::endl; }
}"#.to_string());

        // Process AST statements
        for stmt in statements {
            match stmt {
                Statement::Import { .. } => {
                    // Skipped here because includes are fully handled via required_includes
                }
                Statement::UsingNamespace { ns, .. } => {
                    globals.push(format!("using namespace {};", ns));
                }
                Statement::VarDecl { .. } | Statement::PrintCall { .. } => {
                    let cpp_line = stmt.to_cpp();
                    if !cpp_line.is_empty() {
                        main_body.push(format!("    {}", cpp_line));
                    }
                }
            }
        }

        // Assemble final C++ text
        let mut cpp_code = String::new();
        for global in globals {
            cpp_code.push_str(&format!("{}\n", global));
        }
        cpp_code.push('\n');

        cpp_code.push_str("int main() {\n");
        for line in main_body {
            cpp_code.push_str(&format!("{}\n", line));
        }
        cpp_code.push_str("    return 0;\n");
        cpp_code.push_str("}\n");

        cpp_code
    }
}
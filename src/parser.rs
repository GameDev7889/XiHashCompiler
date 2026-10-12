use crate::ast::{Statement, Expr};
use crate::lexer::Token;

pub struct Parser;

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
        let mut statements = Vec::new();

        // List of all supported variable types in your language
        let known_types = vec!["string", "int", "uint", "float", "double", "ulong", "ufloat", "udouble"];

        for token in tokens {
            let line = &token.raw;
            let ns = token.namespace.clone();

            if line.starts_with("import ") {
                let module = line.strip_prefix("import ").unwrap().trim().to_string();
                statements.push(Statement::Import { module, namespace: ns });
            }
            else if line.starts_with("using namespace ") {
                let target_ns = line.strip_prefix("using namespace ").unwrap().trim().to_string();
                statements.push(Statement::UsingNamespace { ns: target_ns, namespace: ns });
            }
            else {
                // Check if the line starts with any known type keyword
                let mut matched_type: Option<&str> = None;
                for t in &known_types {
                    if line.starts_with(&format!("{} ", t)) {
                        matched_type = Some(t);
                        break;
                    }
                }

                if let Some(var_type) = matched_type {
                    let rest = line.strip_prefix(&format!("{} ", var_type)).unwrap().trim();
                    let parts: Vec<&str> = rest.splitn(2, '=').collect();
                    let var_name = parts[0].trim().to_string();
                    let val_str = parts[1].trim();

                    let expr = Self::parse_expr(val_str);
                    statements.push(Statement::VarDecl {
                        var_type: var_type.to_string(),
                        name: var_name,
                        value: expr,
                        namespace: ns,
                    });
                }
                else if line.contains(".Print(") || line.starts_with("Print(") {
                    if line.starts_with("Console.Print(") {
                        let inner = line.strip_prefix("Console.Print(").unwrap().strip_suffix(")").unwrap();
                        statements.push(Statement::PrintCall {
                            namespace: Some("Console".to_string()),
                            expr: Self::parse_expr(inner),
                        });
                    } else if line.starts_with("Print(") {
                        let inner = line.strip_prefix("Print(").unwrap().strip_suffix(")").unwrap();
                        statements.push(Statement::PrintCall {
                            namespace: ns,
                            expr: Self::parse_expr(inner),
                        });
                    }
                }
            }
        }

        statements
    }

    // Enhanced expression parser handling strings, ints, floats, variables, and math
    fn parse_expr(s: &str) -> Expr {
        let s = s.trim();
        if s.contains('+') {
            let parts: Vec<&str> = s.split('+').collect();
            let left = Self::parse_expr(parts[0]);
            let right = Self::parse_expr(parts[1..].join("+").as_str());
            Expr::Add(Box::new(left), Box::new(right))
        } else if s.starts_with('"') && s.ends_with('"') {
            let inner = &s[1..s.len() - 1];
            Expr::StringLit(inner.to_string())
        } else if let Ok(n) = s.parse::<i64>() {
            // Check if it's a valid integer literal
            Expr::IntLit(n)
        } else if let Ok(f) = s.parse::<f64>() {
            // Check if it's a valid floating-point literal
            Expr::FloatLit(f)
        } else {
            // Otherwise, it's a variable name lookup
            Expr::Variable(s.to_string())
        }
    }
}
use crate::ast::{Statement, Expr, Operator};
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

        // Addition & Subtraction Operator
        if let Some(idx) = Self::find_outer_operator(s, &['+', '-']) {
            let (left_str, right_str) = s.split_at(idx);
            let op_char = right_str.chars().next().unwrap();
            let right_str = &right_str[1..]; // Skip the operator character

            let op = if op_char == '+' { Operator::Add } else { Operator::Sub };
            return Expr::Binary {
                op,
                left: Box::new(Self::parse_expr(left_str)),
                right: Box::new(Self::parse_expr(right_str)),
            };
        }

        // Multiplication & Division Operator
        if let Some(idx) = Self::find_outer_operator(s, &['*', '/']) {
            let (left_str, right_str) = s.split_at(idx);
            let op_char = right_str.chars().next().unwrap();
            let right_str = &right_str[1..]; // Skip the operator character

            let op = if op_char == '*' { Operator::Mul } else { Operator::Div };
            return Expr::Binary {
                op,
                left: Box::new(Self::parse_expr(left_str)),
                right: Box::new(Self::parse_expr(right_str)),
            };
        }

        if s.contains('+') {
            let parts: Vec<&str> = s.split('+').collect();
            let left = Self::parse_expr(parts[0]);
            let right = Self::parse_expr(parts[1..].join("+").as_str());
            Expr::Binary {
                op: Operator::Add,
                left: Box::new(left),
                right: Box::new(right),
            }
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

    // Helper that searches for operators while ignoring text inside string quotes
    fn find_outer_operator(s: &str, target_ops: &[char]) -> Option<usize> {
        let mut in_quotes = false;
        let bytes = s.as_bytes();

        for (i, &b) in bytes.iter().enumerate().rev() {
            let c = b as char;
            if c == '"' {
                in_quotes = !in_quotes;
            }
            if !in_quotes && target_ops.contains(&c) {
                return Some(i);
            }
        }
        None
    }
}
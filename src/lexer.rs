use crate::ast::RequiredInclude;

#[derive(Debug, Clone)]
pub struct Token {
    pub raw: String,
    pub char_pos: usize,
    pub namespace: Option<String>, // Required field that can be null (None)
}

// Holds both tokens and the complete list of include enums
pub struct LexerResult {
    pub tokens: Vec<Token>,
    pub required_includes: Vec<RequiredInclude>,
}

pub struct Lexer;

impl Lexer {
    pub fn tokenize(source: &str, filename: &str) -> LexerResult {
        let mut tokens = Vec::new();
        let mut required_includes = Vec::new();
        let mut current_char_pos = 0;
        let mut active_namespace: Option<String> = None;

        // Default standard library dependencies every script needs
        required_includes.push(RequiredInclude::Iostream);
        required_includes.push(RequiredInclude::String);

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                current_char_pos += line.len() + 1;
                continue;
            }

            let leading_whitespace_len = line.find(trimmed).unwrap_or(0);
            let abs_char_pos = current_char_pos + leading_whitespace_len;
            let mut clean_line = trimmed.to_string();

            if clean_line.ends_with(';') {
                clean_line.pop();
                clean_line = clean_line.trim().to_string();
            } else {
                let missing_pos = abs_char_pos + clean_line.len();
                println!(
                    "WARNING: File: {} has missed a semicolon! At character: {}",
                    filename, missing_pos
                );
            }

            // Detect namespace scopes or imports during lexical analysis
            if clean_line.starts_with("using namespace ") {
                let ns = clean_line.strip_prefix("using namespace ").unwrap().trim().to_string();
                active_namespace = Some(ns.clone());
            } else if clean_line.starts_with("import ") {
                let module = clean_line.strip_prefix("import ").unwrap().trim().to_string();
                if module == "XiHash" && !required_includes.contains(&RequiredInclude::XiHash) {
                    required_includes.push(RequiredInclude::XiHash);
                } else if !required_includes.contains(&RequiredInclude::Custom(module.clone())) {
                    required_includes.push(RequiredInclude::Custom(module));
                }
            }

            tokens.push(Token {
                raw: clean_line,
                char_pos: abs_char_pos,
                namespace: active_namespace.clone(), // Assigned namespace (can be null/None)
            });

            current_char_pos += line.len() + 1;
        }

        LexerResult {
            tokens,
            required_includes,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum RequiredInclude {
    Iostream,
    String,
    XiHash,
    Custom(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

// Extended Expression AST to handle numbers and general math
#[derive(Debug, Clone)]
pub enum Expr {
    StringLit(String),
    IntLit(i64),
    FloatLit(f64),
    Variable(String),
    Binary {
        op: Operator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Import {
        module: String,
        namespace: Option<String>,
    },
    UsingNamespace {
        ns: String,
        namespace: Option<String>,
    },
    VarDecl {
        var_type: String,
        name: String,
        value: Expr,
        namespace: Option<String>,
    },
    PrintCall {
        namespace: Option<String>,
        expr: Expr,
    },
}

// Helper function to map your custom types to native C++ types
pub fn map_type(custom_type: &str) -> String {
    match custom_type {
        "string" => "std::string".to_string(),
        "int" => "int".to_string(),
        "uint" => "unsigned int".to_string(),
        "float" => "float".to_string(),
        "double" => "double".to_string(),
        "ulong" => "unsigned long".to_string(),
        "ufloat" => "float".to_string(),   // Maps to standard C++ float
        "udouble" => "double".to_string(), // Maps to standard C++ double
        other => other.to_string(),
    }
}

impl Expr {
    pub fn to_cpp(&self) -> String {
        match self {
            Expr::StringLit(s) => format!("\"{}\"", s),
            Expr::IntLit(n) => n.to_string(),
            Expr::FloatLit(f) => format!("{}f", f), // Appends 'f' for C++ float literals
            Expr::Variable(name) => name.clone(),
            Expr::Binary { op, left, right } => {
                let op_str = match op {
                    Operator::Add => "+",
                    Operator::Sub => "-",
                    Operator::Mul => "*",
                    Operator::Div => "/",
                };
                format!("({} {} {})", left.to_cpp(), op_str, right.to_cpp())
            }
        }
    }
}

impl Statement {
    pub fn to_cpp(&self) -> String {
        match self {
            Statement::Import { .. } => String::new(),
            Statement::UsingNamespace { ns, .. } => {
                format!("using namespace {};", ns)
            }
            Statement::VarDecl { var_type, name, value, .. } => {
                let cpp_type = map_type(var_type);
                format!("{} {} = {};", cpp_type, name, value.to_cpp())
            }
            Statement::PrintCall { namespace, expr } => {
                match namespace {
                    Some(ns) => format!("{}::Print({});", ns, expr.to_cpp()),
                    None => format!("Print({});", expr.to_cpp()),
                }
            }
        }
    }
}
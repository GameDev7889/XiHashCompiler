// An enum that completely tells the compiler what #includes it needs
#[derive(Debug, Clone, PartialEq)]
pub enum RequiredInclude {
    Iostream,
    String,
    XiHash,
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
    StringLit(String),
    Variable(String),
    Concat(Box<Expr>, Box<Expr>),
}

// Every statement struct now requires a namespace field (Option<String> can be null/None)
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

impl Expr {
    pub fn to_cpp(&self) -> String {
        match self {
            Expr::StringLit(s) => format!("\"{}\"", s),
            Expr::Variable(name) => name.clone(),
            Expr::Concat(left, right) => {
                format!("{} + {}", left.to_cpp(), right.to_cpp())
            }
        }
    }
}

impl Statement {
    pub fn to_cpp(&self) -> String {
        match self {
            Statement::Import { .. } => String::new(), // Handled globally by the compiler's include array
            Statement::UsingNamespace { ns, .. } => {
                format!("using namespace {};", ns)
            }
            Statement::VarDecl { var_type, name, value, .. } => {
                let cpp_type = if var_type == "string" { "std::string" } else { var_type };
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
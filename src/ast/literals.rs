pub enum Literal {
    String(String),
    Integer(i128),
    //TODO:Float(f64),
    //TODO:Bool(bool),
    //TODO:Char(char),
    //TODO:Array(Vec<Literal>),
}


pub enum Expr {
    UnaryAdd {input: Box<Expr> },
    UnaryMinus {input: Box<Expr> },
    Add {lhs: Box<Expr>, rhs: Box<Expr>},
    Sub {lhs: Box<Expr>, rhs: Box<Expr>},
    Mul {lhs: Box<Expr>, rhs: Box<Expr>},
    Div {lhs: Box<Expr>, rhs: Box<Expr>},
    Mod {lhs: Box<Expr>, rhs: Box<Expr>},
    Pow {lhs: Box<Expr>, rhs: Box<Expr>},
    BitCompl {input: Box<Expr>},
    BitAnd {lhs: Box<Expr>, rhs: Box<Expr>},
    BitOr {lhs: Box<Expr>, rhs: Box<Expr>},
    BitXor {lhs: Box<Expr>, rhs: Box<Expr>},
    BitLShift {lhs: Box<Expr>, rhs: Box<Expr>},
    BitRShift {lhs: Box<Expr>, rhs: Box<Expr>},
    LogNot { lhs: Box<Expr> },
    LogAnd {lhs: Box<Expr>, rhs: Box<Expr>},
    LogOr {lhs: Box<Expr>, rhs: Box<Expr>},
    MCall {obj_name: Box<Expr>, method: String, args: Vec<Expr>},
    FCall {function: String, args:Vec<Expr>},
}

///TODO: define this type
pub struct TypeName {
    name: String,
}

pub enum Statement {
    VarDecl { var_name: String, var_type: Option<TypeName>, val: Option<Box<Expr>>},
    Assignment {var_name: String, val: Box<Expr>},
    Return {ret: Box<Expr>},
    Block {stmts: Vec<Statement>},
    //TODO: Conditionals, Loops, etc
}


pub struct FunctionDefn {
    name: String,
    args_type: Vec<TypeName>,
    ret_type : TypeName,
    body : Statement,
}
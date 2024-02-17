use std::fs;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub lua);

#[derive(Debug, Clone)]
pub enum Stat {
    Semicolon,
    Assignment {
        variables: Vec<Exp>,
        init: Vec<Exp>,
    },
    FunctionCall(FunctionCall),
    Label(String),
    Break,
    Return(Vec<Exp>),
    Goto(String),
    Block(Vec<Self>),
    While {
        cond: Exp,
        block: Vec<Self>,
    },
    Repeat {
        block: Vec<Self>,
        cond: Exp,
    },
    If {
        if_clause: IfClause,
        elseif_clauses: Vec<IfClause>,
        else_clause: Vec<Stat>,
    },
    NumericalFor {
        variable: String,
        start: Exp,
        end: Exp,
        step: Option<Exp>,
        block: Vec<Self>,
    },
    GenericFor {
        namelist: Vec<String>,
        explist: Vec<Exp>,
        block: Vec<Self>,
    },
    LocalFunction {
        name: String,
        body: Funcbody,
    },
    LocalDeclarations {
        variables: Vec<AttName>,
        init: Vec<Exp>,
    },
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub base: Exp,
    pub args: Vec<Exp>,
}

#[derive(Debug, Clone)]
pub struct IfClause {
    pub cond: Exp,
    pub block: Vec<Stat>,
}

#[derive(Debug, Clone)]
pub struct AttName {
    pub name: String,
    pub attrib: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Funcbody {
    pub params: Vec<String>,
    pub block: Vec<Stat>,
}

#[derive(Debug, Clone)]
pub enum Exp {
    // TODO: Should `nil` exist or just use `Option<Exp>`?
    Nil,
    Boolean(bool),
    Numeral(NumeralExpr),
    LiteralString(String),
    VarArg,
    FunctionDef(Funcbody),
    FunctionCall(Box<FunctionCall>),
    Name(String),
    Parens(Box<Self>),
    Binary {
        lhs: Box<Self>,
        rhs: Box<Self>,
        op: BinOp,
    },
    Unary {
        op: UnOp,
        exp: Box<Self>,
    },
    Index {
        base: Box<Self>,
        index: Box<Self>,
    },
}

#[derive(Debug, Clone)]
pub enum NumeralExpr {
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Or,
    And,
    Lt,
    Gt,
    LtOrEq,
    GtOrEq,
    NotEq,
    Eq,
    BitOr,
    BitNot,
    BitAnd,
    Shl,
    Shr,
    Concat,
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Mod,
    Exp,
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Negative,
    LogicalNot,
    Length,
    BitNot,
}

fn main() {
    let result = lua::BlockParser::new()
        .parse(&fs::read_to_string("main.lua").unwrap())
        .unwrap();
    dbg!(result);
}

use crate::variables::Value;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Iden(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Expr(Expr),
    Stmt(Stmt),
}

impl From<Expr> for Item {
    fn from(e: Expr) -> Self {
        Item::Expr(e)
    }
}

impl From<Iden> for Item {
    fn from(i: Iden) -> Self {
        Item::Expr(Expr::Identifier(i))
    }
}

impl From<Stmt> for Item {
    fn from(s: Stmt) -> Self {
        Item::Stmt(s)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Add { left: Box<Expr>, right: Box<Expr> },
    Subtract { left: Box<Expr>, right: Box<Expr> },
    Multiply { left: Box<Expr>, right: Box<Expr> },
    Divide { left: Box<Expr>, right: Box<Expr> },
    Lt { left: Box<Expr>, right: Box<Expr> },
    Gt { left: Box<Expr>, right: Box<Expr> },
    Eq { left: Box<Expr>, right: Box<Expr> },
    Neq { left: Box<Expr>, right: Box<Expr> },
    And { left: Box<Expr>, right: Box<Expr> },
    Or { left: Box<Expr>, right: Box<Expr> },
    Not(Box<Expr>),
    Literal(Value),
    Identifier(Iden),
}
impl From<Iden> for Expr {
    fn from(i: Iden) -> Self {
        Self::Identifier(i)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VariableDeclaration {
        iden: Iden,
        init: Option<Expr>,
    },
    FunctionDeclaration {
        iden: Iden,
        args: Vec<Iden>,
        body: Vec<Item>,
    },
    BfFDeclaration {
        iden: Iden,
        body: String,
    },
    If {
        cond: Expr,
        body: Vec<Item>,
    },
    FunctionCall {
        iden: Iden,
        args: Vec<Expr>,
    },
    Loop {
        body: Vec<Item>,
    },

    VarAssignment {
        iden: Iden,
        value: Expr,
    },
    Break,
    HopBack,
    Print(Expr),
    Melo(Iden),
}

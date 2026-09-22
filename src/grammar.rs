//! This is the grammar implementation of my toy programming language.
//! A grammar for the language is given below in EBNF:
//!
//! program     = { declaration } ;
//!
//! declaration = funcDecl | recordDecl ;
//!
//! funcDecl    = ident '(' [ params ] ')' ':' type block ;
//! params      = param { ',' param } ;
//! param       = ident ':' type ;
//!
//! recordDecl  = 'record' ident '{' { field } '}' ;
//! field       = ident ':' type ';' ;
//!
//! type        = 'int' | ident | '*' type ;
//!
//! block       = '{' { statement } '}' ;
//!
//! statement   = varDecl | ifStmt | returnStmt | block | exprStmt ;
//!
//! varDecl     = ident ':' type [ '=' expr ] ';' ;
//! ifStmt      = 'if' '(' expr ')' block [ 'else' ( ifStmt | block ) ] ;
//! returnStmt  = 'return' [ expr ] ';' ;
//! exprStmt    = expr ';' ;
//!
//! expr        = assignment ;
//! assignment  = equality [ '=' assignment ] ;
//! equality    = comparison { ( '==' | '!=' ) comparison } ;
//! comparison  = additive { ( '<' | '>' | '<=' | '>=' ) additive } ;
//! additive    = term { ( '+' | '-' ) term } ;
//! term        = unary { ( '*' | '/' ) unary } ;
//!
//! unary       = ( '-' | '!' | '*' | '&' ) unary | postfix ;
//! args        = expr { ',' expr } ;
//! primary     = intlit | 'null' | ident | '(' expr ')' | ident '{' fieldInits ] '}' ;
//! fieldInits  = fieldInit { ',' fieldInit } ;
//! fieldInit   = ident '=' expr ;

use crate::scanning as scan;

use scan::Span;


/*
pub struct Program {
    decls: Vec<Decl>,
}
*/

pub enum Decl {
    Func(FuncDecl),
    Rec(RecDecl),
}

pub struct FuncDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_ty: Type,
    pub body: Block,
    pub span: Span,
}

pub struct RecDecl {
    pub name: String,
    pub fields: Vec<Param>,
    pub span: Span,
}

pub struct Param {
    pub name: String,
    pub ty: Type,
}

// -- Types --

pub enum Type {
    Int,
    Named(String),
}

// -- Statements --

pub struct Block {
    pub statements: Vec<Stmt>,
    pub span: Span,
}

pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

pub struct ReturnStmt {
    pub returnexpr: Expr,
}

pub enum StmtKind {
    Var(VarDecl),
    If(IfStmt),
    Return(Option<Expr>),
    Block(Block),
    Expr(Expr),
}

pub struct VarDecl {
    pub name: String,
    pub ret_ty: Type,
    pub init: Option<Expr>,
}

pub struct IfStmt {
    pub ifexpr: Expr,
    pub ifblock: Block,
    pub elseexpr: Option<ElseStmt>,
}

pub enum ElseStmt {
    ElseIf(Box<IfStmt>),
    Else(Block),
}

// -- Expressions --

pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

pub enum ExprKind {
    Int(i64),
    Null,
    Var(String),
    Unary {
        op: UnOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Assign {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Field {
        base: Box<Expr>,
        name: String,
    },
    RecordLit {
        name: String,
        fields: Vec<FieldInit>,
    },
}

pub struct FieldInit {
    pub name: String,
    pub value: Expr,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnOp {
    Neg,
    Not,
    Deref,
    AddrOf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use crate::ast::lexer::Terminals;

pub mod lexer;
pub mod parser;
pub mod printer;
mod production;

#[derive(Debug, PartialEq, Clone)]
pub enum NonTerminals {
    Program,
    DeclarationList,
    DeclarationList1,
    Declaration,
    DecStmt,
    Type,
    VarDecStmt,
    FunDecStmt,
    Params,
    ParamList,
    ParamList1,
    Param,
    CompoundStmt,
    LocalDeclarations,
    StatementList,
    StatementList1,
    Statement,
    IfStmt,
    ElseStmtN,
    ElseStmt,
    WhileStmt,
    ForStmt,
    ForExpression,
    ExpressionAssign,
    DoStmt,
    BreakStmt,
    ContinueStmt,
    SwitchStmt,
    CaseDefaultStmt,
    CaseStmtN,
    CaseStmtN1,
    CaseStmt,
    DefaultStmt,
    ReturnStmt,
    Var,
    Var1,
    Expression,
    Expression1,
    AddOp,
    MulOp,
    RelationalOp,
    AdditiveExpression,
    AdditiveExpression1,
    Term,
    Term1,
    Factor,
    VarOrCall,
    VarOrCallRemain,
    Args,
    ArgList,
    ArgList1,
    Value,
    VarStmt,
    CallStmt,
    SelfOp,
    VarOrCallStmtRemain,
    VarOrCallStmt,
    SimpleExpression,
    SimpleExpression1,
    LogicalOp,
}

impl NonTerminals {
    pub fn to_num(&self) -> usize {
        match self {
            NonTerminals::Program => 0,
            NonTerminals::DeclarationList => 1,
            NonTerminals::DeclarationList1 => 2,
            NonTerminals::Declaration => 3,
            NonTerminals::DecStmt => 4,
            NonTerminals::Type => 5,
            NonTerminals::VarDecStmt => 6,
            NonTerminals::FunDecStmt => 7,
            NonTerminals::Params => 8,
            NonTerminals::ParamList => 9,
            NonTerminals::ParamList1 => 10,
            NonTerminals::Param => 11,
            NonTerminals::CompoundStmt => 12,
            NonTerminals::LocalDeclarations => 13,
            NonTerminals::StatementList => 14,
            NonTerminals::StatementList1 => 15,
            NonTerminals::Statement => 16,
            NonTerminals::IfStmt => 17,
            NonTerminals::ElseStmtN => 18,
            NonTerminals::ElseStmt => 19,
            NonTerminals::WhileStmt => 20,
            NonTerminals::ForStmt => 21,
            NonTerminals::ForExpression => 22,
            NonTerminals::ExpressionAssign => 23,
            NonTerminals::DoStmt => 24,
            NonTerminals::BreakStmt => 25,
            NonTerminals::ContinueStmt => 26,
            NonTerminals::SwitchStmt => 27,
            NonTerminals::CaseDefaultStmt => 28,
            NonTerminals::CaseStmtN => 29,
            NonTerminals::CaseStmtN1 => 30,
            NonTerminals::CaseStmt => 31,
            NonTerminals::DefaultStmt => 32,
            NonTerminals::ReturnStmt => 33,
            NonTerminals::Var => 34,
            NonTerminals::Var1 => 35,
            NonTerminals::Expression => 36,
            NonTerminals::Expression1 => 37,
            NonTerminals::AddOp => 38,
            NonTerminals::MulOp => 39,
            NonTerminals::RelationalOp => 40,
            NonTerminals::AdditiveExpression => 41,
            NonTerminals::AdditiveExpression1 => 42,
            NonTerminals::Term => 43,
            NonTerminals::Term1 => 44,
            NonTerminals::Factor => 45,
            NonTerminals::VarOrCall => 46,
            NonTerminals::VarOrCallRemain => 47,
            NonTerminals::Args => 48,
            NonTerminals::ArgList => 49,
            NonTerminals::ArgList1 => 50,
            NonTerminals::Value => 51,
            NonTerminals::VarStmt => 52,
            NonTerminals::CallStmt => 53,
            NonTerminals::SelfOp => 54,
            NonTerminals::VarOrCallStmtRemain => 55,
            NonTerminals::VarOrCallStmt => 56,
            NonTerminals::SimpleExpression => 57,
            NonTerminals::SimpleExpression1 => 58,
            NonTerminals::LogicalOp => 59,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            NonTerminals::Program => "program".to_string(),
            NonTerminals::DeclarationList => "declaration_list".to_string(),
            NonTerminals::DeclarationList1 => "declaration_list'".to_string(),
            NonTerminals::Declaration => "declaration".to_string(),
            NonTerminals::DecStmt => "dec_stmt".to_string(),
            NonTerminals::Type => "type".to_string(),
            NonTerminals::VarDecStmt => "var_dec_stmt".to_string(),
            NonTerminals::FunDecStmt => "fun_dec_stmt".to_string(),
            NonTerminals::Params => "params".to_string(),
            NonTerminals::ParamList => "param_list".to_string(),
            NonTerminals::ParamList1 => "param_list'".to_string(),
            NonTerminals::Param => "param".to_string(),
            NonTerminals::CompoundStmt => "compound_stmt".to_string(),
            NonTerminals::LocalDeclarations => "local_declarations".to_string(),
            NonTerminals::StatementList => "statement_list".to_string(),
            NonTerminals::StatementList1 => "statement_list'".to_string(),
            NonTerminals::Statement => "statement".to_string(),
            NonTerminals::IfStmt => "if_stmt".to_string(),
            NonTerminals::ElseStmtN => "else_stmt_n".to_string(),
            NonTerminals::ElseStmt => "else_stmt".to_string(),
            NonTerminals::WhileStmt => "while_stmt".to_string(),
            NonTerminals::ForStmt => "for_stmt".to_string(),
            NonTerminals::ForExpression => "for_expression".to_string(),
            NonTerminals::ExpressionAssign => "expression_assign".to_string(),
            NonTerminals::DoStmt => "do_stmt".to_string(),
            NonTerminals::BreakStmt => "break_stmt".to_string(),
            NonTerminals::ContinueStmt => "continue_stmt".to_string(),
            NonTerminals::SwitchStmt => "switch_stmt".to_string(),
            NonTerminals::CaseDefaultStmt => "case_default_stmt".to_string(),
            NonTerminals::CaseStmtN => "case_stmt_n".to_string(),
            NonTerminals::CaseStmtN1 => "case_stmt_n'".to_string(),
            NonTerminals::CaseStmt => "case_stmt".to_string(),
            NonTerminals::DefaultStmt => "default_stmt".to_string(),
            NonTerminals::ReturnStmt => "return_stmt".to_string(),
            NonTerminals::Var => "var".to_string(),
            NonTerminals::Var1 => "var'".to_string(),
            NonTerminals::Expression => "expression".to_string(),
            NonTerminals::Expression1 => "expression'".to_string(),
            NonTerminals::AddOp => "add_op".to_string(),
            NonTerminals::MulOp => "mul_op".to_string(),
            NonTerminals::RelationalOp => "relational_op".to_string(),
            NonTerminals::AdditiveExpression => "additive_expression".to_string(),
            NonTerminals::AdditiveExpression1 => "additive_expression'".to_string(),
            NonTerminals::Term => "term".to_string(),
            NonTerminals::Term1 => "term'".to_string(),
            NonTerminals::Factor => "factor".to_string(),
            NonTerminals::VarOrCall => "var_or_call".to_string(),
            NonTerminals::VarOrCallRemain => "var_or_call_remain".to_string(),
            NonTerminals::Args => "args".to_string(),
            NonTerminals::ArgList => "arg_list".to_string(),
            NonTerminals::ArgList1 => "arg_list'".to_string(),
            NonTerminals::Value => "value".to_string(),
            NonTerminals::VarStmt => "var_stmt".to_string(),
            NonTerminals::CallStmt => "call_stmt".to_string(),
            NonTerminals::SelfOp => "self_op".to_string(),
            NonTerminals::VarOrCallStmtRemain => "var_or_call_stmt_remain".to_string(),
            NonTerminals::VarOrCallStmt => "var_or_call_stmt".to_string(),
            NonTerminals::SimpleExpression => "simple_expression".to_string(),
            NonTerminals::SimpleExpression1 => "simple_expression'".to_string(),
            NonTerminals::LogicalOp => "logical_op".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    StmtK,
    ExpK,
    ParamK,
}
#[derive(Debug, PartialEq, Clone)]
pub enum StmtKind {
    IfK,
    WhileK,
    ForK,
    AssignK,
    ReturnK,
    CompoundK,
    BreakK,
    ContinueK,
    DoK,
    SwitchK,
    CaseK,
    DefaultK,
    VarDecK,
    FunDecK,
}
#[derive(Debug, PartialEq, Clone)]
pub enum ExpKind {
    OpK,
    ConstK,
    IdK,
    CallK,
    ArrayK{
        size: usize,
        dimensions: Vec<usize>,
    },
}
#[derive(Debug, PartialEq, Clone)]
pub enum ParamKind {
    Param,
    ParamListK,
}
#[derive(Debug, PartialEq, Clone)]
pub enum ExpType {
    Void,
    Integer,
    Boolean,
    Char,
    Float,
    String,
}

impl ExpType {
    pub fn new(exp_type: ExpType) -> Self {
        match exp_type {
            ExpType::Void => ExpType::Void,
            ExpType::Integer => ExpType::Integer,
            ExpType::Boolean => ExpType::Boolean,
            ExpType::Char => ExpType::Char,
            ExpType::Float => ExpType::Float,
            ExpType::String => ExpType::String,
        }
    }
}

impl Display for ExpType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpType::Void => write!(f, "void"),
            ExpType::Integer => write!(f, "int"),
            ExpType::Boolean => write!(f, "bool"),
            ExpType::Char => write!(f, "char"),
            ExpType::Float => write!(f, "float"),
            ExpType::String => write!(f, "string"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Kind {
    pub stmt: StmtKind,
    pub exp: ExpKind,
    pub param: ParamKind,
}

impl Kind {
    pub fn new() -> Self {
        Self {
            stmt: StmtKind::IfK,
            exp: ExpKind::OpK,
            param: ParamKind::Param,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TreeNode {
    pub child: Vec<Rc<RefCell<TreeNode>>>,
    pub sibling: Option<Rc<RefCell<TreeNode>>>,
    pub is_terminal: bool,
    pub terminal: Terminals,
    pub non_terminal: NonTerminals,
    pub value: String,
    pub kind: Kind,
    pub line: usize,
}

impl TreeNode {
pub fn new() -> Self {
        Self {
            child: Vec::new(),
            sibling: None,
            is_terminal: false,
            terminal: Terminals::Eof,
            non_terminal: NonTerminals::Program,
            value: String::new(),
            kind: Kind::new(),
            line: 0,
        }
    }

    pub fn new_non_terminal(non_terminal: NonTerminals) -> Self {
        Self {
            child: Vec::new(),
            sibling: None,
            is_terminal: false,
            terminal: Terminals::Eof,
            non_terminal,
            value: String::new(),
            kind: Kind::new(),
            line: 0,
        }
    }

    pub fn new_value(terminal: Terminals, value: String) -> Self {
        Self {
            child: Vec::new(),
            sibling: None,
            is_terminal: true,
            terminal,
            non_terminal: NonTerminals::Program,
            value,
            kind: Kind::new(),
            line: 0,
        }
    }
}

pub struct Global {
    pub predict_table: Vec<Vec<i32>>,
}

impl Global {
    pub fn new() -> Self {
        Self {
            predict_table: vec![vec![0; 51]; 62],
        }
    }

    pub fn create_predict_table(&mut self) {
        self.predict_table[NonTerminals::SwitchStmt.to_num()][Terminals::Switch.to_num()] = 43;
        self.predict_table[NonTerminals::DeclarationList1.to_num()][Terminals::Eof.to_num()] = 0;
        self.predict_table[NonTerminals::DeclarationList1.to_num()][Terminals::Int.to_num()] = 2;
        self.predict_table[NonTerminals::DeclarationList1.to_num()][Terminals::Float.to_num()] = 2;
        self.predict_table[NonTerminals::DeclarationList1.to_num()][Terminals::Char.to_num()] = 2;
        self.predict_table[NonTerminals::Param.to_num()][Terminals::Int.to_num()] = 17;
        self.predict_table[NonTerminals::Param.to_num()][Terminals::Float.to_num()] = 17;
        self.predict_table[NonTerminals::Param.to_num()][Terminals::Char.to_num()] = 17;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::Identifier("".to_string()).to_num()] = 21;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::If.to_num()] = 21;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::While.to_num()] = 21;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::Default.to_num()] = 0;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::CloseBrace.to_num()] = 0;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::Do.to_num()] = 21;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::Switch.to_num()] = 21;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::Break.to_num()] = 21;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::Return.to_num()] = 21;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::For.to_num()] = 21;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::Char.to_num()] = 21;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::Case.to_num()] = 0;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::Int.to_num()] = 21;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::Float.to_num()] = 21;
        self.predict_table[NonTerminals::StatementList1.to_num()][Terminals::Continue.to_num()] = 21;
        self.predict_table[NonTerminals::VarStmt.to_num()][Terminals::MinusMinus.to_num()] = 94;
        self.predict_table[NonTerminals::VarStmt.to_num()][Terminals::PlusPlus.to_num()] = 94;
        self.predict_table[NonTerminals::VarStmt.to_num()][Terminals::Equals.to_num()] = 95;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::SemiColon.to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::Identifier("".to_string()).to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::Int.to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::Float.to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::Char.to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::While.to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::If.to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::Do.to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::Switch.to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::Break.to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::Return.to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::For.to_num()] = 96;
        self.predict_table[NonTerminals::ForExpression.to_num()][Terminals::Continue.to_num()] = 96;
        self.predict_table[NonTerminals::CaseStmtN.to_num()][Terminals::Case.to_num()] = 45;
        self.predict_table[NonTerminals::BreakStmt.to_num()][Terminals::Break.to_num()] = 41;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::BangEquals.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::Plus.to_num()] = 66;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::Minus.to_num()] = 66;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::GreaterThan.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::None.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::RightParen.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::EqualsEquals.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::Not.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::And.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::Or.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::LessThan.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::LessThanEquals.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::GreaterThanEquals.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::SemiColon.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::Colon.to_num()] = 0;
        self.predict_table[NonTerminals::SimpleExpression1.to_num()][Terminals::Comma.to_num()] = 0;
        self.predict_table[NonTerminals::Factor.to_num()][Terminals::True.to_num()] = 75;
        self.predict_table[NonTerminals::Factor.to_num()][Terminals::False.to_num()] = 75;
        self.predict_table[NonTerminals::Factor.to_num()][Terminals::LeftParen.to_num()] = 73;
        self.predict_table[NonTerminals::Factor.to_num()][Terminals::Identifier("".to_string()).to_num()] = 74;
        self.predict_table[NonTerminals::Factor.to_num()][Terminals::Integer(0).to_num()] = 75;
        self.predict_table[NonTerminals::Factor.to_num()][Terminals::FloatNumber(0.0).to_num()] = 75;
        self.predict_table[NonTerminals::Factor.to_num()][Terminals::Character('a').to_num()] = 75;
        self.predict_table[NonTerminals::Factor.to_num()][Terminals::String("a".to_string()).to_num()] = 75;
        self.predict_table[NonTerminals::VarOrCallStmtRemain.to_num()][Terminals::LeftBracket.to_num()] = 93;
        self.predict_table[NonTerminals::VarOrCallStmtRemain.to_num()][Terminals::MinusMinus.to_num()] = 93;
        self.predict_table[NonTerminals::VarOrCallStmtRemain.to_num()][Terminals::PlusPlus.to_num()] = 93;
        self.predict_table[NonTerminals::VarOrCallStmtRemain.to_num()][Terminals::Equals.to_num()] = 93;
        self.predict_table[NonTerminals::VarOrCallStmtRemain.to_num()][Terminals::LeftParen.to_num()] = 92;
        self.predict_table[NonTerminals::LogicalOp.to_num()][Terminals::Or.to_num()] = 61;
        self.predict_table[NonTerminals::LogicalOp.to_num()][Terminals::And.to_num()] = 60;
        self.predict_table[NonTerminals::LogicalOp.to_num()][Terminals::Not.to_num()] = 62;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::Identifier("".to_string()).to_num()] = 37;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::SemiColon.to_num()] = 0;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::RightParen.to_num()] = 0;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::Float.to_num()] = 37;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::Int.to_num()] = 37;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::Char.to_num()] = 37;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::Break.to_num()] = 37;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::While.to_num()] = 37;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::If.to_num()] = 37;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::Do.to_num()] = 37;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::Switch.to_num()] = 37;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::Return.to_num()] = 37;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::For.to_num()] = 37;
        self.predict_table[NonTerminals::ExpressionAssign.to_num()][Terminals::Continue.to_num()] = 37;
        self.predict_table[NonTerminals::Value.to_num()][Terminals::True.to_num()] = 83;
        self.predict_table[NonTerminals::Value.to_num()][Terminals::False.to_num()] = 84;
        self.predict_table[NonTerminals::Value.to_num()][Terminals::Integer(0).to_num()] = 79;
        self.predict_table[NonTerminals::Value.to_num()][Terminals::FloatNumber(0.0).to_num()] = 80;
        self.predict_table[NonTerminals::Value.to_num()][Terminals::Character('a').to_num()] = 81;
        self.predict_table[NonTerminals::Value.to_num()][Terminals::String("a".to_string()).to_num()] = 82;
        self.predict_table[NonTerminals::RelationalOp.to_num()][Terminals::LessThan.to_num()] = 55;
        self.predict_table[NonTerminals::RelationalOp.to_num()][Terminals::LessThanEquals.to_num()] = 54;
        self.predict_table[NonTerminals::RelationalOp.to_num()][Terminals::GreaterThan.to_num()] = 57;
        self.predict_table[NonTerminals::RelationalOp.to_num()][Terminals::GreaterThanEquals.to_num()] = 56;
        self.predict_table[NonTerminals::RelationalOp.to_num()][Terminals::EqualsEquals.to_num()] = 58;
        self.predict_table[NonTerminals::RelationalOp.to_num()][Terminals::BangEquals.to_num()] = 59;
        self.predict_table[NonTerminals::CaseDefaultStmt.to_num()][Terminals::Case.to_num()] = 44;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::Break.to_num()] = 28;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::For.to_num()] = 23;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::Do.to_num()] = 25;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::Identifier("".to_string()).to_num()] = 22;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::If.to_num()] = 26;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::Continue.to_num()] = 30;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::Float.to_num()] = 29;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::Int.to_num()] = 29;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::Char.to_num()] = 29;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::Return.to_num()] = 31;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::Switch.to_num()] = 27;
        self.predict_table[NonTerminals::Statement.to_num()][Terminals::While.to_num()] = 24;
        self.predict_table[NonTerminals::DecStmt.to_num()][Terminals::SemiColon.to_num()] = 5;
        self.predict_table[NonTerminals::DecStmt.to_num()][Terminals::LeftParen.to_num()] = 6;
        self.predict_table[NonTerminals::DecStmt.to_num()][Terminals::LeftBracket.to_num()] = 5;
        self.predict_table[NonTerminals::DecStmt.to_num()][Terminals::Equals.to_num()] = 5;
        self.predict_table[NonTerminals::Program.to_num()][Terminals::Int.to_num()] = 1;
        self.predict_table[NonTerminals::Program.to_num()][Terminals::Float.to_num()] = 1;
        self.predict_table[NonTerminals::Program.to_num()][Terminals::Char.to_num()] = 1;
        self.predict_table[NonTerminals::ParamList.to_num()][Terminals::Char.to_num()] = 15;
        self.predict_table[NonTerminals::ParamList.to_num()][Terminals::Int.to_num()] = 15;
        self.predict_table[NonTerminals::ParamList.to_num()][Terminals::Float.to_num()] = 15;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::Comma.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::EqualsEquals.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::Not.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::RightParen.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::GreaterThan.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::Colon.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::LessThanEquals.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::BangEquals.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::None.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::GreaterThanEquals.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::And.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::Or.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::SemiColon.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::LessThan.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::Plus.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::None.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::Asterisk.to_num()] = 70;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::Minus.to_num()] = 0;
        self.predict_table[NonTerminals::Term1.to_num()][Terminals::Slash.to_num()] = 70;
        self.predict_table[NonTerminals::LocalDeclarations.to_num()][Terminals::Float.to_num()] = 19;
        self.predict_table[NonTerminals::LocalDeclarations.to_num()][Terminals::Int.to_num()] = 19;
        self.predict_table[NonTerminals::LocalDeclarations.to_num()][Terminals::Char.to_num()] = 19;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::LeftBracket.to_num()] = 51;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::MinusMinus.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::Identifier("".to_string()).to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::Equals.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::PlusPlus.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::Slash.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::Or.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::LessThanEquals.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::LessThan.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::Minus.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::Comma.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::BangEquals.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::RightParen.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::SemiColon.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::EqualsEquals.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::Plus.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::Not.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::And.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::GreaterThan.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::Colon.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::GreaterThanEquals.to_num()] = 0;
        self.predict_table[NonTerminals::Var1.to_num()][Terminals::Asterisk.to_num()] = 0;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::Default.to_num()] = 0;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::Identifier("".to_string()).to_num()] = 20;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::Char.to_num()] = 20;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::Do.to_num()] = 20;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::Continue.to_num()] = 20;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::Switch.to_num()] = 20;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::Case.to_num()] = 0;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::CloseBrace.to_num()] = 0;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::Return.to_num()] = 20;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::If.to_num()] = 20;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::Float.to_num()] = 20;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::Break.to_num()] = 20;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::While.to_num()] = 20;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::Int.to_num()] = 20;
        self.predict_table[NonTerminals::StatementList.to_num()][Terminals::For.to_num()] = 20;
        self.predict_table[NonTerminals::WhileStmt.to_num()][Terminals::While.to_num()] = 35;
        self.predict_table[NonTerminals::CallStmt.to_num()][Terminals::Identifier("".to_string()).to_num()] = 88;
        self.predict_table[NonTerminals::Var.to_num()][Terminals::Identifier("".to_string()).to_num()] = 50;
        self.predict_table[NonTerminals::FunDecStmt.to_num()][Terminals::LeftParen.to_num()] = 12;
        self.predict_table[NonTerminals::VarOrCall.to_num()][Terminals::Identifier("".to_string()).to_num()] = 76;
        self.predict_table[NonTerminals::ReturnStmt.to_num()][Terminals::Return.to_num()] = 49;
        self.predict_table[NonTerminals::ContinueStmt.to_num()][Terminals::Continue.to_num()] = 42;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::GreaterThanEquals.to_num()] = 64;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::RightParen.to_num()] = 0;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::And.to_num()] = 0;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::GreaterThan.to_num()] = 64;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::LessThanEquals.to_num()] = 64;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::Or.to_num()] = 0;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::BangEquals.to_num()] = 64;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::SemiColon.to_num()] = 0;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::EqualsEquals.to_num()] = 64;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::Not.to_num()] = 0;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::LessThan.to_num()] = 64;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::Comma.to_num()] = 0;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::Colon.to_num()] = 0;
        self.predict_table[NonTerminals::AdditiveExpression1.to_num()][Terminals::None.to_num()] = 0;
        self.predict_table[NonTerminals::CaseStmtN1.to_num()][Terminals::Default.to_num()] = 0;
        self.predict_table[NonTerminals::CaseStmtN1.to_num()][Terminals::Case.to_num()] = 46;
        self.predict_table[NonTerminals::CaseStmtN1.to_num()][Terminals::CloseBrace.to_num()] = 0;
        self.predict_table[NonTerminals::DeclarationList.to_num()][Terminals::Char.to_num()] = 2;
        self.predict_table[NonTerminals::DeclarationList.to_num()][Terminals::Int.to_num()] = 2;
        self.predict_table[NonTerminals::DeclarationList.to_num()][Terminals::Float.to_num()] = 2;
        self.predict_table[NonTerminals::AddOp.to_num()][Terminals::Minus.to_num()] = 68;
        self.predict_table[NonTerminals::AddOp.to_num()][Terminals::Plus.to_num()] = 67;
        self.predict_table[NonTerminals::ArgList.to_num()][Terminals::String("".to_string()).to_num()] = 86;
        self.predict_table[NonTerminals::ArgList.to_num()][Terminals::Character('a').to_num()] = 86;
        self.predict_table[NonTerminals::ArgList.to_num()][Terminals::FloatNumber(0.0).to_num()] = 86;
        self.predict_table[NonTerminals::ArgList.to_num()][Terminals::Integer(0).to_num()] = 86;
        self.predict_table[NonTerminals::ArgList.to_num()][Terminals::False.to_num()] = 86;
        self.predict_table[NonTerminals::ArgList.to_num()][Terminals::True.to_num()] = 86;
        self.predict_table[NonTerminals::ArgList.to_num()][Terminals::Identifier("".to_string()).to_num()] = 86;
        self.predict_table[NonTerminals::ArgList.to_num()][Terminals::LeftParen.to_num()] = 86;
        self.predict_table[NonTerminals::ElseStmt.to_num()][Terminals::Else.to_num()] = 34;
        self.predict_table[NonTerminals::Expression.to_num()][Terminals::Integer(0).to_num()] = 52;
        self.predict_table[NonTerminals::Expression.to_num()][Terminals::FloatNumber(0.0).to_num()] = 52;
        self.predict_table[NonTerminals::Expression.to_num()][Terminals::Character('a').to_num()] = 52;
        self.predict_table[NonTerminals::Expression.to_num()][Terminals::String("a".to_string()).to_num()] = 52;
        self.predict_table[NonTerminals::Expression.to_num()][Terminals::False.to_num()] = 52;
        self.predict_table[NonTerminals::Expression.to_num()][Terminals::True.to_num()] = 52;
        self.predict_table[NonTerminals::Expression.to_num()][Terminals::LeftParen.to_num()] = 52;
        self.predict_table[NonTerminals::Expression.to_num()][Terminals::Identifier("".to_string()).to_num()] = 52;
        self.predict_table[NonTerminals::DefaultStmt.to_num()][Terminals::Default.to_num()] = 48;
        self.predict_table[NonTerminals::DefaultStmt.to_num()][Terminals::CloseBrace.to_num()] = 0;
        self.predict_table[NonTerminals::VarDecStmt.to_num()][Terminals::SemiColon.to_num()] = 97;
        self.predict_table[NonTerminals::VarDecStmt.to_num()][Terminals::Equals.to_num()] = 10;
        self.predict_table[NonTerminals::CaseStmt.to_num()][Terminals::Case.to_num()] = 47;
        self.predict_table[NonTerminals::Params.to_num()][Terminals::Void.to_num()] = 14;
        self.predict_table[NonTerminals::Params.to_num()][Terminals::Char.to_num()] = 13;
        self.predict_table[NonTerminals::Params.to_num()][Terminals::Int.to_num()] = 13;
        self.predict_table[NonTerminals::Params.to_num()][Terminals::Float.to_num()] = 13;
        self.predict_table[NonTerminals::AdditiveExpression.to_num()][Terminals::String("".to_string()).to_num()] = 63;
        self.predict_table[NonTerminals::AdditiveExpression.to_num()][Terminals::Character('a').to_num()] = 63;
        self.predict_table[NonTerminals::AdditiveExpression.to_num()][Terminals::FloatNumber(0.0).to_num()] = 63;
        self.predict_table[NonTerminals::AdditiveExpression.to_num()][Terminals::Integer(0).to_num()] = 63;
        self.predict_table[NonTerminals::AdditiveExpression.to_num()][Terminals::False.to_num()] = 63;
        self.predict_table[NonTerminals::AdditiveExpression.to_num()][Terminals::True.to_num()] = 63;
        self.predict_table[NonTerminals::AdditiveExpression.to_num()][Terminals::Identifier("".to_string()).to_num()] = 63;
        self.predict_table[NonTerminals::AdditiveExpression.to_num()][Terminals::LeftParen.to_num()] = 63;
        self.predict_table[NonTerminals::VarOrCallStmt.to_num()][Terminals::Identifier("".to_string()).to_num()] = 91;
        self.predict_table[NonTerminals::ParamList1.to_num()][Terminals::Comma.to_num()] = 16;
        self.predict_table[NonTerminals::ParamList1.to_num()][Terminals::RightParen.to_num()] = 0;
        self.predict_table[NonTerminals::Declaration.to_num()][Terminals::Char.to_num()] = 4;
        self.predict_table[NonTerminals::Declaration.to_num()][Terminals::Int.to_num()] = 4;
        self.predict_table[NonTerminals::Declaration.to_num()][Terminals::Float.to_num()] = 4;
        self.predict_table[NonTerminals::ForStmt.to_num()][Terminals::For.to_num()] = 36;
        self.predict_table[NonTerminals::DoStmt.to_num()][Terminals::Do.to_num()] = 40;
        self.predict_table[NonTerminals::Term.to_num()][Terminals::False.to_num()] = 69;
        self.predict_table[NonTerminals::Term.to_num()][Terminals::True.to_num()] = 69;
        self.predict_table[NonTerminals::Term.to_num()][Terminals::Identifier("".to_string()).to_num()] = 69;
        self.predict_table[NonTerminals::Term.to_num()][Terminals::LeftParen.to_num()] = 69;
        self.predict_table[NonTerminals::Term.to_num()][Terminals::Integer(0).to_num()] = 69;
        self.predict_table[NonTerminals::Term.to_num()][Terminals::FloatNumber(0.0).to_num()] = 69;
        self.predict_table[NonTerminals::Term.to_num()][Terminals::Character('a').to_num()] = 69;
        self.predict_table[NonTerminals::Term.to_num()][Terminals::String("a".to_string()).to_num()] = 69;
        self.predict_table[NonTerminals::Expression1.to_num()][Terminals::RightParen.to_num()] = 0;
        self.predict_table[NonTerminals::Expression1.to_num()][Terminals::SemiColon.to_num()] = 0;
        self.predict_table[NonTerminals::Expression1.to_num()][Terminals::Comma.to_num()] = 0;
        self.predict_table[NonTerminals::Expression1.to_num()][Terminals::None.to_num()] = 0;
        self.predict_table[NonTerminals::Expression1.to_num()][Terminals::Colon.to_num()] = 0;
        self.predict_table[NonTerminals::Expression1.to_num()][Terminals::Or.to_num()] = 53;
        self.predict_table[NonTerminals::Expression1.to_num()][Terminals::And.to_num()] = 53;
        self.predict_table[NonTerminals::Expression1.to_num()][Terminals::Not.to_num()] = 53;
        self.predict_table[NonTerminals::MulOp.to_num()][Terminals::Slash.to_num()] = 72;
        self.predict_table[NonTerminals::MulOp.to_num()][Terminals::Asterisk.to_num()] = 71;
        self.predict_table[NonTerminals::ArgList1.to_num()][Terminals::Comma.to_num()] = 87;
        self.predict_table[NonTerminals::ArgList1.to_num()][Terminals::RightParen.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Else.to_num()] = 33;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::While.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Break.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::For.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Identifier("".to_string()).to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Switch.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Int.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Float.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Char.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Continue.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::None.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::If.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Return.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Do.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Case.to_num()] = 0;
        self.predict_table[NonTerminals::ElseStmtN.to_num()][Terminals::Default.to_num()] = 0;
        self.predict_table[NonTerminals::IfStmt.to_num()][Terminals::If.to_num()] = 32;
        self.predict_table[NonTerminals::SelfOp.to_num()][Terminals::MinusMinus.to_num()] = 90;
        self.predict_table[NonTerminals::SelfOp.to_num()][Terminals::PlusPlus.to_num()] = 89;
        self.predict_table[NonTerminals::Type.to_num()][Terminals::Float.to_num()] = 8;
        self.predict_table[NonTerminals::Type.to_num()][Terminals::Int.to_num()] = 7;
        self.predict_table[NonTerminals::Type.to_num()][Terminals::Char.to_num()] = 9;
        self.predict_table[NonTerminals::CompoundStmt.to_num()][Terminals::OpenBrace.to_num()] = 18;
        self.predict_table[NonTerminals::SimpleExpression.to_num()][Terminals::Character('a').to_num()] = 65;
        self.predict_table[NonTerminals::SimpleExpression.to_num()][Terminals::String("a".to_string()).to_num()] = 65;
        self.predict_table[NonTerminals::SimpleExpression.to_num()][Terminals::False.to_num()] = 65;
        self.predict_table[NonTerminals::SimpleExpression.to_num()][Terminals::True.to_num()] = 65;
        self.predict_table[NonTerminals::SimpleExpression.to_num()][Terminals::Identifier("".to_string()).to_num()] = 65;
        self.predict_table[NonTerminals::SimpleExpression.to_num()][Terminals::LeftParen.to_num()] = 65;
        self.predict_table[NonTerminals::SimpleExpression.to_num()][Terminals::Integer(0).to_num()] = 65;
        self.predict_table[NonTerminals::SimpleExpression.to_num()][Terminals::FloatNumber(0.0).to_num()] = 65;
        self.predict_table[NonTerminals::Args.to_num()][Terminals::String("".to_string()).to_num()] = 85;
        self.predict_table[NonTerminals::Args.to_num()][Terminals::Character('a').to_num()] = 85;
        self.predict_table[NonTerminals::Args.to_num()][Terminals::FloatNumber(0.0).to_num()] = 85;
        self.predict_table[NonTerminals::Args.to_num()][Terminals::Integer(0).to_num()] = 85;
        self.predict_table[NonTerminals::Args.to_num()][Terminals::False.to_num()] = 85;
        self.predict_table[NonTerminals::Args.to_num()][Terminals::True.to_num()] = 85;
        self.predict_table[NonTerminals::Args.to_num()][Terminals::Identifier("".to_string()).to_num()] = 85;
        self.predict_table[NonTerminals::Args.to_num()][Terminals::LeftParen.to_num()] = 85;
        self.predict_table[NonTerminals::Args.to_num()][Terminals::RightParen.to_num()] = 0;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::LeftBracket.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::LeftParen.to_num()] = 77;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::Asterisk.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::Or.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::Slash.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::Minus.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::LessThanEquals.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::GreaterThan.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::Comma.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::GreaterThanEquals.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::BangEquals.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::Plus.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::LessThan.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::And.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::EqualsEquals.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::SemiColon.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::None.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::Colon.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::RightParen.to_num()] = 78;
        self.predict_table[NonTerminals::VarOrCallRemain.to_num()][Terminals::Not.to_num()] = 78;
    }
}

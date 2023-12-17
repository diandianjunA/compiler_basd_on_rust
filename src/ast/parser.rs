use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::{ExpKind, Global, NonTerminals, StmtKind, TreeNode};
use crate::ast::lexer::{Terminals, Token};
use crate::ast::production::Stack;

pub struct Parser {
    tokens: Vec<Token>,
    pub stack: Stack,
    global: Global,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let stack = Stack::new();
        Self {
            tokens,
            stack,
            global: Global::new(),
        }
    }

    pub fn parse(&mut self) {
        self.global.create_predict_table();
        let mut tokens = self.tokens.clone();
        tokens.reverse();
        let mut token = tokens.pop().unwrap();
        while !self.stack.stack.is_empty() {
            let element = self.stack.last().unwrap();
            if element.borrow().is_terminal {
                if element.borrow().terminal == token.kind {
                    self.stack.pop();
                    match &token.kind {
                        _ => {
                            element.borrow_mut().terminal = token.kind;
                            element.borrow_mut().line = token.line;
                        }
                    }
                } else {
                    panic!("语法错误： {} 不能匹配 {}，第 {} 行", element.borrow().terminal.to_string(), token.kind.to_string(), token.line);
                }
                match tokens.pop() {
                    None => {
                        break;
                    }
                    Some(t) => {
                        token = t;
                    }
                }
            } else {
                let production = self.global.predict_table.get(element.borrow().non_terminal.to_num()).unwrap().get(token.kind.to_num()).unwrap();
                self.stack.pop();
                element.borrow_mut().line = token.line;
                match production {
                    0 => {
                        self.stack.production0();
                    },
                    1 => {
                        self.stack.production1(element);
                    },
                    2 => {
                        self.stack.production2(element);
                    },
                    3 => {
                        self.stack.production3(element);
                    },
                    4 => {
                        self.stack.production4(element);
                    },
                    5 => {
                        self.stack.production5(element);
                    },
                    6 => {
                        self.stack.production6(element);
                    },
                    7 => {
                        self.stack.production7(element);
                    },
                    8 => {
                        self.stack.production8(element);
                    },
                    9 => {
                        self.stack.production9(element);
                    },
                    10 => {
                        self.stack.production10(element);
                    },
                    11 => {
                        self.stack.production11(element);
                    },
                    12 => {
                        self.stack.production12(element);
                    },
                    13 => {
                        self.stack.production13(element);
                    },
                    14 => {
                        self.stack.production14(element);
                    },
                    15 => {
                        self.stack.production15(element);
                    },
                    16 => {
                        self.stack.production16(element);
                    },
                    17 => {
                        self.stack.production17(element);
                    },
                    18 => {
                        self.stack.production18(element);
                    },
                    19 => {
                        self.stack.production19(element);
                    },
                    20 => {
                        self.stack.production20(element);
                    },
                    21 => {
                        self.stack.production21(element);
                    },
                    22 => {
                        self.stack.production22(element);
                    },
                    23 => {
                        self.stack.production23(element);
                    },
                    24 => {
                        self.stack.production24(element);
                    },
                    25 => {
                        self.stack.production25(element);
                    },
                    26 => {
                        self.stack.production26(element);
                    },
                    27 => {
                        self.stack.production27(element);
                    },
                    28 => {
                        self.stack.production28(element);
                    },
                    29 => {
                        self.stack.production29(element);
                    },
                    30 => {
                        self.stack.production30(element);
                    },
                    31 => {
                        self.stack.production31(element);
                    },
                    32 => {
                        self.stack.production32(element);
                    },
                    33 => {
                        self.stack.production33(element);
                    },
                    34 => {
                        self.stack.production34(element);
                    },
                    35 => {
                        self.stack.production35(element);
                    },
                    36 => {
                        self.stack.production36(element);
                    },
                    37 => {
                        self.stack.production37(element);
                    },
                    40 => {
                        self.stack.production40(element);
                    },
                    41 => {
                        self.stack.production41(element);
                    },
                    42 => {
                        self.stack.production42(element);
                    },
                    43 => {
                        self.stack.production43(element);
                    },
                    44 => {
                        self.stack.production44(element);
                    },
                    45 => {
                        self.stack.production45(element);
                    },
                    46 => {
                        self.stack.production46(element);
                    },
                    47 => {
                        self.stack.production47(element);
                    },
                    48 => {
                        self.stack.production48(element);
                    },
                    49 => {
                        self.stack.production49(element);
                    },
                    50 => {
                        self.stack.production50(element);
                    },
                    51 => {
                        self.stack.production51(element);
                    },
                    52 => {
                        self.stack.production52(element);
                    },
                    53 => {
                        self.stack.production53(element);
                    },
                    54 => {
                        self.stack.production54(element);
                    },
                    55 => {
                        self.stack.production55(element);
                    },
                    56 => {
                        self.stack.production56(element);
                    },
                    57 => {
                        self.stack.production57(element);
                    },
                    58 => {
                        self.stack.production58(element);
                    },
                    59 => {
                        self.stack.production59(element);
                    },
                    60 => {
                        self.stack.production60(element);
                    },
                    61 => {
                        self.stack.production61(element);
                    },
                    62 => {
                        self.stack.production62(element);
                    },
                    63 => {
                        self.stack.production63(element);
                    },
                    64 => {
                        self.stack.production64(element);
                    },
                    65 => {
                        self.stack.production65(element);
                    },
                    66 => {
                        self.stack.production66(element);
                    },
                    67 => {
                        self.stack.production67(element);
                    },
                    68 => {
                        self.stack.production68(element);
                    },
                    69 => {
                        self.stack.production69(element);
                    },
                    70 => {
                        self.stack.production70(element);
                    },
                    71 => {
                        self.stack.production71(element);
                    },
                    72 => {
                        self.stack.production72(element);
                    },
                    73 => {
                        self.stack.production73(element);
                    },
                    74 => {
                        self.stack.production74(element);
                    },
                    75 => {
                        self.stack.production75(element);
                    },
                    76 => {
                        self.stack.production76(element);
                    },
                    77 => {
                        self.stack.production77(element);
                    },
                    78 => {
                        self.stack.production78(element);
                    },
                    79 => {
                        self.stack.production79(element);
                    },
                    80 => {
                        self.stack.production80(element);
                    },
                    81 => {
                        self.stack.production81(element);
                    },
                    82 => {
                        self.stack.production82(element);
                    },
                    83 => {
                        self.stack.production83(element);
                    },
                    84 => {
                        self.stack.production84(element);
                    },
                    85 => {
                        self.stack.production85(element);
                    },
                    86 => {
                        self.stack.production86(element);
                    },
                    87 => {
                        self.stack.production87(element);
                    },
                    88 => {
                        self.stack.production88(element);
                    },
                    89 => {
                        self.stack.production89(element);
                    },
                    90 => {
                        self.stack.production90(element);
                    },
                    91 => {
                        self.stack.production91(element);
                    },
                    92 => {
                        self.stack.production92(element);
                    },
                    93 => {
                        self.stack.production93(element);
                    },
                    94 => {
                        self.stack.production94(element);
                    },
                    95 => {
                        self.stack.production95(element);
                    },
                    96 => {
                        self.stack.production96(element);
                    },
                    97 => {
                        self.stack.production97(element);
                    },
                    _ => {
                        panic!("Error: {} can not generate {} first", element.borrow().non_terminal.to_string(), token.kind.to_string());
                    }
                }
            }
        }
    }

    pub fn normal_parse(&mut self) {
        let node = self.stack.tree_root.clone();
        for node in node.borrow().child.iter() {
            node_normalize(node.clone());
        }
    }

    pub fn print(&self) {

    }
}

pub fn node_normalize(mut node: Rc<RefCell<TreeNode>>) {
    loop {
        if !node.borrow().is_terminal {
            let terminals = node.borrow().clone().non_terminal;
            match terminals {
                NonTerminals::DeclarationList => {
                    declaration_list_normalize(node.clone());
                }
                NonTerminals::Declaration => {
                    declaration_normalize(node.clone());
                }
                NonTerminals::Params => {
                    params_normalize(node.clone());
                }
                NonTerminals::Var => {
                    var_normalize(node.clone());
                }
                NonTerminals::LocalDeclarations => {
                    local_declarations_normalize(node.clone());
                }
                NonTerminals::StatementList => {
                    statement_list_normalize(node.clone());
                }
                NonTerminals::CaseStmtN => {
                    case_stmt_n_normalize(node.clone());
                }
                NonTerminals::Expression => {
                    expression_normalize(node.clone());
                }
                NonTerminals::AdditiveExpression => {
                    additive_expression_normalize(node.clone());
                }
                NonTerminals::Term => {
                    term_normalize(node.clone());
                }
                NonTerminals::Factor => {
                    factor_normalize(node.clone());
                }
                NonTerminals::Args => {
                    args_normalize(node.clone());
                }
                NonTerminals::VarOrCallStmt => {
                    var_or_call_stmt_normalize(node.clone());
                }
                NonTerminals::SimpleExpression => {
                    simple_expression_normalize(node.clone());
                }
                _ => {}
            }
        }
        for node in node.borrow().child.iter() {
            node_normalize(node.clone());
        }
        let option = node.borrow().sibling.clone();
        match option {
            None => {
                break;
            }
            Some(node1) => {
                node = node1;
            }
        }
    }
}

pub fn declaration_list_normalize(declaration_list: Rc<RefCell<TreeNode>>) {
    let mut declaration_list1 = declaration_list.borrow_mut().child.pop().unwrap();
    while declaration_list1.borrow().child.len() != 0 {
        let declaration_list2 = declaration_list1.borrow_mut().child.pop().unwrap();
        declaration_list.borrow_mut().child.push(declaration_list1.borrow_mut().child.pop().unwrap());
        declaration_list1 = declaration_list2;
    }
}

pub fn declaration_normalize(declaration: Rc<RefCell<TreeNode>>) {
    if declaration.borrow_mut().child[2].borrow().non_terminal == NonTerminals::DecStmt {
        if declaration.borrow().child[2].borrow().child.len() == 2 {
            let mut var1 = declaration.borrow().child[2].borrow().child[0].clone();
            let var_dec_stmt = declaration.borrow().child[2].borrow().child[1].clone();
            declaration.borrow_mut().kind.stmt = StmtKind::VarDecK;
            let var = Rc::new(RefCell::new(TreeNode::new_non_terminal(NonTerminals::Var)));
            var.borrow_mut().child.push(declaration.borrow().child[1].clone());
            declaration.borrow_mut().child.pop();
            declaration.borrow_mut().child.pop();
            if var1.borrow().child.len() != 0 {
                let mut dimensions = Vec::new();
                while var1.borrow().child.len() != 0 {
                    let t = var1.clone();
                    let t = t.borrow().child[0].borrow().clone().terminal;
                    let num;
                    match t {
                        Terminals::Integer(i) => {
                            num = i;
                        }
                        _ => {
                            panic!("Error: {} is not a integer", t.to_string());
                        }
                    }
                    dimensions.push(num as usize);
                    let var2 = var1.borrow().child[1].clone();
                    var1 = var2;
                }
                var.borrow_mut().kind.exp = ExpKind::ArrayK{size: dimensions.len(),dimensions};
                declaration.borrow_mut().child.push(var);
            } else {
                var.borrow_mut().kind.exp = ExpKind::IdK;
                declaration.borrow_mut().child.push(var);
            }
            if var_dec_stmt.borrow().child.len() == 1 {
                declaration.borrow_mut().child.push(var_dec_stmt.borrow().child[0].clone());
            }
        }else {
            let fun_dec_stmt = declaration.borrow().child[2].borrow().child[0].clone();
            declaration.borrow_mut().kind.stmt = StmtKind::FunDecK;
            declaration.borrow_mut().child.pop();
            declaration.borrow_mut().child.push(fun_dec_stmt.borrow().child[0].clone());
            declaration.borrow_mut().child.push(fun_dec_stmt.borrow().child[1].clone());
        }
    }
}

pub fn params_normalize(params: Rc<RefCell<TreeNode>>) {
    if params.borrow().child[0].borrow().non_terminal == NonTerminals::ParamList {
        let param_list = params.borrow_mut().child.pop().unwrap();
        let mut param_list1 = param_list.borrow_mut().child.pop().unwrap();
        params.borrow_mut().child.push(param_list.borrow_mut().child.pop().unwrap());
        while param_list1.borrow().child.len() != 0 {
            let param_list2 = param_list1.borrow_mut().child.pop().unwrap();
            params.borrow_mut().child.push(param_list1.borrow_mut().child.pop().unwrap());
            param_list1 = param_list2;
        }
    }
}

pub fn var_normalize(var: Rc<RefCell<TreeNode>>) {
    if var.borrow().kind.exp == ExpKind::OpK {
        let mut var1 = var.borrow_mut().child.pop().unwrap();
        if var1.borrow().child.len() != 0 {
            let mut dimensions = Vec::new();
            while var1.borrow().child.len() != 0 {
                let t = var1.clone();
                let t = t.borrow().child[0].borrow().clone().terminal;
                let num;
                match t {
                    Terminals::Integer(i) => {
                        num = i;
                    }
                    _ => {
                        panic!("Error: {} is not a integer", t.to_string());
                    }
                }
                dimensions.push(num as usize);
                let var2 = var1.borrow().child[1].clone();
                var1 = var2;
            }
            var.borrow_mut().kind.exp = ExpKind::ArrayK{size: dimensions.len(),dimensions};
        } else {
            var.borrow_mut().kind.exp = ExpKind::IdK;
        }
    }
}

pub fn local_declarations_normalize(local_declarations: Rc<RefCell<TreeNode>>) {
    let var_dec_stmt = local_declarations.borrow().child[3].clone();
    let mut var1 = local_declarations.borrow().child[2].clone();
    local_declarations.borrow_mut().kind.stmt = StmtKind::VarDecK;
    let var = Rc::new(RefCell::new(TreeNode::new_non_terminal(NonTerminals::Var)));
    var.borrow_mut().child.push(local_declarations.borrow().child[1].clone());
    local_declarations.borrow_mut().child.pop();
    local_declarations.borrow_mut().child.pop();
    local_declarations.borrow_mut().child.pop();
    if var1.borrow().child.len() != 0 {
        let mut dimensions = Vec::new();
        while var1.borrow().child.len() != 0 {
            let t = var1.clone();
            let t = t.borrow().child[0].borrow().clone().terminal;
            let num;
            match t {
                Terminals::Integer(i) => {
                    num = i;
                }
                _ => {
                    panic!("Error: {} is not a integer", t.to_string());
                }
            }
            dimensions.push(num as usize);
            let var2 = var1.borrow().child[1].clone();
            var1 = var2;
        }
        var.borrow_mut().kind.exp = ExpKind::ArrayK{size: dimensions.len(),dimensions};
        local_declarations.borrow_mut().child.push(var);
    } else {
        var.borrow_mut().kind.exp = ExpKind::IdK;
        local_declarations.borrow_mut().child.push(var);
    }
    if var_dec_stmt.borrow().child.len() == 1 {
        local_declarations.borrow_mut().child.push(var_dec_stmt.borrow().child[0].clone());
    }
}

pub fn statement_list_normalize(statement_list: Rc<RefCell<TreeNode>>) {
    let mut statement_list1 = statement_list.borrow_mut().child.pop().unwrap();
    while statement_list1.borrow().child.len() != 0 {
        let statement_list2 = statement_list1.borrow_mut().child.pop().unwrap();
        let statement = statement_list1.borrow_mut().child.pop().unwrap();
        statement_list.borrow_mut().child.push(statement);
        statement_list1 = statement_list2;
    }
}

pub fn case_stmt_n_normalize(case_stmt_n: Rc<RefCell<TreeNode>>) {
    let mut case_stmt_n1 = case_stmt_n.borrow_mut().child.pop().unwrap();
    while case_stmt_n1.borrow().child.len() != 0 {
        let case_stmt_n_ = case_stmt_n1.borrow_mut().child.pop().unwrap();
        let case_stmt = case_stmt_n_.borrow().child[0].clone();
        case_stmt_n.borrow_mut().child.push(case_stmt);
        let case_stmt_n2 = case_stmt_n_.borrow().child[1].clone();
        case_stmt_n1 = case_stmt_n2;
    }
}

pub fn expression_normalize(expression: Rc<RefCell<TreeNode>>) {
    let mut expression1 = expression.borrow_mut().child.pop().unwrap();
    while expression1.borrow().child.len() !=0 {
        let expression2 = expression1.borrow_mut().child.pop().unwrap();
        let additive_expression = expression1.borrow_mut().child.pop().unwrap();
        let logical_op = expression1.borrow_mut().child.pop().unwrap();
        expression.borrow_mut().child.push(logical_op);
        expression.borrow_mut().child.push(additive_expression);
        expression1 = expression2;
    }
}

pub fn additive_expression_normalize(additive_expression: Rc<RefCell<TreeNode>>) {
    let mut additive_expression1 = additive_expression.borrow_mut().child.pop().unwrap();
    while additive_expression1.borrow().child.len() !=0 {
        let additive_expression2 = additive_expression1.borrow_mut().child.pop().unwrap();
        let simple_expression = additive_expression1.borrow_mut().child.pop().unwrap();
        let relational_op = additive_expression1.borrow_mut().child.pop().unwrap();
        additive_expression.borrow_mut().child.push(relational_op);
        additive_expression.borrow_mut().child.push(simple_expression);
        additive_expression1 = additive_expression2;
    }
}

pub fn simple_expression_normalize(simple_expression: Rc<RefCell<TreeNode>>) {
    let mut simple_expression1 = simple_expression.borrow_mut().child.pop().unwrap();
    while simple_expression1.borrow().child.len() !=0 {
        let simple_expression2 = simple_expression1.borrow_mut().child.pop().unwrap();
        let term = simple_expression1.borrow_mut().child.pop().unwrap();
        let add_op = simple_expression1.borrow_mut().child.pop().unwrap();
        simple_expression.borrow_mut().child.push(add_op);
        simple_expression.borrow_mut().child.push(term);
        simple_expression1 = simple_expression2;
    }
}

pub fn term_normalize(term: Rc<RefCell<TreeNode>>) {
    let mut term1 = term.borrow_mut().child.pop().unwrap();
    while term1.borrow().child.len() !=0 {
        let term2 = term1.borrow_mut().child.pop().unwrap();
        let factor = term1.borrow_mut().child.pop().unwrap();
        let mul_op = term1.borrow_mut().child.pop().unwrap();
        term.borrow_mut().child.push(mul_op);
        term.borrow_mut().child.push(factor);
        term1 = term2;
    }
}

pub fn factor_normalize(factor: Rc<RefCell<TreeNode>>) {
    if factor.borrow().child[0].borrow().non_terminal == NonTerminals::VarOrCall {
        let var_or_call = factor.borrow_mut().child.pop().unwrap();
        let var_or_call_remain = var_or_call.borrow_mut().child.pop().unwrap();
        if var_or_call_remain.borrow().child[0].borrow().non_terminal == NonTerminals::Var1 {
            let var = Rc::new(RefCell::new(TreeNode::new_non_terminal(NonTerminals::Var)));
            if var_or_call_remain.borrow().child[0].borrow().child.len() == 0 {
                var.borrow_mut().kind.exp = ExpKind::IdK;
            } else {
                let mut var1 = var_or_call_remain.borrow().child[0].clone();
                let mut dimensions = Vec::new();
                while var1.borrow().child.len() != 0 {
                    let t = var1.clone();
                    let t = t.borrow().child[0].borrow().clone().terminal;
                    let num;
                    match t {
                        Terminals::Integer(i) => {
                            num = i;
                        }
                        _ => {
                            panic!("Error: {} is not a integer", t.to_string());
                        }
                    }
                    dimensions.push(num as usize);
                    let var2 = var1.borrow().child[1].clone();
                    var1 = var2;
                }
                var.borrow_mut().kind.exp = ExpKind::ArrayK{size: dimensions.len(),dimensions};
            }
            var.borrow_mut().child.push(var_or_call.borrow().child[0].clone());
            factor.borrow_mut().child.push(var);
        } else {
            let call_stmt = Rc::new(RefCell::new(TreeNode::new_non_terminal(NonTerminals::CallStmt)));
            call_stmt.borrow_mut().child.push(var_or_call.borrow().child[0].clone());
            call_stmt.borrow_mut().child.push(var_or_call_remain.borrow().child[0].clone());
            factor.borrow_mut().child.push(call_stmt);
        }
    }
}

pub fn args_normalize(args: Rc<RefCell<TreeNode>>) {
    if args.borrow().child.len() == 0 {
        return;
    }
    if args.borrow().child[0].borrow().non_terminal == NonTerminals::ArgList {
        let arg_list = args.borrow_mut().child.pop().unwrap();
        let mut arg_list1 = arg_list.borrow_mut().child.pop().unwrap();
        args.borrow_mut().child.push(arg_list.borrow_mut().child.pop().unwrap());
        while arg_list1.borrow().child.len() != 0 {
            let arg_list2 = arg_list1.borrow_mut().child.pop().unwrap();
            args.borrow_mut().child.push(arg_list1.borrow_mut().child.pop().unwrap());
            arg_list1 = arg_list2;
        }
    }
}

pub fn var_or_call_stmt_normalize(var_or_call_stmt: Rc<RefCell<TreeNode>>) {
    let var_or_call_stmt_remain = var_or_call_stmt.borrow().child[1].clone();
    if var_or_call_stmt_remain.borrow().child[0].borrow().non_terminal == NonTerminals::Var1 {
        var_or_call_stmt.borrow_mut().kind.exp = ExpKind::IdK;
        let var = Rc::new(RefCell::new(TreeNode::new_non_terminal(NonTerminals::Var)));
        if var_or_call_stmt_remain.borrow().child[0].borrow().child.len() == 0 {
            var.borrow_mut().kind.exp = ExpKind::IdK;
        } else {
            let mut var1 = var_or_call_stmt_remain.borrow().child[0].clone();
            let mut dimensions = Vec::new();
            while var1.borrow().child.len() != 0 {
                let t = var1.clone();
                let t = t.borrow().child[0].borrow().clone().terminal;
                let num;
                match t {
                    Terminals::Integer(i) => {
                        num = i;
                    }
                    _ => {
                        panic!("Error: {} is not a integer", t.to_string());
                    }
                }
                dimensions.push(num as usize);
                let var2 = var1.borrow().child[1].clone();
                var1 = var2;
            }
            var.borrow_mut().kind.exp = ExpKind::ArrayK{size: dimensions.len(),dimensions};
        }
        var.borrow_mut().child.push(var_or_call_stmt.borrow().child[0].clone());
        var_or_call_stmt.borrow_mut().child.pop();
        var_or_call_stmt.borrow_mut().child.pop();
        var_or_call_stmt.borrow_mut().child.push(var);
        var_or_call_stmt.borrow_mut().child.push(var_or_call_stmt_remain.borrow().child[1].borrow().child[0].clone());
    } else {
        var_or_call_stmt.borrow_mut().kind.exp = ExpKind::CallK;
        let call_stmt = Rc::new(RefCell::new(TreeNode::new_non_terminal(NonTerminals::CallStmt)));
        call_stmt.borrow_mut().child.push(var_or_call_stmt.borrow().child[0].clone());
        call_stmt.borrow_mut().child.push(var_or_call_stmt_remain.borrow().child[0].clone());
        var_or_call_stmt.borrow_mut().child.pop();
        var_or_call_stmt.borrow_mut().child.pop();
        var_or_call_stmt.borrow_mut().child.push(call_stmt);
    }
}
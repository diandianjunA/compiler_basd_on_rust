pub mod target_code_gen;
pub mod optimize;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::analyzer::analyze::Analyzer;
use crate::ast::lexer::Terminals;
use crate::ast::printer::ParserPrinter;
use crate::ast::StmtKind::VarDecK;
use crate::ast::{ExpKind, NonTerminals, TreeNode};

pub struct CodeGen {
    pub codes: Vec<Code>,
    pub label_count: usize,
    pub temp_count: usize,
    pub analyzer: Rc<RefCell<Analyzer>>,
    pub current_func: String,
    pub var_count: usize,
    pub level: usize,
    pub next_label: String,
    pub pre_label: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StatementKind {
    Declaration,
    Assign,
    Call,
    Func,
    Param,
    If,
    Goto,
    Return,
    Arg,
    Label,
    LoopStart,
    LoopEnd,
}

#[derive(Debug, Clone)]
pub struct Code {
    pub kind: StatementKind,
    pub raw: String,
    pub op: String,
    pub left: String,
    pub right: String,
    pub label: String,
    pub goto: String,
    pub if_goto: String,
    pub param: String,
    pub arg: String,
    pub call: String,
    pub return_: String,
    pub func: String,
    pub assign: String,
    pub param_index: usize,
}

impl Code {
    pub fn new(raw: String) -> Self {
        Self {
            kind: StatementKind::Declaration,
            raw,
            op: "".to_string(),
            left: "".to_string(),
            right: "".to_string(),
            label: "".to_string(),
            goto: "".to_string(),
            if_goto: "".to_string(),
            param: "".to_string(),
            arg: "".to_string(),
            call: "".to_string(),
            return_: "".to_string(),
            func: "".to_string(),
            assign: "".to_string(),
            param_index: 0,
        }
    }
}

impl CodeGen {
    pub fn new(analyzer: Rc<RefCell<Analyzer>>) -> Self {
        Self {
            codes: vec![],
            label_count: 0,
            temp_count: 0,
            analyzer,
            current_func: "".to_string(),
            var_count: 0,
            level: 0,
            next_label: "".to_string(),
            pre_label: "".to_string(),
        }
    }

    pub fn new_label(&mut self) -> String {
        let label = format!("L{}", self.label_count);
        self.label_count += 1;
        label
    }

    pub fn new_temp(&mut self) -> String {
        let temp = format!("T{}", self.temp_count);
        self.temp_count += 1;
        temp
    }

    pub fn new_var(&mut self) -> String {
        let var = format!("V{}", self.var_count);
        self.var_count += 1;
        var
    }

    pub fn add_code(&mut self, code: Code) {
        self.codes.push(code);
    }

    pub fn add_codes(&mut self, codes: Vec<Code>) {
        for code in codes {
            self.codes.push(code);
        }
    }

    pub fn gen_assign(&mut self, assign: &str, left: &str, right: &str, op: &str) -> Code {
        let raw = format!(" {} = {} {} {}", assign, left, op, right);
        let mut code = Code::new(raw);
        code.kind = StatementKind::Assign;
        code.assign = assign.to_string();
        code.left = left.to_string();
        code.right = right.to_string();
        code.op = op.to_string();
        code
    }

    pub fn gen_param(&mut self, param: &str, index: usize) -> Code {
        let raw = format!(" param {}", param);
        let mut code = Code::new(raw);
        code.kind = StatementKind::Param;
        code.param = param.to_string();
        code.param_index = index;
        code
    }

    pub fn gen_arg(&mut self, arg: &str, index: usize) -> Code {
        let raw = format!(" arg {}", arg);
        let mut code = Code::new(raw);
        code.kind = StatementKind::Arg;
        code.arg = arg.to_string();
        code.param_index = index;
        code
    }

    pub fn gen_call(&mut self, func_name: &str) -> Code {
        let raw = format!(" Call {}", func_name);
        let mut code = Code::new(raw);
        code.kind = StatementKind::Call;
        code.call = func_name.to_string();
        code
    }

    pub fn gen_declaration(&mut self, left: &str, right: &str) -> Code {
        let raw = format!(" {} := {}", left, right);
        let mut code = Code::new(raw);
        code.kind = StatementKind::Declaration;
        code.left = left.to_string();
        code.right = right.to_string();
        code
    }

    pub fn gen_goto(&mut self, label: &str) -> Code {
        let raw = format!(" goto {}", label);
        let mut code = Code::new(raw);
        code.kind = StatementKind::Goto;
        code.goto = label.to_string();
        code
    }

    pub fn gen_if_goto(&mut self, identifier: &str, label: &str) -> Code {
        let raw = format!(" if {} != #0 goto {}", identifier, label);
        let mut code = Code::new(raw);
        code.kind = StatementKind::If;
        code.if_goto = identifier.to_string();
        code.label = label.to_string();
        code
    }

    pub fn gen_return(&mut self, value: &str) -> Code {
        let raw = format!(" return {}", value);
        let mut code = Code::new(raw);
        code.kind = StatementKind::Return;
        code.return_ = value.to_string();
        code
    }

    pub fn gen_func(&mut self, func_name: &str) -> Code {
        let raw = format!("func {} :", func_name);
        let mut code = Code::new(raw);
        code.kind = StatementKind::Func;
        code.func = func_name.to_string();
        code
    }

    pub fn loop_start(&mut self) -> Code {
        let raw = format!("loop_start");
        let mut code = Code::new(raw);
        code.kind = StatementKind::LoopStart;
        code
    }
    
    pub fn loop_end(&mut self) -> Code {
        let raw = format!("loop_end");
        let mut code = Code::new(raw);
        code.kind = StatementKind::LoopEnd;
        code
    }

    pub fn gen_code_from_label(&mut self, label: &str) -> Code {
        let raw = format!("{} :", label);
        let mut code = Code::new(raw);
        code.kind = StatementKind::Label;
        code.label = label.to_string();
        code
    }

    pub fn gen_code(&mut self) {
        let tree = self.analyzer.borrow().root.clone();
        let declaration_list = tree.borrow().child[0].clone();
        self.gen_code_from_declaration_list(declaration_list);
    }

    pub fn gen_code_from_declaration_list(&mut self, declaration_list: Rc<RefCell<TreeNode>>) {
        let mut declaration_list = declaration_list.clone();
        for declaration in declaration_list.borrow().child.iter() {
            self.gen_code_from_declaration(declaration.clone());
        }
    }

    pub fn gen_code_from_declaration(&mut self, declaration: Rc<RefCell<TreeNode>>) {
        if declaration.borrow().kind.stmt == VarDecK {
            if declaration.borrow().child.len() > 2 {
                let type_specifier = declaration.borrow().child[0].clone();
                let kind = type_specifier.borrow().child[0].borrow().terminal.clone();
                match kind {
                    Terminals::Char => {
                        let var = declaration.borrow().child[1].clone();
                        let kind = var.borrow().kind.exp.clone();
                        match kind {
                            ExpKind::ArrayK { size,dimensions } => {
                                let expression = declaration.borrow().child[2].clone();
                                let terminals = expression.borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().terminal.clone();
                                match terminals {
                                    Terminals::String(s) => {
                                        let terminals = var.borrow().clone().child[0].borrow().clone().terminal;
                                        let mut var_name = format!("{}", terminals);
                                        let split = var_name.split(' ');
                                        let vec = split.collect::<Vec<&str>>();
                                        var_name = vec[1].to_string();
                                        let mut current_table = self.analyzer.borrow().symbol_table_manager.global_table.clone();
                                        for table in self.analyzer.borrow().symbol_table_manager.tables.iter() {
                                            if table.borrow().func_name == self.current_func {
                                                current_table = table.clone();
                                                break;
                                            }
                                        }
                                        for i in 0..s.len() {
                                            let c_var = format!("{}[{}]", var_name, i);
                                            let mut symbols = vec![];
                                            for symbol in current_table.borrow().symbols.iter() {
                                                if symbol.borrow().name == c_var {
                                                    symbols.push(symbol.clone());
                                                }
                                            }
                                            if symbols.len() == 0 {
                                                let global_table = self.analyzer.borrow().symbol_table_manager.global_table.clone();
                                                for symbol in global_table.borrow().symbols.iter() {
                                                    if symbol.borrow().name == c_var {
                                                        symbols.push(symbol.clone());
                                                    }
                                                }
                                            }
                                            if symbols.len() != 0 {
                                                symbols.sort_by(|a, b| b.borrow().level.cmp(&a.borrow().level));
                                                let mut symbol;
                                                for sym in symbols.iter() {
                                                    if sym.borrow().level <= self.level as i32 {
                                                        symbol = sym.clone();
                                                        let alias = symbol.borrow().alias.clone();
                                                        let code = self.gen_declaration(&alias, &format!("#{}", s.as_bytes()[i]));
                                                        self.add_code(code);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        panic!("{}: 类型不匹配，声明为char数组，初始化时未使用String", declaration.borrow().line)
                                    }
                                }
                            }
                            _ => {
                                let expression = declaration.borrow().child[2].clone();
                                let t = self.gen_code_from_exp(expression);
                                let var = self.gen_code_from_var(declaration.borrow().child[1].clone());
                                let code = self.gen_declaration(&var, &t);
                                self.add_code(code);
                            }
                        }
                    }
                    _ => {
                        let expression = declaration.borrow().child[2].clone();
                        let t = self.gen_code_from_exp(expression);
                        let var = self.gen_code_from_var(declaration.borrow().child[1].clone());
                        let code = self.gen_declaration(&var, &t);
                        self.add_code(code);
                    }
                }
            }
        } else {
            let terminals = declaration.borrow().child[1].borrow().terminal.clone();
            let func_name;
            match terminals {
                Terminals::Identifier(s) => {
                    func_name = s;
                }
                _ => {
                    panic!("{}: 函数声明错误", declaration.borrow().line);
                }
            }
            self.current_func = func_name.clone();
            let code = self.gen_func(&func_name);
            self.add_code(code);
            let mut params = vec![];
            let mut current_table = self.analyzer.borrow().symbol_table_manager.get_current();
            for table in self.analyzer.borrow().symbol_table_manager.tables.iter() {
                if table.borrow().func_name == func_name {
                    current_table = table.clone();
                    break;
                }
            }
            for symbol in current_table.borrow().symbols.iter() {
                if symbol.borrow().flag == 'p' {
                    let alias = symbol.borrow().alias.clone();
                    params.push(alias);
                }
            }
            let mut code = vec![];
            let mut index: usize = 0;
            for param in params.iter() {
                code.push(self.gen_param(param, index));
                index += 1;
            }
            self.add_codes(code);
            self.level += 1;
            let compound_stmt = declaration.borrow().child[3].clone();
            self.gen_code_from_compound_stmt(compound_stmt);
            self.level -= 1;
        }
    }

    pub fn gen_code_from_compound_stmt(&mut self, compound_stmt: Rc<RefCell<TreeNode>>) {
        let statement_list = compound_stmt.borrow().child[0].clone();
        self.gen_code_from_statement_list(statement_list);
    }

    pub fn gen_code_from_statement_list(&mut self, statement_list: Rc<RefCell<TreeNode>>) {
        let mut statement_list = statement_list.clone();
        for statement in statement_list.borrow().child.iter() {
            self.gen_code_from_statement(statement.clone());
        }
    }

    pub fn gen_code_from_statement(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let non_terminal = statement.borrow().child[0].borrow().clone().non_terminal;
        match non_terminal {
            NonTerminals::LocalDeclarations => {
                let local_declarations = statement.borrow().child[0].clone();
                if local_declarations.borrow().child.len() > 2 {
                    let type_specifier = local_declarations.borrow().child[0].clone();
                    let kind = type_specifier.borrow().child[0].borrow().terminal.clone();
                    match kind {
                        Terminals::Char => {
                            let var = local_declarations.borrow().child[1].clone();
                            let kind = var.borrow().kind.exp.clone();
                            match kind {
                                ExpKind::ArrayK { size,dimensions } => {
                                    let expression = local_declarations.borrow().child[2].clone();
                                    let terminals = expression.borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().terminal.clone();
                                    match terminals {
                                        Terminals::String(s) => {
                                            let terminals = var.borrow().clone().child[0].borrow().clone().terminal;
                                            let mut var_name = format!("{}", terminals);
                                            let split = var_name.split(' ');
                                            let vec = split.collect::<Vec<&str>>();
                                            var_name = vec[1].to_string();
                                            let mut current_table = self.analyzer.borrow().symbol_table_manager.global_table.clone();
                                            for table in self.analyzer.borrow().symbol_table_manager.tables.iter() {
                                                if table.borrow().func_name == self.current_func {
                                                    current_table = table.clone();
                                                    break;
                                                }
                                            }
                                            for i in 0..s.len() {
                                                let c_var = format!("{}[{}]", var_name, i);
                                                let mut symbols = vec![];
                                                for symbol in current_table.borrow().symbols.iter() {
                                                    if symbol.borrow().name == c_var {
                                                        symbols.push(symbol.clone());
                                                    }
                                                }
                                                if symbols.len() == 0 {
                                                    let global_table = self.analyzer.borrow().symbol_table_manager.global_table.clone();
                                                    for symbol in global_table.borrow().symbols.iter() {
                                                        if symbol.borrow().name == c_var {
                                                            symbols.push(symbol.clone());
                                                        }
                                                    }
                                                }
                                                if symbols.len() != 0 {
                                                    symbols.sort_by(|a, b| b.borrow().level.cmp(&a.borrow().level));
                                                    let mut symbol;
                                                    for sym in symbols.iter() {
                                                        if sym.borrow().level <= self.level as i32 {
                                                            symbol = sym.clone();
                                                            let alias = symbol.borrow().alias.clone();
                                                            let code = self.gen_declaration(&alias, &format!("#{}", s.as_bytes()[i]));
                                                            self.add_code(code);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => {
                                            panic!("{}: 类型不匹配，声明为char数组，初始化时未使用String", local_declarations.borrow().line)
                                        }
                                    }
                                }
                                _ => {
                                    let expression = local_declarations.borrow().child[2].clone();
                                    let t = self.gen_code_from_exp(expression);
                                    let var = self.gen_code_from_var(local_declarations.borrow().child[1].clone());
                                    let code = self.gen_declaration(&var, &t);
                                    self.add_code(code);
                                }
                            }
                        }
                        _ => {
                            let expression = local_declarations.borrow().child[2].clone();
                            let t = self.gen_code_from_exp(expression);
                            let var = self.gen_code_from_var(local_declarations.borrow().child[1].clone());
                            let code = self.gen_declaration(&var, &t);
                            self.add_code(code);
                        }
                    }
                }
            }
            NonTerminals::IfStmt => {
                let if_stmt = statement.borrow().child[0].clone();
                let expression = if_stmt.borrow().child[0].clone();
                let t = self.gen_code_from_exp(expression);
                let label1 = self.new_label();
                let label2 = self.new_label();
                let code = self.gen_if_goto(&t,  &label1);
                self.add_code(code);
                self.level += 1;
                let compound_stmt = if_stmt.borrow().child[1].clone();
                self.gen_code_from_compound_stmt(compound_stmt);
                self.level -= 1;
                let code = self.gen_goto(&label2);
                self.add_code(code);
                let code = self.gen_code_from_label(&label1);
                self.add_code(code);
                let else_stmt_n = if_stmt.borrow().child[2].clone();
                if else_stmt_n.borrow().child.len() != 0 {
                    let else_stmt = else_stmt_n.borrow().child[0].clone();
                    self.level += 1;
                    let compound_stmt = else_stmt.borrow().child[1].clone();
                    self.gen_code_from_compound_stmt(compound_stmt);
                    self.level -= 1;
                }
                let code = self.gen_code_from_label(&label2);
                self.add_code(code);
            }
            NonTerminals::WhileStmt => {
                let start = self.loop_start();
                self.add_code(start);
                let while_stmt = statement.borrow().child[0].clone();
                let label0 = self.new_label();
                let code = self.gen_code_from_label(&label0);
                self.add_code(code);
                let expression = while_stmt.borrow().child[0].clone();
                let t = self.gen_code_from_exp(expression);
                let label1 = self.new_label();
                let code = self.gen_if_goto(&t, &label1);
                self.add_code(code);
                self.level += 1;
                let temp_next_label = self.next_label.clone();
                self.next_label = label1.clone();
                let temp_pre_label = self.pre_label.clone();
                self.pre_label = label0.clone();
                let compound_stmt = while_stmt.borrow().child[1].clone();
                self.gen_code_from_compound_stmt(compound_stmt);
                self.next_label = temp_next_label;
                self.pre_label = temp_pre_label;
                self.level -= 1;
                let code = self.gen_goto(&label0);
                self.add_code(code);
                let end = self.loop_end();
                self.add_code(end);
                let code = self.gen_code_from_label(&label1);
                self.add_code(code);
            }
            NonTerminals::ForStmt => {
                let start = self.loop_start();
                self.add_code(start);
                let for_stmt = statement.borrow().child[0].clone();
                let for_expression = for_stmt.borrow().child[0].clone();
                let compound_stmt = for_stmt.borrow().child[1].clone();
                let expression_assign1 = for_expression.borrow().child[0].clone();
                if expression_assign1.borrow().child.len() > 0 {
                    self.gen_code_from_var_or_call_stmt(expression_assign1.borrow().child[0].clone());
                }
                let label0 = self.new_label();
                let label1 = self.new_label();
                let code = self.gen_code_from_label(&label0);
                self.add_code(code);
                let expression = for_expression.borrow().child[1].clone();
                let t = self.gen_code_from_exp(expression);
                let code = self.gen_if_goto(&t,  &label1);
                self.add_code(code);
                self.level += 1;
                let temp_next_label = self.next_label.clone();
                let temp_pre_label = self.pre_label.clone();
                self.next_label = label1.clone();
                self.pre_label = label0.clone();
                self.gen_code_from_compound_stmt(compound_stmt);
                self.next_label = temp_next_label;
                self.pre_label = temp_pre_label;
                self.level -= 1;
                let expression_assign2 = for_expression.borrow().child[2].clone();
                if expression_assign2.borrow().child.len() > 0 {
                    self.gen_code_from_var_or_call_stmt(expression_assign2.borrow().child[0].clone());
                }
                let code = self.gen_goto(&label0);
                self.add_code(code);
                let end = self.loop_end();
                self.add_code(end);
                let code = self.gen_code_from_label(&label1);
                self.add_code(code);
            }
            NonTerminals::DoStmt => {
                let start = self.loop_start();
                self.add_code(start);
                let do_stmt = statement.borrow().child[0].clone();
                let label0 = self.new_label();
                let label1 = self.new_label();
                let code = self.gen_code_from_label(&label0);
                self.add_code(code);
                let temp_next_label = self.next_label.clone();
                let temp_pre_label = self.pre_label.clone();
                self.level += 1;
                self.next_label = label1.clone();
                self.pre_label = label0.clone();
                let compound_stmt = do_stmt.borrow().child[0].clone();
                self.gen_code_from_compound_stmt(compound_stmt);
                self.next_label = temp_next_label;
                self.pre_label = temp_pre_label;
                self.level -= 1;
                let expression = do_stmt.borrow().child[1].clone();
                let t = self.gen_code_from_exp(expression);
                let code = self.gen_if_goto(&t, &label1);
                self.add_code(code);
                let code = self.gen_goto(&label0);
                self.add_code(code);
                let end = self.loop_end();
                self.add_code(end);
                let code = self.gen_code_from_label(&label1);
                self.add_code(code);
            }
            NonTerminals::BreakStmt => {
                let x = &self.next_label.clone();
                let code = self.gen_goto(x);
                self.add_code(code);
            }
            NonTerminals::ContinueStmt => {
                let x = &self.pre_label.clone();
                let code = self.gen_goto(x);
                self.add_code(code);
            }
            NonTerminals::SwitchStmt => {
                let switch_stmt = statement.borrow().child[0].clone();
                let expression = switch_stmt.borrow().child[0].clone();
                let t = self.gen_code_from_exp(expression);
                let case_default_stmt = switch_stmt.borrow().child[1].clone();
                let case_stmt_n = case_default_stmt.borrow().child[0].clone();
                let default_stmt = case_default_stmt.borrow().child[1].clone();
                let label1 = self.new_label();
                let mut label2 = self.new_label();
                for case_stmt in case_stmt_n.borrow().child.iter() {
                    let code = self.gen_code_from_label(&label2);
                    self.add_code(code);
                    label2 = self.new_label();
                    let expression = case_stmt.borrow().child[0].clone();
                    let t1 = self.gen_code_from_exp(expression);
                    let t2 = self.new_temp();
                    let code = self.gen_assign(&t2, &t, &t1, "!=");
                    self.add_code(code);
                    let code = self.gen_if_goto(&t2, &label2);
                    self.add_code(code);
                    let statement_list = case_stmt.borrow().child[1].clone();
                    self.level += 1;
                    let temp_next_label = self.next_label.clone();
                    self.next_label = label1.clone();
                    let temp_pre_label = self.pre_label.clone();
                    self.gen_code_from_statement_list(statement_list);
                    self.next_label = temp_next_label;
                    self.pre_label = temp_pre_label;
                    self.level -= 1;
                }
                let code = self.gen_goto(&label2);  // default
                self.add_code(code);
                let code = self.gen_code_from_label(&label2);
                self.add_code(code);
                let statement_list = default_stmt.borrow().child[0].clone();
                self.level += 1;
                let temp_next_label = self.next_label.clone();
                self.next_label = label1.clone();
                let temp_pre_label = self.pre_label.clone();
                self.gen_code_from_statement_list(statement_list);
                self.next_label = temp_next_label;
                self.pre_label = temp_pre_label;
                self.level -= 1;
                let code = self.gen_code_from_label(&label1);
                self.add_code(code);
            }
            NonTerminals::ReturnStmt => {
                let return_stmt = statement.borrow().child[0].clone();
                if return_stmt.borrow().child.len() > 0 {
                    let expression = return_stmt.borrow().child[0].clone();
                    let t = self.gen_code_from_exp(expression);
                    let code = self.gen_return(&t);
                    self.add_code(code);
                } else {
                    let code = self.gen_return("#0");
                    self.add_code(code);
                }
            }
            NonTerminals::VarOrCallStmt => {
                self.gen_code_from_var_or_call_stmt(statement.borrow().child[0].clone());
            }
            _ => {
                panic!("{}: 语句类型错误", statement.borrow().line);
            }
        }
    }

    pub fn gen_code_from_exp(&mut self, exp: Rc<RefCell<TreeNode>>) -> String {
        return if exp.borrow().child.len() == 1 {
            self.gen_code_from_additive_exp(exp.borrow().child[0].clone())
        } else {
            let mut t1 = self.gen_code_from_additive_exp(exp.borrow().child[0].clone());
            let mut t2 = self.gen_code_from_additive_exp(exp.borrow().child[2].clone());
            let terminals = exp.borrow().child[1].borrow().child[0].borrow().terminal.clone();
            let mut op: String;
            match terminals {
                Terminals::And => {
                    op = "&&".to_string();
                }
                Terminals::Or => {
                    op = "||".to_string();
                }
                Terminals::Not => {
                    op = "!".to_string();
                }
                _ => {
                    panic!("不支持的操作符！")
                }
            }
            let t = self.new_temp();
            let code = self.gen_assign(&t, &t1, &t2, &op);
            self.add_code(code);
            t1 = t;
            for i in 3..exp.borrow().child.len() {
                if exp.borrow().child[i].borrow().is_terminal {
                    if exp.borrow().child[i].borrow().child[0].borrow().terminal == Terminals::And {
                        op = "&&".to_string();
                    } else if exp.borrow().child[i].borrow().child[0].borrow().terminal == Terminals::Or {
                        op = "||".to_string();
                    } else if exp.borrow().child[i].borrow().child[0].borrow().terminal == Terminals::Not {
                        op = "!".to_string();
                    } else {
                        panic!("不支持的操作符！")
                    }
                } else {
                    t2 = self.gen_code_from_additive_exp(exp.borrow().child[i].clone());
                    let t = self.new_temp();
                    let code = self.gen_assign(&t, &t1, &t2, &op);
                    self.add_code(code);
                    t1 = t;
                }
            }
            t1
        }
    }

    pub fn gen_code_from_additive_exp(&mut self, additive_exp: Rc<RefCell<TreeNode>>) -> String {
        return if additive_exp.borrow().child.len() == 1 {
            self.gen_code_from_simple_exp(additive_exp.borrow().child[0].clone())
        } else {
            let mut t1 = self.gen_code_from_simple_exp(additive_exp.borrow().child[0].clone());
            let mut t2 = self.gen_code_from_simple_exp(additive_exp.borrow().child[2].clone());
            let terminals = additive_exp.borrow().child[1].borrow().child[0].borrow().terminal.clone();
            let mut op: String;
            match terminals {
                Terminals::LessThan => {
                    op = "<".to_string();
                }
                Terminals::LessThanEquals => {
                    op = "<=".to_string();
                }
                Terminals::GreaterThan => {
                    op = ">".to_string();
                }
                Terminals::GreaterThanEquals => {
                    op = ">=".to_string();
                }
                Terminals::EqualsEquals => {
                    op = "==".to_string();
                }
                Terminals::BangEquals => {
                    op = "!=".to_string();
                }
                _ => {
                    panic!("不支持的操作符！")
                }
            }
            let t = self.new_temp();
            let code = self.gen_assign(&t, &t1, &t2, &op);
            self.add_code(code);
            t1 = t;
            for i in 3..additive_exp.borrow().child.len() {
                if additive_exp.borrow().child[i].borrow().is_terminal {
                    let terminals = additive_exp.borrow().child[i].borrow().child[0].borrow().terminal.clone();
                    match terminals {
                        Terminals::LessThan => {
                            op = "<".to_string();
                        }
                        Terminals::LessThanEquals => {
                            op = "<=".to_string();
                        }
                        Terminals::GreaterThan => {
                            op = ">".to_string();
                        }
                        Terminals::GreaterThanEquals => {
                            op = ">=".to_string();
                        }
                        Terminals::EqualsEquals => {
                            op = "==".to_string();
                        }
                        Terminals::BangEquals => {
                            op = "!=".to_string();
                        }
                        _ => {
                            panic!("不支持的操作符！")
                        }
                    }
                } else {
                    t2 = self.gen_code_from_simple_exp(additive_exp.borrow().child[i].clone());
                    let t = self.new_temp();
                    let code = self.gen_assign(&t, &t1, &t2, &op);
                    self.add_code(code);
                    t1 = t;
                }
            }
            t1
        }
    }

    pub fn gen_code_from_simple_exp(&mut self, simple_exp: Rc<RefCell<TreeNode>>) -> String {
        if simple_exp.borrow().child.len() == 1 {
            self.gen_code_from_term(simple_exp.borrow().child[0].clone())
        } else {
            let mut t1 = self.gen_code_from_term(simple_exp.borrow().child[0].clone());
            let mut t2 = self.gen_code_from_term(simple_exp.borrow().child[2].clone());
            let terminals = simple_exp.borrow().child[1].borrow().child[0].borrow().terminal.clone();
            let mut op: String;
            match terminals {
                Terminals::Plus => {
                    op = "+".to_string();
                }
                Terminals::Minus => {
                    op = "-".to_string();
                }
                _ => {
                    panic!("不支持的操作符！")
                }
            }
            let t = self.new_temp();
            let code = self.gen_assign(&t, &t1, &t2, &op);
            self.add_code(code);
            t1 = t;
            for i in 3..simple_exp.borrow().child.len() {
                if simple_exp.borrow().child[i].borrow().is_terminal {
                    if simple_exp.borrow().child[i].borrow().child[0].borrow().terminal == Terminals::Plus {
                        op = "+".to_string();
                    } else if simple_exp.borrow().child[i].borrow().child[0].borrow().terminal == Terminals::Minus {
                        op = "-".to_string();
                    } else {
                        panic!("不支持的操作符！")
                    }
                } else {
                    t2 = self.gen_code_from_term(simple_exp.borrow().child[i].clone());
                    let t = self.new_temp();
                    let code = self.gen_assign(&t, &t1, &t2, &op);
                    self.add_code(code);
                    t1 = t;
                }
            }
            t1
        }
    }

    pub fn gen_code_from_term(&mut self, term: Rc<RefCell<TreeNode>>) -> String {
        if term.borrow().child.len() == 1 {
            self.gen_code_from_factor(term.borrow().child[0].clone())
        } else {
            let mut t1 = self.gen_code_from_factor(term.borrow().child[0].clone());
            let mut t2 = self.gen_code_from_factor(term.borrow().child[2].clone());
            let terminals = term.borrow().child[1].borrow().child[0].borrow().terminal.clone();
            let mut op: String;
            match terminals {
                Terminals::Asterisk => {
                    op = "*".to_string();
                }
                Terminals::Slash => {
                    op = "/".to_string();
                }
                _ => {
                    panic!("不支持的操作符！")
                }
            }
            let t = self.new_temp();
            let code = self.gen_assign(&t, &t1, &t2, &op);
            self.add_code(code);
            t1 = t;
            for i in 3..term.borrow().child.len() {
                if term.borrow().child[i].borrow().is_terminal {
                    if term.borrow().child[i].borrow().child[0].borrow().terminal == Terminals::Asterisk {
                        op = "*".to_string();
                    } else if term.borrow().child[i].borrow().child[0].borrow().terminal == Terminals::Slash {
                        op = "/".to_string();
                    } else {
                        panic!("不支持的操作符！")
                    }
                } else {
                    t2 = self.gen_code_from_factor(term.borrow().child[i].clone());
                    let t = self.new_temp();
                    let code = self.gen_assign(&t, &t1, &t2, &op);
                    self.add_code(code);
                    t1 = t;
                }
            }
            t1
        }
    }

    pub fn gen_code_from_factor(&mut self, factor: Rc<RefCell<TreeNode>>) -> String {
        if factor.borrow().child[0].borrow().non_terminal == NonTerminals::Expression {
            return self.gen_code_from_exp(factor.borrow().child[0].clone());
        } else if factor.borrow().child[0].borrow().non_terminal == NonTerminals::CallStmt {
            return self.gen_code_from_call(factor.borrow().child[0].clone());
        } else if factor.borrow().child[0].borrow().non_terminal == NonTerminals::Var {
            return self.gen_code_from_var(factor.borrow().child[0].clone());
        } else {
            let terminals = factor.borrow().child[0].borrow().child[0].borrow().terminal.clone();
            match terminals {
                Terminals::Integer(i) => {
                    return format!("#{}", i);
                }
                Terminals::FloatNumber(f) => {
                    return format!("#{}", f.round() as i32);
                }
                Terminals::String(s) => {
                    return format!("#{}", s);
                }
                Terminals::True => {
                    return "#1".to_string();
                }
                Terminals::False => {
                    return "#0".to_string();
                }
                Terminals::Character(c) => {
                    return format!("#{}", c as i32);
                }
                _ => {
                    panic!("{}: 语法错误", factor.borrow().line);
                }
            }
        }
    }

    pub fn gen_code_from_var(&mut self, var: Rc<RefCell<TreeNode>>) -> String {
        let mut current_table = self.analyzer.borrow().symbol_table_manager.global_table.clone();
        for table in self.analyzer.borrow().symbol_table_manager.tables.iter() {
            if table.borrow().func_name == self.current_func {
                current_table = table.clone();
                break;
            }
        }
        let kind = var.borrow().clone().kind.exp;
        let terminals = var.borrow().clone().child[0].borrow().clone().terminal;
        let mut var_name = format!("{}", terminals);
        let split = var_name.split(' ');
        let vec = split.collect::<Vec<&str>>();
        var_name = vec[1].to_string();
        match kind {
            ExpKind::ArrayK {size, dimensions} => {
                for i in dimensions.iter() {
                    var_name = var_name + &format!("[{}]", i);
                }
            }
            _ => {}
        }
        let mut symbols = vec![];
        for symbol in current_table.borrow().symbols.iter() {
            if symbol.borrow().name == var_name {
                symbols.push(symbol.clone());
            }
        }
        if symbols.len() == 0 {
            let global_table = self.analyzer.borrow().symbol_table_manager.global_table.clone();
            for symbol in global_table.borrow().symbols.iter() {
                if symbol.borrow().name.starts_with(&var_name) {
                    symbols.push(symbol.clone());
                }
            }
        }
        if symbols.len() != 0 {
            symbols.sort_by(|a, b| b.borrow().level.cmp(&a.borrow().level));
            let symbol;
            for s in symbols.iter() {
                if s.borrow().level <= self.level as i32 {
                    symbol = s.clone();
                    let alias = symbol.borrow().alias.clone();
                    return alias;
                }
            }
        }
        panic!("变量未定义！")
    }

    pub fn gen_code_from_call(&mut self, call: Rc<RefCell<TreeNode>>) -> String {
        let identifier = call.borrow().child[0].borrow().terminal.clone();
        let mut func_name = String::new();
        match identifier {
            Terminals::Identifier(i) => {
                func_name = i;
            }
            _ => {
                panic!("{}: 语法错误", call.borrow().line);
            }
        }
        let mut params = vec![];
        let args = call.borrow().child[1].clone();
        for i in 0..args.borrow().child.len() {
            params.push(self.gen_code_from_exp(args.borrow().child[i].clone()));
        }
        let mut code = vec![];
        let mut index: usize = 0;
        for param in params.iter() {
            code.push(self.gen_arg(param, index));
            index += 1;
        }
        let call = self.gen_call(&func_name);
        let t = self.new_temp();
        code.push(self.gen_declaration(&t, &call.call));
        self.add_codes(code);
        t
    }

    pub fn gen_code_from_var_or_call_stmt(&mut self, var_or_call_stmt: Rc<RefCell<TreeNode>>) {
        let var_or_call = var_or_call_stmt.borrow().child[0].clone();
        if var_or_call.borrow().non_terminal == NonTerminals::Var {
            let var = self.gen_code_from_var(var_or_call.clone());
            if var_or_call_stmt.borrow().child[1].borrow().non_terminal == NonTerminals::SelfOp {
                let op = var_or_call_stmt.borrow().child[1].borrow().child[0].borrow().terminal.clone();
                let mut op_str = String::new();
                match op {
                    Terminals::PlusPlus => {
                        op_str = "+".to_string();
                    }
                    Terminals::MinusMinus => {
                        op_str = "-".to_string();
                    }
                    _ => {
                        panic!("{}: 语句类型错误", var_or_call_stmt.borrow().line);
                    }
                }
                let t = self.new_temp();
                let code = self.gen_assign(&t, &var, &format!("#1"), &op_str);
                self.add_code(code);
                let code = self.gen_declaration(&var, &t);
                self.add_code(code);
            } else {
                let expression = var_or_call_stmt.borrow().child[1].clone();
                let t = self.gen_code_from_exp(expression);
                let code = self.gen_declaration(&var, &t);
                self.add_code(code);
            }

        } else {
            self.gen_code_from_call(var_or_call.clone());
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
use crate::analyzer::{Symbol, SymbolTable, SymbolTableManager};
use crate::analyzer::analyze::Analyzer;
use crate::ast::lexer::{Lexer, Terminals, Token};
use crate::ast::parser::Parser;
use crate::ast::{ExpKind, ExpType, NonTerminals, TreeNode};
use crate::ast::StmtKind::VarDecK;
use crate::code_gen::{CodeGen, StatementKind};
use crate::code_gen::target_code_gen::Masm;

pub struct LexerPrinter {
    pub tokens: Vec<Token>,
}

impl LexerPrinter {
    pub fn new(content: String) -> Self {
        let mut lexer = Lexer::new(content.clone());
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            if token.kind == Terminals::Whitespace {
                continue;
            }
            if token.kind == Terminals::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        Self {
            tokens,
        }
    }

    pub fn print(&self){
        for token in self.tokens.iter() {
            println!("{:?}", token);
        }
    }
}

pub struct ParserPrinter {
    pub lexer_printer: LexerPrinter,
    pub parser: Parser
}

impl ParserPrinter {
    pub fn new(content: String) -> Self {
        let lexer_printer = LexerPrinter::new(content);
        let mut parser = Parser::new(lexer_printer.tokens.clone());
        parser.parse();
        parser.normal_parse();
        Self {
            lexer_printer,
            parser,
        }
    }

    pub fn print(&mut self) {
        let tree = self.parser.stack.tree_root.clone();
        self.print_tree(tree, 0);
    }

    pub fn print_indent(&mut self, indent: usize) {
        for _ in 0..indent {
            print!(" ");
        }
    }

    fn print_tree(&mut self, tree: Rc<RefCell<TreeNode>>, indent: usize) {
        println!("Program 程序开始");
        let declaration_list = tree.borrow_mut().child[0].clone();
        for declaration in declaration_list.borrow_mut().child.iter() {
            self.print_declaration(declaration.clone(), indent + 4);
        }
    }

    fn print_declaration(&mut self, declaration: Rc<RefCell<TreeNode>>, indent: usize) {
        if declaration.borrow().kind.stmt == VarDecK {
            self.print_indent(indent);
            println!("VarDecK 变量声明");
            let type_ = declaration.borrow_mut().child[0].clone();
            let var = declaration.borrow_mut().child[1].clone();
            self.print_indent(indent + 4);
            print!("TypeK 类型 " );
            println!("{}", type_.borrow().child[0].borrow().terminal);
            let kind = var.borrow().clone().kind.exp;
            match kind {
                ExpKind::IdK => {
                    self.print_indent(indent + 4);
                    print!("IDK 变量名 ");
                    println!("{}", var.borrow().child[0].borrow().terminal);
                }
                ExpKind::ArrayK {size, dimensions} => {
                    self.print_indent(indent + 4);
                    println!("ArrayK 变量数组");
                    self.print_indent(indent + 8);
                    println!("size: {}", size);
                    let terminals = var.borrow().clone().child[0].borrow().clone().terminal;
                    let mut s = format!("{}", terminals);
                    for i in dimensions.iter() {
                        s = s + &format!("[{}]", i);
                    }
                    self.print_indent(indent + 8);
                    println!("{}", s);
                }
                _ => {}
            }
            if declaration.borrow().child.len() > 2 {
                self.print_indent(indent + 4);
                println!("Equals 赋值");
                let exp = declaration.borrow_mut().child[2].clone();
                self.print_exp(exp, indent + 4);
            }
        } else {
            self.print_indent(indent);
            println!("FunDecK 函数声明");
            let type_ = declaration.borrow_mut().child[0].clone();
            let fun = declaration.borrow_mut().child[1].clone();
            self.print_indent(indent + 4);
            print!("TypeK 返回值类型 ");
            println!("{}", type_.borrow().child[0].borrow().terminal);
            self.print_indent(indent + 4);
            print!("函数名 ");
            println!("{}", fun.borrow().terminal);
            self.print_indent(indent + 4);
            println!("Params 参数列表");
            let params = declaration.borrow().child[2].clone();
            for param in params.borrow().child.iter() {
                self.print_param(param.clone(), indent + 8);
            }
            self.print_indent(indent + 4);
            println!("CompoundStmt 函数体");
            self.print_compound_stmt(declaration.borrow_mut().child[3].clone(), indent + 8);
        }
    }

    fn print_param(&mut self, param: Rc<RefCell<TreeNode>>, indent: usize) {
        self.print_indent(indent);
        println!("ParamK 参数");
        if param.borrow().non_terminal == NonTerminals::Param {
            let type_ = param.borrow_mut().child[0].clone();
            let var = param.borrow_mut().child[1].clone();
            self.print_indent(indent + 4);
            print!("TypeK 类型 ");
            println!("{}", type_.borrow().child[0].borrow().terminal);
            let kind = var.borrow().clone().kind.exp;
            match kind {
                ExpKind::IdK => {
                    self.print_indent(indent + 4);
                    print!("IDK 变量名 ");
                    println!("{}", var.borrow().child[0].borrow().terminal);
                }
                ExpKind::ArrayK {size, dimensions} => {
                    self.print_indent(indent + 4);
                    println!("ArrayK 变量数组");
                    self.print_indent(indent + 8);
                    println!("size: {}", size);
                    let terminals = var.borrow().clone().child[0].borrow().clone().terminal;
                    let mut s = format!("{}", terminals);
                    for i in dimensions.iter() {
                        s = s + &format!("[{}]", i);
                    }
                    self.print_indent(indent + 8);
                    println!("{}", s);
                }
                _ => {}
            }
        } else {
            self.print_indent(indent + 4);
            print!("TypeK 类型 ");
            println!("{}", param.borrow().value);
        }
    }

    fn print_compound_stmt(&mut self, compound_stmt: Rc<RefCell<TreeNode>>, indent: usize) {
        self.print_indent(indent);
        println!("CompoundStmt 复合语句");
        let statement_list = compound_stmt.borrow().child[0].clone();
        for statement in statement_list.borrow().child.iter() {
            self.print_statement(statement.clone(), indent + 8);
        }
    }

    fn print_statement_list(&mut self, statement_list: Rc<RefCell<TreeNode>>, indent: usize) {
        for statement in statement_list.borrow().child.iter() {
            self.print_statement(statement.clone(), indent + 8);
        }
    }

    fn print_statement(&mut self, statement: Rc<RefCell<TreeNode>>, indent: usize) {
        let non_terminal = statement.borrow().child[0].borrow().clone().non_terminal;
        match non_terminal {
            NonTerminals::LocalDeclarations => {
                self.print_indent(indent);
                println!("LocalDeclarations 局部变量声明");
                let local_declarations = statement.borrow_mut().child[0].clone();
                let type_ = local_declarations.borrow_mut().child[0].clone();
                let var = local_declarations.borrow_mut().child[1].clone();
                self.print_indent(indent + 4);
                print!("TypeK 类型 ");
                println!("{}", type_.borrow().child[0].borrow().terminal);
                let kind = var.borrow().clone().kind.exp;
                match kind {
                    ExpKind::IdK => {
                        self.print_indent(indent + 4);
                        print!("IDK 变量名 ");
                        println!("{}", var.borrow().child[0].borrow().terminal);
                    }
                    ExpKind::ArrayK {size, dimensions} => {
                        self.print_indent(indent + 4);
                        println!("ArrayK 变量数组");
                        self.print_indent(indent + 8);
                        println!("size: {}", size);
                        let terminals = var.borrow().clone().child[0].borrow().clone().terminal;
                        let mut s = format!("{}", terminals);
                        for i in dimensions.iter() {
                            s = s + &format!("[{}]", i);
                        }
                        self.print_indent(indent + 8);
                        println!("{}", s);
                    }
                    _ => {}
                }
                if local_declarations.borrow().child.len() > 2 {
                    self.print_indent(indent + 4);
                    println!("Equals 赋值");
                    let exp = local_declarations.borrow_mut().child[2].clone();
                    self.print_exp(exp, indent + 4);
                }
            }
            NonTerminals::IfStmt => {
                let if_stmt = statement.borrow_mut().child[0].clone();
                self.print_indent(indent);
                println!("IfStmt if语句");
                let exp = if_stmt.borrow_mut().child[0].clone();
                self.print_indent(indent + 4);
                println!("条件");
                self.print_exp(exp, indent + 4);
                let compound_stmt = if_stmt.borrow_mut().child[1].clone();
                self.print_indent(indent + 4);
                println!("if语句体");
                self.print_compound_stmt(compound_stmt, indent + 4);
                let else_stmt_n = if_stmt.borrow_mut().child[2].clone();
                if else_stmt_n.borrow().child.len() > 0 {
                    let else_stmt = else_stmt_n.borrow_mut().child[0].clone();
                    self.print_indent(indent + 4);
                    println!("else语句体");
                    self.print_compound_stmt(else_stmt, indent + 4);
                }
            }
            NonTerminals::WhileStmt => {
                let while_stmt = statement.borrow_mut().child[0].clone();
                self.print_indent(indent);
                println!("WhileStmt while语句");
                let exp = while_stmt.borrow_mut().child[0].clone();
                self.print_indent(indent + 4);
                println!("条件");
                self.print_exp(exp, indent + 4);
                let compound_stmt = while_stmt.borrow_mut().child[1].clone();
                self.print_indent(indent + 4);
                println!("while语句体");
                self.print_compound_stmt(compound_stmt, indent + 4);
            }
            NonTerminals::ForStmt => {
                let for_stmt = statement.borrow_mut().child[0].clone();
                let for_exp = for_stmt.borrow_mut().child[0].clone();
                self.print_indent(indent);
                println!("ForStmt for语句");
                let exp1 = for_exp.borrow_mut().child[0].clone();
                if exp1.borrow().child.len() > 0 {
                    self.print_indent(indent + 4);
                    println!("前置操作");
                    self.print_var_or_call_stmt(exp1.borrow().child[0].clone(), indent + 4);
                }else {
                    self.print_indent(indent + 4);
                    println!("前置操作: 空");
                }
                let exp2 = for_exp.borrow_mut().child[1].clone();
                self.print_indent(indent + 4);
                println!("循环条件");
                self.print_exp(exp2, indent + 4);
                let exp3 = for_exp.borrow_mut().child[2].clone();
                if exp3.borrow().child.len() > 0 {
                    self.print_indent(indent + 4);
                    println!("后置操作");
                    self.print_var_or_call_stmt(exp3.borrow().child[0].clone(), indent + 4);
                }else {
                    self.print_indent(indent + 4);
                    println!("后置操作: 空");
                }
                let compound_stmt = for_stmt.borrow_mut().child[1].clone();
                self.print_indent(indent + 4);
                println!("for语句体");
                self.print_compound_stmt(compound_stmt, indent + 4);
            }
            NonTerminals::DoStmt => {
                let do_stmt = statement.borrow_mut().child[0].clone();
                self.print_indent(indent);
                println!("DoStmt do语句");
                let compound_stmt = do_stmt.borrow_mut().child[0].clone();
                self.print_indent(indent + 4);
                println!("do语句体");
                self.print_compound_stmt(compound_stmt, indent + 4);
                let exp = do_stmt.borrow_mut().child[1].clone();
                self.print_indent(indent + 4);
                println!("条件");
                self.print_exp(exp, indent + 4);
            }
            NonTerminals::BreakStmt => {
                self.print_indent(indent);
                println!("BreakStmt break语句");
            }
            NonTerminals::ContinueStmt => {
                self.print_indent(indent);
                println!("ContinueStmt continue语句");
            }
            NonTerminals::SwitchStmt => {
                let switch_stmt = statement.borrow_mut().child[0].clone();
                self.print_indent(indent);
                println!("SwitchStmt switch语句");
                let exp = switch_stmt.borrow_mut().child[0].clone();
                self.print_indent(indent + 4);
                println!("条件");
                self.print_exp(exp, indent + 4);
                let case_default_stmt = switch_stmt.borrow_mut().child[1].clone();
                let case_stmt_list = case_default_stmt.borrow_mut().child[0].clone();
                self.print_indent(indent + 4);
                println!("case语句列表");
                for case_stmt in case_stmt_list.borrow_mut().child.iter() {
                    self.print_case_stmt(case_stmt.clone(), indent + 8);
                }
                let default_stmt = case_default_stmt.borrow_mut().child[1].clone();
                if default_stmt.borrow().child.len() > 0 {
                    self.print_indent(indent + 4);
                    println!("default语句");
                    self.print_case_stmt(default_stmt, indent + 8);
                }
            }
            NonTerminals::ReturnStmt => {
                self.print_indent(indent);
                println!("ReturnStmt 返回语句");
                let return_stmt = statement.borrow_mut().child[0].clone();
                let exp = return_stmt.borrow_mut().child[0].clone();
                self.print_exp(exp, indent + 4);
            }
            NonTerminals::VarOrCallStmt => {
                self.print_indent(indent);
                println!("VarOrCallStmt 变量或函数调用");
                let var_or_call_stmt = statement.borrow_mut().child[0].clone();
                let kind = var_or_call_stmt.borrow().clone().kind.exp;
                match kind {
                    ExpKind::IdK => {
                        let var = var_or_call_stmt.borrow_mut().child[0].clone();
                        let kind = var.borrow().clone().kind.exp;
                        match kind {
                            ExpKind::IdK => {
                                self.print_indent(indent + 4);
                                print!("IDK 变量名 ");
                                println!("{}", var.borrow().child[0].borrow().terminal);
                            }
                            ExpKind::ArrayK {size, dimensions} => {
                                self.print_indent(indent + 4);
                                println!("ArrayK 变量数组");
                                self.print_indent(indent + 8);
                                println!("size: {}", size);
                                let terminals = var.borrow().clone().child[0].borrow().clone().terminal;
                                let mut s = format!("{}", terminals);
                                for i in dimensions.iter() {
                                    s = s + &format!("[{}]", i);
                                }
                                self.print_indent(indent + 8);
                                println!("{}", s);
                            }
                            _ => {}
                        }
                        if var_or_call_stmt.borrow().child.len() > 1 {
                            let var_stmt = var_or_call_stmt.borrow_mut().child[1].clone();
                            if var_stmt.borrow().non_terminal == NonTerminals::SelfOp {
                                self.print_indent(indent + 4);
                                print!("SelfOp 自增自减 ");
                                println!("{}", var_stmt.borrow().child[0].borrow().terminal);
                            }else{
                                self.print_indent(indent + 4);
                                println!("Equals 赋值");
                                let exp = var_or_call_stmt.borrow_mut().child[1].clone();
                                self.print_exp(exp, indent + 4);
                            }
                        }
                    }
                    ExpKind::CallK => {
                        self.print_indent(indent + 4);
                        let call_stmt = var_or_call_stmt.borrow_mut().child[0].clone();
                        println!("CallK 函数调用");
                        self.print_indent(indent + 8);
                        println!("函数名 ");
                        self.print_indent(indent + 8);
                        println!("{}", call_stmt.borrow().child[0].borrow().terminal);
                        self.print_indent(indent + 8);
                        println!("参数列表");
                        let args = call_stmt.borrow_mut().child[1].clone();
                        for arg in args.borrow().child.iter() {
                            self.print_exp(arg.clone(), indent + 12);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn print_case_stmt(&mut self, case_stmt: Rc<RefCell<TreeNode>>, indent: usize) {
        let non_terminal = case_stmt.borrow().clone().non_terminal;
        match non_terminal {
            NonTerminals::CaseStmt => {
                self.print_indent(indent);
                println!("CaseStmt case语句");
                let exp = case_stmt.borrow_mut().child[0].clone();
                self.print_indent(indent + 4);
                println!("条件");
                self.print_exp(exp, indent + 4);
                let statement_list = case_stmt.borrow_mut().child[1].clone();
                self.print_indent(indent + 4);
                println!("case语句体");
                self.print_statement_list(statement_list, indent + 4);
            }
            NonTerminals::DefaultStmt => {
                let default_stmt = case_stmt;
                self.print_indent(indent);
                println!("DefaultStmt default语句");
                let statement_list = default_stmt.borrow_mut().child[0].clone();
                self.print_indent(indent + 4);
                println!("default语句体");
                self.print_statement_list(statement_list, indent + 4);
            }
            _ => {}
        }
    }

    fn print_exp(&mut self, exp: Rc<RefCell<TreeNode>>, indent: usize) {
        for child in exp.borrow_mut().child.iter() {
            if child.borrow().non_terminal == NonTerminals::AdditiveExpression {
                self.print_additive_exp(child.clone(), indent);
            } else {
                self.print_indent(indent);
                print!("logical_op 逻辑或运算符 ");
                println!("{}", child.borrow().child[0].borrow().terminal);
            }
        }
    }

    fn print_additive_exp(&mut self, additive_exp: Rc<RefCell<TreeNode>>, indent: usize) {
        for child in additive_exp.borrow_mut().child.iter() {
            if child.borrow().non_terminal == NonTerminals::SimpleExpression {
                self.print_simple_exp(child.clone(), indent);
            } else {
                self.print_indent(indent);
                print!("relational_op 关系运算符 ");
                println!("{}", child.borrow().child[0].borrow().terminal);
            }
        }
    }

    fn print_simple_exp(&mut self, simple_exp: Rc<RefCell<TreeNode>>, indent: usize) {
        for child in simple_exp.borrow_mut().child.iter() {
            if child.borrow().non_terminal == NonTerminals::Term {
                self.print_term(child.clone(), indent);
            } else {
                self.print_indent(indent);
                print!("add_op 加减运算符 ");
                println!("{}", child.borrow().child[0].borrow().terminal);
            }
        }
    }

    fn print_term(&mut self, term: Rc<RefCell<TreeNode>>, indent: usize) {
        for child in term.borrow_mut().child.iter() {
            if child.borrow().non_terminal == NonTerminals::Factor {
                self.print_factor(child.clone(), indent);
            } else {
                self.print_indent(indent);
                print!("mul_op 乘法运算符 ");
                println!("{}", child.borrow().child[0].borrow().terminal);
            }
        }
    }

    fn print_factor(&mut self, factor: Rc<RefCell<TreeNode>>, indent: usize) {
        if factor.borrow().child[0].borrow().non_terminal == NonTerminals::Expression {
            self.print_exp(factor.borrow().child[0].clone(), indent);
        } else if factor.borrow().child[0].borrow().non_terminal == NonTerminals::Var {
            let var = factor.borrow().child[0].clone();
            let kind = var.borrow().clone().kind.exp;
            match kind {
                ExpKind::IdK => {
                    self.print_indent(indent);
                    print!("IDK 变量名 ");
                    println!("{}", var.borrow().child[0].borrow().terminal);
                }
                ExpKind::ArrayK {size, dimensions} => {
                    self.print_indent(indent);
                    println!("ArrayK 变量数组");
                    self.print_indent(indent + 4);
                    println!("size: {}", size);
                    let terminals = var.borrow().clone().child[0].borrow().clone().terminal;
                    let mut s = format!("{}", terminals);
                    for i in dimensions.iter() {
                        s = s + &format!("[{}]", i);
                    }
                    self.print_indent(indent + 4);
                    println!("{}", s);
                }
                _ => {}
            }
        } else if factor.borrow().child[0].borrow().non_terminal == NonTerminals::CallStmt {
            let call_stmt = factor.borrow().child[0].clone();
            self.print_indent(indent);
            println!("CallK 函数调用");
            self.print_indent(indent + 4);
            print!("函数名 ");
            println!("{}", call_stmt.borrow().child[0].borrow().terminal);
            self.print_indent(indent + 4);
            println!("参数列表");
            let args = call_stmt.borrow().child[1].clone();
            for arg in args.borrow().child.iter() {
                self.print_exp(arg.clone(), indent + 8);
            }
        } else {
            let value = factor.borrow().child[0].clone();
            self.print_indent(indent);
            println!("{}", value.borrow().child[0].borrow().terminal);
        }
    }

    fn print_var_or_call_stmt(&mut self, var_or_call_stmt: Rc<RefCell<TreeNode>>, indent: usize) {
        let kind = var_or_call_stmt.borrow().clone().kind.exp;
        match kind {
            ExpKind::IdK => {
                let var = var_or_call_stmt.borrow_mut().child[0].clone();
                let kind = var.borrow().clone().kind.exp;
                match kind {
                    ExpKind::IdK => {
                        self.print_indent(indent + 4);
                        print!("IDK 变量名 ");
                        println!("{}", var.borrow().child[0].borrow().terminal);
                    }
                    ExpKind::ArrayK {size, dimensions} => {
                        self.print_indent(indent + 4);
                        println!("ArrayK 变量数组");
                        self.print_indent(indent + 8);
                        println!("size: {}", size);
                        let terminals = var.borrow().clone().child[0].borrow().clone().terminal;
                        let mut s = format!("{}", terminals);
                        for i in dimensions.iter() {
                            s = s + &format!("[{}]", i);
                        }
                        self.print_indent(indent + 8);
                        println!("{}", s);
                    }
                    _ => {}
                }
                if var_or_call_stmt.borrow().child.len() > 1 {
                    let var_stmt = var_or_call_stmt.borrow_mut().child[1].clone();
                    if var_stmt.borrow().non_terminal == NonTerminals::SelfOp {
                        self.print_indent(indent + 4);
                        print!("SelfOp 自增自减 ");
                        println!("{}", var_stmt.borrow().child[0].borrow().terminal);
                    }else{
                        self.print_indent(indent + 4);
                        println!("Equals 赋值");
                        let exp = var_or_call_stmt.borrow_mut().child[1].clone();
                        self.print_exp(exp, indent + 4);
                    }
                }
            }
            ExpKind::CallK => {
                self.print_indent(indent + 4);
                println!("CallK 函数调用");
                let fun = var_or_call_stmt.borrow_mut().child[0].clone();
                self.print_indent(indent + 8);
                println!("函数名 ");
                println!("{}", fun.borrow().child[1].borrow().terminal);
                self.print_indent(indent + 8);
                println!("参数列表");
                let args = var_or_call_stmt.borrow_mut().child[2].clone();
                for arg in args.borrow().child.iter() {
                    self.print_exp(arg.clone(), indent + 12);
                }
            }
            _ => {}
        }
    }
}

pub struct SymbolTablePrinter {
    pub analyzer: Rc<RefCell<Analyzer>>,
}

impl  SymbolTablePrinter  {
    pub fn new(analyzer: Rc<RefCell<Analyzer>>) -> Self {
        Self {
            analyzer,
        }
    }

    pub fn print(&mut self) {
        println!("SymbolTable 符号表");
        let global_table = self.analyzer.borrow().symbol_table_manager.global_table.clone();
        self.print_table(global_table);
        let tables = self.analyzer.borrow().symbol_table_manager.tables.clone();
        for table in tables.iter() {
            println!();
            self.print_table(table.clone());
        }
    }

    fn print_table(&mut self, table: Rc<RefCell<SymbolTable>>) {
        if table.borrow().func_name != "global" {
            println!("{} 函数符号表", table.borrow().func_name);
        } else {
            println!("global 符号表");
        }
        println!("{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}{:<15}","索引","名字","别名","层号","偏移","标记","类型");
        let symbols = table.borrow().symbols.clone();
        let mut i: usize = 0;
        for symbol in symbols.iter() {
            self.print_symbol(symbol.clone(), i);
            i += 1;
        }
    }

    fn print_symbol(&mut self, symbol: Rc<RefCell<Symbol>>, index: usize) {
        let name = symbol.borrow().name.clone();
        let mut alias = symbol.borrow().alias.clone();
        let level = symbol.borrow().level;
        let offset = symbol.borrow().offset;
        let flag = symbol.borrow().flag;
        if alias == "" &&(flag == 'v'||flag== 'p'||flag == 't') {
            let string = self.analyzer.borrow_mut().new_var();
            alias = string.clone();
            symbol.borrow_mut().alias = string;
        }
        let kind = symbol.borrow().type_.clone();
        let mut kind_str = String::new();
        match kind {
            ExpType::Void => {
                kind_str = "void".to_string();
            }
            ExpType::Integer => {
                kind_str = "int".to_string();
            }
            ExpType::String => {
                kind_str = "string".to_string();
            }
            ExpType::Char => {
                kind_str = "char".to_string();
            }
            ExpType::Boolean => {
                kind_str = "boolean".to_string();
            }
            ExpType::Float => {
                kind_str = "float".to_string();
            }
        }
        let s = format!("{:<16}{:<16}{:<16}{:<16}{:<16}{:<16}{:<16}", index, name, alias, level, offset, flag, kind_str);
        println!("{}", s)
    }
}

pub struct ThreeAddressCodePrinter {
    pub code_generator: CodeGen,
}

impl ThreeAddressCodePrinter {
    pub fn new(analyzer: Rc<RefCell<Analyzer>>) -> Self {
        let mut code_generator = CodeGen::new(analyzer);
        code_generator.gen_code();
        Self {
            code_generator,
        }
    }

    pub fn print(&mut self) {
        let codes = &self.code_generator.codes;
        for code in codes.iter() {
            println!("{}", code.raw);
        }
    }
}

pub struct MasmPrinter {
    pub masm: Masm,
}

impl MasmPrinter {
    pub fn new(mut masm: Masm) -> Self {
        masm.initialize();
        Self {
            masm,
        }
    }

    pub fn print(&mut self) {
        let codes = &self.masm.codes;
        for code in codes.iter() {
            println!("{}", code);
        }
    }
}

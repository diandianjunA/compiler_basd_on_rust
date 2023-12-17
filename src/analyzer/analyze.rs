use std::cell::RefCell;
use std::rc::Rc;
use crate::analyzer::{Symbol, SymbolTable, SymbolTableManager};
use crate::ast::printer::ParserPrinter;
use crate::ast::StmtKind::VarDecK;
use crate::ast::{ExpKind, ExpType, NonTerminals, TreeNode};
use crate::ast::lexer::Terminals;

pub struct Analyzer {
    pub symbol_table_manager: SymbolTableManager,
    pub current_table: Rc<RefCell<SymbolTable>>,
    pub current_level: i32,
    pub current_offset: i32,
    pub data_offset: i32,
    pub var_index: usize,
    pub error: bool,
    pub root: Rc<RefCell<TreeNode>>,
    pub last_line: usize,
}

impl Analyzer {
    pub fn new(root: Rc<RefCell<TreeNode>>, last_line: usize) -> Self {
        let global_table = Rc::new(RefCell::new(SymbolTable::new(0, "global".to_string())));
        let global_table_clone = global_table.clone();
        Self {
            symbol_table_manager: SymbolTableManager {
                tables: vec![],
                global_table,
            },
            current_table: global_table_clone,
            current_level: 0,
            current_offset: 0,
            data_offset: 0,
            var_index: 0,
            error: false,
            root,
            last_line,
        }
    }

    pub fn enter_scope(&mut self, func_name: String) {
        self.current_table.borrow_mut().level = self.current_level;
        self.current_table.borrow_mut().offset = self.current_offset;
        let table = Rc::new(RefCell::new(SymbolTable::new(self.current_level + 1, func_name)));
        self.symbol_table_manager.tables.push(table.clone());
        self.current_table = table.clone();
        self.current_level += 1;
        self.current_offset = 0;
    }

    pub fn exit_scope(&mut self) {
        self.current_level -= 1;
        self.current_table = self.symbol_table_manager.global_table.clone();
        self.current_offset = 0;
    }

    pub fn new_var(&mut self) -> String {
        let var = format!("V{}", self.var_index);
        self.var_index += 1;
        var
    }

    pub fn analyze(&mut self) {
        self.analyze_tree();
    }

    fn analyze_tree(&mut self) {
        let declaration_list = self.root.borrow_mut().child[0].clone();
        for declaration in declaration_list.borrow_mut().child.iter() {
            self.analyze_declaration(declaration.clone());
        }
        for table in self.symbol_table_manager.tables.iter() {
            if table.borrow().is_main {
                return;
            }
        }
        println!("第 {} 行: 没有main函数", self.last_line);
        self.error = true;
    }

    fn analyze_declaration(&mut self, declaration: Rc<RefCell<TreeNode>>) {
        if declaration.borrow().kind.stmt == VarDecK {
            let current_table = self.current_table.clone();
            let type_ = declaration.borrow_mut().child[0].clone();
            let var = declaration.borrow_mut().child[1].clone();
            let var_terminal = var.borrow().child[0].borrow().terminal.clone();
            let mut var_name = "".to_string();
            match var_terminal {
                Terminals::Identifier(i) => {
                    for symbol in current_table.borrow().symbols.iter() {
                        if symbol.borrow().name == i && symbol.borrow().level == self.current_level {
                            println!("第 {} 行: 变量重复定义", var.borrow().child[0].borrow().line);
                            self.error = true;
                        }
                    }
                    var_name = i.clone();
                }
                _ => {
                    println!("第 {} 行: 变量名不合法", var.borrow().child[0].borrow().line);
                    self.error = true;
                }
            }
            let kind = var.borrow().clone().kind.exp;
            let mut type_kind = ExpType::Void;
            if type_.borrow().child[0].borrow().terminal == Terminals::Int {
                type_kind = ExpType::Integer;
            } else if type_.borrow().child[0].borrow().terminal == Terminals::Float {
                type_kind = ExpType::Float;
            } else if type_.borrow().child[0].borrow().terminal == Terminals::Char {
                type_kind = ExpType::Char;
            } else {
                println!("第 {} 行: 变量类型不合法", type_.borrow().child[0].borrow().line);
                self.error = true;
            }
            match kind.clone() {
                ExpKind::IdK => {
                    let symbol = Rc::new(RefCell::new(
                        Symbol::new(var_name, self.current_level
                                    , type_kind.clone()
                                    , self.data_offset
                                    , 'v', 0, None, self.new_var())));
                    symbol.borrow_mut().index = self.var_index - 1;
                    self.current_table.borrow_mut().symbols.push(symbol.clone());
                    self.data_offset += 2;
                }
                ExpKind::ArrayK { size, dimensions } => {
                    for i in dimensions.iter() {
                        if *i <= 0 {
                            println!("第 {} 行: 数组长度必须大于0", var.borrow().child[0].borrow().line);
                            self.error = true;
                        }
                    }
                    let mut temp_dimensions = vec![0; size];
                    let i = dimensions.len() - 1;
                    loop {
                        let mut t_name = var_name.clone();
                        for i in temp_dimensions.iter() {
                            t_name = t_name + &format!("[{}]", i);
                        }
                        let symbol = Rc::new(RefCell::new(
                            Symbol::new(t_name, self.current_level
                                        , type_kind.clone()
                                        , self.current_offset
                                        , 't', 0, Some(dimensions.clone()), self.new_var())));
                        symbol.borrow_mut().index = self.var_index - 1;
                        self.current_table.borrow_mut().symbols.push(symbol.clone());
                        self.current_offset += 2;
                        temp_dimensions[i] += 1;
                        if temp_dimensions[i] >= dimensions[i] - 1 {
                            let mut j = i;
                            loop {
                                if j == 0 {
                                    break;
                                }
                                if temp_dimensions[j] == dimensions[j] {
                                    temp_dimensions[j] = 0;
                                    temp_dimensions[j - 1] += 1;
                                }
                                j -= 1;
                            }
                            if temp_dimensions[0] == dimensions[0] {
                                break;
                            }
                        }
                    }
                }
                _ => {
                    println!("第 {} 行: 变量定义错误", var.borrow().child[0].borrow().line);
                    self.error = true;
                }
            }
            if declaration.borrow().child.len() == 3 {
                let exp = declaration.borrow_mut().child[2].clone();
                let exp_type = self.analyze_exp(exp.clone());
                if exp_type != type_kind {
                    if exp_type == ExpType::String && type_kind == ExpType::Char { // 字符串赋值给字符数组
                        match kind {
                            ExpKind::ArrayK { size: _size, dimensions } => {
                                let e = exp.clone();
                                let terminals = e.borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().terminal.clone();
                                match terminals {
                                    Terminals::String(s) => {
                                        if dimensions.len() == 1 {
                                            if s.len() > dimensions[0] {
                                                println!("第 {} 行: 字符串长度超过数组长度", exp.borrow().child[0].borrow().line);
                                                self.error = true;
                                            } else {
                                                return;
                                            }
                                        } else {
                                            println!("第 {} 行: 不能将字符串赋给一维以上的字符数组", exp.borrow().child[0].borrow().line);
                                            self.error = true;
                                        }
                                    }
                                    _ => {
                                        println!("第 {} 行: 表达式类型不匹配，变量类型为 {}，表达式类型为 {}", var.borrow().child[0].borrow().line, type_kind, exp_type);
                                        self.error = true;
                                    }
                                }
                            }
                            _ => {
                                println!("第 {} 行: 表达式类型不匹配，变量类型为 {}，表达式类型为 {}", var.borrow().child[0].borrow().line, type_kind, exp_type);
                                self.error = true;
                            }
                        }
                    }
                    println!("第 {} 行: 表达式类型不匹配，变量类型为 {}，表达式类型为 {}", var.borrow().child[0].borrow().line, type_kind, exp_type);
                    self.error = true;
                }
            }
        } else {
            let func_name_terminal = declaration.borrow_mut().child[1].borrow().terminal.clone();
            let mut func_name = "".to_string();
            let current_table = self.current_table.clone();
            match func_name_terminal {
                Terminals::Identifier(i) => {
                    for symbol in current_table.borrow().symbols.iter() {
                        if symbol.borrow().name == i && symbol.borrow().level == self.current_level {
                            println!("第 {} 行: 函数名重复定义", declaration.borrow().child[1].borrow().line);
                            self.error = true;
                        }
                    }
                    func_name = i.clone();
                }
                _ => {
                    println!("第 {} 行: 函数名不合法", declaration.borrow().child[1].borrow().line);
                    self.error = true;
                }
            }
            let mut type_kind = ExpType::Void;
            let type_ = declaration.borrow_mut().child[0].clone();
            if type_.borrow().child[0].borrow().terminal == Terminals::Int {
                type_kind = ExpType::Integer;
            } else if type_.borrow().child[0].borrow().terminal == Terminals::Float {
                type_kind = ExpType::Float;
            } else if type_.borrow().child[0].borrow().terminal == Terminals::Char {
                type_kind = ExpType::Char;
            } else {
                println!("第 {} 行: 变量类型不合法", type_.borrow().child[0].borrow().line);
                self.error = true;
            }
            let symbol = Rc::new(RefCell::new(
                Symbol::new(func_name.clone(), self.current_level
                            , type_kind
                            , 0
                            , 'f', 0, None, "".to_string())));
            self.symbol_table_manager.global_table.borrow_mut().symbols.push(symbol.clone());
            self.enter_scope(func_name.clone());
            let mut params = vec![];
            let param_list = declaration.borrow_mut().child[2].clone();
            for param in param_list.borrow().child.iter() {
                if param.borrow().terminal == Terminals::Void {
                    break;
                }
                let type_ = param.borrow_mut().child[0].clone();
                let var = param.borrow_mut().child[1].clone();
                let var_terminal = var.borrow().child[0].borrow().terminal.clone();
                let mut var_name = "".to_string();
                match var_terminal {
                    Terminals::Identifier(i) => {
                        var_name = i.clone();
                    }
                    _ => {
                        println!("第 {} 行: 变量名不合法", var.borrow().child[0].borrow().line);
                        self.error = true;
                    }
                }
                let kind = var.borrow().clone().kind.exp;
                let mut type_kind = ExpType::Void;
                if type_.borrow().child[0].borrow().terminal == Terminals::Int {
                    type_kind = ExpType::Integer;
                } else if type_.borrow().child[0].borrow().terminal == Terminals::Float {
                    type_kind = ExpType::Float;
                } else if type_.borrow().child[0].borrow().terminal == Terminals::Char {
                    type_kind = ExpType::Char;
                } else {
                    println!("第 {} 行: 变量类型不合法", type_.borrow().child[0].borrow().line);
                    self.error = true;
                }
                match kind {
                    ExpKind::IdK => {
                        let symbol = Rc::new(RefCell::new(
                            Symbol::new(var_name, 0
                                        , type_kind
                                        , self.current_offset
                                        , 'p', 0, None, self.new_var())));
                        symbol.borrow_mut().index = self.var_index - 1;
                        params.push(symbol.clone());
                        self.current_offset += 2;
                    }
                    ExpKind::ArrayK { size, dimensions } => {
                        for i in dimensions.iter() {
                            if *i <= 0 {
                                println!("第 {} 行: 数组长度必须大于0", var.borrow().child[0].borrow().line);
                                self.error = true;
                            }
                        }
                        let mut temp_dimensions = vec![0; size];
                        let i = dimensions.len() - 1;
                        loop {
                            let mut t_name = var_name.clone();
                            for i in temp_dimensions.iter() {
                                t_name = t_name + &format!("[{}]", i);
                            }
                            let symbol = Rc::new(RefCell::new(
                                Symbol::new(t_name, self.current_level
                                            , type_kind.clone()
                                            , self.current_offset
                                            , 't', 0, Some(dimensions.clone()), self.new_var())));
                            symbol.borrow_mut().index = self.var_index - 1;
                            self.current_table.borrow_mut().symbols.push(symbol.clone());
                            self.current_offset += 2;
                            temp_dimensions[i] += 1;
                            if temp_dimensions[i] >= dimensions[i] - 1 {
                                let mut j = i;
                                loop {
                                    if j == 0 {
                                        break;
                                    }
                                    if temp_dimensions[j] == dimensions[j] {
                                        temp_dimensions[j] = 0;
                                        temp_dimensions[j - 1] += 1;
                                    }
                                    j -= 1;
                                }
                                if temp_dimensions[0] == dimensions[0] {
                                    break;
                                }
                            }
                        }
                    }
                    _ => {
                        println!("第 {} 行: 变量定义错误", var.borrow().child[0].borrow().line);
                        self.error = true;
                        return;
                    }
                }
            }
            if func_name == "main" {
                self.symbol_table_manager.get_current().borrow_mut().is_main = true;
                if params.len() != 0 {
                    println!("第 {} 行: main函数不能有参数", declaration.borrow().child[1].borrow().line);
                    self.error = true;
                }
            }
            if params.len() > 8 {
                println!("第 {} 行: 参数数目不能超过8个", declaration.borrow().child[1].borrow().line);
                self.error = true;
            }
            for param in params.iter() {
                self.current_table.borrow_mut().symbols.push(param.clone());
            }
            symbol.borrow_mut().param_num = params.len() as i32;
            let compound_stmt = declaration.borrow_mut().child[3].clone();
            self.analyze_compound_stmt(compound_stmt);
            self.exit_scope();
        }
    }

    fn analyze_compound_stmt(&mut self, compound_stmt: Rc<RefCell<TreeNode>>) {
        let statement_list = compound_stmt.borrow().child[0].clone();
        for statement in statement_list.borrow().child.iter() {
            self.analyze_statement(statement.clone());
        }
    }

    fn analyze_statement_list(&mut self, statement_list: Rc<RefCell<TreeNode>>) {
        for statement in statement_list.borrow().child.iter() {
            self.analyze_statement(statement.clone());
        }
    }

    fn analyze_statement(&mut self, statement: Rc<RefCell<TreeNode>>) {
        let non_terminal = statement.borrow().child[0].borrow().clone().non_terminal;
        match non_terminal {
            NonTerminals::LocalDeclarations => {
                let local_declarations = statement.borrow().child[0].clone();
                let current_table = self.current_table.clone();
                let type_ = local_declarations.borrow_mut().child[0].clone();
                let var = local_declarations.borrow_mut().child[1].clone();
                let var_terminal = var.borrow().child[0].borrow().terminal.clone();
                let mut var_name = "".to_string();
                match var_terminal {
                    Terminals::Identifier(i) => {
                        for symbol in current_table.borrow().symbols.iter() {
                            if symbol.borrow().name == i && symbol.borrow().level == self.current_level {
                                println!("第 {} 行: 变量重复定义", var.borrow().child[0].borrow().line);
                                self.error = true;
                            }
                        }
                        var_name = i.clone();
                    }
                    _ => {
                        println!("第 {} 行: 变量名不合法", var.borrow().child[0].borrow().line);
                        self.error = true;
                    }
                }
                let kind = var.borrow().clone().kind.exp;
                let mut type_kind = ExpType::Void;
                if type_.borrow().child[0].borrow().terminal == Terminals::Int {
                    type_kind = ExpType::Integer;
                } else if type_.borrow().child[0].borrow().terminal == Terminals::Float {
                    type_kind = ExpType::Float;
                } else if type_.borrow().child[0].borrow().terminal == Terminals::Char {
                    type_kind = ExpType::Char;
                } else {
                    println!("第 {} 行: 变量类型不合法", type_.borrow().child[0].borrow().line);
                    self.error = true;
                }
                match kind.clone() {
                    ExpKind::IdK => {
                        let symbol = Rc::new(RefCell::new(
                            Symbol::new(var_name, self.current_level
                                        , type_kind.clone()
                                        , self.current_offset
                                        , 't', 0, None, self.new_var())));
                        symbol.borrow_mut().index = self.var_index - 1;
                        self.current_table.borrow_mut().symbols.push(symbol.clone());
                        self.current_offset += 2;
                    }
                    ExpKind::ArrayK { size, dimensions } => {
                        for i in dimensions.iter() {
                            if *i <= 0 {
                                println!("第 {} 行: 数组长度必须大于0", var.borrow().child[0].borrow().line);
                                self.error = true;
                            }
                        }
                        let mut temp_dimensions = vec![0; size];
                        let i = dimensions.len() - 1;
                        loop {
                            let mut t_name = var_name.clone();
                            for i in temp_dimensions.iter() {
                                t_name = t_name + &format!("[{}]", i);
                            }
                            let symbol = Rc::new(RefCell::new(
                                Symbol::new(t_name, self.current_level
                                            , type_kind.clone()
                                            , self.current_offset
                                            , 't', 0, Some(dimensions.clone()), self.new_var())));
                            symbol.borrow_mut().index = self.var_index - 1;
                            self.current_table.borrow_mut().symbols.push(symbol.clone());
                            self.current_offset += 2;
                            temp_dimensions[i] += 1;
                            if temp_dimensions[i] > dimensions[i] {
                                let mut j = i;
                                loop {
                                    if j == 0 {
                                        break;
                                    }
                                    if temp_dimensions[j] >= dimensions[j] {
                                        temp_dimensions[j] = 0;
                                        temp_dimensions[j - 1] += 1;
                                    }
                                    j -= 1;
                                }
                                if temp_dimensions[0] >= dimensions[0] {
                                    break;
                                }
                            }
                        }
                    }
                    _ => {
                        println!("第 {} 行: 变量定义错误", var.borrow().child[0].borrow().line);
                        self.error = true;
                        return;
                    }
                }
                if local_declarations.borrow().child.len() == 3 {
                    let exp = local_declarations.borrow_mut().child[2].clone();
                    let exp_type = self.analyze_exp(exp.clone());
                    if exp_type != type_kind {
                        if exp_type == ExpType::String && type_kind == ExpType::Char { // 字符串赋值给字符数组
                            match kind {
                                ExpKind::ArrayK { size: _size, dimensions } => {
                                    let e = exp.clone();
                                    let terminals = e.borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().child[0].borrow().terminal.clone();
                                    match terminals {
                                        Terminals::String(s) => {
                                            if dimensions.len() == 1 {
                                                if s.len() > dimensions[0] {
                                                    println!("第 {} 行: 字符串长度超过数组长度", exp.borrow().child[0].borrow().line);
                                                    self.error = true;
                                                } else {
                                                    return;
                                                }
                                            } else {
                                                println!("第 {} 行: 不能将字符串赋给一维以上的字符数组", exp.borrow().child[0].borrow().line);
                                                self.error = true;
                                            }
                                        }
                                        _ => {
                                            println!("第 {} 行: 表达式类型不匹配，变量类型为 {}，表达式类型为 {}", var.borrow().child[0].borrow().line, type_kind, exp_type);
                                            self.error = true;
                                        }
                                    }
                                }
                                _ => {
                                    println!("第 {} 行: 表达式类型不匹配，变量类型为 {}，表达式类型为 {}", var.borrow().child[0].borrow().line, type_kind, exp_type);
                                    self.error = true;
                                }
                            }
                        }
                        println!("第 {} 行: 表达式类型不匹配，变量类型为 {}，表达式类型为 {}", var.borrow().child[0].borrow().line, type_kind, exp_type);
                        self.error = true;
                    }
                }
            }
            NonTerminals::IfStmt => {
                let if_stmt = statement.borrow().child[0].clone();
                self.analyze_if_stmt(if_stmt);
            }
            NonTerminals::WhileStmt => {
                let while_stmt = statement.borrow().child[0].clone();
                let exp = while_stmt.borrow().child[0].clone();
                let exp_type = self.analyze_exp(exp.clone());
                if exp_type != ExpType::Boolean {
                    let line = get_exp_line(exp);
                    println!("第 {} 行: 表达式类型不匹配，只有bool类型变量可以作为条件", line);
                    self.error = true;
                }
                let compound_stmt = while_stmt.borrow().child[1].clone();
                self.current_level += 1;
                self.current_table.borrow_mut().is_loop = true;
                self.analyze_compound_stmt(compound_stmt);
                self.current_table.borrow_mut().is_loop = false;
                self.current_level -= 1;
            }
            NonTerminals::ForStmt => {
                let for_stmt = statement.borrow().child[0].clone();
                let for_expression = for_stmt.borrow().child[0].clone();
                let expression_assign1 = for_expression.borrow().child[0].clone();
                if expression_assign1.borrow().child.len() != 0 {
                    let var_or_call_stmt = expression_assign1.borrow().child[0].clone();
                    self.analyze_var_or_call_stmt(var_or_call_stmt);
                }
                let exp = for_expression.borrow().child[1].clone();
                let exp_type = self.analyze_exp(exp.clone());
                if exp_type != ExpType::Boolean {
                    let line = get_exp_line(exp);
                    println!("第 {} 行: 表达式类型不匹配，只有bool类型变量可以作为条件", line);
                    self.error = true;
                }
                let expression_assign2 = for_expression.borrow().child[2].clone();
                if expression_assign2.borrow().child.len() != 0 {
                    let var_or_call_stmt = expression_assign2.borrow().child[0].clone();
                    self.analyze_var_or_call_stmt(var_or_call_stmt);
                }
                let compound_stmt = for_stmt.borrow().child[1].clone();
                self.current_level += 1;
                self.current_table.borrow_mut().is_loop = true;
                self.analyze_compound_stmt(compound_stmt);
                self.current_table.borrow_mut().is_loop = false;
                self.current_level -= 1;
            }
            NonTerminals::DoStmt => {
                let do_stmt = statement.borrow().child[0].clone();
                let exp = do_stmt.borrow().child[1].clone();
                let exp_type = self.analyze_exp(exp.clone());
                if exp_type != ExpType::Boolean {
                    let line = get_exp_line(exp);
                    println!("第 {} 行: 表达式类型不匹配，只有bool类型变量可以作为条件", line);
                    self.error = true;
                }
                let compound_stmt = do_stmt.borrow().child[0].clone();
                self.current_level += 1;
                self.current_table.borrow_mut().is_loop = true;
                self.analyze_compound_stmt(compound_stmt);
                self.current_table.borrow_mut().is_loop = false;
                self.current_level -= 1;
            }
            NonTerminals::BreakStmt => {
                if !self.current_table.borrow().is_loop {
                    println!("第 {} 行: break语句只能在循环语句或者switch语句中使用", statement.borrow().child[0].borrow().line);
                    self.error = true;
                }
            }
            NonTerminals::ContinueStmt => {
                if !self.current_table.borrow().is_loop {
                    println!("第 {} 行: continue语句只能在循环语句或者switch语句中使用", statement.borrow().child[0].borrow().line);
                    self.error = true;
                }
            }
            NonTerminals::SwitchStmt => {
                let switch_stmt = statement.borrow().child[0].clone();
                let exp = switch_stmt.borrow().child[0].clone();
                let exp_type = self.analyze_exp(exp);
                let case_stmt_list = switch_stmt.borrow().child[1].clone();
                for case_stmt in case_stmt_list.borrow().child.iter() {
                    self.analyze_case_stmt(case_stmt.clone(), exp_type.clone());
                }
            }
            NonTerminals::ReturnStmt => {
                if self.current_table.borrow().func_name == "global" {
                    println!("第 {} 行: return语句只能在函数中使用", statement.borrow().child[0].borrow().line);
                    self.error = true;
                }
                let return_stmt = statement.borrow().child[0].clone();
                let exp = return_stmt.borrow().child[0].clone();
                let exp_type = self.analyze_exp(exp);
                let func_name = self.current_table.borrow().func_name.clone();
                let mut func_symbol = None;
                for symbol in self.current_table.borrow().symbols.iter() {
                    if symbol.borrow().name == func_name {
                        func_symbol = Some(symbol.clone());
                    }
                }
                if let Some(symbol) = func_symbol {
                    if symbol.borrow().type_ != exp_type {
                        println!("第 {} 行: 函数返回值类型不匹配，函数返回值类型为 {}，表达式类型为 {}", return_stmt.borrow().line, symbol.borrow().type_, exp_type);
                        self.error = true;
                    }
                }
            }
            NonTerminals::VarOrCallStmt => {
                let var_or_call_stmt = statement.borrow().child[0].clone();
                self.analyze_var_or_call_stmt(var_or_call_stmt);
            }
            _ => {
                println!("第 {} 行: 语句类型错误", statement.borrow().line);
                self.error = true;
            }
        }
    }

    fn analyze_if_stmt(&mut self, if_stmt: Rc<RefCell<TreeNode>>) {
        let exp = if_stmt.borrow().child[0].clone();
        let exp_type = self.analyze_exp(exp);
        if exp_type != ExpType::Boolean {
            println!("第 {} 行: 表达式类型不匹配，只有bool类型变量可以作为条件", if_stmt.borrow().line);
            self.error = true;
        }
        let compound_stmt = if_stmt.borrow().child[1].clone();
        self.current_level += 1;
        self.analyze_compound_stmt(compound_stmt);
        self.current_level -= 1;
        let else_stmt_n = if_stmt.borrow().child[2].clone();
        if else_stmt_n.borrow().child.len() != 0 {
            let else_stmt = else_stmt_n.borrow().child[0].clone();
            self.current_level += 1;
            self.analyze_compound_stmt(else_stmt);
            self.current_level -= 1;
        }
    }

    fn analyze_case_stmt(&mut self, case_stmt: Rc<RefCell<TreeNode>>, exp_type: ExpType) {
        let non_terminal = case_stmt.borrow().clone().non_terminal;
        match non_terminal {
            NonTerminals::CaseStmt => {
                let exp = case_stmt.borrow().child[0].clone();
                let exp_type1 = self.analyze_exp(exp);
                if exp_type1 != exp_type {
                    println!("第 {} 行: 表达式类型不匹配，switch条件类型为 {}，case条件类型为 {}", case_stmt.borrow().line, exp_type, exp_type1);
                    self.error = true;
                }
                let statement_list = case_stmt.borrow().child[1].clone();
                self.current_table.borrow_mut().is_loop = true;
                self.analyze_statement_list(statement_list);
                self.current_table.borrow_mut().is_loop = false;
            }
            NonTerminals::DefaultStmt => {
                let statement_list = case_stmt.borrow().child[0].clone();
                self.current_table.borrow_mut().is_loop = true;
                self.analyze_statement_list(statement_list);
                self.current_table.borrow_mut().is_loop = false;
            }
            _ => {}
        }
    }

    fn analyze_exp(&mut self, exp: Rc<RefCell<TreeNode>>) -> ExpType {
        let mut exp_type = ExpType::Void;
        for child in exp.borrow_mut().child.iter() {
            if child.borrow().non_terminal == NonTerminals::AdditiveExpression {
                let exp_type1 = self.analyze_additive_exp(child.clone());
                if exp_type != ExpType::Void {
                    if exp_type1 != exp_type {
                        println!("第 {} 行: 表达式类型不匹配", child.borrow().child[0].borrow().line);
                        self.error = true;
                    }
                } else {
                    exp_type = exp_type1;
                }
            } else {
                if exp_type != ExpType::Boolean {
                    println!("第 {} 行: 表达式类型 {} 不适配关系运算符: {}", child.borrow().child[0].borrow().line, exp_type, child.borrow().child[0].borrow().terminal);
                    self.error = true;
                }
            }
        }
        if exp.borrow().child.len() > 1 {
            return ExpType::Boolean;
        }
        exp_type
    }

    fn analyze_additive_exp(&mut self, additive_exp: Rc<RefCell<TreeNode>>) -> ExpType {
        let mut exp_type = ExpType::Void;
        for child in additive_exp.borrow_mut().child.iter() {
            if child.borrow().non_terminal == NonTerminals::SimpleExpression {
                let exp_type1 = self.analyze_simple_exp(child.clone());
                if exp_type != ExpType::Void {
                    if exp_type1 != exp_type {
                        println!("第 {} 行: 表达式类型不匹配", child.borrow().child[0].borrow().line);
                        self.error = true;
                    }
                } else {
                    exp_type = exp_type1;
                }
            } else {
                if exp_type == ExpType::Boolean || exp_type == ExpType::String || exp_type == ExpType::Char {
                    println!("第 {} 行: 表达式类型 {} 不适配关系运算符: {}", child.borrow().child[0].borrow().line, exp_type, child.borrow().child[0].borrow().terminal);
                    self.error = true;
                }
            }
        }
        if additive_exp.borrow().child.len() > 1 {
            return ExpType::Boolean;
        }
        exp_type
    }

    fn analyze_simple_exp(&mut self, simple_exp: Rc<RefCell<TreeNode>>) -> ExpType {
        let mut exp_type = ExpType::Void;
        for child in simple_exp.borrow_mut().child.iter() {
            if child.borrow().non_terminal == NonTerminals::Term {
                let exp_type1 = self.analyze_term(child.clone());
                if exp_type != ExpType::Void {
                    if exp_type1 != exp_type {
                        println!("第 {} 行: 表达式类型不匹配", child.borrow().child[0].borrow().line);
                        self.error = true;
                    }
                } else {
                    exp_type = exp_type1;
                }
            } else {
                if exp_type == ExpType::Boolean || exp_type == ExpType::String || exp_type == ExpType::Char {
                    println!("第 {} 行: 表达式类型 {} 不适配加减运算符: {}", child.borrow().child[0].borrow().line, exp_type, child.borrow().child[0].borrow().terminal);
                    self.error = true;
                }
            }
        }
        exp_type
    }

    fn analyze_term(&mut self, term: Rc<RefCell<TreeNode>>) -> ExpType {
        let mut exp_type = ExpType::Void;
        for child in term.borrow_mut().child.iter() {
            if child.borrow().non_terminal == NonTerminals::Factor {
                let exp_type1 = self.analyze_factor(child.clone());
                if exp_type != ExpType::Void {
                    if exp_type1 != exp_type {
                        println!("第 {} 行: 表达式类型不匹配", child.borrow().child[0].borrow().line);
                        self.error = true;
                    }
                } else {
                    exp_type = exp_type1;
                }
            } else {
                if exp_type == ExpType::Boolean || exp_type == ExpType::String || exp_type == ExpType::Char {
                    println!("第 {} 行: 表达式类型 {} 不适配乘除运算符: {}", child.borrow().child[0].borrow().line, exp_type, child.borrow().child[0].borrow().terminal);
                    self.error = true;
                }
            }
        }
        exp_type
    }

    fn analyze_factor(&mut self, factor: Rc<RefCell<TreeNode>>) -> ExpType {
        if factor.borrow().child[0].borrow().non_terminal == NonTerminals::Expression {
            return self.analyze_exp(factor.borrow().child[0].clone());
        } else if factor.borrow().child[0].borrow().non_terminal == NonTerminals::Var {
            return self.analyze_var(factor.borrow().child[0].clone());
        } else if factor.borrow().child[0].borrow().non_terminal == NonTerminals::CallStmt {
            return self.analyze_call(factor.borrow().child[0].clone());
        } else {
            let terminal = factor.borrow().child[0].borrow().child[0].borrow().terminal.clone();
            match terminal {
                Terminals::Integer(_i) => {
                    return ExpType::Integer;
                }
                Terminals::FloatNumber(_f) => {
                    return ExpType::Float;
                }
                Terminals::Character(_c) => {
                    return ExpType::Char;
                }
                Terminals::True => {
                    return ExpType::Boolean;
                }
                Terminals::False => {
                    return ExpType::Boolean;
                }
                Terminals::String(_s) => {
                    return ExpType::String;
                }
                _ => {
                    println!("第 {} 行: 表达式类型不匹配", factor.borrow().line);
                    self.error = true;
                    return ExpType::Void;
                }
            }
        }
    }

    fn analyze_var_or_call_stmt(&mut self, var_or_call_stmt: Rc<RefCell<TreeNode>>) {
        if var_or_call_stmt.borrow().child[0].borrow().non_terminal == NonTerminals::Var {
            let exp_type = self.analyze_var(var_or_call_stmt.clone().borrow().child[0].clone());
            if var_or_call_stmt.borrow().child[1].borrow().non_terminal == NonTerminals::Expression {
                let exp = var_or_call_stmt.borrow().child[1].clone();
                let exp_type1 = self.analyze_exp(exp);
                if exp_type != exp_type1 {
                    println!("第 {} 行: 表达式类型不匹配, 变量为 {}， 表达式为 {}", var_or_call_stmt.borrow().child[0].borrow().child[0].borrow().line, exp_type, exp_type1);
                    self.error = true;
                }
            } else {
                if exp_type != ExpType::Integer {
                    println!("第 {} 行: 表达式类型不匹配，只有int类型可以使用自加自减运算", var_or_call_stmt.borrow().child[1].borrow().child[0].borrow().line);
                    self.error = true;
                }
            }
        } else {
            self.analyze_call(var_or_call_stmt.clone().borrow().child[0].clone());
        }
    }

    fn analyze_var_or_call(&mut self, var_or_call: Rc<RefCell<TreeNode>>) -> ExpType {
        if var_or_call.borrow().non_terminal == NonTerminals::Var {
            return self.analyze_var(var_or_call.clone());
        } else {
            return self.analyze_call(var_or_call.clone());
        }
    }

    fn analyze_var(&mut self, var: Rc<RefCell<TreeNode>>) -> ExpType {
        let var_terminal = var.borrow().child[0].borrow().terminal.clone();
        let mut var_name = "".to_string();
        match var_terminal {
            Terminals::Identifier(i) => {
                var_name = i.clone();
            }
            _ => {
                println!("第 {} 行: 变量名不合法", var.borrow().child[0].borrow().line);
                self.error = true;
            }
        }
        let current_table = self.current_table.clone();
        let mut symbols = vec![];
        for symbol in current_table.borrow().symbols.iter() {
            if symbol.borrow().name.starts_with(&var_name) {
                symbols.push(symbol.clone());
            }
        }
        if symbols.len() == 0 {
            let global_table = self.symbol_table_manager.global_table.clone();
            for symbol in global_table.borrow().symbols.iter() {
                if symbol.borrow().name.starts_with(&var_name) {
                    symbols.push(symbol.clone());
                }
            }
        }
        if symbols.len() != 0 {
            symbols.sort_by(|a, b| b.borrow().level.cmp(&a.borrow().level));
            let symbol = symbols[0].clone();
            let kind = var.borrow().kind.exp.clone();
            match kind {
                ExpKind::IdK => {
                    return symbol.borrow().type_.clone();
                }
                ExpKind::ArrayK { size: _size, dimensions } => {
                    let option = symbol.borrow().dimension.clone();
                    match option {
                        None => {
                            println!("第 {} 行: 变量类型错误，变量不是数组", var.borrow().child[0].borrow().line);
                            self.error = true;
                        }
                        Some(current_dimension) => {
                            if dimensions.len() != current_dimension.len() {
                                println!("第 {} 行: 变量类型错误，数组维度不匹配", var.borrow().child[0].borrow().line);
                                self.error = true;
                            }
                            for i in 0..dimensions.len() {
                                if current_dimension[i] <= dimensions[i] {
                                    println!("第 {} 行: 数组越界，数组长度只有 {}，而变量索引为 {}", var.borrow().child[0].borrow().line, current_dimension[i], dimensions[i]);
                                    self.error = true;
                                }
                            }
                            return symbol.borrow().type_.clone();
                        }
                    }
                }
                _ => {
                    println!("第 {} 行: 变量类型错误，未知的类型", var.borrow().child[0].borrow().line);
                    self.error = true;
                }
            }
        }
        println!("第 {} 行: 变量未定义", var.borrow().child[0].borrow().line);
        self.error = true;
        return ExpType::Void;
    }

    fn analyze_call(&mut self, call: Rc<RefCell<TreeNode>>) -> ExpType {
        let call_terminal = call.borrow().child[0].borrow().terminal.clone();
        let mut call_name = "".to_string();
        match call_terminal {
            Terminals::Identifier(i) => {
                call_name = i.clone();
            }
            _ => {
                println!("第 {} 行: 函数名不合法", call.borrow().child[0].borrow().line);
                self.error = true;
            }
        }
        let global_table = self.symbol_table_manager.global_table.clone();
        for symbol in global_table.borrow().symbols.iter() {
            if symbol.borrow().name == call_name {
                let args = call.borrow().child[1].clone();
                let mut args_type = vec![];
                for arg in args.borrow().child.iter() {
                    let arg_type = self.analyze_exp(arg.clone());
                    args_type.push(arg_type);
                }
                if args_type.len() != symbol.borrow().param_num as usize {
                    println!("第 {} 行: 参数个数不匹配", call.borrow().child[0].borrow().line);
                    self.error = true;
                }
                let table = self.symbol_table_manager.get(call_name).unwrap();
                let params = table.borrow().get_params();
                for i in 0..args_type.len() {
                    if args_type[i] != params[i].borrow().type_ {
                        println!("第 {} 行: 参数类型不匹配", call.borrow().child[0].borrow().line);
                        self.error = true;
                    }
                }
                return symbol.borrow().type_.clone();
            }
        }
        println!("第 {} 行: 函数未定义", call.borrow().child[0].borrow().line);
        self.error = true;
        return ExpType::Void;
    }
}

pub fn get_exp_line(exp: Rc<RefCell<TreeNode>>) -> usize {
    let line;
    if exp.borrow().child.len() > 1 {
        line = exp.borrow().child[1].borrow().child[0].borrow().line;
        return line;
    }
    let add_exp = exp.borrow().child[0].clone();
    return get_add_exp_line(add_exp);
}

pub fn get_add_exp_line(add_exp: Rc<RefCell<TreeNode>>) -> usize {
    let line;
    if add_exp.borrow().child.len() > 1 {
        line = add_exp.borrow().child[1].borrow().child[0].borrow().line;
        return line;
    }
    let simple_exp = add_exp.borrow().child[0].clone();
    return get_simple_exp_line(simple_exp);
}

pub fn get_simple_exp_line(simple_exp: Rc<RefCell<TreeNode>>) -> usize {
    let line;
    if simple_exp.borrow().child.len() > 1 {
        line = simple_exp.borrow().child[1].borrow().child[0].borrow().line;
        return line;
    }
    let term = simple_exp.borrow().child[0].clone();
    return get_term_line(term);
}

pub fn get_term_line(term: Rc<RefCell<TreeNode>>) -> usize {
    let line;
    if term.borrow().child.len() > 1 {
        line = term.borrow().child[1].borrow().child[0].borrow().line;
        return line;
    }
    let factor = term.borrow().child[0].clone();
    return get_factor_line(factor);
}

pub fn get_factor_line(factor: Rc<RefCell<TreeNode>>) -> usize {
    return if factor.borrow().child[0].borrow().non_terminal == NonTerminals::Expression {
        get_exp_line(factor.borrow().child[0].clone())
    } else {
        factor.borrow().child[0].borrow().child[0].borrow().line
    };
}
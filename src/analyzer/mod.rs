pub mod analyze;

use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::ExpType;

pub struct SymbolTable {
    pub is_main: bool,
    pub symbols: Vec<Rc<RefCell<Symbol>>>,
    pub level: i32,
    pub offset: i32,
    pub func_name: String,
    pub is_loop: bool,
}

impl SymbolTable {
    pub fn new(level: i32, func_name: String) -> Self {
        Self {
            is_main: false,
            symbols: vec![],
            level,
            offset: 0,
            func_name,
            is_loop: false,
        }
    }

    pub fn push(&mut self, symbol: Rc<RefCell<Symbol>>) {
        self.symbols.push(symbol);
    }

    pub fn get(&self, name: &str) -> Option<Rc<RefCell<Symbol>>> {
        for symbol in self.symbols.iter() {
            if symbol.borrow().name == name {
                return Some(symbol.clone());
            }
        }
        None
    }

    pub fn get_by_alias(&self, alias: &str) -> Option<Rc<RefCell<Symbol>>> {
        for symbol in self.symbols.iter() {
            if symbol.borrow().alias == alias {
                return Some(symbol.clone());
            }
        }
        None
    }

    pub fn get_params(&self) -> Vec<Rc<RefCell<Symbol>>> {
        let mut params = vec![];
        for symbol in self.symbols.iter() {
            if symbol.borrow().flag == 'p' {
                params.push(symbol.clone());
            }
        }
        params
    }
}

pub struct SymbolTableManager {
    pub tables: Vec<Rc<RefCell<SymbolTable>>>,
    pub global_table: Rc<RefCell<SymbolTable>>,
}

impl SymbolTableManager {
    pub fn new() -> Self {
        let global_table = Rc::new(RefCell::new(SymbolTable::new(0, "global".to_string())));
        Self {
            tables: vec![],
            global_table,
        }
    }

    pub fn push(&mut self, table: Rc<RefCell<SymbolTable>>) {
        self.tables.push(table);
    }

    pub fn get(&self, name: String) -> Option<Rc<RefCell<SymbolTable>>> {
        for table in self.tables.iter() {
            if table.borrow().func_name == name {
                return Some(table.clone());
            }
        }
        None
    }

    pub fn get_by_alias(&self, alias: String) -> Option<Rc<RefCell<Symbol>>> {
        for table in self.tables.iter() {
            for symbol in table.borrow().symbols.iter() {
                if symbol.borrow().alias == alias {
                    return Some(symbol.clone());
                }
            }
        }
        for symbol in self.global_table.borrow().symbols.iter() {
            if symbol.borrow().alias == alias {
                return Some(symbol.clone());
            }
        }
        None
    }

    pub fn get_current(&self) -> Rc<RefCell<SymbolTable>> {
        self.tables.last().unwrap().clone()
    }
}

pub struct Symbol {
    pub name: String,
    pub level: i32,
    pub type_: ExpType,
    pub offset: i32,
    pub flag: char,
    pub param_num: i32,
    pub dimension: Option<Vec<usize>>,
    pub index: usize,
    pub alias: String,
}

impl Symbol {
    pub fn new(name: String, level: i32, type_: ExpType, offset: i32, flag: char, param_num: i32, dimension: Option<Vec<usize>>, alias: String) -> Self {
        Self {
            name,
            level,
            type_,
            offset,
            flag,
            param_num,
            dimension,
            index: 0,
            alias,
        }
    }
}

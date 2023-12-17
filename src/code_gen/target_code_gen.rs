use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::analyzer::analyze::Analyzer;
use crate::code_gen::{Code, CodeGen, StatementKind};
use crate::code_gen::StatementKind::{Func, Return};

pub struct Masm {
    pub codes: Vec<String>,
    pub tac_codes: Vec<Code>,
    pub register_manager: RegisterManager,
    pub analyzer: Rc<RefCell<Analyzer>>,
    pub stack: Vec<i32>,
    pub param_offset: i32,
    pub temp_map: HashMap<String, i32>,
    pub temp_offset: i32,
}

pub struct Procedure {
    pub name: String,
    pub codes: Vec<Code>,
    pub is_main: bool,
}

impl Masm {
    pub fn new(analyzer: Rc<RefCell<Analyzer>>, tac_codes: Vec<Code>) -> Self {
        Self {
            codes: vec![],
            tac_codes,
            register_manager: RegisterManager::new(),
            analyzer,
            stack: vec![],
            param_offset: 0,
            temp_map: HashMap::new(),
            temp_offset: 0,
        }
    }

    pub fn initialize(&mut self) {
        self.codes.push("ASSUME CS:CODE, DS:DATA, SS:STACK ".to_string());
        self.codes.push(" DATA SEGMENT".to_string());
        let global_table = self.analyzer.borrow().symbol_table_manager.global_table.clone();
        for symbol in global_table.borrow().symbols.iter() {
            if symbol.borrow().flag == 'v' {
                let name = symbol.borrow().name.clone();
                self.codes.push(format!("   {} DW {}", name, 0));
                self.param_offset += 2;
            }
        }
        for _i in 0..8 {
            self.codes.push(format!("   DW {}", 0));
        }
        self.codes.push(" DATA ENDS".to_string());
        self.codes.push(" STACK SEGMENT".to_string());
        self.codes.push("  DW 200 DUP(0)".to_string());
        self.codes.push(" STACK ENDS".to_string());
        self.codes.push(" CODE SEGMENT".to_string());
        self.codes.push("  MOV AX, DATA".to_string());
        self.codes.push("  MOV DS, AX".to_string());
        let mut procedures = Vec::new();
        let mut main = Procedure {
            name: "".to_string(),
            codes: vec![],
            is_main: false,
        };
        let mut global = Vec::new();
        let mut iter = self.tac_codes.iter().enumerate();
        while let Some((i, code)) = iter.next() {
            if code.kind == Func {
                let mut procedure = Procedure {
                    name: code.func.clone(),
                    codes: vec![],
                    is_main: false,
                };
                while let Some((j, inner_code)) = iter.next() {
                    if inner_code.kind == Return {
                        procedure.codes.push(inner_code.clone());
                        break;
                    }
                    procedure.codes.push(inner_code.clone());
                }
                if code.func == "main" {
                    procedure.is_main = true;
                    main = procedure;
                } else {
                    procedures.push(procedure);
                }
            } else {
                global.push(code.clone());
            }
        }
        for code in global {
            self.gen_global_declaration(&code);
        }
        self.gen_main(&main);
        for procedure in procedures {
            self.gen_proc(&procedure);
        }
        self.codes.push(" CODE ENDS".to_string());
        self.codes.push("END".to_string());
    }

    pub fn gen_global_declaration(&mut self, code: &Code) {
        let left = code.left.clone();
        if left.contains("V") {
            let right = code.right.clone();
            let symbol = self.analyzer.borrow().symbol_table_manager.get_by_alias(left).unwrap();
            let offset = symbol.borrow().offset;
            if right.contains("#") {
                let value = right.replace("#", "");
                self.codes.push(format!("  MOV AX, {}", value));
                self.codes.push(format!("  MOV WORD PTR DS:[{}], AX", offset));
            } else if right.contains("V") {
                let right_symbol = self.analyzer.borrow().symbol_table_manager.get_by_alias(right).unwrap();
                let right_offset = right_symbol.borrow().offset;
                self.codes.push(format!("  MOV AX, WORD PTR DS:[{}]", right_offset));
                self.codes.push(format!("  MOV WORD PTR DS:[{}], AX", offset));
            } else if right.contains("T") {
                if self.temp_map.contains_key(&right) {
                    let temp_offset = self.temp_map.get(&right).unwrap();
                    self.codes.push(format!("  MOV AX, WORD PTR SS:[BP - {}]", temp_offset));
                    self.codes.push(format!("  MOV WORD PTR DS:[{}], AX", offset));
                } else {
                    let temp_offset = self.temp_offset;
                    self.temp_map.insert(right.clone(), temp_offset);
                    self.temp_offset += 2;
                    self.codes.push(format!("  MOV AX, WORD PTR SS:[BP - {}]", temp_offset));
                    self.codes.push(format!("  MOV WORD PTR DS:[{}], AX", offset));
                }
            } else {
                self.codes.push(format!("  CALL FAR PTR {}", right));
                self.codes.push(format!(" MOV WORD PTR DS:[{}], AX", offset));
            }
        } else if left.contains("T") {
            let offset: i32;
            if self.temp_map.contains_key(&left) {
                offset = *self.temp_map.get(&left).unwrap();
            } else {
                offset = self.temp_offset.clone();
                self.temp_map.insert(left.clone(), offset);
                self.temp_offset += 2;
            }
            let right = code.right.clone();
            if right.contains("#") {
                let value = right.replace("#", "");
                self.codes.push(format!("  MOV AX, {}", value));
                self.codes.push(format!("  MOV WORD PTR SS:[BP - {}], AX", offset));
            }
        }
    }

    pub fn gen_declaration(&mut self, code: &Code) {
        let left = code.left.clone();
        let mut c = "".to_string();
        if left.contains("V") {
            let symbol = self.analyzer.borrow().symbol_table_manager.get_by_alias(left).unwrap();
            let offset = symbol.borrow().offset;
            if symbol.borrow().flag == 'v' {
                c = format!("  MOV WORD PTR DS:[{}], AX", offset);
            } else {
                c = format!("  MOV WORD PTR SS:[BP - {}], AX", offset);
            }
        } else if left.contains("T") {
            let offset: i32;
            if self.temp_map.contains_key(&left) {
                offset = *self.temp_map.get(&left).unwrap();
            } else {
                offset = self.temp_offset.clone();
                self.temp_map.insert(left.clone(), offset);
                self.temp_offset += 2;
            }
            c = format!("  MOV WORD PTR SS:[BP - {}], AX", offset);
        }
        let right = code.right.clone();
        if right.contains("#") {
            let value = right.replace("#", "");
            self.codes.push(format!("  MOV AX, {}", value));
            self.codes.push(c);
        } else if right.contains("V") {
            let right_symbol = self.analyzer.borrow().symbol_table_manager.get_by_alias(right).unwrap();
            let right_offset = right_symbol.borrow().offset;
            if right_symbol.borrow().flag == 'v' {
                self.codes.push(format!("  MOV AX, WORD PTR DS:[{}]", right_offset));
            } else {
                self.codes.push(format!("  MOV AX, WORD PTR SS:[BP - {}]", right_offset));
            }
            self.codes.push(c);
        } else if right.contains("T") {
            if self.temp_map.contains_key(&right) {
                let temp_offset = self.temp_map.get(&right).unwrap();
                self.codes.push(format!("  MOV AX, WORD PTR SS:[BP - {}]", temp_offset));
                self.codes.push(c);
            } else {
                let temp_offset = self.temp_offset;
                self.temp_map.insert(right.clone(), temp_offset);
                self.temp_offset += 2;
                self.codes.push(format!("  MOV AX, WORD PTR SS:[BP - {}]", temp_offset));
                self.codes.push(c);
            }
        } else {
            self.codes.push(format!("  CALL FAR PTR {}", right));
            self.codes.push(c);
        }
    }

    pub fn gen_main(&mut self, procedure: &Procedure) {
        self.codes.push(format!("MAIN: "));
        let table = self.analyzer.borrow().symbol_table_manager.get("main".to_string()).unwrap();
        self.temp_offset = (table.clone().borrow().symbols.len() as i32) * 2;
        for i in 0..procedure.codes.len() {
            if procedure.codes[i].kind == Return {
                self.codes.push("  MOV AX, 4C00H".to_string());
                self.codes.push("  INT 21H".to_string());
            } else {
                self.gen_code(&procedure.codes[i]);
            }
        }
    }

    pub fn gen_proc(&mut self, procedure: &Procedure) {
        self.codes.push(format!(""));
        self.codes.push(format!("{} PROC FAR", procedure.name));
        self.codes.push(format!("  PUSH BP"));
        self.codes.push(format!("  MOV BP, SP"));
        let table = self.analyzer.borrow().symbol_table_manager.get(procedure.name.clone()).unwrap();
        self.temp_offset = (table.clone().borrow().symbols.len() as i32) * 2;
        for i in 0..procedure.codes.len() {
            self.gen_code(&procedure.codes[i]);
        }
        self.codes.push(format!("{} ENDP", procedure.name));
    }

    pub fn gen_code(&mut self, code: &Code) {
        match code.kind {
            Return => {
                self.codes.push(format!("  MOV SP, BP"));
                self.codes.push(format!("  POP BP"));
                self.codes.push(format!("  RET"));
            }
            StatementKind::Declaration => {
                self.gen_declaration(code);
            }
            StatementKind::Assign => {
                let assign = code.assign.clone();
                let left = code.left.clone();
                let right = code.right.clone();
                let op = code.op.clone();
                let mut c = "".to_string();
                if left.contains("V") {
                    let symbol = self.analyzer.borrow().symbol_table_manager.get_by_alias(left).unwrap();
                    let offset = symbol.borrow().offset;
                    if symbol.borrow().flag == 'v' {
                        c = format!("  MOV AX, WORD PTR DS:[{}]", offset);
                    } else {
                        c = format!("  MOV AX, WORD PTR SS:[BP - {}]", offset);
                    }
                } else if left.contains("T") {
                    let offset: i32 = *self.temp_map.get(&left).unwrap();
                    c = format!("  MOV AX, WORD PTR SS:[BP - {}]", offset);
                }
                if right.contains("#") {
                    let value = right.replace("#", "");
                    self.codes.push(c);
                    self.codes.push(format!("  MOV BX, {}", value));
                } else if right.contains("V") {
                    let right_symbol = self.analyzer.borrow().symbol_table_manager.get_by_alias(right).unwrap();
                    let right_offset = right_symbol.borrow().offset;
                    self.codes.push(c);
                    if right_symbol.borrow().flag == 'v' {
                        self.codes.push(format!("  MOV BX, WORD PTR DS:[{}]", right_offset));
                    } else {
                        self.codes.push(format!("  MOV BX, WORD PTR SS:[BP - {}]", right_offset));
                    }
                } else if right.contains("T") {
                    let temp_offset = self.temp_map.get(&right).unwrap();
                    self.codes.push(c);
                    self.codes.push(format!("  MOV BX, WORD PTR SS:[BP - {}]", temp_offset));
                }
                match op.as_str() {
                    "+" => {
                        self.codes.push(format!("  ADD AX, BX"));
                    }
                    "-" => {
                        self.codes.push(format!("  SUB AX, BX"));
                    }
                    "*" => {
                        self.codes.push(format!("  MUL BX"));
                    }
                    "/" => {
                        self.codes.push(format!("  DIV BX"));
                    }
                    "<" => {
                        self.codes.push(format!("  CMP AX, BX"));
                        self.codes.push(format!("  PUSHF"));
                        self.codes.push(format!("  POP AX"));
                        self.codes.push(format!("  AND AX, 0001H"));
                    }
                    ">" => {
                        self.codes.push(format!("  CMP AX, BX"));
                        self.codes.push(format!("  PUSHF"));
                        self.codes.push(format!("  POP AX"));
                        self.codes.push(format!("  TEST AX, 0041H"));
                        self.codes.push(format!("  PUSHF"));
                        self.codes.push(format!("  POP AX"));
                        self.codes.push(format!("  AND AX, 0040H"));

                    }
                    "<=" => {
                        self.codes.push(format!("  CMP AX, BX"));
                        self.codes.push(format!("  PUSHF"));
                        self.codes.push(format!("  POP AX"));
                        self.codes.push(format!("  AND AX, 0041H"));
                    }
                    ">=" => {
                        self.codes.push(format!("  CMP AX, BX"));
                        self.codes.push(format!("  PUSHF"));
                        self.codes.push(format!("  POP AX"));
                        self.codes.push(format!("  TEST AX, 0001H"));
                        self.codes.push(format!("  PUSHF"));
                        self.codes.push(format!("  POP AX"));
                        self.codes.push(format!("  AND AX, 0040H"));
                    }
                    "==" => {
                        self.codes.push(format!("  CMP AX, BX"));
                        self.codes.push(format!("  PUSHF"));
                        self.codes.push(format!("  POP AX"));
                        self.codes.push(format!("  AND AX, 0040H"));
                    }
                    "!=" => {
                        self.codes.push(format!("  CMP AX, BX"));
                        self.codes.push(format!("  PUSHF"));
                        self.codes.push(format!("  POP AX"));
                        self.codes.push(format!("  TEST AX, 0040H"));
                        self.codes.push(format!("  PUSHF"));
                        self.codes.push(format!("  POP AX"));
                        self.codes.push(format!("  AND AX, 0040H"));
                    }
                    "&&" => {
                        self.codes.push(format!("  AND AX, BX"));
                    }
                    "||" => {
                        self.codes.push(format!("  OR AX, BX"));
                    }
                    "!" => {
                        self.codes.push(format!("  NOT AX"));
                    }
                    _ => {}
                }
                if assign.contains("V") {
                    let symbol = self.analyzer.borrow().symbol_table_manager.get_by_alias(assign).unwrap();
                    let offset = symbol.borrow().offset;
                    if symbol.borrow().flag == 'v' {
                        self.codes.push(format!("  MOV WORD PTR DS:[{}], AX", offset));
                    } else {
                        self.codes.push(format!("  MOV WORD PTR SS:[BP - {}], AX", offset));
                    }
                    self.codes.push(format!("  MOV WORD PTR DS:[{}], AX", offset));
                } else if assign.contains("T") {
                    if self.temp_map.contains_key(&assign) {
                        let temp_offset = self.temp_map.get(&assign).unwrap();
                        self.codes.push(format!("  MOV WORD PTR SS:[BP - {}], AX", temp_offset));
                    } else {
                        let temp_offset = self.temp_offset;
                        self.temp_map.insert(assign.clone(), temp_offset);
                        self.temp_offset += 2;
                        self.codes.push(format!("  MOV WORD PTR SS:[BP - {}], AX", temp_offset));
                    }
                }
            }
            StatementKind::Param => {
                let param = code.param.clone();
                let symbol = self.analyzer.borrow().symbol_table_manager.get_by_alias(param).unwrap();
                let offset = self.param_offset + code.param_index as i32;
                self.codes.push(format!("  MOV AX, WORD PTR DS:[{}]", offset));
                self.codes.push(format!("  MOV WORD PTR SS:[BP - {}], AX", symbol.borrow().offset));
            }
            StatementKind::Func => {}
            StatementKind::Label => {
                self.codes.push(format!("{}:", code.label));
            }
            StatementKind::Goto => {
                self.codes.push(format!("  JMP {}", code.goto));
            }
            StatementKind::If => {
                let left = code.if_goto.clone();
                let label = code.label.clone();
                let x = self.temp_map.get(&left).unwrap();
                self.codes.push(format!("  MOV AX, WORD PTR SS:[BP - {}]", x));
                self.codes.push(format!("  CMP AX, 0"));
                self.codes.push(format!("  JNE {}", label));
            }
            StatementKind::Arg => {
                let arg = code.arg.clone();
                if arg.contains("#") {
                    let value = arg.replace("#", "");
                    self.codes.push(format!("  MOV AX, {}", value));
                    let offset = self.param_offset + code.param_index as i32;
                    self.codes.push(format!("  MOV WORD PTR DS:[{}], AX", offset));
                } else if arg.contains("V") {
                    let symbol = self.analyzer.borrow().symbol_table_manager.get_by_alias(arg).unwrap();
                    let offset = self.param_offset + code.param_index as i32;
                    if symbol.borrow().flag == 'v' {
                        self.codes.push(format!("  MOV AX, WORD PTR DS:[{}]", symbol.borrow().offset));
                    } else {
                        self.codes.push(format!("  MOV AX, WORD PTR SS:[BP - {}]", symbol.borrow().offset));
                    }
                    self.codes.push(format!("  MOV WORD PTR DS:[{}], AX", offset));
                } else if arg.contains("T") {
                    let temp_offset = self.temp_map.get(&arg).unwrap();
                    let offset = self.param_offset + code.param_index as i32;
                    self.codes.push(format!("  MOV AX, WORD PTR SS:[BP - {}]", temp_offset));
                    self.codes.push(format!("  MOV WORD PTR DS:[{}], AX", offset));
                }
            }
            StatementKind::Call => {
                self.codes.push(format!("  CALL FAR PTR {}", code.func));
            }
        }
    }
}
pub struct RegisterManager {
    pub registers: Vec<Register>,
}

impl RegisterManager {
    pub fn new() -> Self {
        let mut registers = vec![];
        let names = vec!["AX", "BX", "CX", "DX", "SI", "DI", "BP", "SP"];
        for name in names {
            registers.push(Register::new(name.to_string()));
        }
        Self {
            registers,
        }
    }

    pub fn get_free_register(&mut self) -> Option<&mut Register> {
        for register in self.registers.iter_mut() {
            if register.is_free {
                register.is_free = false;
                return Some(register);
            }
        }
        None
    }

    pub fn get_register(&mut self, name: &str) -> Option<&mut Register> {
        for register in self.registers.iter_mut() {
            if register.name == name {
                return Some(register);
            }
        }
        None
    }

    pub fn free_register(&mut self, name: &str) {
        for register in self.registers.iter_mut() {
            if register.name == name {
                register.is_free = true;
                register.value = None;
                return;
            }
        }
    }

    pub fn free_all_register(&mut self) {
        for register in self.registers.iter_mut() {
            register.is_free = true;
            register.value = None;
        }
    }

    pub fn get_register_value(&self, name: &str) -> Option<String> {
        for register in self.registers.iter() {
            if register.name == name {
                return register.value.clone();
            }
        }
        None
    }

    pub fn set_register_value(&mut self, name: &str, value: String) {
        for register in self.registers.iter_mut() {
            if register.name == name {
                register.value = Some(value);
                return;
            }
        }
    }

}

pub struct Register {
    pub name: String,
    pub is_free: bool,
    pub value: Option<String>,
}

impl Register {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_free: true,
            value: None,
        }
    }
}

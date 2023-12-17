use std::collections::{HashMap, HashSet};
use crate::code_gen::{Code, StatementKind};

pub struct Optimizer {
    pub codes: Vec<Code>,
    pub const_map: HashMap<String, i32>,
}

impl Optimizer {
    pub fn new(codes: Vec<Code>) -> Self {
        Self {
            codes,
            const_map: HashMap::new(),
        }
    }

    pub fn optimize(&mut self) {
        self.optimize_const();
        self.optimize_redundancy();
    }

    fn optimize_const(&mut self) {
        let mut remove_index = vec![];
        let mut i = 0;
        for code in self.codes.iter_mut() {
            match code.kind {
                StatementKind::Declaration => {
                    let left = code.left.clone();
                    let right = code.right.clone();
                    if self.const_map.contains_key(&right) {
                        let right = format!("#{}", self.const_map.get(&right).unwrap());
                        code.right = right;
                    }
                    code.raw = format!(" {} := {}", code.left, code.right);
                    if code.right.contains("#") {
                        let right = code.right.replace("#", "");
                        self.const_map.insert(left, right.parse::<i32>().unwrap());
                        remove_index.push(i);
                    } else {
                        if self.const_map.contains_key(&left) {
                            self.const_map.remove(&left);
                        }
                    }
                }
                StatementKind::Assign => {
                    let left = code.left.clone();
                    let right = code.right.clone();
                    if self.const_map.contains_key(&left) {
                        let left = format!("#{}", self.const_map.get(&left).unwrap());
                        code.left = left;
                    }
                    if self.const_map.contains_key(&right) {
                        let right = format!("#{}", self.const_map.get(&right).unwrap());
                        code.right = right;
                    }
                    code.raw = format!(" {} = {} {} {}", code.assign, code.left, code.op, code.right);
                    let op = code.op.clone();
                    if code.left.contains("#") && code.right.contains("#") {
                        let left = code.left.replace("#", "");
                        let right = code.right.replace("#", "");
                        let left = left.parse::<i32>().unwrap();
                        let right = right.parse::<i32>().unwrap();
                        let result = match op.as_str() {
                            "+" => left + right,
                            "-" => left - right,
                            "*" => left * right,
                            "/" => left / right,
                            ">" => if left > right { 1 } else { 0 },
                            "<" => if left < right { 1 } else { 0 },
                            "==" => if left == right { 1 } else { 0 },
                            "!=" => if left != right { 1 } else { 0 },
                            "&&" => if left != 0 && right != 0 { 1 } else { 0 },
                            "||" => if left != 0 || right != 0 { 1 } else { 0 },
                            "!" => if left == 0 { 1 } else { 0 },
                            ">=" => if left >= right { 1 } else { 0 },
                            "<=" => if left <= right { 1 } else { 0 },
                            _ => panic!("运算符错误！"),
                        };
                        self.const_map.insert(code.assign.clone(), result);
                        remove_index.push(i);
                    } else {
                        if self.const_map.contains_key(&code.assign) {
                            self.const_map.remove(&code.assign);
                        }
                    }
                }
                StatementKind::Call => {}
                StatementKind::Func => {}
                StatementKind::Param => {}
                StatementKind::If => {
                    let expr = code.if_goto.clone();
                    if self.const_map.contains_key(&expr) {
                        let expr = format!("#{}", self.const_map.get(&expr).unwrap());
                        code.if_goto = expr;
                    }
                    if code.if_goto.contains("#") {
                        let expr = code.if_goto.replace("#", "");
                        let expr = expr.parse::<i32>().unwrap();
                        if expr == 0 {
                            code.kind = StatementKind::Goto;
                            code.if_goto = code.label.clone();
                            code.raw = format!(" goto {}", code.label);
                        } else {
                            remove_index.push(i);
                        }
                    }
                }
                StatementKind::Goto => {}
                StatementKind::Return => {
                    let return_ = code.return_.clone();
                    if self.const_map.contains_key(&return_) {
                        let return_ = format!("#{}", self.const_map.get(&return_).unwrap());
                        code.return_ = return_;
                        code.raw = format!(" return {}", code.return_);
                    }
                }
                StatementKind::Arg => {
                    let arg = code.arg.clone();
                    if self.const_map.contains_key(&arg) {
                        let arg = format!("#{}", self.const_map.get(&arg).unwrap());
                        code.arg = arg;
                        code.raw = format!(" arg {}", code.arg);
                    }
                }
                StatementKind::Label => {}
            }
            i = i + 1;
        }
        remove_index.reverse();
        for index in remove_index.iter() {
            self.codes.remove(*index);
        }
    }

    pub fn optimize_redundancy(&mut self) {
        let mut remove_index = vec![];
        let mut temp_map = HashMap::new();
        let length = self.codes.len();
        let mut label_map: HashMap<String,String> = HashMap::new();
        for i in 0..length {
            match &mut self.codes[i].kind {
                StatementKind::Declaration => {
                    let right_clone = self.codes[i].right.clone();
                    if temp_map.contains_key(&right_clone) {
                        if let Some(&x) = temp_map.get(&right_clone) {
                            let assign;
                            let left;
                            let op;
                            let right;
                            remove_index.push(x);
                            let code1: &Code = &self.codes[x];
                            assign = self.codes[i].left.clone();
                            left = code1.left.clone();
                            op = code1.op.clone();
                            right = code1.right.clone();
                            self.codes[i].kind = StatementKind::Assign;
                            self.codes[i].assign = assign.clone();
                            self.codes[i].left = left.clone();
                            self.codes[i].op = op.clone();
                            self.codes[i].right = right.clone();
                            self.codes[i].raw = format!(" {} = {} {} {}", assign, left, op, right);
                        }
                    }
                }
                StatementKind::Assign => {
                    let assign = self.codes[i].assign.clone();
                    temp_map.insert(assign, i);
                }
                StatementKind::Call => {}
                StatementKind::Func => {}
                StatementKind::Param => {}
                StatementKind::If => {
                    let label = self.codes[i].label.clone();
                    if label_map.contains_key(&label) {
                        let x = label_map.get(&label).unwrap();
                        self.codes[i].label = x.clone();
                        self.codes[i].raw = format!(" if {} goto {}", self.codes[i].if_goto, x);
                    }
                }
                StatementKind::Goto => {
                    let label = self.codes[i].label.clone();
                    if label_map.contains_key(&label) {
                        let x = label_map.get(&label).unwrap();
                        self.codes[i].label = x.clone();
                        self.codes[i].raw = format!(" goto {}", x);
                    }
                    let j = i + 1;
                    if j < length {
                        if self.codes[j].kind != StatementKind::Label {
                            remove_index.push(j);
                        }
                    }
                }
                StatementKind::Return => {}
                StatementKind::Arg => {}
                StatementKind::Label => {
                    let j = i - 1;
                    let l = self.codes[i].label.clone();
                    if self.codes[j].kind == StatementKind::Label {
                        let label = self.codes[j].label.clone();
                        if label_map.contains_key(&label) {
                            let x = label_map.get(&label).unwrap();
                            label_map.insert(l.clone(), x.clone());
                        } else {
                            label_map.insert(l.clone(), label);
                        }
                        remove_index.push(i);
                    }
                }
            }
        }
        for i in 0..length {
            match &mut self.codes[i].kind {
                StatementKind::If => {
                    let label = self.codes[i].label.clone();
                    if label_map.contains_key(&label) {
                        let x = label_map.get(&label).unwrap();
                        self.codes[i].label = x.clone();
                        self.codes[i].raw = format!(" if {} goto {}", self.codes[i].if_goto, x);
                    }
                }
                StatementKind::Goto => {
                    let label = self.codes[i].goto.clone();
                    if label_map.contains_key(&label) {
                        let x = label_map.get(&label).unwrap();
                        self.codes[i].label = x.clone();
                        self.codes[i].raw = format!(" goto {}", x);
                    }
                    let j = i + 1;
                    if j < length {
                        if self.codes[j].kind != StatementKind::Label {
                            remove_index.push(j);
                        }
                    }
                }
                _ => {}
            }
        }
        let set: HashSet<_> = remove_index.into_iter().collect();
        let mut remove_index: Vec<_> = set.into_iter().collect();
        remove_index.sort();
        remove_index.reverse();
        for index in remove_index.iter() {
            self.codes.remove(*index);
        }
    }

    pub fn print(&self) {
        for code in self.codes.iter() {
            println!("{}", code.raw);
        }
    }
}

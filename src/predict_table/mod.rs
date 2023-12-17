use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;


mod first;
mod follow;
mod select;

static TERMINALS: [&str; 50] = ["None", "Integer(i64)", "FloatNumber(f64)", "Character(char)", "String(String)"
    , "Plus", "Minus", "Asterisk", "Slash"
    , "LeftParen", "RightParen", "Eof", "Int"
    , "Float", "Char", "Void", "Identifier", "Equals", "If"
    , "Else", "GreaterThan", "LessThan", "GreaterThanEquals", "LessThanEquals", "EqualsEquals"
    , "BangEquals", "OpenBrace", "CloseBrace", "True", "False"
    , "While", "Return", "Comma", "SemiColon", "Colon"
    , "And", "Or", "Not", "Switch", "Case", "Default", "Break", "Continue", "Do"
    , "LeftBracket", "RightBracket", "For", "$", "PlusPlus", "MinusMinus"];

static NON_TERMINALS: [&str; 60] = ["program", "declaration_list", "declaration_list'", "declaration", "dec_stmt"
    , "type", "var_dec_stmt", "fun_dec_stmt", "params", "param_list", "param_list'"
    , "param", "compound_stmt", "local_declarations", "statement_list", "statement_list'"
    , "statement", "if_stmt", "else_stmt_n", "else_stmt", "while_stmt"
    , "for_stmt", "for_expression", "expression_assign", "do_stmt"
    , "break_stmt", "continue_stmt", "switch_stmt", "case_default_stmt", "case_stmt_n", "case_stmt_n'"
    , "case_stmt", "default_stmt", "return_stmt", "var", "var'", "expression", "expression'", "add_op", "mul_op", "relational_op"
    , "additive_expression", "additive_expression'","simple_expression", "simple_expression'", "logical_op", "term", "term'", "factor", "var_or_call", "var_or_call_remain", "args"
    , "arg_list", "arg_list'", "value", "var_stmt", "call_stmt", "self_op", "var_or_call_stmt_remain", "var_or_call_stmt"];


#[derive(Debug)]
pub struct PredictTable {
    table: HashMap<String, HashMap<String, Vec<String>>>,
}

impl PredictTable {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    fn insert(&mut self, left: String, right: String, predict: &String) {
        let entry = self.table.entry(left).or_insert(HashMap::new());
        entry.entry(right).or_insert(Vec::new()).push(predict.clone());
    }

    fn get(&self, left: &str, right: &str) -> Option<&Vec<String>> {
        let entry = self.table.get(left)?;
        entry.get(right)
    }
}

pub fn generate_predict_table(select: HashMap<String, HashSet<String>>) -> PredictTable {
    let mut predict_table = PredictTable::new();
    for (key, value) in select.iter() {
        let mut iter = key.split("->");
        let left = iter.next().unwrap().trim().to_string();
        for s in value.iter() {
            predict_table.insert(left.clone(), s.to_string(), key);
        }
    }
    predict_table
}

pub fn get_predict_table() -> PredictTable {
    let file_path = "./rule.txt";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("文件打开失败: {}", e)
    };
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut productions = HashMap::new();
    for line in content.lines() {
        let mut iter = line.split(":");
        let left = iter.next().unwrap().trim().to_string();
        let right = iter.next().unwrap().trim().to_string();
        let mut right_iter = right.split("|");
        let mut right_vec = Vec::new();
        while let Some(right) = right_iter.next() {
            let right = right.trim().to_string();
            let mut right = right.split(" ");
            let mut right_vec_inner = Vec::new();
            while let Some(right) = right.next() {
                right_vec_inner.push(right.to_string());
            }
            right_vec.push(right_vec_inner);
        }
        productions.insert(left, right_vec);
    }
    let first_set = first::generate_first_set(&productions);
    let follow_set = follow::generate_follow_set(&productions, &first_set);
    let select_set = select::generate_select_set(&productions, &first_set, &follow_set);
    generate_predict_table(select_set)
}

#[test]
fn main() {
    let file_path = "./rule.txt";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("文件打开失败: {}", e)
    };
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut productions = HashMap::new();
    for line in content.lines() {
        let mut iter = line.split(":");
        let left = iter.next().unwrap().trim().to_string();
        let right = iter.next().unwrap().trim().to_string();
        let mut right_iter = right.split("|");
        let mut right_vec = Vec::new();
        while let Some(right) = right_iter.next() {
            let mut right = right.trim().to_string();
            let mut right = right.split(" ");
            let mut right_vec_inner = Vec::new();
            while let Some(right) = right.next() {
                right_vec_inner.push(right.to_string());
            }
            right_vec.push(right_vec_inner);
        }
        productions.insert(left, right_vec);
    }
    let first_set = first::generate_first_set(&productions);
    let follow_set = follow::generate_follow_set(&productions, &first_set);
    let select_set = select::generate_select_set(&productions, &first_set, &follow_set);
    let predict_table = generate_predict_table(select_set.clone());
    println!("{:#?}", predict_table);
}

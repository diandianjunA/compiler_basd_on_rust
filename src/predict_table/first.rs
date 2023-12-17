use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use crate::predict_table::TERMINALS;
use crate::predict_table::NON_TERMINALS;

pub(crate) fn generate_first_set(productions: &HashMap<String,Vec<Vec<String>>>) -> HashMap<String, RefCell<HashSet<String>>> {
    let mut first_set = HashMap::new();
    for s in TERMINALS.iter() {
        first_set.insert(s.to_string(), RefCell::new(HashSet::new()));
    }
    for s in NON_TERMINALS.iter() {
        first_set.insert(s.to_string(), RefCell::new(HashSet::new()));
    }
    for s in NON_TERMINALS.iter() {
        dfs(s, &productions, &first_set);
    }
    first_set
}
fn dfs(str: &str, productions: &HashMap<String, Vec<Vec<String>>>, first_set: &HashMap<String, RefCell<HashSet<String>>>) {
    // 使用 RefCell 进行内部可变性，注意这里要加上.borrow_mut()
    let temp = &mut *first_set.get(str).unwrap().borrow_mut();
    if temp.len() != 0 {
        return;
    }
    let set = productions.get(str).unwrap();
    let mut flag = true;
    for s in set.iter() {
        for s1 in s.iter() {
            if TERMINALS.contains(&&**s1) {
                temp.insert(s1.to_string());
                flag = false;
                break;
            } else {
                dfs(s1, productions, first_set);
                let temp1 = &*first_set.get(s1).unwrap().borrow();
                for s2 in temp1.iter() {
                    if s2 != "None" {
                        temp.insert(s2.to_string());
                    }
                }
                if !temp1.contains("None") {
                    flag = false;
                    break;
                }
            }
        }
    }
    if flag {
        temp.insert("None".to_string());
    }
}
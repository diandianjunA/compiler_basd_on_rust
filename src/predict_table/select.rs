use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use crate::predict_table::TERMINALS;

pub fn generate_select_set(productions: &HashMap<String,Vec<Vec<String>>>, first_set: &HashMap<String, RefCell<HashSet<String>>>, follow_set: &HashMap<String, RefCell<HashSet<String>>>) -> HashMap<String, HashSet<String>> {
    let mut select_set: HashMap<String, HashSet<String>> = HashMap::new();
    for production in productions.iter() {
        let left = production.0;
        let right = production.1;
        for s in right.iter() {
            let mut temp = HashSet::new();
            let mut flag = true;
            for i in 0..s.len() {
                if TERMINALS.contains(&&**&s[i]) {
                    if s[i] != "None" {
                        temp.insert(s[i].to_string());
                        flag = false;
                    }
                    break;
                } else {
                    // println!("{}", s[i]);
                    let temp1 = &*first_set.get(&s[i]).unwrap().borrow();
                    for s1 in temp1.iter() {
                        if s1 != "None" {
                            temp.insert(s1.to_string());
                        }
                    }
                    if !temp1.contains("None") {
                        flag = false;
                        break;
                    }
                }
            }
            if flag {
                let temp1 = &*follow_set.get(left).unwrap().borrow();
                for s1 in temp1.iter() {
                    temp.insert(s1.to_string());
                }
            }
            let p = left.clone() + "->" + &s.join(" ");
            select_set.insert(p, temp);
        }
    }
    select_set
}

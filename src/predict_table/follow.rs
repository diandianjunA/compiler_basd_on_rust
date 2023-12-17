use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use crate::predict_table::TERMINALS;
use crate::predict_table::NON_TERMINALS;

pub fn generate_follow_set(productions: &HashMap<String,Vec<Vec<String>>>, first_set: &HashMap<String, RefCell<HashSet<String>>>) -> HashMap<String, RefCell<HashSet<String>>> {
    let mut follow_set: HashMap<String, RefCell<HashSet<String>>> = HashMap::new();
    for s in NON_TERMINALS.iter() {
        follow_set.insert(s.to_string(), RefCell::new(HashSet::new()));
        if s == &"program" {
            let temp = &mut *follow_set.get(*s).unwrap().borrow_mut();
            temp.insert("$".to_string());
        }
    }
    let mut flag = true;
    while flag {
        flag = false;
        for production in productions.iter() {
            let left = production.0;
            let right = production.1;
            for s in right.iter() {
                for i in 0..s.len() {
                    if NON_TERMINALS.contains(&&**&s[i]) {
                        let temp = &mut *follow_set.get(&s[i]).unwrap().borrow_mut();
                        let len = temp.len();
                        if i == s.len() - 1 {
                            if left == &s[i] {
                                continue;
                            }
                            let temp1 = &*follow_set.get(left).unwrap().borrow();
                            for s1 in temp1.iter() {
                                temp.insert(s1.to_string());
                            }
                            if temp.len() != len {
                                flag = true;
                            }
                        } else {
                            let next = &s[i + 1];
                            if TERMINALS.contains(&&**next) {
                                let len = temp.len();
                                temp.insert(next.to_string());
                                if temp.len() != len {
                                    flag = true;
                                }
                            } else {
                                let temp1 = &*first_set.get(next).unwrap().borrow();
                                let len = temp.len();
                                for s1 in temp1.iter() {
                                    if s1 != "None" {
                                        temp.insert(s1.to_string());
                                    }
                                }
                                if temp.len() != len {
                                    flag = true;
                                }
                                if temp1.contains("None") {
                                    if left == &s[i] {
                                        continue;
                                    }
                                    let len = temp.len();
                                    if left != next {
                                        let temp1 = &*follow_set.get(left).unwrap().borrow();
                                        for s1 in temp1.iter() {
                                            temp.insert(s1.to_string());
                                        }
                                    } else {
                                        for s1 in temp1.iter() {
                                            temp.insert(s1.to_string());
                                        }
                                    }
                                    if temp.len() != len {
                                        flag = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    follow_set
}


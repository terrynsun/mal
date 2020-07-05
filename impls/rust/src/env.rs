use std::collections::HashMap;

use super::types::*;

#[derive(Debug)]
pub struct MalEnv<'a> {
    map: HashMap<String, MalType>,
    outer: Option<&'a MalEnv<'a>>,
}

fn default_add(args: MalList) -> MalResult<MalType> {
    Ok(MalType::Int(
        args.into_iter().fold(0, |acc, e| {
            if let MalType::Int(i) = e { acc + i } else { acc }
        })
    ))
}

fn default_sub(args: MalList) -> MalResult<MalType> {
    // So far assumes there are only two arguments, and panics otherwise.
    if args.len() == 2 {
        if let Some(MalType::Int(i1)) = args.get(0) {
            if let Some(MalType::Int(i2)) = args.get(1) {
                return Ok(MalType::Int(i1 - i2));
            }
        }
    }
    Err(MalError::RuntimeError(format!("(-) has invalid arguments: {:?}", args)))
}

fn default_mul(args: MalList) -> MalResult<MalType> {
    Ok(MalType::Int(
        args.into_iter().fold(1, |acc, e| {
            if let MalType::Int(i) = e { acc * i } else { acc }
        })
    ))
}

fn default_div(args: MalList) -> MalResult<MalType> {
    // So far assumes there are only two arguments, and panics otherwise.
    if args.len() == 2 {
        if let Some(MalType::Int(i1)) = args.get(0) {
            if let Some(MalType::Int(i2)) = args.get(1) {
                return Ok(MalType::Int(i1 / i2));
            }
        }
    }
    Err(MalError::RuntimeError(format!("(/) has invalid arguments: {:?}", args)))
}

impl<'a> MalEnv<'a> {
    pub fn default() -> MalEnv<'a> {
        let mut env = MalEnv::new(None);
        env.set(String::from("+"), MalType::Fun(default_add));
        env.set(String::from("-"), MalType::Fun(default_sub));
        env.set(String::from("*"), MalType::Fun(default_mul));
        env.set(String::from("/"), MalType::Fun(default_div));
        env
    }

    pub fn new(outer: Option<&'a MalEnv>) -> MalEnv<'a> {
        MalEnv {
            map: HashMap::new(),
            outer: {
                if let Some(e) = outer {
                    Some(&e)
                } else {
                    None
                }
            }
        }
    }

    pub fn set(&mut self, key: String, val: MalType) {
        self.map.insert(key, val);
    }

    pub fn find(&self, key: &str) -> bool {
        if self.map.contains_key(key) {
            true
        } else {
            if let Some(env) = self.outer {
                env.find(key)
            } else {
                false
            }
        }
    }

    pub fn get(&self, key: &String) -> Option<MalType> {
        // Technically this isn't implemented the way the instruction suggested, which may or may
        // not become important later.
        if self.map.contains_key(key) {
            Some(self.map.get(key).unwrap().clone())
        } else {
            if let Some(env) = self.outer {
                env.get(key)
            } else {
                None
            }
        }
    }
}

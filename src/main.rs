use boss::CSPStreamWorkerPool;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::thread;

#[derive(Clone)]
struct Data {
    line: String,
    keys: Vec<String>,
}

fn process_line(data: Data) -> Vec<Option<KeyType>> {
    let vl: std::result::Result<Value, serde_json::error::Error> = serde_json::from_str(&data.line);
    let mut vr = Vec::new();
    match vl {
        Ok(v) => {
            for k in &data.keys {
                let randkey = &v[k];
                let mut kt = KeyType {
                    key: k.to_string(),
                    vtype: VType::Null,
                };
                let mut res: Option<KeyType> = None;
                match randkey {
                    // String or String cast'able to a number
                    Value::String(s) => {
                        if s.parse::<f32>().is_ok() {
                            kt.vtype = VType::CastableStringToNumber;
                        } else {
                            kt.vtype = VType::String;
                        }
                        res = Some(kt);
                    }
                    Value::Number(_) => {
                        kt.vtype = VType::Number;
                        res = Some(kt);
                    }
                    Value::Array(_) => {
                        kt.vtype = VType::Array;
                        res = Some(kt);
                    }
                    Value::Bool(_) => {
                        kt.vtype = VType::Bool;
                        res = Some(kt);
                    }
                    Value::Object(_) => {
                        kt.vtype = VType::JSON;
                        res = Some(kt);
                    }
                    _ => (),
                }
                vr.push(res);
            }
        }
        Err(_) => {
            // Print it, for further evaluation.
        }
    }
    vr
}

#[derive(Clone)]
struct KeyType {
    key: String,
    vtype: VType,
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
enum VType {
    String,
    Number,
    CastableStringToNumber,
    Array,
    Bool,
    JSON,
    Null,
}

impl fmt::Display for VType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VType::String => write!(f, "String"),
            VType::Number => write!(f, "Number"),
            VType::CastableStringToNumber => write!(f, "CastableStringToNumber"),
            VType::Array => write!(f, "Array"),
            VType::Bool => write!(f, "Bool"),
            VType::JSON => write!(f, "JSON"),
            VType::Null => write!(f, "Does not exist"),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut types: HashMap<String, HashSet<VType>> = HashMap::new();
    let boss = CSPStreamWorkerPool::new(None, Some(50_000), process_line);
    let keys: Vec<_> = env::args().skip(1).collect();
    let rv = boss.clone();
    thread::spawn(move || {
        for line in stdin.lock().lines() {
            boss.send_data(Data {
                line: line.unwrap(),
                keys: keys.to_owned(),
            });
        }
        boss.finish();
    });
    for r in rv {
        for okt in r {
            if let Some(kt) = okt {
                let e = types.entry(kt.key).or_insert_with(HashSet::new);
                e.insert(kt.vtype);
            }
        }
    }
    // Display results
    for (key, vtype) in &types {
        println!("{}: {:?}", key, vtype);
    }
}

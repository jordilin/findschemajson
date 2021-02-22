use serde_json::Value;
use std::env;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    io::BufReader,
};

use rayon::prelude::*;
use std::sync::mpsc::channel;

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
    let input = BufReader::new(stdin);
    let mut types: HashMap<String, HashSet<VType>> = HashMap::new();
    let keys: Vec<_> = env::args().skip(1).collect();
    let (sender, receiver) = channel();
    // provide several functions - type keys, different keys, etc... depending on args
    // and pass it over to the mapper below.
    mapper(input, keys, sender);
    reducer(receiver, &mut types);
    // Display results
    // Do something in here at some point
    display(types);
}

fn mapper(
    input: BufReader<io::Stdin>,
    keys: Vec<String>,
    sender: std::sync::mpsc::Sender<Vec<Option<KeyType>>>,
) {
    input
        .lines()
        .map(Result::unwrap)
        .par_bridge()
        .map(|line| {
            // TODO pass process_line as a parameter - accept any function
            // that can process data
            process_line(Data {
                line: line.clone(),
                keys: keys.to_owned(),
            })
        })
        .for_each_with(sender, |s, k| s.send(k).unwrap());
}

fn reducer(
    receiver: std::sync::mpsc::Receiver<Vec<Option<KeyType>>>,
    types: &mut HashMap<String, HashSet<VType>>,
) {
    for r in receiver {
        for okt in r {
            if let Some(kt) = okt {
                let e = types.entry(kt.key).or_insert_with(HashSet::new);
                e.insert(kt.vtype);
            }
        }
    }
}

fn display(types: HashMap<String, HashSet<VType>>) {
    for (key, vtype) in types {
        println!("{}: {:?}", key, vtype);
    }
}

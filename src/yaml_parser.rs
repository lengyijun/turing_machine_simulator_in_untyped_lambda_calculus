//! yaml format is subset of <https://turingmachine.io>, but more stricter:
//! symbols : `0` and `1`
//! `blank` must be `0`
//! if no `write`, write `0` by default (not tested)
//! `next state` must be provided
//! `input` will be ignored currently

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stmt {
    pub write: Option<String>,
    pub L: Option<String>,
    pub R: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transition01 {
    #[serde(rename(deserialize = "0"))]
    pub zero: Option<Stmt>,

    #[serde(rename(deserialize = "1"))]
    pub one: Option<Stmt>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TuringMachine {
    pub blank: char,

    #[serde(rename(deserialize = "start state"))]
    pub start_state: String,

    // state -> (alpha -> Stmt)
    pub table: HashMap<String, Transition01>,
    // don't support input
    // input: Option<String>,
}

impl TuringMachine {
    pub fn validate(&self) {
        assert_eq!(self.blank, '0');
    }

    pub fn states(&self) -> HashSet<String> {
        let mut res: HashSet<String> = self.table.keys().cloned().collect();
        res.insert(self.start_state.clone());
        res
    }
}

#[test]
fn main() {
    let yaml_str = include_str!("../tm_yaml/1RB1LB_1LA1RZ.yaml");

    let tm: TuringMachine = serde_yaml::from_str(yaml_str).unwrap();
    tm.validate();
    println!("{:?}", tm);
    assert_eq!(tm, TuringMachine {
        blank: '0',
        start_state: "A".to_owned(),
        table: HashMap::from([
            ("A".to_owned(), Transition01 {
                zero: Some(Stmt {
                    write: Some("1".to_owned()),
                    L: None,
                    R: Some("B".to_owned())
                }),
                one: Some(Stmt {
                    write: Some("1".to_owned()),
                    L: Some("B".to_owned()),
                    R: None,
                })
            }),
            ("B".to_owned(), Transition01 {
                zero: Some(Stmt {
                    write: Some("1".to_owned()),
                    L: Some("A".to_owned()),
                    R: None,
                }),
                one: None
            })
        ])
    });
}

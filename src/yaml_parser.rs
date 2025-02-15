//! yaml format is subset of <https://turingmachine.io>, but more stricter:
//! symbols : `0` and `1`
//! `blank` must be `0`
//! if no `write`, write `0` by default (not tested)
//! `next state` must be provided
//! `input` will be ignored currently

use indexmap::IndexMap;
use indexmap::IndexSet;
use serde::Deserialize;
use serde::Serialize;
use serde_yaml::Value;

type State = String;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stmt {
    pub write: Option<usize>,
    pub L: Option<State>,
    pub R: Option<State>,
}

impl Stmt {
    fn states(&self) -> impl Iterator<Item = State> {
        [&self.L, &self.R]
            .into_iter()
            .filter_map(Option::as_ref)
            .cloned()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transition01 {
    #[serde(rename(deserialize = "0"))]
    pub zero: Option<Stmt>,

    #[serde(rename(deserialize = "1"))]
    pub one: Option<Stmt>,
}

impl Transition01 {
    fn from_value(value: Value) -> Self {
        let mut res = Self {
            zero: None,
            one: None,
        };
        let Value::Mapping(mapping) = value else {
            unreachable!()
        };
        for (k, v) in mapping {
            let k: usize = serde_yaml::from_value(k).unwrap();
            let v: Stmt = serde_yaml::from_value(v).unwrap();
            match k {
                0 => res.zero = Some(v),
                1 => res.one = Some(v),
                _ => unreachable!(),
            }
        }
        res
    }
    fn states(&self) -> impl Iterator<Item = State> {
        [&self.zero, &self.one]
            .into_iter()
            .flatten()
            .flat_map(Stmt::states)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct RawTuringMachine {
    blank: char,

    #[serde(rename(deserialize = "start state"))]
    start_state: State,

    // state -> (alpha -> Stmt)
    table: Value,
    // don't support input
    // input: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TuringMachine {
    pub blank: char,

    pub start_state: State,

    // state -> (alpha -> Stmt)
    pub table: IndexMap<State, Transition01>,
    // don't support input
    // input: Option<String>,
}

impl TuringMachine {
    pub fn new(yaml_str: &str) -> Self {
        let raw_tm: RawTuringMachine = serde_yaml::from_str(yaml_str).unwrap();

        let Value::Mapping(mapping) = raw_tm.table else {
            unreachable!()
        };
        let mut table = IndexMap::new();
        for (k, v) in mapping {
            let key: State = serde_yaml::from_value(k).unwrap();
            let transition: Transition01 = Transition01::from_value(v);
            table.insert(key, transition);
        }

        Self {
            blank: raw_tm.blank,
            start_state: raw_tm.start_state,
            table,
        }
    }

    pub fn validate(&self) {
        assert_eq!(self.blank, '0');
    }

    /// The order is important
    /// start_state first, then other states
    pub fn states(&self) -> IndexSet<State> {
        let mut res: IndexSet<State> = IndexSet::new();
        res.insert(self.start_state.clone());
        res.extend(self.table.keys().cloned());
        res.extend(self.table.values().flat_map(Transition01::states));
        res
    }
}

#[test]
fn test_transition() {
    let t = Transition01 {
        zero: Some(Stmt {
            write: Some(1),
            L: None,
            R: Some("B".to_owned()),
        }),
        one: Some(Stmt {
            write: Some(1),
            L: Some("B".to_owned()),
            R: None,
        }),
    };
    println!("{:?}", serde_yaml::to_value(t));
}

#[test]
fn main() {
    let yaml_str = include_str!("../tm_yaml/1RB1LB_1LA1RZ.yaml");

    let tm: TuringMachine = TuringMachine::new(yaml_str);
    tm.validate();
    println!("{:?}", tm);
    assert_eq!(tm, TuringMachine {
        blank: '0',
        start_state: "A".to_owned(),
        table: IndexMap::from([
            ("A".to_owned(), Transition01 {
                zero: Some(Stmt {
                    write: Some(1),
                    L: None,
                    R: Some("B".to_owned())
                }),
                one: Some(Stmt {
                    write: Some(1),
                    L: Some("B".to_owned()),
                    R: None,
                })
            }),
            ("B".to_owned(), Transition01 {
                zero: Some(Stmt {
                    write: Some(1),
                    L: Some("A".to_owned()),
                    R: None,
                }),
                one: None
            })
        ])
    });
}

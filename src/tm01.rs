//! compile crate::yaml_parser::* into more structured one

use crate::turing_machine::tape::move_left;
use crate::turing_machine::tape::move_right;
use crate::yaml_parser::TuringMachine;
use bimap::BiMap;
use lambda_calculus::IntoChurchNum;
use lambda_calculus::Term;
use lambda_calculus::Var;
use lambda_calculus::abs;
use lambda_calculus::app;
use lambda_calculus::data::boolean::fls;
use lambda_calculus::data::boolean::tru;
use lambda_calculus::data::num::church::eq;
use lambda_calculus::data::pair::pair;
use lambda_calculus::tuple;

type State = usize;

#[derive(Debug)]
enum Mmove {
    L,
    R,
}

impl Mmove {
    fn term(&self) -> Term {
        match self {
            Mmove::L => move_left(),
            Mmove::R => move_right(),
        }
    }
}

#[derive(Debug)]
struct Stmt {
    write: usize,
    mmove: Mmove,
    state: State,
}

impl Stmt {
    fn new(stmt: &crate::yaml_parser::Stmt, env: &Env) -> Self {
        let mmove;
        let state;

        match (&stmt.L, &stmt.R) {
            (None, None) => unreachable!(),
            (None, Some(s)) => {
                mmove = Mmove::R;
                state = *env.bmap.get_by_left(s).unwrap();
            }
            (Some(s), None) => {
                mmove = Mmove::L;
                state = *env.bmap.get_by_left(s).unwrap();
            }
            (Some(_), Some(_)) => panic!("Both L and R provided"),
        }

        Self {
            write: stmt.write.as_ref().map(|s| s.parse().unwrap()).unwrap_or(0),
            mmove,
            state,
        }
    }

    fn term(stmt: Option<&Self>, current_state: State) -> Term {
        match stmt {
            Some(stmt) => {
                app!(
                    pair(),
                    tru(),
                    tuple!(
                        stmt.write.into_church(),
                        stmt.mmove.term(),
                        stmt.state.into_church()
                    )
                )
            }
            None => {
                app!(pair(), fls(), current_state.into_church())
            }
        }
    }
}

#[derive(Debug)]
struct Transition01 {
    zero: Option<Stmt>,
    one: Option<Stmt>,
}

impl Transition01 {
    fn from_transition(hm: &crate::yaml_parser::Transition01, env: &Env) -> Self {
        let with_env_stmt = |stmt| Stmt::new(stmt, env);

        Self {
            zero: hm.zero.as_ref().map(with_env_stmt),
            one: hm.one.as_ref().map(with_env_stmt),
        }
    }

    /// current_state is only used when halt
    fn term(&self, current_state: State) -> Term {
        app!(
            eq(),
            Var(1),
            0.into_church(),
            Stmt::term(self.zero.as_ref(), current_state),
            Stmt::term(self.one.as_ref(), current_state)
        )
    }
}

#[derive(Debug)]
pub struct Tm01 {
    /// table[0] is start_state
    table: Vec<Transition01>,
}

impl Tm01 {
    pub fn from_tm(tm: TuringMachine) -> Self {
        let env = Env::new(&tm);

        let mut table = Vec::new();

        for i in 0..env.bmap.len() {
            let state = env.bmap.get_by_right(&i).unwrap();
            table.push(Transition01::from_transition(&tm.table[state], &env));
        }

        Self { table }
    }

    /// @return
    /// state → tape_head → <boolean, X>
    ///                         true, (write × move × state)
    ///                        false, state
    ///
    pub fn machine(&self) -> Term {
        // at least one state
        assert!(!self.table.is_empty());

        // only one state
        // trival case
        if self.table.len() == 1 {
            return abs!(2, self.table[0].term(0));
        }

        abs!(2, machine(0, &self.table))
    }
}

fn machine(current_state: State, v: &[Transition01]) -> Term {
    assert!(v.len() >= 2);

    if current_state == v.len() - 2 {
        app!(
            eq(),
            Var(2),
            current_state.into_church(),
            v[current_state].term(current_state),
            v[current_state + 1].term(current_state + 1)
        )
    } else {
        app!(
            eq(),
            Var(2),
            current_state.into_church(),
            v[current_state].term(current_state),
            machine(current_state + 1, v)
        )
    }
}

struct Env {
    bmap: BiMap<String, State>,
}

impl Env {
    fn new(tm: &TuringMachine) -> Self {
        let states = tm.states();

        let mut bmap: BiMap<String, State> = BiMap::new();

        for state in states {
            bmap.insert_no_overwrite(state, bmap.len()).unwrap();
        }
        assert_eq!(bmap.get_by_left(&tm.start_state), Some(&0));
        Self { bmap }
    }
}

#[test]
fn main() {
    let yaml_str = include_str!("../tm_yaml/1RB1LB_1LA1RZ.yaml");

    let tm: TuringMachine = TuringMachine::new(yaml_str);
    tm.validate();

    let tm01 = Tm01::from_tm(tm);
    println!("{:?}", tm01);
}

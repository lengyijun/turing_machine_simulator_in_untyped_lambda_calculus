pub mod tm01;
pub mod turing_machine;
pub mod yaml_parser;

use crate::tm01::Tm01;
use crate::yaml_parser::TuringMachine;
use lambda_calculus::Term;

pub fn yaml_to_turing_machine(yaml_str: &str) -> Term {
    let tm: TuringMachine = TuringMachine::new(yaml_str);
    let tm01 = Tm01::from_tm(tm);

    tm01.machine()
}

#[cfg(test)]
mod test_bb {
    // Examples copied from https://wiki.bbchallenge.org/wiki/BB(2)

    use crate::turing_machine::tape::run;
    use crate::yaml_to_turing_machine;
    use lambda_calculus::*;

    #[test]
    fn test_bb2() {
        assert_eq!(
            beta(
                run(yaml_to_turing_machine(include_str!(
                    "../tm_yaml/0RB1RZ_1LA1RB.yaml"
                ))),
                HSP,
                0
            ),
            0.into_church() // halt at state A
        );

        assert_eq!(
            beta(
                run(yaml_to_turing_machine(include_str!(
                    "../tm_yaml/1RB0LB_1LA1RZ.yaml"
                ))),
                HSP,
                0
            ),
            1.into_church() // halt at state B
        );

        assert_eq!(
            beta(
                run(yaml_to_turing_machine(include_str!(
                    "../tm_yaml/1RB1LB_1LA1RZ.yaml"
                ))),
                HSP,
                0
            ),
            1.into_church() // halt at state B
        );

        assert_eq!(
            beta(
                run(yaml_to_turing_machine(include_str!(
                    "../tm_yaml/1RB1RZ_0LB1LA.yaml"
                ))),
                HSP,
                0
            ),
            0.into_church() // halt at state A
        );

        // need 7 minutes in debug mode
        // need 3 minutes in release mode
        assert_eq!(
            beta(
                run(yaml_to_turing_machine(include_str!(
                    "../tm_yaml/1RB1RZ_1LB1LA.yaml"
                ))),
                HSP,
                0
            ),
            0.into_church() // halt at state A
        );
    }

    #[test]
    fn test_bb3() {
        // 5 steps
        // need 3 minutes in release mode
        assert_eq!(
            beta(
                run(yaml_to_turing_machine(include_str!(
                    "../tm_yaml/1RB---_0LB1LC_0RC1RZ.yaml"
                ))),
                HSP,
                0
            ),
            1.into_church() // halt at state C
                            // C -> 1 is choosen by random
        );

        // 6 steps
        // need 10 minutes in release mode
        assert_eq!(
            beta(
                run(yaml_to_turing_machine(include_str!(
                    "../tm_yaml/1RB1RZ_1LC0LA_---1LA.yaml"
                ))),
                HSP,
                0
            ),
            0.into_church() // halt at state A
        );

        // 6 steps
        // need 18 minutes in release mode
        assert_eq!(
            beta(
                run(yaml_to_turing_machine(include_str!(
                    "../tm_yaml/1RB1LB_1RC0RC_0LA1RZ.yaml"
                ))),
                HSP,
                0
            ),
            1.into_church() // halt at state C
                            // C -> 1 is choosen by random
        );
    }
}

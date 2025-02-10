use lambda_calculus::data::boolean::tru;
use lambda_calculus::data::num::church::eq;
use lambda_calculus::data::pair::pair;
use lambda_calculus::*;
use turing_machine_simulator_in_untyped_lambda_calculus::turing_machine::tape::*;

/// state → tape_head → <boolean, X>
///                         true, (write × move × state)
///                        false, state
fn bb2() -> Term {
    abs!(
        2,
        app!(
            eq(),
            Var(2),
            0.into_church(),
            // state = A
            app!(
                eq(),
                Var(1),
                0.into_church(),
                app!(
                    pair(),
                    tru(),
                    tuple!(1.into_church(), move_right(), 1.into_church())
                ),
                app!(
                    pair(),
                    tru(),
                    tuple!(1.into_church(), move_left(), 1.into_church())
                )
            ),
            // state = B
            app!(
                eq(),
                Var(1),
                0.into_church(),
                app!(
                    pair(),
                    tru(),
                    tuple!(1.into_church(), move_left(), 0.into_church())
                ),
                app!(
                    pair(),
                    tru(),
                    tuple!(1.into_church(), move_left(), 0.into_church())
                )
            )
        )
    )
}

/*
#[test]
fn test_bb2() {
    // expect : beta() diverge
    assert_eq!(beta(run(bb2()), HSP, 0), 1.into_church()); // wrong answer

    // assert_eq!(beta(run(bb2), CBN, 0), 1.into_church()); // wrong answer
    // assert_eq!(beta(run(bb2), HNO, 0), 1.into_church()); // stack overflow
    // assert_eq!(beta(run(bb2), NOR, 0), 1.into_church()); // stack overflow
}

 */

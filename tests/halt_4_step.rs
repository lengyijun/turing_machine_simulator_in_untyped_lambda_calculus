use lambda_calculus::data::boolean::{fls, tru};
use lambda_calculus::data::num::church::eq;
use lambda_calculus::data::pair::pair;
use lambda_calculus::*;
use turing_machine_simulator_in_untyped_lambda_calculus::turing_machine::tape::*;

/// state → tape_head → <boolean, X>
///                         true, (write × move × state)
///                        false, state
///
/// halt after 2 step
///
///        0     1
/// A    1RB    1RB
/// B    1LA    halt
///
///
/// A   [0]
/// B    1[0]
/// A   [1]1
/// B    1[1]
/// halt
///
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
                    tuple!(1.into_church(), move_right(), 1.into_church())
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
                app!(pair(), fls(), 1.into_church())
            )
        )
    )
}

#[test]
fn test_bb2() {
    assert_eq!(beta(run(bb2()), HSP, 0), 1.into_church());
}

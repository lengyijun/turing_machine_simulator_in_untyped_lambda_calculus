use lambda_calculus::combinators::Y;
use lambda_calculus::data::list::church::cons;
use lambda_calculus::data::list::church::nil;
use lambda_calculus::data::num::church::eq;
use lambda_calculus::data::num::church::mul;
use lambda_calculus::data::num::church::pred;
use lambda_calculus::*;
use turing_machine_simulator_in_untyped_lambda_calculus::turing_machine::tape::*;

#[test]
fn test_y() {
    let f = app!(
        Y(),
        abs!(
            2,
            app!(
                eq(),
                Var(1),
                1.into_church(),
                1.into_church(),
                app!(mul(), Var(1), app(Var(2), app(pred(), Var(1))))
            )
        ),
        3.into_church()
    );
    assert_eq!(beta(f.clone(), NOR, 0), 6.into_church());
    assert_eq!(beta(f.clone(), HNO, 0), 6.into_church());
    // assert_eq!(beta(f.clone(), CBN, 0), 6.into_church());
    // assert_eq!(beta(f.clone(), HSP, 0), 6.into_church());
}

#[test]
fn test_new_tape() {
    assert_eq!(beta(app!(pi!(1, 3), new_tape()), HSP, 0), 0.into_church());
    assert_eq!(beta(app!(pi!(2, 3), new_tape()), HSP, 0), nil());
    assert_eq!(beta(app!(pi!(3, 3), new_tape()), HSP, 0), nil());
}

#[test]
fn test_write() {
    let x = app!(write(), 0.into_church(), new_tape());
    assert_eq!(beta(app!(pi!(2, 3), x.clone()), HSP, 0), nil(),);
    assert_eq!(beta(app!(pi!(3, 3), x.clone()), HSP, 0), nil());
    assert_eq!(beta(app!(pi!(1, 3), x.clone()), HSP, 0), 0.into_church());
}

#[test]
fn test_move_left() {
    let x = app!(move_left(), new_tape());
    assert_eq!(beta(app!(pi!(1, 3), x.clone()), HSP, 0), 0.into_church());
    assert_eq!(
        beta(app!(pi!(2, 3), x.clone()), NOR, 0),
        beta(app!(cons(), 0.into_church(), nil()), NOR, 0)
    );
    assert_eq!(beta(app!(pi!(3, 3), x.clone()), HSP, 0), nil());
}

#[test]
fn test_move_right() {
    let x = app!(move_right(), new_tape());
    assert_eq!(beta(app!(pi!(1, 3), x.clone()), HSP, 0), 0.into_church());
    assert_eq!(beta(app!(pi!(2, 3), x.clone()), HSP, 0), nil(),);
    assert_eq!(
        beta(app!(pi!(3, 3), x.clone()), NOR, 0),
        beta(app!(cons(), 0.into_church(), nil()), NOR, 0)
    );
}

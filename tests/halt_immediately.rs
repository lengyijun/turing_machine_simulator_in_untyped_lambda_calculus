use lambda_calculus::data::boolean::fls;
use lambda_calculus::data::pair::pair;
use lambda_calculus::*;
use turing_machine_simulator_in_untyped_lambda_calculus::turing_machine::tape::*;

/// state → tape_head → <boolean, X>
///                         true, (write × move × state)
///                        false, state
fn halt_immediately() -> Term {
    abs!(2, app!(pair(), fls(), 1.into_church()))
}

#[test]
fn test_halt_immediately() {
    assert_eq!(beta(run(halt_immediately()), HSP, 0), 1.into_church());
    assert_eq!(beta(run(halt_immediately()), CBN, 0), 1.into_church());
    assert_eq!(beta(run(halt_immediately()), HNO, 0), 1.into_church());
    assert_eq!(beta(run(halt_immediately()), NOR, 0), 1.into_church());
}

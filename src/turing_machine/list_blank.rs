//! Infinite list
//! <https://leanprover-community.github.io/mathlib4_docs/Mathlib/Computability/Tape.html#Turing.ListBlank>
//! This file is based on data.list.church

use lambda_calculus::Var;
use lambda_calculus::abs;
use lambda_calculus::app;
use lambda_calculus::data::boolean::{fls, tru};
use lambda_calculus::data::num::convert::IntoChurchNum;
use lambda_calculus::data::pair::{fst, pair, snd};
use lambda_calculus::term::Term;

/// Produces a `nil`, the last link of a Church-encoded list; equivalent to `boolean::tru`.
///
/// NIL ≡ λab.a ≡ λ λ 2 ≡ TRUE
pub fn nil() -> Term {
    tru()
}

/// Applied to a Church-encoded list it determines if it is empty.
///
/// IS_NIL ≡ λl.l TRUE (λax.FALSE) ≡ λ 1 TRUE (λ λ FALSE)
///
/// # Example
/// ```
/// use lambda_calculus::data::list::church::{is_nil, nil};
/// use lambda_calculus::*;
///
/// assert_eq!(beta(app(is_nil(), nil()), NOR, 0), true.into());
/// ```
pub fn is_nil() -> Term {
    abs(app!(Var(1), tru(), abs!(2, fls())))
}

/// Applied to two terms it returns them contained in a Church-encoded list.
///
/// CONS ≡ λaxnc.c a ((λl.l) x n c) ≡ λ λ λ λ 1 4 ((λ 1) 3 2 1)
///
/// # Example
/// ```
/// use lambda_calculus::data::list::church::{nil, cons};
/// use lambda_calculus::*;
///
/// let list_consed =
///     app!(
///         cons(),
///         1.into_church(),
///         app!(
///             cons(),
///             2.into_church(),
///             app!(
///                 cons(),
///                 3.into_church(),
///                 nil()
///             )
///         )
///     );
///
/// let list_into = vec![1, 2, 3].into_church();
///
/// assert_eq!(
///     beta(list_consed, NOR, 0),
///     list_into
/// );
/// ```
pub fn cons() -> Term {
    abs!(
        4,
        app!(Var(1), Var(4), app!(abs(Var(1)), Var(3), Var(2), Var(1)))
    )
}

/// Applied to a Church-encoded list it returns its first element.
/// If list is empty, return `0`
///
/// HEAD ≡ λl.l UD (λht.h) ≡ λ 1 UD (λ λ 2)
///
/// # Example
/// ```
/// use lambda_calculus::data::list::church::head;
/// use lambda_calculus::*;
///
/// let list = vec![1, 2, 3].into_church();
///
/// assert_eq!(
///     beta(app(head(), list), NOR, 0),
///     1.into_church()
/// );
/// ```
pub fn head() -> Term {
    abs(app!(Var(1), 0_usize.into_church(), abs!(2, Var(2))))
}

/// Applied to a Church-encoded list it returns a new list with all its elements but the first one.
/// If list is empty, return `nil()`
///
/// TAIL ≡ λl.FST (l (PAIR UD NIL) (λap. PAIR (SND p) (CONS a (SND p))))
///      ≡ λ FST (1 (PAIR UD NIL) (λ λ PAIR (SND 1) (CONS 2 (SND 1))))
///
/// # Example
/// ```
/// use lambda_calculus::data::list::church::tail;
/// use lambda_calculus::*;
///
/// let list = vec![1, 2, 3].into_church();
///
/// assert_eq!(
///     beta(app(tail(), list), NOR, 0),
///     vec![2, 3].into_church()
/// );
/// ```
pub fn tail() -> Term {
    abs(app!(
        fst(),
        app!(
            Var(1),
            app!(pair(), nil(), nil()),
            abs!(
                2,
                app!(
                    pair(),
                    app(snd(), Var(1)),
                    app!(cons(), Var(2), app(snd(), Var(1)))
                )
            )
        )
    ))
}

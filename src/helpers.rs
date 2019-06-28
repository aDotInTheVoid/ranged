//! Internal helpers
use crate::{Num, Range};
use std::cmp::{Ord, Ordering};

/// Internal metadata on a `Range`
struct meta {
    /// Is one of the values `0`
    zero: bool,
    /// Is one of the values less then `0`
    neg: bool,
    /// Is one of the values more then `0`
    pos: bool,
}

enum Domain {
    Plus,
    PlusInc,
    Zero,
    NegInc,
    Neg,
    All,
    Error,
}

fn domain<T: Num+Ord>(range: Range<T>) -> Domain {
    let min = range.min;
    let max = range.max;
    match min.cmp(&T::zero()) {
        Ordering::Less => {domain_min_neg(max)}
        Ordering::Equal => {domain_min_0(max)}
        Ordering::Greater => {Domain::PlusInc}
    }
}

fn domain_min_neg<T: Num>(val: T) -> Domain {
    match val.cmp(&T::zero()) {
        Ordering::Less => {Domain::Neg}
        Ordering::Equal => {Domain::NegInc}
        Ordering::Greater => {Domain::All}
    }
}

fn domain_min_0<T: Num>(val: T) -> Domain {
    match val.cmp(&T::zero()) {
        Ordering::Less => {Domain::Error}
        Ordering::Equal => {Domain::Zero}
        Ordering::Greater => {Domain::PlusInc}
    }
}

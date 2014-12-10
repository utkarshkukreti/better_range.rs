use std::num::Int;

pub struct Range<T> {
    from: T,
    to: T,
    step: T,
    done: bool
}

pub trait Step {
    fn zero() -> Self;
    fn one() -> Self;
    fn next(from: Self, step: Self) -> Option<Self>;
    fn infinity() -> Self;
}

impl Step for int {
    fn zero() -> int { 0 }
    fn one() -> int { 1 }
    fn next(from: int, step: int) -> Option<int> {
        from.checked_add(step)
    }
    fn infinity() -> int { Int::max_value() }
}

pub fn from<T: Step>(from: T) -> Range<T> {
    Range {
        from: from,
        to: Step::infinity(),
        step: Step::one(),
        done: false
    }
}

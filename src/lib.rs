use std::num::Int;

pub struct Range<T> {
    from: T,
    to: T,
    step: T
}

pub trait Step {
    fn zero() -> Self;
    fn one() -> Self;
    fn add(Self, Self) -> Option<Self>;
}

impl Step for int {
    fn zero() -> int { 0 }
    fn one() -> int { 1 }
    fn add(a: int, b: int) -> Option<int> { a.checked_add(b) }
}

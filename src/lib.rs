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

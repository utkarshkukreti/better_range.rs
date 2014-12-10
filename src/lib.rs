#![feature(macro_rules, phase)]
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

impl<T: Copy + PartialOrd + Step> Iterator<T> for Range<T> {
    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else if self.from > self.to {
            None
        } else {
            let ret = self.from;
            match Step::next(self.from, self.step) {
                Some(new) => self.from = new,
                None => self.done = true
            }
            Some(ret)
        }
    }
}

pub fn from<T: Step>(from: T) -> Range<T> {
    Range {
        from: from,
        to: Step::infinity(),
        step: Step::one(),
        done: false
    }
}

pub fn to<T: Step>(to: T) -> Range<T> {
    Range {
        from: Step::zero(),
        to: to,
        step: Step::one(),
        done: false
    }
}

pub fn step<T: Step>(step: T) -> Range<T> {
    Range {
        from: Step::zero(),
        to: Step::infinity(),
        step: step,
        done: false
    }
}

#[cfg(test)]
mod test {
    #[phase(plugin)]
    extern crate stainless;

    pub use super::{from, step, to};

    macro_rules! eq {
        ($range:expr, $slice:expr) => {
            assert_eq!($range.collect::<Vec<_>>().as_slice(), $slice);
        }
    }

    describe! better_range {
        it "works for trivial cases" {
            eq!(from(-1).take(4), [-1, 0, 1, 2])
            eq!(from(1).take(5), [1, 2, 3, 4, 5]);
            eq!(to(4), [0, 1, 2, 3, 4])
            eq!(step(4).take(5), [0, 4, 8, 12, 16]);
        }
    }
}

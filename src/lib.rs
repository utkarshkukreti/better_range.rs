#![feature(macro_rules, phase)]
use std::num::Int;

pub struct Range<T> {
    from: T,
    to: T,
    step: T,
    done: bool,
    reverse: bool,
    inclusive: bool
}

pub trait Step: Copy + PartialOrd {
    fn zero() -> Self;
    fn one() -> Self;
    fn next(from: Self, step: Self) -> Option<Self>;
    fn infinity() -> Self;
}

macro_rules! impl_step_int {
    ($($ty:ty),+) => {
        $(
            impl Step for $ty {
                fn zero() -> $ty { 0 }
                fn one() -> $ty { 1 }
                fn next(from: $ty, step: $ty) -> Option<$ty> {
                    from.checked_add(step)
                }
                fn infinity() -> $ty { Int::max_value() }
            }
        )+
    }
}

impl_step_int!(u8, u16, u32, u64, uint, i8, i16, i32, i64, int)

impl<T: Step> Iterator<T> for Range<T> {
    #[inline]
    fn next(&mut self) -> Option<T> {
        match (self.done, self.reverse, self.inclusive) {
            (true, _, _) => None,
            (_, true, true)   if self.to > self.from  => None,
            (_, true, false)  if self.to >= self.from => None,
            (_, false, true)  if self.from > self.to  => None,
            (_, false, false) if self.from >= self.to => None,
            _ => {
                let ret = self.from;
                match Step::next(self.from, self.step) {
                    Some(new) => self.from = new,
                    None => self.done = true
                }
                Some(ret)
            }
        }
    }
}

pub fn from<T: Step>(from: T) -> Range<T> {
    Range::new().from(from)
}

pub fn to<T: Step>(to: T) -> Range<T> {
    Range::new().to(to)
}

pub fn until<T: Step>(until: T) -> Range<T> {
    Range::new().until(until)
}

pub fn step<T: Step>(step: T) -> Range<T> {
    Range::new().step(step)
}

impl<T: Step> Range<T> {
    pub fn new() -> Range<T> {
        Range {
            from: Step::zero(),
            to: Step::infinity(),
            step: Step::one(),
            done: false,
            reverse: false,
            inclusive: true
        }
    }

    pub fn from(self, from: T) -> Range<T> {
        Range {
            from: from,
            ..self
        }
    }

    pub fn to(self, to: T) -> Range<T> {
        Range {
            to: to,
            inclusive: true,
            ..self
        }
    }

    pub fn until(self, until: T) -> Range<T> {
        Range {
            to: until,
            inclusive: false,
            ..self
        }
    }

    pub fn step(self, step: T) -> Range<T> {
        let reverse = step < Step::zero();
        Range {
            step: step,
            reverse: reverse,
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    #[phase(plugin)]
    extern crate stainless;

    pub use self::test::Bencher;
    pub use super::{from, step, to};

    macro_rules! eq {
        ($range:expr, $slice:expr) => {
            assert_eq!($range.collect::<Vec<_>>().as_slice(), $slice);
        }
    }

    describe! better_range {
        it "works for trivial cases" {
            eq!(from(-1i).take(4), [-1, 0, 1, 2])
            eq!(from(1i).take(5), [1, 2, 3, 4, 5]);
            eq!(to(4i), [0, 1, 2, 3, 4])
            eq!(step(4i).take(5), [0, 4, 8, 12, 16]);
        }

        it "handles chaining" {
            eq!(from(0i).to(10).step(2), [0, 2, 4, 6, 8, 10])
            eq!(from(0i).step(20).take(4), [0, 20, 40, 60])
        }

        it "works with negative steps" {
            eq!(from(10i).to(0).step(-3), [10, 7, 4, 1]);
            eq!(from(0i).to(10).step(-3), []);
            eq!(from(-10i).to(-20).step(-5), [-10, -15, -20]);
        }

        it "handles exclusive ranges" {
            eq!(from(10i).until(20).step(5), [10, 15])
            eq!(from(10i).until(-10).step(-5), [10, 5, 0, -5]);
        }

        describe! benches {
            bench "native range 1 to 1 million" (b) {
                b.iter(|| {
                    let mut ret = 0;
                    for i in ::std::iter::range(1i, 1_000_000) {
                        ret ^= i;
                    }
                    ret
                });
            }

            bench "better_range from 1 to 1 million" (b) {
                b.iter(|| {
                    let mut ret = 0;
                    for i in from(1i).until(1_000_000) {
                        ret ^= i;
                    }
                    ret
                });
            }

            bench "native range_step 1 to 10 million step 10" (b) {
                b.iter(|| {
                    let mut ret = 0;
                    for i in ::std::iter::range_step(1i, 10_000_000, 10) {
                        ret ^= i;
                    }
                    ret
                });
            }

            bench "better_range from 1 to 10 million step 10" (b) {
                b.iter(|| {
                    let mut ret = 0;
                    for i in from(1i).until(10_000_000).step(10) {
                        ret ^= i;
                    }
                    ret
                });
            }
        }
    }
}

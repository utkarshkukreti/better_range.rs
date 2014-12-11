#![feature(macro_rules, phase)]
use std::num::{Float, Int};

pub struct Range<T> {
    from: T,
    to: Option<T>,
    done: bool,
    inclusive: bool
}

pub struct RangeStep<T> {
    from: T,
    to: Option<T>,
    step: T,
    done: bool,
    reverse: bool,
    inclusive: bool
}

pub trait First {
    fn first() -> Self;
}

pub trait Next {
    fn next(now: Self) -> Option<Self>;
}

pub trait Step<T> {
    fn default() -> Self;
    fn step(now: Self, step: T) -> Option<Self>;
    fn is_negative(step: T) -> bool;
}

macro_rules! impl_step_int {
    ($($ty:ty),+) => {
        $(
            impl First for $ty {
                fn first() -> $ty { 0 }
            }

            impl Next for $ty {
                fn next(now: $ty) -> Option<$ty> {
                    if now == Int::max_value() {
                        None
                    } else {
                        Some(now + 1)
                    }
                }
            }

            impl Step<$ty> for $ty {
                fn default() -> $ty { 1 }
                fn step(now: $ty, step: $ty) -> Option<$ty> {
                    now.checked_add(step)
                }
                fn is_negative(step: $ty) -> bool {
                    step < 0
                }
            }
        )+
    }
}

impl_step_int!(u8, u16, u32, u64, uint, i8, i16, i32, i64, int)

macro_rules! impl_step_float {
    ($($ty:ty),+) => {
        $(
            impl First for $ty {
                fn first() -> $ty { 0.0 }
            }

            impl Next for $ty {
                fn next(now: $ty) -> Option<$ty> {
                    Some(now + 1.0)
                }
            }

            impl Step<$ty> for $ty {
                fn default() -> $ty { 1.0 }
                fn step(now: $ty, step: $ty) -> Option<$ty> {
                    Some(now + step)
                }
                fn is_negative(step: $ty) -> bool {
                    step < 0.0
                }
            }
        )+
    }
}

impl_step_float!(f32, f64)

impl Next for char {
    fn next(now: char) -> Option<char> {
        std::char::from_u32(now as u32 + 1)
    }
}

impl<T: Copy + Next + PartialOrd> Iterator<T> for Range<T> {
    #[inline]
    fn next(&mut self) -> Option<T> {
        match (self.done, self.inclusive, self.to) {
            (true, _, _) => None,
            (_, false, Some(to)) if self.from >= to => None,
            (_, true, Some(to)) if self.from > to => None,
            _ => {
                let ret = self.from;
                match Next::next(self.from) {
                    Some(new) => self.from = new,
                    None => self.done = true
                }
                Some(ret)
            }
        }
    }
}

impl<T: Copy + PartialOrd + Step<T>> Iterator<T> for RangeStep<T> {
    #[inline]
    fn next(&mut self) -> Option<T> {
        match (self.done, self.inclusive, self.reverse, self.to) {
            (true, _, _, _) => None,
            (_, false, false, Some(to)) if self.from >= to => None,
            (_, false, true, Some(to)) if self.from <= to => None,
            (_, true, false, Some(to)) if self.from > to => None,
            (_, true, true, Some(to)) if self.from < to => None,
            _ => {
                let ret = self.from;
                match Step::step(self.from, self.step) {
                    Some(new) => self.from = new,
                    None => self.done = true
                }
                Some(ret)
            }
        }
    }
}

pub fn from<T: Next>(from: T) -> Range<T> {
    Range {
        from: from,
        to: None,
        inclusive: true,
        done: false
    }
}

pub fn to<T: First + Next>(to: T) -> Range<T> {
    Range {
        from: First::first(),
        to: Some(to),
        inclusive: true,
        done: false
    }
}

pub fn until<T: First + Next>(until: T) -> Range<T> {
    Range {
        from: First::first(),
        to: Some(until),
        inclusive: true,
        done: false
    }
}

pub fn step<T: Copy + First + Next + Step<T>>(step: T) -> RangeStep<T> {
    RangeStep {
        from: First::first(),
        to: None,
        step: step,
        inclusive: true,
        reverse: Step::is_negative(step),
        done: false
    }
}

impl<T: Next> Range<T> {
    fn to(self, to: T) -> Range<T> {
        Range {
            to: Some(to),
            inclusive: true,
            ..self
        }
    }

    fn until(self, to: T) -> Range<T> {
        Range {
            to: Some(to),
            inclusive: false,
            ..self
        }
    }
}

impl<T: Copy + First + Next + Step<T>> Range<T> {
    fn step(self, step: T) -> RangeStep<T> {
        RangeStep {
            from: self.from,
            to: self.to,
            step: step,
            inclusive: self.inclusive,
            reverse: Step::is_negative(step),
            done: false
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
            eq!(to(4.0f32), [0., 1., 2., 3., 4.])
            eq!(step(0.4f32).take(3), [0.0, 0.4, 0.8])
        }

        it "handles chaining" {
            eq!(from(0i).to(10).step(2), [0, 2, 4, 6, 8, 10])
            eq!(from(0i).step(20).take(4), [0, 20, 40, 60])
            eq!(from(1.1f32).to(2.2).step(0.4), [1.1, 1.5, 1.9])
        }

        it "works with negative steps" {
            eq!(from(10i).to(0).step(-3), [10, 7, 4, 1]);
            eq!(from(0i).to(10).step(-3), []);
            eq!(from(-10i).to(-20).step(-5), [-10, -15, -20]);
            eq!(from(-10.0f32).to(-20.).step(-5.), [-10., -15., -20.]);
        }

        it "handles exclusive ranges" {
            eq!(from(10i).until(20).step(5), [10, 15])
            eq!(from(10i).until(-10).step(-5), [10, 5, 0, -5]);
            eq!(from(10.0f32).until(-10.0).step(-5.), [10.0, 5.0, 0.0, -5.0]);
        }

        it "handles edge cases for about-to-{over,under}flow integers" {
            eq!(from(252u8), [252, 253, 254, 255])
            eq!(from(125i8), [125, 126, 127])
            eq!(from(240u8).step(5), [240, 245, 250, 255])
            eq!(from(115i8).step(5), [115, 120, 125])
            eq!(from(-123i8).step(-1), [-123, -124, -125, -126, -127, -128])
        }

        it "handles char ranges" {
            eq!(from('a').to('c'), ['a', 'b', 'c'])
            eq!(from('0').to('5'), ['0', '1', '2', '3', '4', '5'])
            eq!(from(::std::char::MAX), [::std::char::MAX])
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

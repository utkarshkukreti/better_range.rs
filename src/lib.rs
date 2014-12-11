#![feature(macro_rules, phase)]
use std::num::Int;

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
                #[allow(unused_comparisons)]
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

pub fn range<T: First + Next>() -> Range<T> {
    Range {
        from: First::first(),
        to: None,
        inclusive: true,
        done: false
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
    pub fn to(self, to: T) -> Range<T> {
        Range {
            to: Some(to),
            inclusive: true,
            ..self
        }
    }

    pub fn until(self, to: T) -> Range<T> {
        Range {
            to: Some(to),
            inclusive: false,
            ..self
        }
    }
}

impl<T: Copy + First + Next + Step<T>> Range<T> {
    pub fn step(self, step: T) -> RangeStep<T> {
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

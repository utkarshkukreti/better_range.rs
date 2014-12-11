#![feature(macro_rules, phase)]
extern crate better_range;

extern crate test;
#[phase(plugin)]
extern crate stainless;

pub use better_range::{from, step, to, range};

macro_rules! eq {
    ($range:expr, $slice:expr) => {
        assert_eq!($range.collect::<Vec<_>>().as_slice(), $slice);
    }
}

describe! better_range_ {
    it "works for trivial cases" {
        eq!(from(-1i).take(4), [-1, 0, 1, 2])
        eq!(from(1i).take(5), [1, 2, 3, 4, 5]);
        eq!(to(4i), [0, 1, 2, 3, 4])
        eq!(step(4i).take(5), [0, 4, 8, 12, 16]);
        eq!(to(4.0f32), [0., 1., 2., 3., 4.])
        eq!(step(0.4f32).take(3), [0.0, 0.4, 0.8])
        eq!(range::<u8>().take(5), [0, 1, 2, 3, 4])
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
        eq!(range::<i8>().step(-4).take(5), [0, -4, -8, -12, -16])
    }

    it "handles exclusive ranges" {
        eq!(from(10i).until(20).step(5), [10, 15])
        eq!(from(10i).until(-10).step(-5), [10, 5, 0, -5]);
        eq!(from(10.0f32).until(-10.0).step(-5.), [10.0, 5.0, 0.0, -5.0]);
        eq!(range().until(5u), [0, 1, 2, 3, 4])
    }

    it "handles edge cases for about-to-{over,under}flow integers" {
        eq!(from(252u8), [252, 253, 254, 255])
        eq!(from(125i8), [125, 126, 127])
        eq!(from(240u8).step(5), [240, 245, 250, 255])
        eq!(from(115i8).step(5), [115, 120, 125])
        eq!(from(-123i8).step(-1), [-123, -124, -125, -126, -127, -128])
        eq!(range::<u8>().step(100), [0, 100, 200])
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

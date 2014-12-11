#![feature(macro_rules, phase)]
extern crate better_range;

extern crate test;
#[phase(plugin)]
extern crate stainless;

pub use better_range::{from, step, to, range};

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

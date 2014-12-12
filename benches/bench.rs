#![feature(macro_rules, phase)]
extern crate better_range;

extern crate test;
#[phase(plugin)]
extern crate stainless;

pub use better_range::{from, step, to, range};
pub use test::black_box;

describe! benches {
    bench "while loop 1 to 1 million" (b) {
        b.iter(|| {
            let mut i = 0i;
            while i < 1_000_000 {
                black_box(i);
                i += 1
            }
        });
    }

    bench "native range 1 to 1 million" (b) {
        b.iter(|| {
            for i in ::std::iter::range(1i, 1_000_000) {
                black_box(i)
            }
        });
    }

    bench "better_range from 1 to 1 million" (b) {
        b.iter(|| {
            for i in from(1i).until(1_000_000) {
                black_box(i)
            }
        });
    }

    bench "while loop 1 to 10 million step 10" (b) {
        b.iter(|| {
            let mut i = 0i;
            while i < 10_000_000 {
                black_box(i);
                i += 10
            }
        });
    }

    bench "native range_step 1 to 10 million step 10" (b) {
        b.iter(|| {
            for i in ::std::iter::range_step(1i, 10_000_000, 10) {
                black_box(i)
            }
        });
    }

    bench "better_range from 1 to 10 million step 10" (b) {
        b.iter(|| {
            for i in from(1i).until(10_000_000).step(10) {
                black_box(i)
            }
        });
    }
}

#[macro_use]
extern crate criterion;

use criterion::{Bencher, Criterion, Fun};

extern crate common;

use common::{bytes_reflow, bytes_reflow_in_place, bytes_reflow_in_place_reserve};

fn fibonaccis(c: &mut Criterion) {
    const runs: usize = 1 << 10;
    let copied = Fun::new(
        "Copied",
        |b: &mut Bencher, (v, width): &(Vec<u8>, usize)| {
            b.iter(|| {
                let mut used = v.clone();
                for _ in 0..runs {
                    used = bytes_reflow(&used, *width)
                }
            })
        },
    );
    let in_place = Fun::new(
        "In Place",
        |b: &mut Bencher, (v, width): &(Vec<u8>, usize)| {
            b.iter(|| {
                let mut used = v.clone();
                bytes_reflow_in_place_reserve(&mut used, *width);
                for _ in 0..runs {
                    bytes_reflow_in_place(&mut used, *width)
                }
            })
        },
    );

    let functions = vec![copied, in_place];

    let text = b"The byte is a unit of digital information that most commonly consists of eight bits, representing a binary number.";

    c.bench_functions(
        "Reflowing Text",
        functions,
        (text.to_vec(), "representing".len()),
    );
}

criterion_group!(benches, fibonaccis);
criterion_main!(benches);

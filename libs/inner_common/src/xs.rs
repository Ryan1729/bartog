pub type Seed = [u8; 16];

pub type Xs = [core::num::Wrapping<u32>; 4];

fn xorshift(xs: &mut Xs) -> u32 {
    let mut t = xs[3];

    xs[3] = xs[2];
    xs[2] = xs[1];
    xs[1] = xs[0];

    t ^= t << 11;
    t ^= t >> 8;
    xs[0] = t ^ xs[0] ^ (xs[0] >> 19);

    xs[0].0
}

use core::ops::Range;

pub fn range(xs: &mut Xs, range: Range<u32>) -> u32 {
    let min = range.start;
    let one_past_max = range.end;

    (xorshift(xs) % (one_past_max - min)) + min
}

const SCALE: u32 = 1 << f32::MANTISSA_DIGITS;

#[allow(unused)]
fn zero_to_one(xs: &mut Xs) -> f32 {
    (range(xs, 0..SCALE + 1) as f32 / SCALE as f32) - 1.
}

#[allow(unused)]
fn minus_one_to_one(xs: &mut Xs) -> f32 {
    (range(xs, 0..(SCALE * 2) + 1) as f32 / SCALE as f32) - 1.
}

pub fn shuffle<A>(xs: &mut Xs, slice: &mut [A]) {
    for i in 1..slice.len() as u32 {
        // This only shuffles the first u32::MAX_VALUE - 1 elements.
        let r = range(xs, 0..i + 1) as usize;
        let i = i as usize;
        slice.swap(i, r);
    }
}

pub fn new_seed(xs: &mut Xs) -> Seed {
    let s0 = xorshift(xs).to_le_bytes();
    let s1 = xorshift(xs).to_le_bytes();
    let s2 = xorshift(xs).to_le_bytes();
    let s3 = xorshift(xs).to_le_bytes();

    [
        s0[0], s0[1], s0[2], s0[3],
        s1[0], s1[1], s1[2], s1[3],
        s2[0], s2[1], s2[2], s2[3],
        s3[0], s3[1], s3[2], s3[3],
    ]
}

pub fn from_seed(mut seed: Seed) -> Xs {
    // 0 doesn't work as a seed, so use this one instead.
    if seed == [0; 16] {
        seed = 0xBAD_5EED_u128.to_le_bytes();
    }

    macro_rules! wrap {
        ($i0: literal, $i1: literal, $i2: literal, $i3: literal) => {
            core::num::Wrapping(
                u32::from_le_bytes([
                    seed[$i0],
                    seed[$i1],
                    seed[$i2],
                    seed[$i3],
                ])
            )
        }
    }

    [
        wrap!( 0,  1,  2,  3),
        wrap!( 4,  5,  6,  7),
        wrap!( 8,  9, 10, 11),
        wrap!(12, 13, 14, 15),
    ]
}

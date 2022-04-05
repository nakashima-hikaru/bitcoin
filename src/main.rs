#![feature(test)]
extern crate test;
use primitive_types::U256;

use crate::{secp256k1::S256Point, signature::Signature};
mod field_element;
mod point;
mod secp256k1;
mod signature;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_multiple(b: &mut Bencher) {
        let g = S256Point::get_generic_point();
        let n = S256Point::get_order_of_generic_point();
        b.iter(|| n * g);
    }
}

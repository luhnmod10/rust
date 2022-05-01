#![feature(test)]
extern crate luhnmod10;
extern crate test;

#[bench]
fn bench_valid(b: &mut test::Bencher) {
    b.iter(|| luhnmod10::valid("4242424242424242"));
}

#[bench]
fn bench_valid_long(b: &mut test::Bencher) {
    b.iter(|| luhnmod10::valid("42424242424242424242424242424242424242424242424242424242424242424242424242424240"));
}

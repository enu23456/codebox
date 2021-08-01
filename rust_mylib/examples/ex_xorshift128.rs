//! # ex_xorshift128.rs
//! モンテカルロ法による円周率の計算

extern crate rust_mylib;

fn main() {
    let mut counter: f64 = 0.0;
    let trials = 1_000_000;
    let mut rng = rust_mylib::lib_rng::xorshift128::Xorshift128::initialize(0);
    for _ in 0..trials {
        let x: u64 = rng.next() as u64;
        let y: u64 = rng.next() as u64;
        //println!("{}, {}", x, y);
        if 0xFFFF_FFFF_FFFF_FFFF - (x * x) > (y * y) {
            counter += 1.0;
        }
    }
    println!("pi is {}", (4.0 * counter) / trials as f64);
    return;
}

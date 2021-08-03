//! # ex_xorshift128.rs
//! モンテカルロ法による円周率の計算

extern crate rust_mylib;

fn main() {
    println!("calc pi by Monte Carlo method");

    println!("RNG: xorshift128");
    let mut counter0: f32 = 0.0;
    let trials = 1_000_000;
    let mut rng = rust_mylib::lib_rng::xorshift128::Xorshift128::initialize(0);
    for _ in 0..trials {
        let x: u64 = rng.next() as u64;
        let y: u64 = rng.next() as u64;
        if 0xFFFF_FFFF_FFFF_FFFF - (x * x) > (y * y) {
            counter0 += 1.0;
        }
    }
    println!("pi is {} ", (4.0 * counter0) / trials as f32);

    println!("RNG: xorshift128");
    let mut counter1: f32 = 0.0;
    let trials = 1_000_000;
    let mut rng = rust_mylib::lib_rng::lcg64::LCG64::initialize(0);
    for _ in 0..trials {
        let x: u128 = rng.next() as u128;
        let y: u128 = rng.next() as u128;
        if 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF - (x * x) > (y * y) {
            counter1 += 1.0;
        }
    }
    println!("pi is {} ", (4.0 * counter1) / trials as f32);
    println!("RNG: xorshift128");
    let mut counter2: f32 = 0.0;
    let trials = 1_000_000;
    let mut rng = rust_mylib::lib_rng::splitmix64::Splitmix64::initialize(0);
    for _ in 0..trials {
        let x: u128 = rng.next() as u128;
        let y: u128 = rng.next() as u128;
        //println!("{}, {}", x, y);
        if 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF - (x * x) > (y * y) {
            counter2 += 1.0;
        }
    }
    println!("pi is {} ", (4.0 * counter2) / trials as f32);
    return;
}

//! モンテカルロ法により円周率を求めるプログラム
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! rand = "0.8.3"

use rand::prelude::*;

const NUM_TRIAL: u64 = 1_000_000;
const RADIUS: f64 = 1.0;

fn main() {
    let mut points: u64 = 0;
    let mut rng = thread_rng();
    for _ in 0..NUM_TRIAL {
        let x: f64 = rng.gen_range(0.0..RADIUS);
        let y: f64 = rng.gen_range(0.0..RADIUS);
        let dist_sq: f64 = (x * x) + (y * y);
        if dist_sq <= (RADIUS * RADIUS) {
            points += 1;
        }
        //println!("x:{}, y:{}, dist_sq:{}", x, y, dist_sq);
    }
    println!("pi = {}", (points as f64) * 4.0 / (NUM_TRIAL as f64))
}

//! ライプニッツ級数の計算を題材とした、処理時間計測プログラム
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! なし

use std::time::Instant;

fn main() {
    const CALC_NUM: u32 = 10_000_000;
    let timer = Instant::now();
    let mut pi_leibniz: f64 = 0.0;
    println!("calc start!!");
    for n in 0..CALC_NUM {
        // この for の部分がライプニッツ級数の計算部分
        pi_leibniz += i64::pow(-1, n) as f64 / ((2 * n + 1) as f64);
    }
    let elasped = timer.elapsed();
    println!("calc end!!");
    println!("PI = {:?}, at {} calc", 4.0 * pi_leibniz, CALC_NUM);
    println!("elasped time: {} [milli-sec]", elasped.subsec_millis());
}

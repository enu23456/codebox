//! 線形探索と二分探索を行い、計算量の比較を行うプログラム
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! rand = "0.8.3"

use rand::prelude::*;

const NUM_MAX: u64 = 100;

fn serch_linear(num: u64) -> u64 {
    let mut cycle = 1;
    let mut temp = 0;
    loop {
        if temp == num {
            break;
        } else {
            cycle += 1;
            temp += 1;
        }
    }
    return cycle;
}
fn serch_binary(num: u64) -> u64 {
    let mut cycle = 1;
    let mut temp = NUM_MAX >> 1;
    loop {
        //print!("{},", temp);  // for debug
        if temp == num || cycle >= 0xFFFF_FFFF_FFFF_FFFF {
            //println!();  // for debug
            break;
        } else {
            cycle += 1;
            if num > temp {
                temp += (NUM_MAX >> cycle) + 1;
            } else {
                temp -= (NUM_MAX >> cycle) + 1;
            }
        }
    }
    return cycle;
}

fn main() {
    const TRIALS: u64 = 10;
    let mut avg_linear: f64 = 0.0;
    let mut avg_binary: f64 = 0.0;
    let mut rng = thread_rng();
    /* // for debug. 0から99までの数字全てでちゃんと収束するかどうかを確認
    for i in 0..99 {
        let calc_binary: u64 = serch_binary(i);
        println!("num:{}, cycle:{}", i, calc_binary);
    }
    */
    for i in 0..TRIALS {
        let num: u64 = rng.gen_range(0..NUM_MAX);
        let calc_linear: u64 = serch_linear(num as u64);
        let calc_binary: u64 = serch_binary(num as u64);
        println!(
            "{}th number: {}. calc_linear: {}. calc_binary: {}",
            i, num, calc_linear, calc_binary
        );
        avg_linear += calc_linear as f64;
        avg_binary += calc_binary as f64;
    }
    avg_linear /= 10.0;
    avg_binary /= 10.0;
    println!(
        "average calc_num ... linear: {}, binary: {}",
        avg_linear, avg_binary
    )
}

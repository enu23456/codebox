//! 1 から NUM_MAX までの数に対する約数カウントと素数判定を行う
//!
//! 素数とは、約数が２つだけ存在する自然数
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! なし

fn main() {
    const NUM_MAX: u64 = 15;
    for i in 1..NUM_MAX {
        let mut divisor: u64 = 0;
        for j in 1..(i + 1) {
            if i % j == 0 {
                divisor += 1;
            }
        }
        println!(
            "{}: divisor = {}, is_prime = {}",
            i,
            divisor,
            (divisor == 2)
        );
    }
}

//! Fizzbuzz プログラム
//!
//! FizzBuzz とは、下記のルールに従って出力（発言）する遊びある
//!   3の倍数の場合は「Fizz」
//!   5の倍数の場合は「Buzz」
//!   15の倍数の場合は「Fizz Buzz」
//!   それ以外の場合は数字
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! なし

fn main() {
    const NUM_END: u32 = 20;
    let mut x: u32 = 1;

    while x <= NUM_END {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
        x += 1;
    }
}

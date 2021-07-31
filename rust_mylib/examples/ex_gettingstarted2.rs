//! mathライブラリの使用例

use rust_mylib::lib_gettingstarted::odd_checker;

fn main() {
    // 素数判定関数の使用例
    let arg = 5;
    if odd_checker::is_odd(arg) == true {
        println!("{} is odd", arg)
    } else {
        println!("{} is not odd", arg)
    }
    
    return;
}
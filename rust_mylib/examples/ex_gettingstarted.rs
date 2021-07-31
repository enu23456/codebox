//! mathライブラリの使用例

extern crate rust_mylib;

fn main() {
    // 素数判定関数の使用例
    let arg = 5;
    if rust_mylib::lib_gettingstarted::odd_checker::is_odd(arg) == true {
        println!("{} is odd", arg)
    } else {
        println!("{} is not odd", arg)
    }
    
    return;
}
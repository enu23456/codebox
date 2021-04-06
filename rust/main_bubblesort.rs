//! バブルソートを行うプログラム
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! なし
//!
//! 参考資料
//! https://qiita.com/jlkiri/items/f0bb8b1fa9da3bfc68ee

/// 引数で与えられたベクタに対してバブルソートを行う
fn sort_bubble(array: &mut Vec<i32>) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j + 1] < array[j] {
                array.swap(j, j + 1);
                // println!("{:?}", array);
            }
        }
    }
}

fn main() {
    let mut array = vec![3, 2, 5, 1, 4];
    println!("{:?}", array);
    sort_bubble(&mut array);
    println!("{:?}", array);
}

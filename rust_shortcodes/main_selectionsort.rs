//! 選択ソートを行うプログラム
//!
//! 選択ソートとは、最大値やまたは最小値を探索し配列最後の要素と入れ替える手法
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! なし
//!
//! 参考資料
//! https://webbibouroku.com/Blog/Article/rust-sort-algo

/// 引数で与えられたベクタに対して選択ソートを行う
fn sort_selection(array: &mut Vec<i32>) {
    for i in 0..array.len() - 1 {
        let mut min = i;
        for j in i..array.len() {
            if array[min] > array[j] {
                min = j;
            }
        }
        array.swap(i, min);
        println!("i:{}, min:{}, {:?}", i, min, array);
    }
}

fn main() {
    let mut array = vec![3, 2, 5, 1, 4];
    println!("{:?}", array);
    sort_selection(&mut array);
    println!("{:?}", array);
}

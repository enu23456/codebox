//! Rust のテスト方法確認用プログラム
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! なし
//!
//! テストに関するメモ
//! + Rustが参照できるのはlib.rs, Cargo.tomlに定義されたクレートのみ。つまりmain.rsへのテストは書けない。
//!
//! + 慣習として、テストはソースファイル中のローカルなtestsモジュールにまとめることになっている。
//!   これにより、テスト以外の設定でビルドしたときにテスト用の関数がビルド対象から除外されるメリットがある。
//! + テストの実行の仕方：cargo test
//! 参考文献
//! http://ytyaru.hatenablog.com/entry/2020/09/18/000000
//! https://tech-blog.optim.co.jp/entry/2019/02/19/173000

pub fn add_two(a: i32) -> i32 {
    return internal_adder(a, 2);
}

fn internal_adder(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)] // cargo testコマンド実行時だけビルドさせる（cargo build時はしない）ための宣言
mod tests {
    use super::*; // 非公開関数のテストを行うために必要
    #[test]
    fn internal() {
        //assert_eq!(4, add_two(2));
        assert_eq!(4, internal_adder(2, 2));
        //assert_eq!(5, internal_adder(2, 2));
    }
}

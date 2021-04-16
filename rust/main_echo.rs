//! 入力された文字をそのまま出力するプログラム
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! なし

fn main() {
    println!("文字を入力してください");
    let mut text = String::new(); // バッファ確保
    std::io::stdin().read_line(&mut text).unwrap(); //  一行読む
    text = text.trim_end().to_owned(); // 末尾に付く改行コードを削除
    print!("入力された文字は ");
    print!("{}", text);
    println!(" です。");
}

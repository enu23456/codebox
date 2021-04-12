//! 文字入力を行うプログラム
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! なし
//!
//! 参考資料
//! https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8

fn main() {
    println!("何か文字列を入力して下さい:");

    let mut text_input = String::new(); // バッファを確保
    std::io::stdin().read_line(&mut text_input).ok(); // 一行読む
    let text_trimed = text_input.trim().to_string(); // 改行コードが末尾にくっついてくるので削る

    println!("入力された文字列の文字数。改行を含めた文字数が含まれます");
    println!("{}", text_input.chars().count());
    println!("配列に格納されている文字コードは以下の通りです");
    println!("text: {:?}", text_input.as_bytes());

    println!("入力された文字列の文字数。そのままの文字数が表示されます");
    println!("{}", text_trimed.chars().count());
    println!("配列に格納されている文字コードは以下の通りです");
    println!("text: {:?}", text_trimed.as_bytes());

    return;
}

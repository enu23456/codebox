//! コンソールからの入力に対し、暗号化を行いその結果を出力を行うプログラム
//!
//! 暗号化処理は、簡単な換字式であり、１文字単位で各文字をビット反転させるものである
//! 換字式暗号: Substitution cipher
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! なし

fn substitute(x: u8) -> u8 {
    return !x;
}

fn main() {
    println!("何か文字列を入力して下さい:");

    let mut input = String::new(); // バッファを確保
    std::io::stdin().read_line(&mut input).ok(); // 一行読む

    let mut text: Vec<char> = input.trim().chars().collect(); // char配列に変換
    println!("input text: {:?}", text);
    println!("--- start convert ---");
    for i in 0..text.len() {
        print!(
            "{} th word: {}(0x{:X}). ",
            i, text[i] as char, text[i] as u8
        );
        text[i] = substitute(text[i] as u8) as char;
        println!("convert: {}(0x{:x})", text[i] as char, text[i] as u8);
    }
    println!("--- end convert ---");
    println!("converted text: {:?}", text);
    // 入力した時（input）と同じ型にする時は以下を追加すること
    //let text_str: String = text.iter().collect();
    //println!("{}", &text_str);
}

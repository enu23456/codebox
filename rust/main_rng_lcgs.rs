//! 線形合同法により乱数を生成するプログラム
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! なし

/// 乱数を格納する変数
struct Rand32 {
    value: u32,
}

impl Rand32 {
    /// 乱数値を引数で初期化する
    /// 引数に0を入力すると計算式上乱数が上手く機能しない（ずっと0が出力される）点に注意すること
    fn initialize(x: u32) -> Rand32 {
        return Rand32 { value: x };
    }

    /// 乱数を回し（計算を行い）、その結果を返す
    fn next(&mut self) -> u32 {
        self.value = ((48271 * self.value as u64) & 0x0000_0000_FFFF_FFFF) as u32;
        return self.value;
    }
}

fn main() {
    let mut rnd = Rand32::initialize(123);
    for _ in 0..10 {
        println!("{}", rnd.next());
    }
    return;
}

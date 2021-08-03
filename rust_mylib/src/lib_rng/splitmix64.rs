//! # splitmix64.rs
//!
//! ## はじめに
//! 本モジュールは、splitmix64により乱数を生成するものである。
//!
//! ## 必要な依存関係（Cargo.toml の [dependencies]）
//! なし
//!
//! ## 参考資料
//! https://en.wikipedia.org/wiki/Xorshift

/// 乱数生成器のパラメータを格納する構造体
pub struct Splitmix64 {
    s: u64,
}

impl Splitmix64 {
    pub fn initialize(seed: u64) -> Splitmix64 {
        return Splitmix64 { s: seed };
    }

    pub fn next(&mut self) -> u64 {
        self.s = (self.s as u128 + 0x9E3779B97f4A7C15) as u64;
        let mut result: u64 = self.s;
        result = ((result ^ (result >> 30)) as u128 * 0xBF58476D1CE4E5B9) as u64;
        result = ((result ^ (result >> 27)) as u128 * 0x94D049BB133111EB) as u64;
        return result ^ (result >> 31);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_splitmix64() {
        let mut rnd = Splitmix64::initialize(0);
        assert_eq!(16294208416658607535, rnd.next());
        assert_eq!(7960286522194355700, rnd.next());
        assert_eq!(487617019471545679, rnd.next());
        assert_eq!(17909611376780542444, rnd.next());
        assert_eq!(1961750202426094747, rnd.next());
    }
}

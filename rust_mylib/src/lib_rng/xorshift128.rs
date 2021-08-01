//! # xorshift128.rs
//!
//! ## はじめに
//! 本モジュールは、Xorshiftにより乱数を生成するものである。
//!
//! ## 必要な依存関係（Cargo.toml の [dependencies]）
//! なし
//!
//! ## テスト
//! 下記リンクよりパラメータと出力値を参考にし、
//! 乱数生成値確認用として使用した。
//! https://www.timbreofprogram.info/blog/archives/384
//!
//! ## 所感
//! 全てのパラメータ（x,y,z,w)がゼロにならないように注意すること
//! ## 参考資料
//! https://ja.wikipedia.org/wiki/Xorshift
//! https://www.timbreofprogram.info/blog/archives/384

/// 乱数生成器のパラメータを格納する構造体
pub struct Xorshift128 {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl Xorshift128 {
    /// 乱数生成器を初期化する
    pub fn initialize(seed: u32) -> Xorshift128 {
        return Xorshift128 {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: seed,
        };
    }

    /// 乱数を回し（計算を行い）、その結果を返す
    pub fn next(&mut self) -> u32 {
        let t: u32 = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
        return self.w;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xorshift() {
        let mut rnd = Xorshift128::initialize(0);
        assert_eq!(3656013424, rnd.next());
        assert_eq!(504890837, rnd.next());
        assert_eq!(2421774896, rnd.next());
        assert_eq!(2421770299, rnd.next());
    }
}

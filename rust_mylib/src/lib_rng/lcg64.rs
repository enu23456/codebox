//! # lcg64.rs
//!
//! ## はじめに
//! 本モジュールは、線形合同法により乱数を生成するものである。
//!
//! ## 必要な依存関係（Cargo.toml の [dependencies]）
//! なし
//!
//! ## 乱数の生成式とパラメータについて
//! 線形合同法では下記の漸化式により次の出力値（乱数）を求めている
//! X(n+1) = (A * X(n) + B) mod M
//! 上記式のA, B, Mについては M > A, M > B, A > 0, B > 0 という制約がある。
//!
//! ## パラメータMの設定について
//! パラメータMについて、ゼロを入力すると2^64として計算を行う
//!
//! ## 参考資料
//! https://oupo.hatenadiary.com/entry/20171219/1513609225

/// 線形合同法による乱数生成器のパラメータを格納する構造体
pub struct LCG64 {
    a: u64,
    b: u64,
    m: u64,
    x: u64,
}

impl LCG64 {
    /// 乱数値を引数で初期化する
    pub fn initialize(seed: u64) -> LCG64 {
        return LCG64 {
            a: 0x5d588b656c078965,
            b: 0x269ec3,
            m: 0,
            x: seed,
        };
    }

    /// LCGのパラメータをセットする
    fn initialize_with_param(a: u64, b: u64, m: u64, seed: u64) -> LCG64 {
        return LCG64 {
            a: a,
            b: b,
            m: m,
            x: seed,
        };
    }

    /// 乱数を回し（計算を行い）、その結果を返す
    pub fn next(&mut self) -> u64 {
        if self.m == 0 {
            self.x = (((self.a as u128 * self.x as u128) + self.b as u128) & 0xFFFF_FFFF_FFFF_FFFF)
                as u64;
        } else {
            self.x = (((self.a as u128 * self.x as u128) + self.b as u128) % self.m as u128) as u64;
        }
        return self.x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lcg64() {
        let mut rnd = LCG64::initialize_with_param(0x5d588b656c078965, 0x269ec3, 0, 0);
        assert_eq!(2531011, rnd.next());
        assert_eq!(8181017474514197682, rnd.next());
        assert_eq!(789582199073780477, rnd.next());
    }
}

//! # simplehasher32.rs
//!
//! ## はじめに
//! 本モジュールは、MurmurHash3の引数を32bit数値に限定した簡易ハッシュである。
//! つまり、32bitの数値を、別の32bitの数値に変換するモジュールである。
//!
//! ## 必要な依存関係（Cargo.toml の [dependencies]）
//! なし
//!
//! ## 参考資料
//! https://en.wikipedia.org/wiki/MurmurHash

fn simplehasher32_calc(val: u32) -> u32 {
    let mut k = val;
    let mut h = 123456789;
    /* scramble */
    k = (k as u64 * 3432918353) as u32;
    k = (k << 15) | (k >> 17);
    k = (k as u64 * 461845907) as u32;
    h ^= k;
    /* finalize */
    h ^= 1;
    h ^= h >> 16;
    h = (h as u64 * 2246822507) as u32;
    h ^= h >> 13;
    h = (h as u64 * 3266489909) as u32;
    h ^= h >> 16;
    return h;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mur32add32_calc() {
        assert_eq!(3950633843, simplehasher32_calc(0));
        assert_eq!(743998833, simplehasher32_calc(1));
        assert_eq!(954056702, simplehasher32_calc(2));
        assert_eq!(3097307535, simplehasher32_calc(3));
        assert_eq!(2966544326, simplehasher32_calc(4));
    }
}

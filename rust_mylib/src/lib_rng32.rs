//! # lib_rng32.rs
//! ## はじめに
//! 本モジュールは、種々の32bit疑似乱数を生成する疑似乱数生成器をまとめたものである。
//! ユーザーが自由に疑似乱数生成器を選び、使用する事ができる
//!
//! ## 使用方法
//! exampleフォルダ内のex_mylib_prng32を確認してください
//!
//! ## 所感（やり残しなど含め）
//! + Rust分かんない
//! + 全然Rustっぽく書いてない感じが半端ない
//! + new_with_typeも本当はenumを引数に取りたかったが、そうすると上手くいかない
//! + 今後もRustを勉強しつつ、本ライブラリをブラッシュアップしていけると良いな

pub enum GeneratorType {
    XORSHIFT128,
    LCG32,
}

pub struct RNG32 {
    alpha: u32,
    beta: u32,
    gamma: u32,
    delta: u32,
    gentype: GeneratorType,
}

impl RNG32 {
    pub fn new(seed: u32) -> RNG32 {
        let param = generate_param(seed);
        let ret = RNG32 {
            alpha: param.0,
            beta: param.1,
            gamma: param.2,
            delta: param.3,
            gentype: GeneratorType::XORSHIFT128,
        };
        return ret;
    }
    pub fn new_with_type(seed: u32, type_num: u8) -> RNG32 {
        let param = generate_param(seed);
        let gentype: GeneratorType = match type_num {
            0 => GeneratorType::XORSHIFT128,
            _ => GeneratorType::LCG32,
        };
        let ret = RNG32 {
            alpha: param.0,
            beta: param.1,
            gamma: param.2,
            delta: param.3,
            gentype: gentype,
        };
        return ret;
    }
    pub fn reset(&mut self, seed: u32) {
        let param = generate_param(seed);
        self.alpha = param.0;
        self.beta = param.1;
        self.gamma = param.2;
        self.delta = param.3;
        return;
    }
    pub fn next(&mut self) -> u32 {
        match self.gentype {
            GeneratorType::LCG32 => {
                return self.next_lcg32();
            }
            GeneratorType::XORSHIFT128 => {
                return self.next_xorshift128();
            }
        }
    }

    fn next_lcg32(&mut self) -> u32 {
        self.delta = (((1103515245 * self.delta as u64) + 12345) & 0xFFFF_FFFF) as u32;
        return self.delta;
    }
    fn next_xorshift128(&mut self) -> u32 {
        let t: u32 = self.alpha ^ (self.alpha << 11);
        self.alpha = self.beta;
        self.beta = self.gamma;
        self.gamma = self.delta;
        self.delta = (self.delta ^ (self.delta >> 19)) ^ (t ^ (t >> 8));
        return self.delta;
    }
}

fn generate_param(seed: u32) -> (u32, u32, u32, u32) {
    let mut ret = (0u32, 0u32, 0u32, 0u32);

    let mut k = seed;
    let mut h = 2166136261;
    k = (k as u64 * 0xcc9e2d51) as u32;
    k = (k << 15) | (k >> 17);
    k = (k as u64 * 0x1b873593) as u32;
    h ^= k;
    h = (h << 13) | (h >> 19);
    h = ((h as u64) * 5 + 0xe6546b64) as u32;

    for i in 0..4 {
        h ^= h >> 16;
        h = (h as u64 * 0x85ebca6b) as u32;
        h ^= h >> 13;
        h = (h as u64 * 0xc2b2ae35) as u32;
        h ^= h >> 16;
        // Rustのタプルはret.iのようにはできないための苦肉の策
        match i {
            0 => ret.0 = h,
            1 => ret.1 = h,
            2 => ret.2 = h,
            _ => ret.3 = h,
        }
    }
    return ret;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lib_rng32() {
        let mut rnd = RNG32::new(0);
        assert_eq!(3598290415, rnd.next());
        assert_eq!(1646212294, rnd.next());
        assert_eq!(1022361848, rnd.next());
    }
}

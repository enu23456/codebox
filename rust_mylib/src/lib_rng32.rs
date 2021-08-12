//! # lib_rng32.rs
//! ## はじめに
//! 本モジュールは、種々の32bit疑似乱数を生成する疑似乱数生成器をまとめたものです。
//! ユーザーが自由に疑似乱数生成器（アルゴリズム）を選び、使用する事ができます。
//! また、性能よりも処理を容易に理解できること（平易な記述であること）を優先しています。
//!
//! ## 使用方法
//! exampleフォルダ内のex_lib_prng32を確認してください

/// 生成器の種類。それぞれの概要は以下の通り
/// + LCG32
/// LCG: Linear congruential generator, 線形合同法
/// X[n+1] = (A * X[n] + B) mod M の式より値を生成する方式
///
/// + XORSHIFT32x4
/// 演算が排他的論理和とビットシフトのみで構成された高速な乱数生成方式
/// 32bit変数4つで内部状態が構成されており、周期は2^32-1。
///
/// + XOSHIRO32x4p
/// xorshiftに加算を加えることで、より乱数の質を高めたもの。
///
/// + XOSHIRO32x4pp
/// XOSHIRO32x4pより更に乱数の質を高めたもの
///
/// + XOSHIRO32x4ss
/// XOSHIRO32x4pより更に乱数の質を高めたもの
/// XOSHIRO32x4ppとは計算処理の一部が微妙に違う

pub enum GeneratorType {
    LCG32,
    XORSHIFT32x4,
    XOSHIRO32x4p,
    XOSHIRO32x4pp,
    XOSHIRO32x4ss,
}

/// 乱数生成器の構造体
/// 内部状態を構成する配列と生成器の種類情報を持っています
pub struct RNG32 {
    state: [u32; 4],
    gentype: GeneratorType,
}

impl RNG32 {
    /// 引数の値をシードとして乱数生成器のインスタンスを生成します。
    /// 乱数生成アルゴリズムはxoroshiro32x4**で固定です。
    pub fn new(seed: u32) -> RNG32 {
        let param = generate_param(seed);
        let ret = RNG32 {
            state: param,
            gentype: GeneratorType::XOSHIRO32x4ss,
        };
        return ret;
    }
    /// 引数の値をシードとし、指定した乱数生成アルゴリズムにて乱数生成器のインスタンスを生成します。
    pub fn new_with_type(seed: u32, gentype: GeneratorType) -> RNG32 {
        let param = generate_param(seed);
        let ret = RNG32 {
            state: param,
            gentype: gentype,
        };
        return ret;
    }
    /// 乱数生成器のパラメータを、引数の値を元に設定します。
    pub fn reset(&mut self, seed: u32) {
        let param = generate_param(seed);
        self.state = param;
        return;
    }
    /// 乱数を生成します。
    pub fn next(&mut self) -> u32 {
        match self.gentype {
            GeneratorType::LCG32 => {
                return self.next_lcg32();
            }
            GeneratorType::XORSHIFT32x4 => {
                return self.next_xorshift32x4();
            }
            GeneratorType::XOSHIRO32x4p => {
                return self.next_xoshiro32x4p();
            }
            GeneratorType::XOSHIRO32x4pp => {
                return self.next_xoshiro32x4pp();
            }
            _ => {
                return self.next_xoshiro32x4ss();
            }
        }
    }

    fn next_lcg32(&mut self) -> u32 {
        self.state[3] = (((1103515245 * self.state[3] as u64) + 12345) & 0xFFFF_FFFF) as u32;
        return self.state[3];
    }
    fn next_xorshift32x4(&mut self) -> u32 {
        let t: u32 = self.state[0] ^ (self.state[0] << 11);
        self.state[0] = self.state[1];
        self.state[1] = self.state[2];
        self.state[2] = self.state[3];
        self.state[3] = (self.state[3] ^ (self.state[3] >> 19)) ^ (t ^ (t >> 8));
        return self.state[3];
    }
    fn next_xoshiro32x4p(&mut self) -> u32 {
        let result: u32 = (self.state[0] as u64 + self.state[3] as u64) as u32;
        let t: u32 = self.state[1] << 9;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = (self.state[3] << 11) | (self.state[3] >> 21);
        return result;
    }
    fn next_xoshiro32x4pp(&mut self) -> u32 {
        let temp: u64 = self.state[0] as u64 + self.state[3] as u64;
        let result: u32 = (((temp << 7) | (temp >> 25)) + self.state[0] as u64) as u32;
        let t: u32 = self.state[1] << 9;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = (self.state[3] << 11) | (self.state[3] >> 21);
        return result;
    }
    fn next_xoshiro32x4ss(&mut self) -> u32 {
        let temp: u64 = self.state[1] as u64 * 5;
        let result: u32 = (((temp << 7) | (temp >> 25)) * 9) as u32;
        let t: u32 = self.state[1] << 9;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = (self.state[3] << 11) | (self.state[3] >> 21);
        return result;
    }
}

/// 引数の値から乱数生成器に用いる4つの32bit整数を生成します。
fn generate_param(seed: u32) -> [u32; 4] {
    let mut ret = [0; 4];

    let mut k = seed;
    let mut h = 2166136261;
    k = (k as u64 * 3432918353) as u32;
    k = (k << 15) | (k >> 17);
    k = (k as u64 * 461845907) as u32;
    h ^= k;
    h = (h << 13) | (h >> 19);
    h = ((h as u64) * 5 + 3864292196) as u32;

    for i in 0..4 {
        h ^= h >> 16;
        h = (h as u64 * 2246822507) as u32;
        h ^= h >> 13;
        h = (h as u64 * 3266489909) as u32;
        h ^= h >> 16;
        ret[i] = h;
    }
    return ret;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lib_rng32() {
        let mut rng_lcg32 = RNG32::new_with_type(0, GeneratorType::LCG32);
        assert_eq!(2958721311, rng_lcg32.next());
        assert_eq!(1725602924, rng_lcg32.next());
        assert_eq!(3460674101, rng_lcg32.next());

        let mut rng_xoroshiro32x4 = RNG32::new_with_type(0, GeneratorType::XORSHIFT32x4);
        assert_eq!(3598290415, rng_xoroshiro32x4.next());
        assert_eq!(1646212294, rng_xoroshiro32x4.next());
        assert_eq!(1022361848, rng_xoroshiro32x4.next());

        let mut rng_xoroshiro32x4p = RNG32::new_with_type(0, GeneratorType::XOSHIRO32x4p);
        assert_eq!(528756791, rng_xoroshiro32x4p.next());
        assert_eq!(202781300, rng_xoroshiro32x4p.next());
        assert_eq!(1279680063, rng_xoroshiro32x4p.next());

        let mut rng_xoroshiro32x4pp = RNG32::new_with_type(0, GeneratorType::XOSHIRO32x4pp);
        assert_eq!(461455112, rng_xoroshiro32x4pp.next());
        assert_eq!(3909492950, rng_xoroshiro32x4pp.next());
        assert_eq!(1794869524, rng_xoroshiro32x4pp.next());

        let mut rng_xoroshiro32x4ss = RNG32::new_with_type(0, GeneratorType::XOSHIRO32x4ss);
        assert_eq!(4105684694, rng_xoroshiro32x4ss.next());
        assert_eq!(3685076946, rng_xoroshiro32x4ss.next());
        assert_eq!(3332898383, rng_xoroshiro32x4ss.next());
    }
}

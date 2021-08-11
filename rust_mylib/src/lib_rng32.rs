//! # lib_rng32.rs
//! ## はじめに
//! 本モジュールは、種々の32bit疑似乱数を生成する疑似乱数生成器をまとめたものである。
//! ユーザーが自由に疑似乱数生成器を選び、使用する事ができる
//!
//! ## 使用方法
//! exampleフォルダ内のex_mylib_prng32を確認してください
//!
//! ## 所感
//! new_with_typeでは、２番目の引数を数値ではなくenumで指定できるようにしたい

pub enum GeneratorType {
    LCG32,
    XORSHIFT32x4,
    XOROSHIRO32x4p,
    XOROSHIRO32x4pp,
    XOROSHIRO32x4ss,
}

pub struct RNG32 {
    state: [u32; 4],
    gentype: GeneratorType,
}

impl RNG32 {
    pub fn new(seed: u32) -> RNG32 {
        let param = generate_param(seed);
        let ret = RNG32 {
            state: param,
            gentype: GeneratorType::XOROSHIRO32x4ss,
        };
        return ret;
    }
    pub fn new_with_type(seed: u32, type_num: u8) -> RNG32 {
        let param = generate_param(seed);
        let gentype: GeneratorType = match type_num {
            0 => GeneratorType::LCG32,
            1 => GeneratorType::XORSHIFT32x4,
            2 => GeneratorType::XOROSHIRO32x4p,
            3 => GeneratorType::XOROSHIRO32x4pp,
            _ => GeneratorType::XOROSHIRO32x4ss,
        };
        let ret = RNG32 {
            state: param,
            gentype: gentype,
        };
        return ret;
    }
    pub fn reset(&mut self, seed: u32) {
        let param = generate_param(seed);
        self.state = param;
        return;
    }
    pub fn next(&mut self) -> u32 {
        match self.gentype {
            GeneratorType::LCG32 => {
                return self.next_lcg32();
            }
            GeneratorType::XORSHIFT32x4 => {
                return self.next_xorshift32x4();
            }
            GeneratorType::XOROSHIRO32x4p => {
                return self.next_xoroshiro32x4p();
            }
            GeneratorType::XOROSHIRO32x4pp => {
                return self.next_xoroshiro32x4pp();
            }
            _ => {
                return self.next_xoroshiro32x4ss();
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
    fn next_xoroshiro32x4p(&mut self) -> u32 {
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
    fn next_xoroshiro32x4pp(&mut self) -> u32 {
        let temp: u64 = self.state[0] as u64 + self.state[3] as u64;
        let result: u32 = (((temp << 7) | (temp >> 25)) * self.state[0] as u64) as u32;
        let t: u32 = self.state[1] << 9;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = (self.state[3] << 11) | (self.state[3] >> 21);
        return result;
    }
    fn next_xoroshiro32x4ss(&mut self) -> u32 {
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
        let mut rng_lcg32 = RNG32::new_with_type(0, 0);
        assert_eq!(2958721311, rng_lcg32.next());
        assert_eq!(1725602924, rng_lcg32.next());
        assert_eq!(3460674101, rng_lcg32.next());

        let mut rng_xoroshiro32x4ss = RNG32::new_with_type(0, 255);
        assert_eq!(4105684694, rng_xoroshiro32x4ss.next());
        assert_eq!(3685076946, rng_xoroshiro32x4ss.next());
        assert_eq!(3332898383, rng_xoroshiro32x4ss.next());
    }
}

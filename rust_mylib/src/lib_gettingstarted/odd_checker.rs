//! # gettingstarted.rs
//! ## はじめに
//! 本モジュールは動作確認やライブラリの雛形とすることを目的としたモジュールである。

/// 奇数か偶数かを判定する
/// ## Note
/// + 引数（判定を行う数値）の型はu32である点に留意すること
pub fn is_odd(num: u32) -> bool {
    return if num % 2 == 1 { true } else { false };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_odd() {
        assert_eq!(true, is_odd(1));
        assert_eq!(false, is_odd(2));
        assert_eq!(true, is_odd(3));
        assert_eq!(false, is_odd(16));
        assert_eq!(true, is_odd(121));
    }
}

/**
 * @file xoroshiro64ss.c
 * @brief xoroshiro64ss による疑似乱数生成の実装検証用プログラム
 * @note 乱数の周期は (2**64) - 1
 * @note 参考: http://prng.di.unimi.it/xoroshiro64starstar.c
 * @note 引用元のコメントその1
 *       Written in 2018 by David Blackman and Sebastiano Vigna (vigna@acm.org)
 *
 *       To the extent possible under law, the author has dedicated all copyright
 *       and related and neighboring rights to this software to the public domain
 *       worldwide. This software is distributed without any warranty.
 *
 *       See <http://creativecommons.org/publicdomain/zero/1.0/>.
 * @note 引用元のコメントその2
 *       This is xoroshiro64** 1.0, our 32-bit all-purpose, rock-solid,
 *       small-state generator. It is extremely fast and it passes all tests we
 *       are aware of, but its state space is not large enough for any parallel
 *       application.
 *
 *       For generating just single-precision (i.e., 32-bit) floating-point
 *       numbers, xoroshiro64* is even faster.
 *
 *       The state must be seeded so that it is not everywhere zero.
 */

#include <stdint.h>
#include <stdio.h>  // main関数でのみ使用。モジュールとして使用する場合は不要

/* 乱数の初期値。{0, 0} 以外の値を入力すること */
static uint32_t Xoroshiro64ss_s[2] = {1, 2};

/**
 * @brief 乱数生成時に用いる計算関数
 */
/* ------------------------------------------------------------------------- */
static inline uint32_t Xoroshiro64ss_rotl(const uint32_t x, int k)
{
    return (x << k) | (x >> (32 - k));
}

/**
 * @brief 乱数を取得する
 */
/* ------------------------------------------------------------------------- */
uint32_t Xoroshiro64ss_next(void)
{
    const uint32_t s0 = Xoroshiro64ss_s[0];
    uint32_t s1 = Xoroshiro64ss_s[1];
    const uint32_t result = Xoroshiro64ss_rotl(s0 * 0x9E3779BB, 5) * 5;

    s1 ^= s0;
    Xoroshiro64ss_s[0] = Xoroshiro64ss_rotl(s0, 26) ^ s1 ^ (s1 << 9);  // a, b
    Xoroshiro64ss_s[1] = Xoroshiro64ss_rotl(s1, 13);  // c

    return result;
}
/**
 * @brief 乱数パラメータ(= Xoroshiro64ss_s に格納する2つの値)を設定する
 * @note 簡便のためパラメータは下記のように設定している
 *       Xoroshiro64ss_s[0]: 引数の値(引数が 0 の場合は 1)
 *       Xoroshiro64ss_s[1]: 引数の値(引数が 0 の場合は 1)の2倍
 * @note このパラメータ設定法が良いかは検討の余地あり
 */
/* ------------------------------------------------------------------------- */
void Xoroshiro64ss_initialize(uint64_t val)
{
    if (val != 0) {
        Xoroshiro64ss_s[0] = val;
    } else {
        Xoroshiro64ss_s[0] = 1;
    }
    Xoroshiro64ss_s[1] = Xoroshiro64ss_s[0] << 1;
    return;
}

/**
 * @brief メイン関数. 乱数を 5 つ生成した結果を表示する
 */
/* ------------------------------------------------------------------------- */
void main()
{
    printf("+++++++ start of program +++++++\n");

    uint64_t num = 0;
    Xoroshiro64ss_initialize(7);
    for (uint8_t i = 0; i < 5; i++) {
        num = Xoroshiro64ss_next();
        printf("%u, ", num);
    }
    printf("\n");

    printf("+++++++ end of program +++++++\n");
    return;
}

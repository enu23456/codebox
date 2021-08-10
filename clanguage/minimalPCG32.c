/** 簡単なPCG乱数生成器の実装検証プログラム
 * @note 最近作られた（？）性能が良いと期待されている乱数生成アルゴリズムの簡単なバージョン
 *       作者が言うには、これでも十分な場合もあるらしい
 * @note 構造体pcg_random_tの初期値は適当です。
 * @note 参考文献
 *   https://www.pcg-random.org/download.html
 */

// *Really* minimal PCG32 code / (c) 2014 M.E. O'Neill / pcg-random.org
// Licensed under Apache License 2.0 (NO WARRANTY, etc. see website)

#include <stdio.h>  // printf出力のために追加
#include <stdint.h>
#include <inttypes.h>  // uint64_t型をprintf出力するために追加

typedef struct {
    uint64_t state;
    uint64_t inc;
} pcg32_random_t;

uint32_t pcg32_random_r(pcg32_random_t* rng)
{
    uint64_t oldstate = rng->state;
    // Advance internal state
    rng->state = oldstate * 6364136223846793005ULL + (rng->inc | 1);
    // Calculate output function (XSH RR), uses old state for max ILP
    uint32_t xorshifted = ((oldstate >> 18u) ^ oldstate) >> 27u;
    uint32_t rot = oldstate >> 59u;
    return (xorshifted >> rot) | (xorshifted << ((-rot) & 31));
}

void main(void)
{
    pcg32_random_t pcg32 = {1, 2};
    for (int i = 0; i < 5; i++) {
        uint32_t ret = pcg32_random_r(&pcg32);
        printf("random: %10u, internal: (%" PRIu64 ", %" PRIu64 ")\n", ret, pcg32.state, pcg32.inc);
    }
}
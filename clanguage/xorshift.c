/**
 * @file xorshift.c
 * @brief xorshift による疑似乱数生成の実装検証用プログラム
 * @note ひとまずは 32bit の xorshift のみ実装。他の実装は未定
 */
#include "stdint.h"
#include "stdio.h"

typedef struct {
    uint32_t val;
} xorshift32x1_t;

/**
 * @brief 32bit変数1つだけで構成された xorshift 疑似乱数
 * @note 周期は (2**32) - 1
 * @note 初期値に 0 は使用しないこと
 * @note 初期値(引数)を 1 に選んだ時の乱数の推移
 *       270369, 67634689, 2647435461, 307599695, 2398689233
 * @note 参考: https://ja.wikipedia.org/wiki/Xorshift
 */
/* ------------------------------------------------------------------------- */
uint32_t Xorshift32x1_next(xorshift32x1_t *param)
{
    param->val = param->val ^ (param->val << 13);
    param->val = param->val ^ (param->val >> 17);
    return param->val = param->val ^ (param->val << 5);
}

/**
 * @brief 疑似乱数の周期評価用関数
 * @note ただの評価用関数であり、運用で用いることは無い
 */
/* ------------------------------------------------------------------------- */
void Xorshift32x1_eval_period(void)
{
    xorshift32x1_t test;
    test.val = 1;
    Xorshift32x1_next(&test);
    uint32_t init = test.val;
    uint32_t dummy = 0;
    for (uint64_t i = 1; i < 5000000000; i++) {
        dummy = Xorshift32x1_next(&test);
        if (init == dummy) {
            printf("equal at %u\n", i);
        }
    }
    return;
}

/**
 * @brief メイン関数
 */
/* ------------------------------------------------------------------------- */
void main()
{
    printf("+++++++ start of program +++++++\n");

    xorshift32x1_t xorshift;
    xorshift.val = 1;
    for (uint8_t i = 0; i < 5; i++) {
        Xorshift32x1_next(&xorshift);
        printf("%u, ", xorshift.val);
    }
    printf("\n");
    Xorshift32x1_eval_period();

    printf("+++++++ end of program +++++++\n");
    return;
}

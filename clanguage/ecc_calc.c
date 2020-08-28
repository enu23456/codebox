/**
 * @file ecc_calc.c
 * @brief 楕円曲線上の計算を行うサンプルプログラム
 * @note パラメータは下記を参考にした
 *       https://euniclus.com/article/about-ecc/
 */
/* ------------------------------------------------------------------------- */
#include <stdint.h>
#include <stdio.h>

#define ECC_A (0)  // parameter of ECC
#define ECC_B (7)  // parameter of ECC
#define ECC_MOD (223)  // parameter of ECC
#define ECC_GX (47)  // base point
#define ECC_GY (71)  // base point
#define ECC_KEY (5)  // private_key.

/**
 * @brief べき乗余を求める
 */
/* ------------------------------------------------------------------------- */
uint16_t Math_calc_pow(uint16_t a, uint16_t b, uint16_t n)
{
    uint32_t dummy = 1;
    for (uint16_t i = 0; i < b; i++) {
        dummy = (dummy * a) % n;
    }
    return (uint16_t)dummy;
}

/**
 * @brief 有限体の乗法逆元（モジュラ逆数）を求める
 */
/* ------------------------------------------------------------------------- */
uint16_t ECC_calc_inv_mod(uint16_t k, uint16_t mod)
{
    return Math_calc_pow(k, mod - 2, mod);
}

/**
 * @brief 楕円曲線上の加法処理を行う
 */
/* ------------------------------------------------------------------------- */
void ECC_add(uint8_t *point1, uint8_t *point2)
{
    int16_t lam = 0;
    int16_t x = 0;
    int16_t y = 0;

    if (point1[0] == point2[0] && point1[1] == point2[1]) {
        lam = ((3 * ((point1[0] * point1[0]) + ECC_MOD) * ECC_calc_inv_mod(2 * point1[1], ECC_MOD))) % ECC_MOD;
        if (lam < 0) {
            lam = ECC_MOD + lam;
        }

        x = ((lam * lam) - (2 * point1[0])) % ECC_MOD;
        y = (lam * (point1[0] - x) - point1[1]) % ECC_MOD;
    } else {
        uint16_t dummy = (point2[0] > point1[0]) ? (point2[0] - point1[0]) : (ECC_MOD + point2[0] - point1[0]);
        lam = ((point2[1] - point1[1]) * ECC_calc_inv_mod(dummy, ECC_MOD)) % ECC_MOD;
        if (lam < 0) {
            lam = ECC_MOD + lam;
        }

        x = ((lam * lam) - point1[0] - point2[0]) % ECC_MOD;
        y = (lam * (point1[0] - x) - point1[1]) % ECC_MOD;
    }
    point1[0] = (x > 0) ? (uint16_t)x : (uint16_t)(ECC_MOD + x);
    point1[1] = (y > 0) ? (uint16_t)y : (uint16_t)(ECC_MOD + y);

    return;
}

/**
 * @brief 楕円曲線上の乗法処理を行う
 */
/* ------------------------------------------------------------------------- */
void ECC_mul(uint8_t *point, uint8_t scalar)
{
    if (scalar == 0) {
        point[0] = 0;
        point[1] = 0;
        return;
    } else if (scalar == 1) {
        return;
    }

    uint8_t dummy[2];
    for (uint8_t i = 0; i < 2; ++i) {
        dummy[i] = point[i];
    }
    for (uint8_t i = 2; i < scalar + 1; i++) {
        ECC_add(point, dummy);
    }
    return;
}
/**
 * @brief メイン関数
 */
/* ------------------------------------------------------------------------- */
int main(void)
{
    printf("+++++++ start of program +++++++\n");

    /* パラメータの表示 */
    printf("Elliptic curve : y**2 = x**3 + (%d * x) + %d mod %d\n", ECC_A, ECC_B, ECC_MOD);
    printf("point G: (%d, %d)\n", ECC_GX, ECC_GY);
    printf("private key: %d\n", ECC_KEY);

    /* 公開鍵を計算と表示 */
    uint8_t public_key[2] = {ECC_GX, ECC_GY};

    ECC_mul(public_key, ECC_KEY);
    printf("public key: (%d, %d)\n", public_key[0], public_key[1]);

    printf("+++++++ end of program +++++++\n");

    return 0;
}
/**
 * @file encryption_rsa.c
 * @brief RSA技術検証用の、uint16 配列の暗号化/復号化プログラム
 * @note 暗号強度 は最大16bit(uint16 の中で収めているため)
 */
#include <stdint.h>
#include <stdio.h>

static uint16_t RSA_d = 0;  // private key.

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
 * @brief 最大公約数を求める
 */
/* ------------------------------------------------------------------------- */
uint16_t Math_calc_gcd(uint16_t a, uint16_t b)
{
    uint16_t dummy = 0;
    while (b != 0) {
        dummy = a;
        a = b;
        b = dummy % b;
    }
    return a;
}

/**
 * @brief 最小公倍数を求める
 */
/* ------------------------------------------------------------------------- */
uint16_t Math_calc_lcm(uint16_t a, uint16_t b)
{
    return (a * b) / Math_calc_gcd(a, b);
}

/**
 * @brief 暗号化処理のためのパラメータ設定を行う
 */
/* ------------------------------------------------------------------------- */
void RSA_generate_key_16bit(uint16_t RSA_p, uint16_t RSA_q, uint16_t *RSA_n, uint16_t *RSA_e)
{
    *RSA_n = RSA_p * RSA_q;

    uint16_t fai = Math_calc_lcm(RSA_p - 1, RSA_q - 1);

    *RSA_e = 16;  // 初期値は最大暗号強度のビット数(16bit)とした
    while (Math_calc_gcd(fai, *RSA_e) != 1) {
        *RSA_e += 1;
    }

    RSA_d = 16;  // 初期値は最大暗号強度のビット数(16bit)とした
    while ((*RSA_e * RSA_d) % fai != 1) {
        RSA_d += 1;
    }
    //printf("%d, %d, %d\n", fai, RSA_e, RSA_d);
}

/**
 * @brief 暗号化処理を行う
 */
/* ------------------------------------------------------------------------- */
void RSA_encrypt(uint16_t *data, uint16_t length, uint16_t RSA_n, uint16_t RSA_e)
{
    for (uint16_t i = 0; i < length; i++) {
        //printf("%d, ", *(data + i));
        *(data + i) = Math_calc_pow(*(data + i), RSA_e, RSA_n);
        //printf("%d\n", *(data + i));
    }
    return;
}

/**
 * @brief 復号化処理を行う
 */
/* ------------------------------------------------------------------------- */
void RSA_decrypt(uint16_t *data, uint16_t length, uint16_t RSA_n)
{
    for (uint16_t i = 0; i < length; i++) {
        *(data + i) = Math_calc_pow(*(data + i), RSA_d, RSA_n);
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

    /* 平文と文字長さを設定する */
    uint16_t targetData[] = {32, 15, 27, 5};
    uint16_t dataLength = 4;
    printf("    plain: ");
    for (uint16_t i = 0; i < dataLength; i++) {
        printf("%d, ", targetData[i]);
    }
    printf("\n");

    /* 暗号のパラメータ設定を行う */
    uint16_t RSA_p = 263;  // RSA_p * RSA_q が16bitに収まるような適当な素数を選ぶ
    uint16_t RSA_q = 227;  // RSA_p * RSA_q が16bitに収まるような適当な素数を選ぶ
    uint16_t RSA_n = 0;  // public key.
    uint16_t RSA_e = 0;  // public key.
    RSA_generate_key_16bit(RSA_p, RSA_q, &RSA_n, &RSA_e);
    //printf("public key: %d, %d\n", RSA_n, RSA_e);

    /* 暗号化を行う */
    RSA_encrypt(targetData, dataLength, RSA_n, RSA_e);
    printf("encrypted: ");
    for (uint16_t i = 0; i < dataLength; i++) {
        printf("%d, ", targetData[i]);
    }
    printf("\n");

    /* 復号化を行う */
    RSA_decrypt(targetData, dataLength, RSA_n);
    printf("decrypted: ");
    for (uint16_t i = 0; i < dataLength; i++) {
        printf("%d, ", targetData[i]);
    }
    printf("\n");

    printf("+++++++ end of program +++++++\n");

    return 0;
}

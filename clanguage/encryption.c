/**
 * @file encryption.c
 * @brief 文字列の暗号化／復号化プログラム
 * @note 暗号化/復号化には線形合同法を用いた CTR モードを採用している
 */
#include <stdint.h>
#include <stdio.h>

#define LCG_SEED (113) /* 0 - 255 の値を入力 */

const uint8_t LCG_CONST_A = 217; /* 線形合同法のパラメータ */
const uint8_t LCG_CONST_B = 199; /* 線形合同法のパラメータ */

/**
 * @brief 暗号化処理を行う
 * @test >>> uint8_t dummy = 24;
         >>> Encryption_Encrypt(&dummy, 1);
         >>> printf("encrypted: %d\n", dummy);
         encrypted: 136
 */
/* ------------------------------------------------------------------------- */
void Encryption_Encrypt(uint8_t *data, uint16_t length)
{
    uint8_t random_number = LCG_SEED;

    for (int i = 0; i < length; i++) {
        random_number = (random_number * LCG_CONST_A) + LCG_CONST_B;
        //printf("pointer: %p, before: %d, ", data + i, *(data + i));
        *(data + i) = *(data + i) ^ random_number;
        //printf("random: %d, after: %d\n", random_number, *(data + i));
    }
    return;
}

/**
 * @brief 復号化処理を行う
 * @note 復号化は暗号化と同じ処理を行う
 * @test >>> uint8_t dummy = 136;
         >>> Encryption_Decrypt(&dummy, 1);
         >>> printf("decrypted: %d\n", dummy);
         decrypted: 24
 */
/* ------------------------------------------------------------------------- */
void Encryption_Decrypt(uint8_t *data, uint16_t length)
{
    Encryption_Encrypt(data, length);
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
    uint8_t targetData[] = "Hello world";
    uint16_t dataLength = 11;
    printf("dataLength: %d\n", dataLength);
    printf("plainData: %s\n", targetData);

    /* 暗号化を行う */
    Encryption_Encrypt(targetData, dataLength);
    printf("encrypted: %s\n", targetData);

    /* 復号化を行う */
    Encryption_Decrypt(targetData, dataLength);
    printf("decrypted: %s\n", targetData);

    printf("+++++++ end of Program +++++++\n");

    return 0;
}

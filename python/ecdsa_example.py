"""楕円曲線電子署名に必要なアルゴリズムモジュール
Note:
    注意：安全性など全く担保されていないため、軽い参考程度にご使用ください
    また、適当に数値変えても正しく動いてくれません
Note:
    本モジュールには下記が含まれています
    HashGenerator: ハッシュ値を生成するクラス
    ECDSA: 楕円曲線電子署名の署名／検証クラス
Note:
    参考にしたURLは下記
    https://kiririmode.hatenablog.jp/entry/20181216/1544956622
    https://euniclus.com/article/ecdsa-implementation/
"""


class HashGenerator:
    """ハッシュ値を生成するクラス
    Note:
        ハッシュ値の長さは32bit
        0、および 2**32 以上の数値は使用できない点に注意
    Note:
        ハッシュ生成アルゴリズムには疑似乱数生成アルゴリズムの xorshift を使用している
    """

    def __init__(self, seed=0):
        """初期値の設定と、内部でのハッシュ値生成を行う
        """
        self.value = seed

        # xorshift 処理の部分
        self.value = self.value ^ (self.value << 13 & 0xFFFFFFFF)
        self.value = self.value ^ (self.value >> 17 & 0xFFFFFFFF)
        self.value = self.value ^ (self.value << 5 & 0xFFFFFFFF)
        self.value = self.value & 0xFFFFFFFF

    def get(self):
        """ハッシュ値を返す
        """
        return self.value


class ECDSA:
    """楕円曲線電子署名アルゴリズム(ECDSA)のクラス
    Note:
        楕円曲線に関するパラメータは固定値で設定している
    """

    def __init__(self):
        """初期化関数
        """
        self.a = 66
        self.b = 73
        self.mod = 251
        self.order = 251  # 位数
        self.base = [58, 35]  # 基準点、あらかじめ設定する
        return

    def generate_public_key(self, private_key):
        """公開鍵を生成する
        """
        ret = self.__multi(self.base, private_key)  # P = e * G
        return ret

    def sign(self, hash_val, random_key, private_key):
        """署名する
        """
        r = self.__multi(self.base, random_key)  # R = k * G
        r_inv = self.__inv_mod_pow(random_key, self.order)
        s = (hash_val + private_key * r[0]) * r_inv % self.order
        return r[0], s

    def validate(self, hash_val, r, s, p):
        """検証する
        """
        s_inv = self.__inv_mod_pow(s, self.order)
        res_0 = self.__multi(self.base, (hash_val * s_inv) % self.order)
        res_1 = self.__multi(p, (r * s_inv) % self.order)
        res = self.__add(res_0, res_1)
        print("res:", res[0], "[for debug. in EDCSA class]")  # デバッグ用出力
        return res[0] == r

    def __multi(self, index, scalar):
        """楕円曲線上の乗法処理を行う
        """
        if scalar == 0:
            return 0
        elif scalar == 1:
            return index
        result = index
        for i in range(2, scalar + 1):
            result = self.__add(result, index)
        return result

    def __add(self, index1, index2):
        """楕円曲線上の加法処理を行う
        """
        x = 0
        y = 0
        if index1 != index2:
            lam = ((index2[1] - index1[1]) * self.__inv_mod_pow(index2[0] - index1[0], self.mod)) % self.mod
            x = ((pow(lam, 2)) - index1[0] - index2[0]) % self.mod
            y = (lam * (index1[0] - x) - index1[1]) % self.mod
        elif index1 == index2:
            lam = ((3 * (pow(index1[0], 2)) + self.a) * self.__inv_mod_pow(2 * index1[1], self.mod)) % self.mod
            x = (pow(lam, 2) - 2 * index1[0]) % self.mod
            y = (lam * (index1[0] - x) - index1[1]) % self.mod
        return [x, y]

    @staticmethod
    def __inv_mod_pow(k, mod):
        """有限体の乗法逆元（モジュラ逆数）を求める
        """
        return pow(k, mod - 2, mod)


def main():
    """メイン関数
    """

    private_key = 97
    ecdsa = ECDSA()

    # 公開鍵を生成し、みんなへ公開（したつもりとする）
    public_key = ecdsa.generate_public_key(private_key)
    print("p:", public_key)

    # 送るメッセージを1-250の間で選択する
    message = 219

    # ハッシュ値を生成する
    hg_alice = HashGenerator(message)
    hash_alice = message * hg_alice.get()
    print("hash:", hash_alice)

    # 検証に必要なパラメータを作成する
    random_key = 162
    r, s = ecdsa.sign(hash_alice, random_key, private_key)
    print("r, s:", r, s)

    # 相手に送るのは、既に公開している公開鍵に加え、
    # メッセージ、検証に必要なパラメータ２種（ここでは r と s）

    # 検証を行う（ハッシュ生成は相手側でも同じ処理を行う）
    hg_bob = HashGenerator(message)
    hash_bob = message * hg_bob.get()
    print("result of validation:", ecdsa.validate(hash_bob, r, s, public_key))

    return


if __name__ == "__main__":
    main()

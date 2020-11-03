"""ブロックチェーンの原理理解用プログラム
Note:
    本ブロックチェーンのプログラムは下記の制約をしている
    トランザクションは、32bitの、アドレス相当の情報のみを持つ
    トランザクションに、金額や送金先情報はない（固定の金額を固定の送金債に送金するのみの仕様）
Note:
    https://www.sejuku.net/blog/75079
    https://gaiax-blockchain.com/sha-256
"""

import hashlib
import random

random.seed(0)
DIFFICULTY = 4


class Block:
    """ブロックチェーンを構成するブロックのクラス(初期化時にハッシュ値などは計算される)
    Note:
        出力結果の閲覧性を優先し、難易度は定数 DIFFUCULTY で与えている
    """

    def __init__(self, tx, previous_hash):
        self.transaction = tx
        self.hash_prev = previous_hash
        self.hash_tx = self._calc_hash(str(self.transaction).encode('ascii'))
        self.nonce, self.block_hash = self._mining()

    def _mining(self):
        """ ゴールデンノンス（条件を満たすノンス）と、その時のハッシュ値を返す
        """
        nonce_temp = 0
        while True:
            proof_string = self.hash_tx + str(nonce_temp)
            calced = self._calc_hash(proof_string.encode("ascii"))
            if calced[:DIFFICULTY:].count('0') == DIFFICULTY:
                break
            else:
                nonce_temp += 1
        return nonce_temp, calced

    def _calc_hash(self, num):
        """ハッシュ値を計算する
        """
        return hashlib.sha256(num).hexdigest()


if __name__ == "__main__":
    # 最初のブロックを作成
    block = Block(0, '-')

    for i in range(5):
        print(i, "th block info")
        for key, value in block.__dict__.items():
            print(key, ':', value)
        print("")
        hash_newest_block = block.hash_tx
        transaction = random.randint(0, 0xFFFFFFFF)
        block = Block(transaction, hash_newest_block)

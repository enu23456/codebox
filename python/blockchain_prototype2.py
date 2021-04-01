""" 技術理解を目的とした自作ブロックチェーンのサンプルプログラム
Note:
    システムとしては、あるアドレスから発信されたテキストを1つのブロックとして記録する
Note:
    参考文献
    https://www.sejuku.net/blog/75079
    https://gaiax-blockchain.com/sha-256
    赤澤直樹（2019）『Pythonで動かして学ぶ！新しいブロックチェーンの教科書』 翔泳社．
    松浦健一郎、司ゆき（2018）『入門 仮想通貨の作り方』 秀和システム.
"""

import hashlib
import datetime
import time

DIFFICULTY = 5


class Block(object):
    """ ブロックチェーンを構成するブロックのクラス
    Note:
        データは以下のような構造となっている
        ・blockheader  ブロックに関する情報がまとめられたデータ群
            ・prev_hash  前のブロックのハッシュ値
            ・tx_hash  今のブロックにあるトランザクションのハッシュ値
                       通常はマークルルートだが、複数のトランザクションを格納しないため、
                       マークるルートの言葉は使用していない
            ・timestamp  ブロックを生成（の指示を）した日時
            ・difficulty  ブロックのハッシュ値を作る際の難易度
            ・nonce  ノンス値
        ・transaction  取引（動作）を記したデータ群
            ・tweeter_address  行った人のアドレス
            ・tweet_content  行った人の呟いたテキスト内容
        ・blockhash  ノンスが決まった時の、blockheaderのハッシュ値
                     本来は格納する必要はないが、処理確認のために実装をしている
        ・generation_time  ノンスの探索にかかった時間。
                           本来は格納する必要はないが、処理確認のために実装をしている

    """
    def __init__(self, prev_hash, date, address, tweet, difficult):
        self.blockheader = {
            "prev_hash": prev_hash,
            "tx_hash": "",
            "timestamp": date,
            "difficulty": difficult,
            "nonce": 0
        }
        self.transaction = {
            "tweeter_address": address,
            "tweet_content": tweet,
        }
        self.blockhash = 0
        self.generation_time = 0

        self.calc_tx_hash()
        self.mining()
        return

    def calc_tx_hash(self):
        """トランザクションのハッシュ値を計算する
        """
        hash = self.calc_dict_hash(self.transaction)
        self.blockheader["tx_hash"] = hash
        return

    def mining(self):
        """条件を満たすような nonce を求める
        Note:
            条件：blockheader のハッシュ値が、先頭にゼロが difficulty の数以上に並ぶ
        Note:
            条件を満たすような nonce の時のハッシュ値は self.blockhash に格納している
            （本来は格納する必要はないが、処理確認に都合が良いので実装をしている）
        """
        start_time = int(time.time() * 1000)
        while True:
            hash = self.calc_dict_hash(self.blockheader)
            if hash[:self.blockheader["difficulty"]:].count('0') == self.blockheader["difficulty"]:
                self.blockhash = hash
                end_time = int(time.time() * 1000)
                self.generation_time = (str(end_time - start_time) + "[milli-sec]")
                break
            else:
                self.blockheader["nonce"] += 1
        return

    @staticmethod
    def calc_dict_hash(dict):
        """辞書のハッシュ値（辞書内の全ての値で算出したハッシュ値）を計算する
        """
        text = ""
        for value in dict.values():
            text += str(value)
        hash = hashlib.sha256(text.encode("ascii")).hexdigest()

        return hash


if __name__ == "__main__":
    blockchain = []
    print("generate three blocks")

    # 最初のブロック（ジェネシスブロック）を生成。前のブロックがないためハッシュは0埋めの値を設定
    prev_hash = "0000000000000000000000000000000000000000000000000000000000000000"
    date = datetime.datetime.now().strftime("%Y/%m/%d %H:%M:%S")
    address = "my_address"
    tweet = "hello world! It's genesis block"
    difficult = DIFFICULTY
    block = Block(prev_hash, date, address, tweet, difficult)
    blockchain.append(block)
    print("1st block generated")

    # 2個目のブロックを生成。
    prev_hash = Block.calc_dict_hash(blockchain[-1].blockheader)
    date = datetime.datetime.now().strftime("%Y/%m/%d %H:%M:%S")
    address = "my_address"
    tweet = "Good morning. It's nice weather today"
    difficult = DIFFICULTY
    block = Block(prev_hash, date, address, tweet, difficult)
    blockchain.append(block)
    print("2nd block generated")

    # 3個目のブロックを生成。
    prev_hash = Block.calc_dict_hash(blockchain[-1].blockheader)
    date = datetime.datetime.now().strftime("%Y/%m/%d %H:%M:%S")
    address = "my_address"
    tweet = "Good night. Very sleepy"
    difficult = DIFFICULTY
    block = Block(prev_hash, date, address, tweet, difficult)
    blockchain.append(block)
    print("3nd block generated")
    print()

    # ブロックを生成した結果を表示する
    print("Result")
    for i in range(len(blockchain)):
        print("=======", i, "th block's content =======")
        print("--- blockheader ---")
        for key, value in blockchain[i].blockheader.items():
            print(key, ":", value)
        print("--- transaction ---")
        for key, value in blockchain[i].transaction.items():
            print(key, ":", value)
        print("--- other ---")
        print("blockhash :", blockchain[i].blockhash)
        print("generation_time :", blockchain[i].generation_time)
        print()

"""チェンジレスポンス認証の原理検証モジュール
Note:
    チェンジレスポンス認証(challenge and response authentication)とは、
    認証の際、暗号学的ハッシュ関数を利用することによりパスワードを秘密にしたまま認証する手法
Note:
    チェンジレスポンス認証 の簡単な流れ
    （前提）id に対応するパスワードはサーバー側と共有されているものとする
    1. クライアントがサーバーへ認証を要求する
    2. サーバーが乱数（チャレンジ）を生成し、 クライアントへ送信する
    3. サーバーから送られてきたチャレンジと自身のパスワードからハッシュ値を生成する
    4. 生成したハッシュ値 （レスポンス） を サーバーへ送信する
    5. サーバー側もクライアントと同様に ハッシュ値を生成し、同値か確認する
"""

import random
import hashlib


def generate_hash(challenge, password):
    """challenge と password からハッシュ値を生成する
    Note:
        ２つの文字列を結合した後、MD5にてハッシュ値を生成している
    """
    temp = challenge + password
    # print(temp)
    hash_value = hashlib.md5(temp.encode()).hexdigest()
    # print(hash_value)

    return hash_value


class ServerSide(object):
    def __init__(self):
        self.ch = "0"
        self.id = 0
        self.pw = "0"
        return

    def set_id(self, value):
        """送られた user_id と、その user_id に対応する password をセットする
        Note:
            本例では簡単のため ID は 1-255 までの数値としている
        """
        ret = False
        if (value < 256) or (value > 0):
            self.id = value
            self.set_pw(self.id)
            ret = True
        return ret

    def set_pw(self, num):
        """パスワードをセットする
        Note:
            本例では簡単のため (256 - id) を3桁の文字列に変換したものということにしている（main関数内のコメントも参照のこと）
        """
        self.pw = str(256 - num).zfill(3)
        return

    def get_challenge(self):
        """乱数（チャレンジ）を生成し、 送信する
        """
        self.ch = str(random.randint(1, 255)).zfill(3)
        return self.ch

    def validate_hash(self, hash_client):
        """ユーザーから送られたハッシュ値と、サーバー側で生成したハッシュ値が同じものか確認する
        """

        ret = False
        hash_server = generate_hash(self.ch, self.pw)

        if hash_server == hash_client:
            ret = True

        return ret


def main():
    # ここに 1-255 の範囲で id を入力する
    # 注意：簡単のためパスワードを (256 - id) を3桁の文字列に変換したものということにする
    # また、この password は既にサーバー側は知っているものとする
    user_id = 210

    password = str(256 - user_id).zfill(3)
    print("user_id is", user_id, "  password is", password)

    # 1. クライアントがサーバーへ認証を要求する
    # 本例では サーバーに user_id を送る(password は送らない)）
    ss = ServerSide()
    ret = ss.set_id(user_id)

    if ret == False:
        print("user_id is not exist")
        return

    # 2. サーバーが乱数（チャレンジ）を生成し、 クライアントへ送信する
    challenge = ss.get_challenge()
    print("challenge is", challenge)

    # 3. サーバーから送られてきたチャレンジと自身のパスワードからハッシュ値を生成する
    response = generate_hash(challenge, password)

    # 4. 生成したハッシュ値 （レスポンス） を サーバーへ送信する
    # 5. サーバー側もクライアントと同様に ハッシュ値を生成し、同値か確認する
    # ここで、パスワードそのものをサーバーに送っていないことに注意する
    if ss.validate_hash(response) == True:
        print("Authentication seccess")
    else:
        print("Authentication failure")

    return


if __name__ == "__main__":
    main()

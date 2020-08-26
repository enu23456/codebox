""" YOTARO breaking(ブロック崩し)

※動作条件は、PythonをインストールされてあるPCなら基本的に動作する。。。はずです
※本プログラムの動作保証は一切いたしません。自己責任でご使用ください

本プログラムは一般的なブロック崩しと比べ、以下の特徴があります
　・ボールが白いブロックに衝突した際の跳ね返りはランダム（ブロック内貫通もあり）
　・ボールが白いブロックに衝突した際にブロックが消えるかどうかもランダム
つまり、一般的なブロック崩しとはボールの挙動が異なる、全く新しいブロック崩しです。
。。。はい、そこ。衝突処理が面倒だからランダムにしただけとか言わないの

また、スコアリング機能も搭載しており、下記の通りになっています。
　・ボールが白いブロックに衝突した際に(消えた消えないに関わらず)ポイントが加算される
　。ポイントはウィンドウの上側に位置するバーに表示される

本プログラムは難易度を設定できるようになっており、
難易度に応じてボールのスピード、バーの幅が変化します。
難易度は下記変数の値を変えることで変更することができます
DIFFICULT(この三重引用符コメントのすぐ下)

なお、本プログラムは下記サイトのコードを改変して作成しております
https://news.mynavi.jp/article/zeropython-10/
本プログラムの改変については、私から制限するつもりは特にありません。
というか、これを元にもっと良いゲームを作ってください

ゲームの遊び方
マウスを動かすと、それに応じてバーも動きますので、
ボールがバーの下にいかないよう上手くバーを動かしてください。
ボールがバーの下にいってしまうとゲームオーバーです。
アイボリー色のブロックを全て崩せばゲームクリアです
ゲームオーバー、もしくはゲームクリア時にマウスをクリックすると
ブロックが元に戻った状態でゲームを再開することができます
"""
# +++ ここから設定箇所 +++++++++++++++
DIFFICULT = 2  # 1: EASY, 2: NORMAL, 3: HARD
# +++ ここまで設定箇所 +++++++++++++++

from tkinter import *
import random

yotaro = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0,
    1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0,
    1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0,
    0, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
    0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0,
    0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
    0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0,
    0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0,
    0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
    0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
    0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
    0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
    0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0
]

BALL_WIDTH = 8
WIN_WIDTH = 400
WIN_HEIGHT = 560
BLOCK_NUM_V = 15
BLOCK_NUM_H = 16

if DIFFICULT == 1:  # EASY
    GAME_SPEED = 50
    BAR_WIDTH = 120
elif DIFFICULT == 2:
    GAME_SPEED = 40  # NORMAL
    BAR_WIDTH = 90
else:
    GAME_SPEED = 30  # HARD
    BAR_WIDTH = 60

blocks = []
block_size = {"x": 25, "y": 25}
ball = {"dirx": 10, "diry": -10, "x": 240, "y": 500, "w": BALL_WIDTH}
bar = {"x": 0, "w": BAR_WIDTH}
is_gameover = False
point = 0

# ウィンドウの作成
win = Tk()
cv = Canvas(win, width=WIN_WIDTH, height=WIN_HEIGHT)
cv.pack()


# ゲームの初期化
def init_game():
    global is_gameover, point
    is_gameover = False
    ball["x"] = random.randint(0, WIN_WIDTH)
    ball["y"] = 500
    ball["diry"] = -10
    point = 0
    # ブロックを配置する
    for iy in range(0, BLOCK_NUM_V):
        for ix in range(0, BLOCK_NUM_H):
            # color = "ivory3"
            x1 = ix * block_size["x"]
            x2 = x1 + block_size["x"]
            y1 = iy * block_size["y"]
            y2 = y1 + block_size["y"]
            if yotaro[(iy * BLOCK_NUM_H) + ix] == 1:
                blocks.append([x1, y1, x2, y2, "ivory2"])
                # blocks.append([x1, y1, x2, y2, color])
    win.title("START")


# オブジェクトを描画する
def draw_objects():
    cv.delete('all')  # 既存の描画を破棄
    cv.create_rectangle(0, 0, WIN_WIDTH, WIN_HEIGHT, fill="gray10", width=0)
    # ブロックを一つずつ描画
    for w in blocks:
        x1, y1, x2, y2, c = w
        cv.create_rectangle(x1, y1, x2, y2, fill=c, width=0)
    # ボールを描画
    cv.create_oval(ball["x"] - ball["w"], ball["y"] - ball["w"],
                   ball["x"] + ball["w"], ball["y"] + ball["w"], fill="deep pink")
    # バーを描画
    cv.create_rectangle(bar["x"], (WIN_HEIGHT - 10), bar["x"] + bar["w"], WIN_HEIGHT, fill="yellow")


# ボールの移動
def move_ball():
    global is_gameover, point
    if is_gameover: return
    bx = ball["x"] + ball["dirx"]
    by = ball["y"] + ball["diry"]
    # 上左右の壁に当たった？
    if bx < 0 or bx > WIN_WIDTH:
        ball["dirx"] *= -1
    if by < 0:
        ball["diry"] *= -1
    # プレイヤーの操作するバーに当たった？
    if by > (WIN_HEIGHT - 10) and (bar["x"] <= bx <= (bar["x"] + bar["w"])):
        ball["diry"] *= -1
        by = (WIN_HEIGHT - 10) - random.randint(0, 10)
    # ボールがブロックに当たった？
    hit_i = -1
    for i, w in enumerate(blocks):
        x1, y1, x2, y2, color = w
        w3 = ball["w"] // 2
        if (x1 - w3 <= bx <= x2 + w3) and (y1 - w3 <= by <= y2 + w3):
            hit_i = i
            break
    if hit_i >= 0:
        if random.randint(0, 1) == 0:
            del blocks[hit_i]
        if random.randint(0, 1) == 0:
            ball["dirx"] *= -1
        if random.randint(0, 1) == 0:
            ball["diry"] *= -1
        point += 10
        win.title("score = " + str(point))
    # ゲームオーバー？
    if by >= WIN_HEIGHT:
        win.title("GAME OVER!! score =" + str(point))
        is_gameover = True
    if not blocks:
        win.title("GAME CLEAR!! score = " + str(point))
        ball["dirx"] *= -1
        ball["diry"] *= -1
        is_gameover = True
    if 0 <= bx <= WIN_WIDTH:
        ball["x"] = bx
    if 0 <= by <= WIN_HEIGHT:
        ball["y"] = by


def game_loop():
    draw_objects()
    move_ball()
    win.after(GAME_SPEED, game_loop)


# マウスイベントの処理
def motion(e):  # マウスポインタの移動
    bar["x"] = e.x - BAR_WIDTH // 2


def click(e):  # クリックでリスタート
    if is_gameover:
        init_game()


# マウスイベントを登録
win.bind('<Motion>', motion)
win.bind('<Button-1>', click)
# ゲームのメイン処理
init_game()
game_loop()
win.mainloop()

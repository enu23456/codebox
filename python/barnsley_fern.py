"""バーンスレーのシダ のアニメーション作成プログラム
Note:
    バーンスレーのシダとはフラクタル図形の1つ
Note:
    46行目のコメントアウトを外すと gifファイルでアニメーションを保存できる
""""""

"""

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation
from random import random

fig = plt.figure()
ims = []

x = 0.5
y = 0.5
x_arr = []
y_arr = []

for i in range(3600):
    r = random()
    if r < 0.02:
        x, y = 0.5, 0.27 * y
    elif 0.02 <= r <= 0.17:
        x, y = -0.139 * x + 0.263 * y + 0.57, 0.246 * x + 0.224 * y - 0.036
    elif 0.17 < r <= 0.3:
        x, y = 0.17 * x - 0.215 * y + 0.408, 0.222 * x + 0.176 * y + 0.0893
    elif 0.3 < r <= 1:
        x, y = 0.781 * x + 0.034 * y + 0.1075, -0.032 * x + 0.739 * y + 0.27
    x_arr.append(x*2)
    y_arr.append(y)
    if i % 6 == 0:
        plt.xlim(0.55, 1.55)
        plt.ylim(0.0, 1.0)
        plt.axes().set_aspect('equal')
        plt.xticks(color="None")
        plt.yticks(color="None")
        plt.tick_params(length=0)
        im, = plt.plot(x_arr, y_arr, 'o', color='green', markersize=2)
        ims.append([im])

ani = animation.ArtistAnimation(fig, ims)
# ani.save('barnsley_fern.gif', writer="imagemagick")
plt.show()

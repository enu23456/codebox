"""BubbleSort のアニメーション作成プログラム
Note:
    53行目のコメントアウトを外すと gifファイルでアニメーションを保存できる
"""

import random
import matplotlib.pyplot as plt
import matplotlib.animation as animation

random.seed(5)
array = [random.randint(1, 16) for i in range(15)]
j = 0
k = 0


def _update_plot(i, fig, im):
    plt.cla()
    global array
    global j
    global k

    plt.xlim(-1, 15)
    plt.ylim(0, 16)
    plt.axes().set_aspect('equal')
    plt.xticks(color="None")
    plt.yticks(color="None")
    plt.tick_params(length=0)
    if i < 5:
        im.append(plt.bar(list(range(len(array))), array))
    else:
        if array[k] > array[k + 1]:
            temp = array[k + 1]
            array[k + 1] = array[k]
            array[k] = temp
        im.append(plt.bar(list(range(len(array))), array))
        if k != len(array) - 2 - j or j != len(array) - 2:
            im.append(plt.bar(k + 1, array[k + 1]))
        # print(j, k)
        # print(array)
        if k == len(array) - 2 - j and j == len(array) - 2:
            pass
        elif k == len(array) - 2 - j:
            k = 0
            j += 1
        else:
            k += 1


fig = plt.figure()
im = []
ani = animation.FuncAnimation(fig, _update_plot, fargs=(fig, im), frames=120, interval=300)

# ani.save("Sample.gif", writer='imagemagick')
plt.show()

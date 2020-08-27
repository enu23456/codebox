"""BogoSort のアニメーション作成プログラム
Note:
    50行目のコメントアウトを外すと gifファイルでアニメーションを保存できる
"""

import random
import matplotlib.pyplot as plt
import matplotlib.animation as animation

random.seed(1)
length = 6
array = list(range(1, length+1))
array_raw = array.copy()
random.shuffle(array)
print(array_raw)
is_increment = True


def _update_plot(i, fig, im):
    plt.cla()
    global array

    plt.xlim(-1, length)
    plt.ylim(0, length + 1)
    plt.axes().set_aspect('equal')
    plt.xticks(color="None")
    plt.yticks(color="None")
    plt.tick_params(length=0)
    if i < 5:
        im.append(plt.bar(list(range(len(array))), array))
    else:
        flag = True
        for j in range(length):
            if array[j] != array_raw[j]:
                flag = False
                break
        if flag == False:
            random.shuffle(array)
            im.append(plt.bar(list(range(len(array))), array))
        else:
            im.append(plt.bar(list(range(len(array))), array, color="r"))

    return


fig = plt.figure()
im = []
ani = animation.FuncAnimation(fig, _update_plot, fargs=(fig, im), frames=520, interval=10)

# ani.save("BogoSort.gif", writer='imagemagick')
plt.show()

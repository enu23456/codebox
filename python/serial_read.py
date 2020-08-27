"""シリアル通信で読み取り表示し続けるプログラム
Note:
    動作確認OK
Note:
    参考: https://ameblo.jp/hitochan007/entry-12109258430.html
    参考: https://shizenkarasuzon.hatenablog.com/entry/2018/09/29/180625
"""
import serial  # シリアル通信モジュールをインポート

ser = serial.Serial("COM3", 115200)  # 受信用ポートをOpen

print('受信用ポート番号：', ser.portstr)  # 受信用ポート番号を確認

while True:
    line = ser.readline()
    text = line.decode('utf-8')  # 受信したデータをバイト型から文字列型に変換
    print(text.strip())  # 文字列.strip(): 特定の文字列や空白文字を削除するメソッド

ser.close()  # 受信型のポートをopen. ここに到達することはないが行儀よく書く

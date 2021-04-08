//! 入力画像に対し、単純平均法によるグレースケール化処理を行いPNG保存するプログラム
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! image = "0.22.3"
//!
//! 参考資料
//! https://doitu.info/blog/5badf7568dbc7a000fc280c6

use image::open;
use image::Rgb;

fn main() {
    // 画像の読み出し
    let mut img = open("hydrangea.jpg").unwrap().to_rgb();

    // 画像の大きさを出力.
    let (width, height) = img.dimensions();
    println!("width: {}, height: {}", width, height);

    for x in 0..width {
        for y in 0..height {
            // ピクセルの色を取得する（分かりやすくするためRGBの変数を作ってそこに格納している）
            let pixel = img.get_pixel(x, y);
            let red: u16 = pixel[0] as u16;
            let green: u16 = pixel[1] as u16;
            let blue: u16 = pixel[2] as u16;
            // 新たに設定する値を計算する）
            let gray_simple = ((red + green + blue) / 3) as u8;
            let new_color: Rgb<u8> = Rgb([gray_simple, gray_simple, gray_simple]);
            // ピクセルに値を設定する
            img.put_pixel(x, y, new_color);
        }
    }
    // 画像の保存
    img.save("result.png").unwrap();
}

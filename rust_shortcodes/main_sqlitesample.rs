//! SQLite の使用サンプルプログラム
//!
//! 必要な依存関係（Cargo.toml の [dependencies]）
//! sqlite = "0.24.0"
//!
//! 参考資料
//! https://gouf.hatenablog.com/entry/2019/04/16/200240

fn main() {
    // インメモリ データベースとして接続先を生成
    let connection = sqlite::open(":memory:").unwrap();

    // テーブルの作成とデータの挿入
    connection
        .execute(
            "
            CREATE TABLE users (name TEXT, age INTEGER);
            INSERT INTO users (name, age) VALUES ('Alice', 42);
            INSERT INTO users (name, age) VALUES ('Bob', 69);
            ",
        )
        .unwrap();

    // 年齢が 15才より上のレコードを選択
    // カラム名と値のセットを標準出力に送出
    connection
        .iterate("SELECT * FROM users WHERE age > 15", |pairs| {
            for &(column, value) in pairs.iter() {
                println!("{} = {}", column, value.unwrap());
            }
            true
        })
        .unwrap();
}

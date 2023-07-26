// 問題文 (ABC180 A - box)
use proconio::input;
// �
// N 個のボールが入っていた箱から A 個のボールを取り出し、新たに 
// B 個のボールを入れました。今、箱にはボールが何個入っていますか?

fn main() {
    // 標準入力を定義
    input! {
        n: i32,
        a: i32,
        b: i32,
    }

    println!("{}", n - a + b);
    println!("hello")
}

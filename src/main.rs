fn main() {
    let a = 26; //型推論変数
    // a = 25; （※これはできない）
    let mut _aA = 5; //変更可能
    _aA = 10;

    //型明示
    let b:i32 = 10; //32ビット整数
    let c:f64 = 3.14; //64ビット小数点
    let d: &str = "aa"; //文字列

    println!("Hello, world!");
    println!("a = {}", a);
    println!("b = {}", b);
}

use std::ptr::null_mut;
use windows::core::PCWSTR;
use windows::Win32::UI::WindowsAndMessaging::*;

struct MyData {
    a: i32,
    b: f64,
    c: String,
}

impl MyData {
    fn none_new() -> Self {
        Self {
            a: 1,
            b: 2.0,
            c: "3".to_string(),
        }
    }

    fn new(a: i32, b: f64, c: String) -> Self {
        Self { a, b, c }
    }

    fn print(&self) {
        println!("a = {}, b = {}, c = {}", self.a, self.b, self.c);
    }
}

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

    let text = "メッセージ";
    let title = "タイトル";

    let text_w: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let title_w: Vec<u16> = title.encode_utf16().chain(std::iter::once(0)).collect();

    let mut list: Vec<&str> = Vec::new();
    list.push("aaa");
    list.push("bbb");
    println!("{:?}", list);
    println!("{:?}", list[0]);

    let my_data = MyData::new(1, 1.0, "1".to_string());
    my_data.print();

    //所有権
    let s1 = String::from("s1");
    let s2 = s1; //s1を代入することで、s1は使えなくなる
    println!("{}", s2);
    // println!("{}", s1); //これはNG

    let s3 = String::from("s3");
    let s4 = s3.clone(); //clone関数で、s3もs4も使えるように
    println!("{}", s3); //可能
    println!("{}", s4); //可能

    let s5 = 10;
    let s6 = s5; //データ型（整数、bool、char等は代入でも可能）
    println!("{}", s5);
    println!("{}", s6);

    unsafe {
        MessageBoxW(
            None,
            PCWSTR(text_w.as_ptr()),
            PCWSTR(title_w.as_ptr()),
            MB_OK | MB_HELP,
        );
    }
}

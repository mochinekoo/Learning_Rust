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

    unsafe {
        MessageBoxW(
            None,
            PCWSTR(text_w.as_ptr()),
            PCWSTR(title_w.as_ptr()),
            MB_OK | MB_HELP,
        );
    }
}

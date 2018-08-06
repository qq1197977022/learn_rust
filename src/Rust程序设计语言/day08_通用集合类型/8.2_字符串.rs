/*
一.什么是字符串
    1.字节集合 + 方法
    2.Rust核心只有一种字符串str, 即字符串切片, 通常以借用形式出现; String由标准库提供, 没有写进Rust核心部分
    3.标准库包括其他字符串类型
        1.OsString, OsStr
        2.CString, CStr
二.内存表现
    1.String是对Vec<u8>的封装
*/
fn main() {
    let s1 = String::from("foo");
    let s2 = String::from("你好");
    let s3 = String::from("Здравствуйте");
    println!("{}\t{}\t{}", s1.len(), s2.len(), s3.len());
    println!("{}", &s3[0..2]);

    for c in s1.chars() {
        print!("{}\t", c);
    }

    println!();
    for c in s2.chars() {
        print!("{}\t", c);
    }

    println!();
    for c in s3.chars() {
        print!("{}\t", c);
    }

    println!();
    println!("{:?}\n\t{:?}\n\t{}", s1.as_bytes(), s2.bytes(), s3);
}

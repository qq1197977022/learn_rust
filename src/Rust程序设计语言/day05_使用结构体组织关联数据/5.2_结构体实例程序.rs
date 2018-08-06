use std::fmt;


fn main() {
    println!("===================================一.结构体实例程序: 计算长方形面积");
    _fn1();
    _fn2();
    _fn3();


    println!("===================================二.通过实现trait扩展功能");
    _fmt1();
}
//一.结构体实例程序: 计算长方形面积
fn _fn1() {
    let w = 30;
    let h = 50;
    let s = _area1(w, h);
    println!("{}", s);
}
fn _area1(w: u32, h: u32) -> u32 {  //不能体现矩形宽高关联性
    w * h
}
//使用元组重构
fn _fn2() {
    let rect = (3, 50);
    let s = _area2(rect);
    println!("{}", s);
}
fn _area2(rect: (u32, u32)) -> u32 {    //代码中数据意义不明确
    rect.0 * rect.1
}
//使用结构体重构
struct Rect {
    w: u32,
    h: u32
}
fn _fn3() {
    let rect = Rect {
        w: 30,
        h: 40
    };
    let s = _area3(&rect);
    println!("{}", s);
}
fn _area3(rect: &Rect) -> u32 { //结构体字段体现宽高关联性, 且代码数据意义明确
    rect.w * rect.h
}


/*
二.通过实现trait扩展功能
    1.基本类型默认实现Display格式化
    2.结构体输出格式具有多样性 ~ 不确定性, 因而Rust没有提供默认格式化方式, 而由用户自行实现
*/
fn _fmt1() {
    _fmt1_derive1();
    _fmt1_derive2();
}
fn _fmt1_derive1() {
    #[derive(Debug)]    //通过添加注解实现Debug trait
    struct Rect {
        w: u32,
        h: u32
    }
    let rect = Rect {
        w: 20,
        h: 3
    };
    println!("{:?}", rect);
    println!("{:#?}", rect);
}
fn _fmt1_derive2() {
    struct Rect {
        w: u32,
        h: u32
    }
    impl fmt::Display for Rect {    //手动实现Display trait
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "宽: {}, 高: {}", self.w, self.h)
    }
    }
    let rect = Rect {
        w: 20,
        h: 3
    };
    println!("{}", rect);
}
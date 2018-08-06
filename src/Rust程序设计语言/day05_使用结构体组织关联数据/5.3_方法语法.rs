use std::fmt;
/*
一.方法语法
    1.与函数类似但有所不同-----------------------------------------------------------
        1.在结构体/ 枚举/ trait对象上下文中定义 ~ 这里只介绍结构体上下文
        2.首参self表示调用该方法的结构体实例
        3.方法避免了在函数签名中重复self类型, 主要优点在于提高代码组织性 ~ 实例行为即方法组织在impl块中, 而不是随意分散
二.C/C++ 和 Rust方法调用-----------------------------------------------------------
    1.C/C++中有两个不同运算符调用方法
        1..: 直接在对象上调用方法
        2.->: 在对象指针上调用方法 ~ 先解引用指针再调用
            eg. objPtr->method() 等效 (*objPtr).method()
    2.Rust方法调用有自动引用/ 解引用功能
        1.自动引用: 自动添加&, &mut
        2.自动解引用: 自动添加*

        Note: 自动化实现原理
            1.根据方法接收者self和方法签名计算
三.关联函数-----------------------------------------------------------
    1.本质: 定义在impl块中的函数
    2.与结构体关联, 而不是结构体某具体实例对象
    3.调用语法: 结构体名::关联函数名() ~ ::语法用于关联函数和模块命名空间
*/
fn main() {

//    _fn1_method();
    _fn3_associated_fn();
}
fn _fn3_associated_fn() {
    struct Rect {
        w: u32,
        h: u32
    }
    impl Rect {
        fn area(&self) -> u32 {
            self.w * self.h
        }
        fn rect(w: u32) -> Rect {   //关联函数 ~ 常用作构造函数
            Rect {
                w,  //字段初始化简写
                h: w
            }
        }
    }
    impl fmt::Display for Rect {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "宽: {}, 高: {}", self.w, self.h)
        }
    }
    let rect = Rect::rect(30);
    println!("{};\tarea = {}", rect, rect.area());
}


fn _fn1_method() {
    struct Rect {
        w: u32,
        h: u32
    }
    impl Rect { //在结构体上定义方法
        fn area1(&self) -> u32 {
            /*
            1.引用传递, 不获取所有权
            2.不可变借用 ~ 只读
            */
            self.w * self.h //自动解引用
        }
        fn area2(&mut self) -> u32 {
            /*
            1.引用传递, 不获取所有权
            2.可变借用 ~ 可读写
            */
            self.w = self.w * 2;    //自动解引用
            self.h = self.h * 2;
            self.w * self.h
        }
        fn area3(mut self) -> u32 {
            /*
            1.栈数据传递, 拷贝
            2.可变变量 ~ 可读写
            */
            self.w = self.w * 2;
            self.h = self.h * 2;
            self.w * self.h
        }

        fn can_hold(&self, rect: &Rect) -> bool {
            if self.w >= rect.w && self.h >= rect.h {
                true
            } else {
                false
            }
        }
    }
    let rect1 = Rect {
        w: 20,
        h: 70
    };
    let mut rect2 = Rect {
        w: 20,
        h: 70
    };
    let rect3 = Rect {
        w: 20,
        h: 70
    };

    let s1 = rect1.area1();    //方法调用 ~ 自动引用
    let s2 = rect2.area2();
    let s3 = rect3.area3();
    println!("area of rect1: {}", s1);
    println!("area of rect2: {}", s2);
    println!("area of rect3: {}", s3);

    let r1_can_hold_r2 = rect1.can_hold(&rect2);
    let r2_can_hold_r1 = rect2.can_hold(&rect1);
    println!("rect1_can_hold_rect2: {}", r1_can_hold_r2);
    println!("rect2_can_hold_rect1: {}", r2_can_hold_r1);
}


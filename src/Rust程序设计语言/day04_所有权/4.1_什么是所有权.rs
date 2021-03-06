/*
一.什么是所有权--------------------------------------------
    1.程序管理运行时使用内存方式
        1.手动
        2.自动
            1.GC
            2.所有权系统
                1.编译器根据所有权规则在编译时检查, 无运行时开销
二.堆栈--------------------------------------------
    1.堆栈是程序执行期间可供使用的内存, 了解值位于堆还是栈上很有必要
    2.堆栈特点
        1.栈存
            1.LIFO: 后进先出
            2.增加/ 移出数据: 入栈/ 出栈
            3.快速
                1.访问数据方式: 只在栈顶存取数据, 无需查找
                2.栈数据大小固定
                    eg. 指针是大小固定的数据类型, 因而存储在栈上, 但访问实际数据时, 必须访问指针
        2.堆存
            1.存储编译时大小未知 或 运行时可变大小数据
            2.缺乏组织
                1.在堆上分配内存: 请求堆存时, 操作系统把堆上一块足够大的空间标识为已使用, 并返回该位置的指针  ~ 简称: 分配
            3.慢速
                1.访问数据方式: 必须通过存储在栈上指针来访问 ~ 现代处理器在内存中跳转越少越快 ~ 缓存
                2.分配堆存消耗时间
    3.从堆栈看函数调用
        1.调用函数时, 形参和局部变量入栈, 调用结束时出栈
            1.实际上进出栈对象是变量绑定值
    4.从堆栈看所有权
        1.mark: 记录堆数据使用情况
        2.min: 最小化堆数据冗余
        3.clear: 清理无效堆数据
三.所有权规则--------------------------------------------
    1.一言蔽之: 值有且只有一个所有者, 所有者离开作用域时其值即被抛弃 ~ 一夫一妻制, 夫唱妇随
    2.所有者即与值绑定的变量
四.变量作用域--------------------------------------------
    1.栈数据所有者离开作用域时即出栈
    3.堆数据
五.内存与分配--------------------------------------------
    1.堆数据所有者离开作用域时, 自动调用drop函数 ~ 受C++资源获取即初始化模式影响 ~ RAII模式
六.变量与数据交互方式--------------------------------------------
    1.栈数据: 拷贝
        1.栈数据大小已知固定, 拷贝速度快, 对运行时影响较小
        2.Rust默认采用拷贝语义
    2.堆数据
        1.移动
            1.堆数据编译时大小未知, 运行时大小可变, 拷贝速度慢, 通常数据量大, 拷贝对运行时影响较大
            2.Rust默认采用移动语义 ~ 所有权规则体现
        2.克隆
七.所有权与函数--------------------------------------------
    1.参数传递: 本质是赋值, 因而涉及变量与数据交互方式
        1.栈数据: 拷贝
        2.堆数据: 移动
    2.返回值: 转移作用域
*/
#![feature(non_ascii_idents)]
fn main() {
//    _fn4();   //变量作用域
//    _fn5();   //内存与分配
//    _fn6_move();    //移动
//    _fn6_clone();    //克隆
//    _fn7_pass();  //函数参数传递
    _fn7_return();  //函数返回值


    println!("-----------------------------------------");
}


fn _fn7_return() {
    let _s1 = gives_ownership();
    let s2 = String::from("Hello");
    let _s3 = takes_and_gives_back(s2);
}
fn gives_ownership() -> String {
    let s = String::from("Hello");
    s
}
fn takes_and_gives_back(s: String) -> String {
    s
}

fn _fn7_pass() {
    let s = String::from("Hello");
    _takes_ownership(s);

    let n = 5;
    _makes_copy(n);
}
fn _takes_ownership(s: String) {
    println!("{}", s);
}
fn _makes_copy(n: i32) {
    println!("{}", n);
}


fn _fn6_总结() {
    /*
    一.Rust注解
        1.Copy trait
            1.适用于栈数据
            2.Copy类型
                1.简单标量值
                    1.整型
                    2.布尔
                    3.浮点型
                    4.元组 ~ Copy类型元素
        2.Drop trait
    */
}
fn _fn6_clone() {
    let s1 = String::from("Hello");
    /*
    1.从字符串字面创建String类型数据, 绑定其标头值到变量s1
    2.String类型
        1.数据结构
            1.标头值是栈数据
                1.指针ptr ~ 指向堆数据 ~ 实际数据
                2.长度len
                3.容量cap
            2.堆数据
    */
    let _s2 = s1.clone();   //拷贝s1堆数据(深拷贝), 并绑定其标头值到变量_s2
}
fn _fn6_move() {
    let x = 5;  //把数据5绑定到变量x
    let _y = x;  //把与变量x绑定的数据的拷贝绑定到变量y
    /*
    1.数据5是已知固定大小的i32类型栈数据
    */

    let s1 = String::from("Hello");
    /*
    1.从字符串字面创建String类型数据, 绑定其标头值到变量s1
    2.String是引用类型
        1.标头值是栈数据
            1.指针ptr ~ 指向堆数据 ~ 实际数据
            2.长度len
            3.容量cap
    */
    let _s2 = s1;
    /*
    1.把与变量s1绑定的数据(标头值)的拷贝绑定到变量s2 ~ 浅拷贝
    2.使s1无效 ~ 卸磨杀驴版浅拷贝 ~ 移动
    */
}


fn _fn5() {
    let mut s = String::from("Hello");
    s.push_str(", World");
    println!("{}", s);
}


//#[allow(dead_code)]
fn _fn4() {
    //#![allow(unused_variables)]
    let _s = "hello";
    /*
    1.默认
        1.变量: #![warn(unused_variables)]
        2.函数: #![warn(dead_code)]
    2.解决:
        1.allow
        2._前缀
    */
}

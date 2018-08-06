/*
一.定义枚举
二.枚举成员关联值
三.方法语法 ~ 同结构体
四.Option枚举
    1.Note
        1.Option枚举在标准库中定义
        2.被包含在prelude中, 即无需显式引入
        3.其成员可以直接访问, 无需Option::前缀
        4.<T>是泛型参数, 即Some成员关联值类型任意
        5.Option<T>和T是不同类型
    2.变量两种状态
        1.有空值null的语言: 非空值/ 空值null
            1.null概念意义: 目前缺失或无效的值, 即没有值
        2.Rust没有空值null: 非空值/ 非法
            1.参见null发明者的忏悔: https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html#option-%E6%9E%9A%E4%B8%BE%E5%92%8C%E5%85%B6%E7%9B%B8%E5%AF%B9%E4%BA%8E%E7%A9%BA%E5%80%BC%E7%9A%84%E4%BC%98%E5%8A%BF
    3.对存在/ 缺失概念的表示
        1.有空值null的语言
            1.非空值: 存在
            2.空值null: 缺失/ 无效
        2.Rust: Option<T>枚举
            enum Option<T> {
                Some(T),    //一些值
                None    //没有值
            }
            1.Some: 存在关联值
            2.None: 和null表达意义相同
        3.为什么Option<T>比null好
            1.Option<T>和T是不同类型
            2.Rust编译器确保内置类型必有有效值, 无需判空
            3.仅使用Option<T>时, 需要考虑空值, 但Rust编译器保证在使用值前处理为空的情况 ~ 即对Option<T>进行T运算前, 必须转换为T, 进而避免把空值当做非空值使用
            4.换言之
                   1.对于可能为空的值, 必须显式将其放入Option<T>中
                2.使用该值时, 必须明确处理值为空的情况
                3.非Option<T>类型值, 可安全认为其值非空
                4.这是Rust有意为之的设计选择, 以避免空值保证Rust代码安全性
*/
fn main() {
    _fn1();
    _fn2();
    _fn3_method();
    _fn4_option();
}


fn _fn4_option() {
    let n1 = Some(5);
    let str = Some("字符串字面 ~ 即字符串切片");
    let num: Option<i32> = None;    //使用None需要指明Option<T>类型, 编译器仅通过None值无法推断Some成员关联值类型

    let n2 = 5;

    let sum = n1 + n2;
}


fn _fn3_method() {
    #[allow(dead_code)]
    enum Message {
        Quit,   //无关联值成员
        Move {x: i32, y: i32},  //带关联值成员
        Write(String),
        ChangeColor(i32, i32, i32)
    }
    impl Message {  //在枚举上定义方法
        fn call(&self) {
            println!("枚举方法语法");
        }
    }

    let quit = Message::Quit;
    quit.call();
}


fn _fn2() {
    println!("------struct------");
    _fn2_struct();
    println!("------enum------");
    _fn2_enum();
}
//1.通过结构体关联IP类型和具体IP地址
fn _fn2_struct() {
    #[derive(Debug)]    //添加注解实现Debug trait
    enum IpAddrKind {
        V4,
        V6
    }
    #[derive(Debug)]    //添加注解实现Debug trait
    struct IP {
        kind: IpAddrKind,
        addr: String
    }
    /*
    1.枚举成员位于其标识符命名空间
        1.IpAddrKind::V4和IpAddrKind::V4都是IpAddrKind类型
    */
    let home = IP {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1")
    };
    let loopback = IP {
        kind: IpAddrKind::V6,
        addr: String::from("::1")
    };

    println!("{:?}", home);
    println!("{:#?}", loopback);
}
/*
2.使用枚举重构
    1.相对结构体优势: 枚举成员关联值类型可不同
*/
fn _fn2_enum() {
    #[derive(Debug)]    //添加注解实现Debug trait
    enum IP {
        V4(u8, u8, u8, u8),
        V6(String)
    }
    let home = IP::V4(127, 0, 0, 1);
    let loopback = IP::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:#?}", loopback);
}


fn _fn1() {
    #[derive(Debug)]    //添加注解实现Debug trait
    enum IpAddrKind {   //定义枚举
        V4,
        V6
    }
    let v4 = IpAddrKind::V4;    //实例化枚举成员
    let v6 = IpAddrKind::V6;
    println!("{:?}", v4);
    println!("{:?}", v6);
}


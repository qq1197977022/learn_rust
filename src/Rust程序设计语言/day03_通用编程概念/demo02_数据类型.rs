fn main() {
    let num: f32 = "42".parse().expect("...");  //类型注解
    println!("\"42\"-->{}", num);

    let num1 = 19_888;
    let num2 = 19_888u32;
    let num3 = 19_888i128;
    let num4 = 19_888isize;
    let num5 = 19888usize;
    println!("num1 = {}, num2 = {}, num3 = {}, num4 = {}, num5 = {}", num1, num2, num3, num4, num5);

    let num6 = 19_888;
    println!("num6 = {}", num6);
    let num6 = 19_888u32;
    println!("num6 = {}", num6);
    let num6 = 19_888i128;
    println!("num6 = {}", num6);
    let num6 = 19_888isize;
    println!("num6 = {}", num6);
    let num6 = 19888usize;
    println!("num6 = {}", num6);

    //元组
    let tup = (1, 'A', "HelloWorld", 3.14);
    println!("{} {} {} {}", tup.0, tup.1, tup.2, tup.3);    //索引

    let (x, y, z, m) = tup; //使用模式匹配, 解构元组
    println!("x = {}, y = {}, z = {}, m = {}", x, y, z, m);

    //数组
    let num_arr = [1, 2, 3, 4];
    println!("{} {}", num_arr[0], num_arr[3]);
}


/*
一.数据类型
    A.杂项
        1.Rust中所有值都属于一种明确的类型, 这告诉Rust它被指定为何种数据, 以明确处理方式
        2.Rust是静态类型, 即编译时必须确定变量类型, 便于在编译时检查其他使用该变量的地方其类型是否有效
            1.rust不能在变量类型只能在运行时确定的情况下工作, 这会使编译器变得更复杂且只能为代码提供更少的保障, 因必须记录所有变量的所有可能类型
            2.使用编译器推断功能, 当多种类型均有可能时, 必须添加类型注解 ~ 第二章猜数字parse()函数
    B.分类
        1.标量类型
            1.整型
                1.有符号: 定长i8~i128 + 架构相关isize
                2.无符号: 定长u8~u128 + 架构相关usize
                Note
                    1.字面
                        1.十进制
                        2.八进制
                        3.十六进制
                        4.二进制
                        5.byte ~ 仅u8
                    2.分隔符: _
                    3.类型后缀 ~ 非byte
                    4.默认i32 ~ 速度快, 甚至在64位系统上也是
            2.浮点: f32 + f64
                1.默认: f64 ~ 现代CPU中与f32速度几乎一样, 但精度更高
                2.采用IEEE-754标准, f32, f64分别是单精度/双精度浮点数
            3.布尔: true/false
            4.字符: Unicode标量值
        2.复合类型: 由多个其他类型的值组合而成
            1.原生复合类型
                1.数组: 固定长度同类型元素集合 ~ 标准库提供可变长度集合vector
                    1.数组在栈上分配一整块连续内存
                2.元组 ~ 同Python
*/




























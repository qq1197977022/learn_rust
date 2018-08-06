/*
一.定义_实例化结构体
二.字段初始化简写 ~ 形参与字段同名
三.结构体更新语法: 基于旧实例创建新实例对象
        1.非更新语法
        2.更新语法
四.元组结构体
五.类单元结构体 ~ 类似(), 即unit类型
    1.特点: 无字段
    2.常见用法: 需要在某类型上实现trait, 但不需要在类型内存储数据时使用
六.结构体字段所有权 ~ 待续...
*/

//定义结构体
#[allow(dead_code)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
//    _fn1();
//    _fn2();
//    _fn3_update();
//    _fn4_tuple_struct();
//    _fn5_unit_struct();
}


#[allow(dead_code)]
fn _fn5_unit_struct() {
    //定义类单元结构体
    struct Color();
    struct Point {};
    struct PC;
}


fn _fn4_tuple_struct() {
    //定义元组结构体
    struct Color (i32, i32, u32);
    struct Point (u32, f32, i32);
    //实例化元组结构体
    let black = Color(0, 1, 3);
    let origin = Point(7, 0.6, 9);

    println!("black: {}\t{}", black.0, black.1);    //访问元组结构体元素
    println!("origin: {}\t{}", origin.1, origin.2);

    let Color(a, b, c) = black; //使用模式匹配解构元组结构体
    println!("{}\t{}\t{}", a, b, c);
}


fn _fn3_update() {
    let user1 = User {
        email: String::from("1197977022@qq.com"),
        name: String::from("Alex"),
        active: true,
        sign_in_count: 250
    };
    let user2 = User {
        email: String::from("2607705215@qq.com"),
        name: String::from("Tom"),
        active: user1.active,   //基于旧实例创建新实例对象 ~ 不使用更新语法
        sign_in_count: user1.sign_in_count
    };
    let user3 = User {
        email: String::from("2607705215@qq.com"),
        name: String::from("Alice"),
        ..user1 //基于旧实例创建新实例对象 ~ 更新语法
    };
    println!("{}\t{}", user2.name, user2.active);
    println!("{}\t{}", user3.name, user3.sign_in_count);
}


fn _fn2() {
    let email = String::from("1197977022@qq.com");
    let name = String::from("Alex");
    let alex = _fn2_short(email, name);
    println!("{}\t{}", alex.name, alex.active)
}
fn _fn2_short(email: String, name: String) -> User {
    User {
        name,   //形参和字段同名, 简写
        email: email,  //不简写
        active: false,
        sign_in_count: 775200
    }
}


#[allow(unused_variables)]
fn _fn1() {
    //实例化结构体
    let mut tom = User {
        email: String::from("1197977022@qq.com"),
        name: String::from("Tom"),
        active: true,
        sign_in_count: 775200
    };
    //修改结构体实例对象字段值
    tom.name = String::from("杨一帆");
}
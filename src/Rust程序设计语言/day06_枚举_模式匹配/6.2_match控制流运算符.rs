/*
一.match控制流运算符
    1.一个值与一系列模式匹配 ~ 类似switch
    2.模式分类
        1.字面值
        2.变量
        3.通配符
        4.etc
二.匹配Option<T>
三.match匹配穷尽
    1.必须穷举所有可能性
四._通配符
    1.当要穷举元素巨多, 但只关注冰山一角时, 可使用_通配符, 改穷举为列举
*/
fn main() {
    _fn1_match1();
    _fn1_match2();

    _fn2_match_option();
}


fn _fn4_() {
    let num = 0u8;
    match num {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => () //单元类型也成为nil, 唯一一个值即(), 表示没有有意义的值可以返回, 函数默认返回值
    }
}


fn _fn3_exhaustive() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1)
            /*
            报错: non-exhaustive patterns: `None` not covered
            1.Rust匹配是穷尽的 ~ 必须穷举所有可能性
            2.免于假设拥有一个实际上为空的值 ~ 有空值null语言做法 ~ 参见6.1
            */
        }
    }
}


#[allow(unused_variables)]
fn _fn2_match_option() {    //匹配Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}\t\t{:?}\t\t{:?}", five, six, none);
}


fn _fn1_match2() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum State {
        Alabama,
        Alaska
    }
    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(State)  //成员带关联值
    }
    let value = value_in_cents(Coin::Quarter(State::Alaska));
    println!("{}", value);

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }
}


fn _fn1_match1() {
    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter
    }
    let value = value_in_cents(Coin::Nickel);
    println!("{}", value);

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {    //匹配硬币类型, 返回面值
            Coin::Penny => 1,
            Coin::Nickel => {
                println!("Lucky Nickel");
                5
            },
            Coin::Dime => 10,
            Coin::Quarter => 25
        }
    }
}

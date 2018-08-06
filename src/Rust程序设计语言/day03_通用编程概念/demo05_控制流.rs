use std::io;

fn main() {
    loop {
        println!("请输入: ");
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read line");
        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入任意正整数<5");
                continue;
            }, //通配符值_匹配所有Err值
        };
        fun1(num);
        fun2(num);
        fun3(num);
        fun4();

        let num = num as f32;   //类型转换
        println!("{} C is {}F", num, c_to_f(num));

        let num = num as i32;
        println!("---------------->{}", fib(num));



        if num == 32 {
            break;
        }
    }


}

fn fun1(num: i32) {
    print!("多重条件\n\t");
    //多重条件
    if num == 0 {
        println!("num = {}", num);
    } else if num == 1 {
        println!("num = {}", num);
    }else if num == 2 {
        println!("num = {}", num);
    } else {
        println!("num = {}", num);
    }
}
fn fun2(num: i32) {
    print!("在let语句中使用if表达式\n\t");
    let num = if num == 0 {
        num
    } else if num == 1 {
        num
    }else if num == 2 {
        num
    } else {
        -1
    };
    println!("num = {}", num);
}
fn fun3(num: i32) {
    println!("while ~ 条件循环");
    let mut num = num;
    while num > 0 {
        println!("\t************");
        num -= 1;
    }
}
fn fun4() {
    print!("for循环\n\t");
    let arr = [11, 22, 33, 44];

    let mut i = 0;
    while  i < 4 {
        println!("arr[{}] = {}", i, arr[i]);
        i += 1;
    }
    /*
    while循环遍历数组
        1.易错 ~ 索引越界
        2.低效 ~ 编译器增加了运行时代码来对每次循环各元素进行条件检查
    */
    i = 0;
    for ele in arr.iter() { //迭代
        println!("arr[{}] = {}", i, ele);
        i += 1;
    }

    for i in 0..4 { //range
        println!("{}", arr[i]);
    }
    for i in (0..4).rev() { //range反转
        println!("{}", arr[i]);
    }
}
//练习题
fn c_to_f(c: f32) -> f32 {   //摄氏度转华氏度: F = 32+1.8*C
    print!("摄氏度转华氏度\n\t");
    32.0 + 1.8 * c
}
fn fib(n: i32) -> i32 {
    print!("n阶斐波那契数列\n\t");
    if 0 < n && n < 3 {
        1
    }else if n >= 3 {
        fib(n-1) + fib(n-2)
    } else {
        -1
    }
}
/*
一.if表达式
    1.以关键字if开头
    2.if表达式中与条件关联的代码块有事称之为arms ~ 猜数字游戏中match表达式
二.多重条件
三.在let语句中使用if表达式
四.循环
    1.loop: 无限循环
    2.while: 条件循环
    3.for: 遍历
*/














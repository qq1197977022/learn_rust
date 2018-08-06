extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;  //Rng是一个trait, 定义随机数生成器应实现的方法
/*
1.在当前作用域显式引入标准库中的io库
2.需要的类型不在prelude中时, 需要显式引入
*/

fn main() {
    let rand_num = rand::thread_rng().gen_range(1, 5);
    println!("rand_num = {}", rand_num);

    loop {
        println!("please input your guess: ");
        let mut guess_num = String::new();  //创建空字符串
        /*
        1.声明可变变量 ~ Rust变量默认不可变
        2.创建String空实例并绑定到可变变量guess
        3.:: 关联函数 ~ 针对类型, 而不是该类型实例 ~ 某些语言称之为静态方法
        4.rust拥有静态强类型系统, 也有类型推断
        */
        io::stdin().read_line(&mut guess_num)   //逻辑行第一部分
            .expect("Failed to read line"); //第二部分
        /*
        1.stdin()构造当前进程的标准输入句柄 ~ 全局缓冲的引用
        2.读取输入到指定缓冲
        3.&表示引用 ~ 共享访问, 无需拷贝多次
        4.read_line返回Result ~ 枚举
            1.枚举类型持有固定集合的值 ~ 成员
                eg. Result成员
                    1.Ok: 表示操作成功, 包含成功时产生的值
                    2.Err: 表示操作失败, 包含失败的前因后果
            2.作用: 错误处理
        */
        /*let guess_num: u32 = guess_num.trim().parse().expect("转换失败");*/
        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入任意正整数<5");
                continue;
            }, //通配符值_匹配所有Err值
        };
        /*
        1.用新值隐藏旧值, 以复用变量名guess_num而不是创建新变量名, 常用于值类型转换场景
        2.trim去掉read_line函数读取的换行符'\n'
        3.parse将字符串解析为数字类型 ~ 可解析到多种数字类型, 因此需要指明解析类型 ~ 变量后冒号':'指定其类型
        */
        println!("You guessed: {}", guess_num); //使用{}占位符

        match guess_num.cmp(&rand_num) {    //guess_num指明了类型为u32, rand_num也会被自动推断为u32 ~ 默认i32
            Ordering::Less => println!("small!"),
            Ordering::Greater => println!("big!"),
            Ordering::Equal => {
                println!("win!");
                break;
            },
        }
        /*
        1.cmp返回Ordering类型 ~ 枚举
            1.成员: Less, Greater, Equal
        2.match表达式由包含不同模式的分支组成
        */
    }
}
/*
extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop {
        println!("请输入一个正整数<5");
        let mut guess_num = String::new();
        io::stdin().read_line(&mut guess_num).expect("读取失败");
        let guess_num: u32 = match guess_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("解析失败 ~ 请输入一个正整数<5");
                continue;
            },
        };
        let rand_num = rand::thread_rng().gen_range(1, 5);
        println!("guess_num = {}, rand_num = {}", guess_num, rand_num);

        match guess_num.cmp(&rand_num) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            },
        }
    }
}
*/
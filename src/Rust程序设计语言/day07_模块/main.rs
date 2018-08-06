/*
1.命名空间语法: ::
2.私有性规则
    1.公有: 可被任意父模块访问
    2.私有: 可被直接父/ 任意子模块访问

*/
extern crate communicator;  //导入根模块
/*
1.所有模块都位于与库crate同名的根模块(communicator)内部
2.即使在子模块中使用外部crate也应该在根模块中导入
*/
use a::series::of;  //从模块 导入指定名称到作用域
use Companies::Baidu;  //从枚举 导入指定名称到作用域
use Companies::{
    Alibaba,
    Tencent,
    Mi
};
//use Companies::*;   //使用glob运算符*, 一次性导入所有名称
/*
1.避免名称冗余
2.当从同一命名空间导入多项时, 可用大括号以逗号区分列举
*/
fn main() {
    communicator::client2::conn();


    of::nested_modules();


    let baidu = Baidu;
    let alibaba = Alibaba;
    let tencent = Tencent;
    let mi = Mi;
    let google = Companies::Google;
    println!("{:?}\t\t{:?}\t\t{:?}\t\t{:?}\t\t{:?}", baidu, alibaba, tencent, mi, google)
}


pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("使用use关键字导入模块/ 模块中定义")
            }
        }
    }
}

#[derive(Debug)]
enum Companies {
    Baidu,
    Alibaba,
    Tencent,
    Mi,
    Google
}



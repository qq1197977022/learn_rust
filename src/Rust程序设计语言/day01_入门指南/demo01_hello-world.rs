fn main() {
    println!("Hello, world!");
}
/*
一.HelloWorld
    1.多词文件名以下划线分隔
    2.函数体必须以{}定界 ~ 有些语言, 当函数体只有一行时可省略花括号
    3.缩进: 4个空格, 而不是1个tab
    4.带!的调用是宏调用, 不是普通函数, 宏是Rust元编程的关键所在
    5.分号";"表示一个表达式的结束和下一表达式的开始
    6.Rust是一种预编译静态类型语言
二.Cargo
    7.Cargo: Rust的构建系统和包管理工具
        eg.构建代码, 下载依赖库并编译
        1.Cargo期望源文件位于 src 目录，将项目根目录留给 README、license 信息、配置文件和其他跟代码无关的文件。这样，Cargo 帮助你保持项目干净整洁，一切井井有条
        2.如果没有用 Cargo 创建项目，比如 hello_world 目录中的项目，可以通过将代码放入 src 目录，并创建一个合适的 Cargo.toml，将其转化为一个 Cargo 项目
    8.TOML(Tom's Obvious Minimal Language): 类似于INI, 但做了额外改进; 被用作Cargo的配置文件格式
        [package]   # 部分标题: 表明下属语句用于配置包
        name = "learn_rust"
        version = "0.1.0"
        authors = ["kc"]

        [dependencies]  # 项目依赖crates列表 ~ 称Rust代码包为crate
    9.Cargo 使用 Cargo.lock 来记录程序的依赖
*/

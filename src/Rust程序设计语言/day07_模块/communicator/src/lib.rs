#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        /*::client2::conn();  //:: ~ 从根模块开始并列出完整路径

        super::client2::conn(); //super ~ 访问父模块*/

        use super::client2;
        /*
        1.use默认相对根模块
        2.super:: 使use相对父模块
        */
        client2::conn();
    }
}


//一.模块声明
mod network1 {
    fn conn() {}
    mod server1 {   //子模块
        fn conn() {}
    }
}
mod client1 {
    fn conn() {}
}


/*
二.将模块移动到其他文件
    1.Rust默认只知道二进制crate项目src/main.rs和库crate项目src/lib.rs中的内容
    2.模块文件系统规则
        1.没有子模块: eg. client2
        2.有子模块: eg. network2
*/
mod network2;   //保留模块声明, 移动模块内容到模块名命名的文件中
pub mod client2;



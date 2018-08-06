/*
1.存储键值对 ~ 通过哈希函数实现键值映射 ~ 决定如何将键值放入内存
2.哈希map和vector类似
    1.同质: 键值分别同类型
    2.实际数据存储在堆上
*/
use std::collections::HashMap;  //HashMap相对vector和字符串不常用, 所以不在prelude中
fn main() {
    let mut hm = HashMap::new();
    hm.insert(String::from("A"), 65);
    hm.insert(String::from("B"), 66);
    hm.insert(String::from("a"), 97);
    hm.insert(String::from("b"), 98);
    println!("{:?}", hm);
    println!("{:#?}", hm);

    /*let s1 = "hello every one, I'm your father";

    for word in s1.split_whitespace() {
        print!("{}\t", word);
    }*/
    {
        let tmp = hm.entry(String::from("C")).or_insert(67);
        println!("{}", tmp);
    }
    println!("{:#?}", hm);
}
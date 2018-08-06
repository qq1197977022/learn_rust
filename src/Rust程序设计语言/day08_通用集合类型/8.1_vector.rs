/*
1.Vec<T>用于连续存储同类型值 ~ 堆存分配内容, 连续可增长的数组类型
*/
fn main() {
    _demo01();
    _demo02();
}
fn _demo01() {
    let _v1: Vec<i32> = Vec::new();    //1.构造函数

    let _v2 = vec![1, 2, 3]; //2.vec!宏 ~ 自动推导

    let mut v3 = Vec::new();    //3.自动推导
    v3.push(111);
    v3.push(222);
    v3.push(333);
    println!("{}\t{}\t{:?}\t", v3[0], v3[2], v3.get(1));
    /*
    两种方法访问vector元素
        1.[]索引语法
        2.get方法
    */
    v3[2] = 888;
    println!("{}\t{}\t{:?}\t", v3[0], v3[2], v3.get(1));

    for ele in &v3 {    //遍历
        println!("{}", ele);
    }
    for ele in &mut v3 {
        *ele *= 10; //解引用, 访问元素值
        println!("{}", ele);
    }
}
/*
利用枚举实现在vector中存储不同类型元素
    1.枚举成员都是同一自定义枚举类型
    2.枚举成员关联值类型任意
*/
fn _demo02() {
    #[derive(Debug)]
    enum Table {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row = vec![
        Table::Int(3),
        Table::Text(String::from("blue")),
        Table::Float(3.14)
    ];
    for ele in &row {
        println!("{:?}", ele);
    }
}

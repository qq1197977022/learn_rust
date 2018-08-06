fn main() {
    _fn1();
}


fn _fn1() {
    _fn1_match(Some(3u8));  //形参有类型注解, 此处u8后缀可省, 编译器能自动推断
}


fn _fn1_match(u8_value: Option<u8>) {
    match u8_value {
        Some(3) => println!("three"),
        _ => () //使用match进行单一模式匹配时, 但必须额外使用_通配符, 以符合match匹配的穷尽性
    }
}
//使用if let重构
fn _fn1_if_let(u8_value: Option<u8>) {
    if let Some(3) = u8_value {
        println!("three");
    }
    /*
    1.if let else和单一模式匹配的match等效
    2.if let和单一模式匹配的非穷尽match等效
    */
}


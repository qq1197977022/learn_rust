/*
一.概述
    1.Rust标准库包含一系列称为集合的数据结构
    2.特点
        1.大部分数据类型代表一个特定值, 集合可包含多个值
        2.不同于数组和元组, 集合实际数据存储在堆存 ~ 编译时大小可未知, 且运行时可变
        3.不同集合不同能力和代价 ~ 择优
    3.数据结构
        1.栈存标头值
            1.指针ptr ~ 指向堆存 ~ 实际数据
            2.长度
            3.容量
        2.堆数据
    4.广泛使用的三个集合
        1.vector: 连续存储
        2.String: 字符集
        3.哈希map: 键值对
二.总结
*/

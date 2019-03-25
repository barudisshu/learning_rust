本章覆盖有：

- 简单、自由(free)函数和方法需要避免写入生命周期指示器(lifetime specifiers)，因为它们是被推断的
- 为什么包含引用的结构体(structs)、元组-结构体(tuple-structs)、枚举(enums)需要生命周期指示器(lifetime specifiers)
- 如何为结构体(structs)、元组-结构体(tuple-structs)、枚举(enums)编写生命周期指示器(lifetime specifiers)
- 为什么包含指向泛型参数的结构体需要生命周期边界(协变、逆协变)

## Lifetime Elision



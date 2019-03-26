本章覆盖有：

- 简单、自由(free)函数和方法需要避免写入生命周期指示器(lifetime specifiers)，因为它们是被推断的
- 为什么包含引用的结构体(structs)、元组-结构体(tuple-structs)、枚举(enums)需要生命周期指示器(lifetime specifiers)
- 如何为结构体(structs)、元组-结构体(tuple-structs)、枚举(enums)编写生命周期指示器(lifetime specifiers)
- 为什么包含指向泛型参数的结构体需要生命周期边界(协变、逆协变)

## Lifetime Elision

上一章节，我们看到每个函数签名，都必须确定引用返回值，即要么是`'static`的lifetime specifier，要么是关联的函数参数的lifetime specifier。

有时避免编写这种烦人的注解。

```rust
trait Tr {
    fn f(x: &u8) -> &u8;
}
```

这代码是被允许的。返回的值是个引用，它没用指定生命周期，但它也不是`'static`的，因此这个隐式生命周期指示器(implicit lifetime specifier)必须是函数参数中的其中一个。因为函数参数仅有一个，因此它的lifetime specifier无可厚非就是该参数关联的lifetime specifier。换言之，这个声明函数等效于下面的写法：

```rust
trait Tr {
    fn f<'a>(x: &'a u8) -> &'a u8;
}
```

甚至下面的声明也是有效的：

```rust
trait Tr {
    fn f(b: bool, x: (u32, &u8)) -> &u8;
}
```

因为仅有一个引用，因此可以推断出返回值指向的对象。

下面的代码也是有效的：

```rust
trait Tr {
    fn f(x: &u8) -> (&u8, &f64, bool, &Vec<String>);
}
```

这种情况，返回值有好几个引用，但参数的引用仍然仅有一种。

参数引用类型仅有一种时，你甚至可以省略其返回值类型，而用在其它类型中指定。

```rust
trait Tr {
    fn f<'a>(x: &'a u8) -> (&u8, &'a f64, bool, &'static Vec<String>);
}
```

这里的返回值包含三个引用：第一个没有指定，第二字段有`'a`，第三个用了`'static`生命周期指示器。然而，参数中仍然仅有一个引用，因此返回的第一个字段的引用有一个隐式`'a`生命周期指示器。

这种省略机制称为“**lifetime elision**”。为了简化语法，生命周期指示器遇到这种仅有一个非静态(non-static)值时，可以 __删去(elided)__ 。

## Lifetime Elision with Object-Oriented Programming

考虑下面：

```rust
trait Tr {
    fn f(&self, y: &u8) -> (&u8, &f64, bool, &Vec<String>);
}
```




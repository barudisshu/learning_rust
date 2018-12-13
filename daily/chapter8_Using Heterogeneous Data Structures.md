本章覆盖有：

异构数据结构，就是不同类型的组合，包括有，

- Tuples
- Structs
- Tuple-Structs

用于不同类型的分组对象。本章最后，会介绍一些代码风格的规约。

## The Tuples

数组和向量可以包含多个条目元素，每个条目必须是相同类型。如果你希望一个对象存储不同类型的几个子对象，可以如下实现：

```rust
let data = (10000000, 183.19, 'Q');
let copy_of_data = data;
print!("{}, {}, {}", data.0, copy_of_data.1, data.2);
```

结果将打印：“10000000, 183.19, Q”。

“data”是一个组合对象，由三个对象组成。这三个对象有不同类型：整型、浮点型、字符。

这类对象称为：`tuple`。

tuple的声明和array类似。唯一不同是tuple用的是小括号，数组用的是方括号。

tuple的条目称为字段(field)。

tuple的类型可以显式指明：

```rust
let data: (i32, f64, char) = (10000000, 183.19, 'Q');
```

你可以用点符号(dot-notation)访问tuple的field的值。

类似地，tuple可以是mutable的：

```rust
let mut data = (10000000, 183.19, 'Q');
data.0 = -5;
data.2 = 'x';
print!("{}, {}, {}", data.0, data.1, data.2);
```

输出为：“-5, 183.19, x”。

类似于数组，tuple可以有任意个字段(field)。如果一个field都没有，这个tuple的类型是“()”。

前面已经介绍过“empty tuples”。下面解析下：

一个不同的地方是，tuples不同通过变量下标进行访问：

```rust
let array = [12, 13, 14];
let tuple = (12, 13, 14);
let i = 0;
print!("{}", array[i]);
print!("{}", tuple.i);
```

最后一行是不合法的，因为不能在运行时确定tuple的field的值。即使用const来标识也不行，因为tuple.i表述的是它的field，语法上认为`<expr>.<integer>`，解析器会认为取其field，实际上可看做是一个方法调用，主要是为了语义上的一致性。

> 为什么Rust这样定义：
>实际上，可以参考[源码](https://github.com/AndrewScull/rust/blob/1cc8b6ec664f30b43f75551e95299d943c8a4e6a/src/libcore/tuple.rs)得知，tuple在宏中，满足`<expr>.<integer>`，后面的数字被看做identifier处理了。
>可以参考[rfc](https://github.com/rust-lang/rfcs/blob/master/text/0184-tuple-accessors.md)这样设计的原因。


## The Structs

Tuple在条目较少时比较好用。当字段过多了，会带来一些障碍：

```rust
let data = (10, 'x', 12, 183.19, 'Q', false, -9);
print!("{}", data.2 + data.6);
```

上述代码，并不能一眼看出输出结果是3。另外，任何tuple字段是按顺序定义的，当类型足够多时，会变得难于阅读：

```rust
let data1 = (10, 'x', 12, 183.19, 'Q', false, -9);
let mut data2: (u16, char, i16, f64, bool, char, i16);
data2 = data1;
```

另外，对于tuple来说，添加或删除一个filed，data.2就可能偏移到data.3。这样处理也很麻烦。

因此，Rust给出了一个有具体类型的结构，相当于C语言的构造体：

```rust
struct SomeData {
	integer: i32,
	fractional: f32,
	character: char,
	five_bytes: [u8; 5],
}
let data = SomeData {
	integer: 10_000_000,
	fractional: 183.19,
	character: 'Q',
	five_bytes: [9, 0, 250, 60, 200],
};

print!("{}, {}, {}, {}",
	data.five_bytes[3], data.integer,
	data.fractional, data.character);
```

结果打印：60， 10000000， 183.19， Q。

Rust中的结构体，是有`struct`关键字声明的一个block，它声明了一个“SomeData”类型。任何该类型的对象，都有且仅有固定的字段，以及每个字段的类型也是固定的。

要声明一个该类型的变量“data”，以及初始化这个类型对象。直接用“name”带上对应字段即可。












































本章覆盖有：

- 字符串如何存储字符，为什么不能直接访问
- 如何用iterators读取字符串字符或字符串字节
- 如何使用可变iterators修改slices，arrays，vectors
- 如何使用适配器：`filter`，`map`和`enumerate`
- 如何使用消费者：`any`，`all`，`count`，`sum`，`min`，`max`和`collect`
- 惰性处理的概念

## String Characters

前面介绍了Rust有静态字符串和动态字符串，它们的编码都是UTF-8。这种编码使用1到6个字节序列表示每个Unicode字符，因此一个字符串不是简单的字符数组，而是表示字符序列的字节的数组。

既然是一个字符串，那么表达式`s[0]`表示的是什么？它是第一个字符(character)，还是第一个字节(byte)？

都不是，Rust中不允许这种字符串表达式。要获取第一个字节，必须将字符串转换为一个字节切片(a slice of bytes)。

```rust
let s = "abc012è€";
for i in 0..s.len() {
	println!("{}: {}", i, s.as_bytes()[i]);
}
```

结果将打印：

```
0: 97
1: 98
2: 99
3: 48
4: 49
5: 50
6: 195
7: 168
8: 226
9: 130
10: 172
```

再次重温，这里的变量`s`的类型是`&str`的，表示静态字符串。它是Rust中的一种特殊引用，由`指针: 长度`键值对组成，指向字符串缓冲区。

函数`as_bytes`将字符串转换为一个不可变的`u8`切片。这种转换在运行期0消耗，因为字符缓冲区里面的东西不是别的，就是字节序列(sequence of bytes)的。

UTF-8可以表述任何ASCII字符，上述代码中字符`a`，`b`，`c`，`0`，`1`，`2`，打印的值就是ASCII码。

字符`è`由一对字节表示，包含值195和168。字符`€`由三个字节表示，包含值226，130和172。

## Scanning a String

要对字符串进行处理，需要对其进行扫描。

类似于字符串“€èe”，我们想要打印第三个字符。首先，我们需要扫描第一个字符的三个字节，因为字符“€”由三个字节序列表示；接着扫描第二个字符“è”，它由两字节序列表示；接着扫描第三个字节，字符“e”仅由一个字节序列表示。

在计算机科学里，有“迭代”(有时叫“游标”)的概念，它解压处理一个序列的当前位置，并在当前位置向前递进。这种操作可以用于字符串的扫描。因此，我们需要一个 __字符串迭代器(string iterator)__。

```rust
fn print_nth_char(s: &str, mut n: u32) {
    let mut iter: std::str::Chars = s.chars();
    loop {
        let item: Option<char> = iter.next();
        match item {
            Some(c) => if n == 1 { print!("{}", c); },
            None => { break; },
        }
        n -= 1;
    }
}
print_nth_char("€èe", 3);
```

该函数的作用是，给定一个字符串`s`，数字`n`，如果有对应位置，则打印`s`中位置`n`的字符，否则不作任何处理。

Rust标准库中提供了字符串迭代类型“Chars”。给定一个字符串“s”，通过`s.chars()`获得字符串迭代器。

任何迭代器有`next`函数，该函数范围基础序列当前位置的下一个条目，并向前推进。然而，大部分序列有终点。所以，迭代器返回的下一个值，需要存在有这个位置。考虑到这个原因，Rust迭代器的`next`返回的是一个`Option<T>`类型，没有则是`None`。

使用`match`语句，用`Some`触发处理下一个字符，`None`来退出无尽的循环。

给定一个字符串，打印它的字符编码：

```rust
fn print_codes(s: &str) {
	let mut iter = s.chars();
	loop {
		match iter.next() {
			Some(c) => { println!("{}: {}", c, c as u32); },
			None => { break; },
		}
	}
}
print_codes("€èe");
```

结果输出：

```
€: 8364
è: 232
e: 101
```

## Using Iterators in `for` Loops

上面的写法有点累赘，因此，应该在语句上进行彻底的简化：

```rust
fn print_codes(s: &str) {
	for c in s.chars() {
		println!("{}: {}", c, c as u32);
	}
}
print_codes("€èe");
```

`for`循环后面跟着的`in`关键后的表达式可以是一个迭代器。

那么迭代器究竟是什么？它不是一个类型，而是一个类型规范。迭代器可以认为是包含`next`方法，返回`Option<T>`值的任何表达式。

之前，我们在for循环中用过`range`。这样，所有有上限值得range都是迭代器了，因为它们有`next`函数。

```rust
// std::ops::Range<u32> 是一个迭代器
let _v1 = (0u32..10).next();

// std::ops::RangeFrom<u32> 是一个迭代器
let _v2 = (5u32..).next();

// 不合法的：std::ops::RangeTo<u32> 不是一个迭代器
// let _v3 = (..8u32).next();

// 不合法的：std::ops::RangeFull 不是一个迭代器
// let _v4 = (..).next();
```

除了字符之外，也可以对字符串的对应的字节进行迭代：

```rust
for byte in "€èe".bytes() {
	print!("{} ", byte);
}
```

结果打印为：“`226 130 172 195 168 101`”。前面三个数表示的是`€`字符；紧接着的两个表示的是`è`字符；最后一个数表示的是`e`对应ASCII码。

该段程序可以拆分为：

```rust
let string: &str = "€èe";
let string_it: std::str::Bytes = string.bytes();
for byte in string_it {
	print!("{} ", byte);
}
```


































































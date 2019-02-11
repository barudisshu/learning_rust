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
其中，前一个`chars`函数的返回值类型是`std::str::Chars`，这个的`bytes`函数，返回的值类型是`std::str::Bytes`。

`Chars`和`Bytes`都是字符串的迭代类型，以及`Chars`的`next`函数返回的是字符串的下一个字符，而对于`Bytes`的`next`函数返回的是字符串的下一个字节。

这类函数和`as_bytes`函数不同，它们返回的是字符串对应其字节上的引用切片。

典型地，对于slice、array或vector也一样。字符串、slice、arrays、vectors都不是迭代器。但，就如字符串包含一个迭代器`chars`函数一样，slice、array、vector同样包含一个迭代函数`iter`。

```rust
for item_ref in (&[11u8, 22, 33]).iter() {
	// *item_ref += 1;
	print!("{} ", *item_ref);
}
for item_ref in [44, 55, 66].iter() {
	// *item_ref += 1;
	print!("{} ", *item_ref);
}
for item_ref in vec!['a', 'b', 'c'].iter() {
	// *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
	print!("{} ", *item_ref);
}
```

结果将打印出：“11 22 33 44 55 66 a b c”。

该程序可以改为，

```rust
let slice: &[u8] = &[11u8, 22, 33];
let slice_it: std::slice::Iter<u8> = slice.iter();
for item_ref in slice_it {
	// *item_ref += 1;
	print!("{} ", *item_ref);
}
let arr: [i32; 3] = [44, 55, 66];
let arr_it: std::slice::Iter<i32> = arr.iter();
for item_ref in arr_it {
	// *item_ref += 1;
	print!("{} ", *item_ref);
}
let vec: Vec<char> = vec!['a', 'b', 'c'];
let vec_it: std::slice::Iter<char> = vec.iter();
for item_ref in vec_it {
	// *item_ref = if *item_ref == 'b' { 'B' } else { '-' };
	print!("{} ", *item_ref);
}
```

`iter`函数，作用于类型`T`的切片的每个元素，或作用于类型`T`的数组的每个元素，或作用于类型`T`的向量的每个元素。返回值类型为`std::slice::Iter<T>`。正如其名，它的返回值类型是一个迭代器类型，因此它可以用于`for`循环中。

当在类型为`T`的范围上迭代时，loop变量的类型是`T`；当在字符迭代器上迭代时，loop内变量的类型是`char`；相反，当在类型为`T`的序列上迭代时，它的循环变量是`&T`类型，即它的引用。

因此，要访问一个序列的上迭代的变量，需要使用反引用符号(`*`)。

上面代码有三处注释掉的语句，因为是不合法的。事实上，循环体内的变量是不可变的。这种不可变性，是基于`slice`、`arr`和`vec`的不可变声明的变量定义。

前面看到，`byte`类型的字符串迭代器的创建是使用了`bytes`函数，

```rust
for byte in "€èe".bytes() {
	print!("{} ", byte);
}
```

有另外一种对字符串的字节迭代的方法，首先是创建字符串字节的切片引用，即使用`as_bytes`函数，然后再对其切片引用迭代，

```rust
for byte in "€èe".as_bytes().iter() {
	print!("{} ", byte);
}
```

## Iterations Without Mutation

迭代器的典型用法通常仅用来读取序列。

当要对字符串上的字符进行迭代时，尝试改变它是荒诞的，因为这些字符是由确切存在的不同字节数表示。假设，字符`è`由两个字节表示，那么`e`必须由一个字节表示。这在Rust标准库，没法通过字符迭代器改变一个字符的做法(比如`e`)，来改变另一个字符(比如`è`)。

当对字符串的字节进行迭代时，要对其进行变更是不安全的，因为新创建的字节可能不是一个有效的UTF-8字符。因此，Rust标准库中也没有办法，通过迭代更改字节的方式，来更改字符串。

当通过`chars()`迭代器函数对字符串的Range进行迭代时，循环体内的变量值，会使用Range类型的最先初始化时候的值，尽管在循环体内发生了更改，

```rust
let mut r = "abc".chars();
for i in r {
	r = "XY".chars();
	print!("{} {}; ", i, r.next().unwrap());
}
```

将会打印输出：“a x; b X; c X;”。虽然`r`的的值在循环内部进行了resign，但循环体仍然使用的时初始化时候的值。

在任何迭代中循环变量被初始化，

```rust
let r = 0..5;
for mut i in r {
	i += 10;
	print!("{} ", i);
}
```

这里会打印：“10 11 12 13 14”，因为`i`在循环体内使用了`mut`从句定义，但`i`在下一次迭代仍然会被重新初始化，`r`的值依然不变。

因此，对于字符串和Range来说，不能通过迭代器的方式来更改它内部序列的条目。


## Iterations with Mutation

但有时候，确实会有这样的需求，要求更改序列的内部条目。前面我看到迭代器不能处理这样的需求，即使是一个可变的迭代器也不能。

实际上，一个可变迭代器，可以或可能通过另一个序列的迭代进行创建，而不是用来改变这个序列。

一个可变迭代器的可能用法是，

```rust
let slice1 = &[3, 4, 5];
let slice2 = &[7, 8];
let mut iterator = slice1.iter();
for item_ref in iterator {
	print!("[{}] ", *item_ref);
}
iterator = slice2.iter();
for item_ref in iterator {
	print!("({}) ", *item_ref);
}
```

变量`iterator`首先引用参考了序列`slice1`，然后是`slice2`。

一个迭代器类似于一个引用，这里一个可变引用(mutable reference)不等同于一个可变对象(a reference to a mutable object)的引用这个概念。

但如果你想通过一个迭代器来变更这样一个序列，你不能使用常规的迭代器(可变的mutable或不可变的immutable)，即使是，

```rust
let mut slice = &mut [3, 4, 5];
{
	let mut iterator = slice.iter();
	for mut item_ref in iterator {
		*item_ref += 1;
	}
}
print!("{:?}", slice);
```

尽管这段程序有好几处用到了`mut`从句，它在循环语句内产生了一个编译错误，因为`*item_ref`仍然是不可变的。

所以，你需要另外一种迭代器类型，可变迭代器(__`mutating iterator`__ )，它必须由一个可变序列进行初始化，

```rust
let slice = &mut [3, 4, 5];
{
	let iterator = slice.iter_mut();
	for item_ref in iterator {
		*item_ref += 1;
	}
}
print!("{:?}", slice);
```

结果将打印：“`[4, 5, 6]`”。

除了删掉一些不必要的`mut`从句，与上一段代码侧重于相比，此处将`iter`调用替换为了`iter_mut`。顾名思义，函数表述为，“get an iterator to read it”，“get an iterator to mutate it”。

你还可以显式指定迭代器的类型，

```rust
let slice = &mut [3, 4, 5];
{
	let iterator: std::slice::IterMut<i32> = slice.iter_mut();
	for item_ref in iterator {
		*item_ref += 1;
	}
}
print!("{:?}", slice);
```

其中，`iter`返回一个`Iter<T>`的值类型，`iter_mut`返回一个`IterMut<T>`的值类型。

























































本章覆盖有：

- 如何使用closed ranges和open-ended ranges
- 如何用slice处理array或vector的portions

## The Ranges

for loop的一种写法：

```rust
for i in 0..12 { println!("{}", i); }
```

其实还有另一种可能的写法：

```rust
let dozen = 0..12;
for i in dozen { println!("{}", i); }
```

这是显示了`0..12`从句不是`for`语句的语法，而是一个表达式，而且它的值可以指派给一个变量。以及这个值可以用于`for`语句。这中值类型称为“range”。

下面是使用range的更多相关代码：

```rust
let range: std::ops::Range<usize> = 3..8;
println!("{:?}, {}, {}, {}",
	range, range.start, range.end, range.len());
for i in range { print!("{}, ", i); }
```

打印输出：

```
3..8, 3, 8, 5
3, 4, 5, 6, 7,
```

从第一行得知，任何range都是一个`Range<T>`泛型类型的具象化，其中`T`必须是一个整形类型以表示range的极限。

第二条语句输出变量range的相关信息，第一个是它自身的debug值3..8；`start`和`end`为range的字段内容，为3和8。它表明了`Range`类型对象包含有两个字段。实际上，除此之外再没其它东西了。

然后`len`函数被调用，它是`end - start`的简单求值，即 `8 - 3 = 5`。

最后，range被用于for loop，用于浏览从`start`到`end`的值。这里的迭代值个数和`len`函数给的相同。

`Range<T>`类型的参数化类型`T`，可以有两个参数进行推断：

```rust
let r1 = 3u8..12u8;
let r2 = 3u8..12;
let r3 = 3..12u8;
let r4 = 3..12;
let r5 = -3..12;
let r6 = 3..12 as i64;
print!(
	"{} {} {} {} {} {}",
	std::mem:;size_of_val(&r1),
	std::mem::size_of_val(&r2),
	std::mem::size_of_val(&r3),
	std::mem::size_of_val(&r4),
	std::mem::size_of_val(&r5),
	std::mem::size_of_val(&r6));
```

打印：“2 2 2 8 8 16”。

变量`r1`的两个极值(extrames)声明为`u8`类型，因此它有确定的类型，`u8`占一个字节，整个range占两个字节。

变量`r2`和`r3`仅其中一个声明为`u8`，另外一个留待不指定。因此它强制为`u8`类型。

变量`r4`和`r5`都不指定，因此泛型参数`T`由默认值`i32`表示。

变量`r6`其中一个极值显式表述为`i64`，另一个未指定，所以`T`必须是`i64`。

注意下面所有语句都是不合法的：

```rust
let r1 = 3u8..12i8;
let r2: std::ops::Range<u32> = -3..12;
let r3: std::ops::Range<i32> = 3i16..12;
```

第一条语句两个极值类型不同。第二条语句，-3不是`u32`类型的，最后一个语句，`3i16`不是`i32`类型的。

下面的语句可能允许，但可能出错，会出现编译告警：

```rust
let _r1 = 3u8..1200;
let _r2 = 3..5_000_000_000;
```

两者都超出了整形的长度限制，其中第一条语句类型是`Range<u8>`，第二条是`Range<i32>`。

下面语句被允许并且不带告警。即使他们可能无意义：

```rust
let _r1 = false .. true;
let _r2 = "hello" .. "world";
let _r3 = 4.2 .. 7.9;
```

实际上，这种荒诞的range不可以用于`for`循环中。


## Passing a Sequence to a Function

让我们假设你需要创建一个函数，获取一个8个记录的数组参数，并返回数组中最小的值。

```rust
fn min(arr: [i32; 8]) -> i32 {
	let mut minimum = arr[0];
	for i in 1..arr.len() {
		if arr[i] < minimum { minimum = arr[i]; }
	}
	minimum
}
print!("{}", min([23, 17, 12, 16, 15, 28, 17, 30]));
```

程序会正确地打印12。但，这个`min`函数有某些缺陷：

- 它拿的是整个数组的拷贝，需要大量的时间转换，并在栈空间和堆空间缓存了大量空间。
- 它不能处理数组的部分请求。
- 它仅能接收一个仅8个数的数组。如果传递了7或9个记录的数组，或得到一个编译错误。
- 它不能传递一个vector作为参数。

为了克服第一个缺陷，你可以传递数组的引用，由值传递(by value)变为引用传递(by reference)，使用下面代码：

```rust
fn min(arr: &[i32; 8]) -> i32 {
	let mut minimum = arr[0];
	for i in 1..arr.len() {
		if arr[i] < minimum { minimum = arr[i]; }
	}
	minimum
}
print!("{}", min(&[23, 17, 12, 16, 15, 28, 17, 30]));
```

这里不用更改函数体内容，仅添加两处`&`，一处在入参声明部分，即函数签名；另一个处是方法调用。函数体中的`arr`引用会隐式反引用处理。

为了克服第二个缺陷，你可以添加参数指定从哪个条目开始，另一个参数指定需要处理多少个：

```rust
fn min(arr: &[i32; 8], start: usize, count: usize) -> i32 {
	// Let's assume 'start' is between 0 and 7,
	// and 'count' is between 1 and 8 - start.
	let mu minimum = arr[start];
	for i in start + 1..start + count {
		if arr[i] < minimum { minimum = arr[i]; }
	}
	minimum
}
print!("{}", min(&[23, 17, 12, 16, 15, 28, 17, 30], 3, 2));
```

结果将打印输出15。实际上，它指定了处理两个条目，从位置3开始。即从`[16，15]`处理。

然而，还遗留两个缺陷。

考虑到我们的函数仅需要知道内存的开始地址，要处理多少个条目，序列条目的类型。因此不要求知道这个序列是不是大序列的一部分，更不想知道更大的序列在哪里开始和结束。

另外，考虑到任何vector将它的数据存储在栈分配的数组，因此只要知道要处理的条目在哪里，这个函数也可以处理。


## The Slices
































































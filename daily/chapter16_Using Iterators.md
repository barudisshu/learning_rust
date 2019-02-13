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

回到上个小节关于变更slice、array、vector元素的值的问题，下面是其同样的实现，

```rust
for item_ref in (&mut [11u8, 22, 33]).iter_mut() {
	*item_ref += 1;
	print!("{} ", *item_ref);
}
for item_ref in [44, 55, 66].iter_mut() {
	*item_ref += 1;
	print!("{} ", *item_ref);
}
for item_ref in vec!['a', 'b', 'c'].iter_mut() {
	*item_ref = if *item_ref == 'b' { 'B' } else { '-' };
	print!("{} ", *item_ref);
}
```

将会打印：“12 13 34 45 56 67 - B -”。

该段程序可以拆分为下面的代码，

```rust
let slice: &mut [u8] = &mut [11u8, 22, 33];
let slice_it: std::slice::IterMut<u8> = slice.iter_mut();
for item_ref in slice_it {
	*item_ref += 1;
	print!("{} ", *item_ref);
}
let mut arr: [i32; 3] = [44, 55, 66];
let arr_it: std::slice::IterMut<i32> = arr.iter_mut();
for item_ref in arr_it {
	*item_ref += 1;
	print!("{} ", *item_ref);
}
let mut vec: Vec<char> = vec!['a', 'b', 'c'];
let vec_it: std::slice::IterMut<char> = vec.iter_mut();
for item_ref in vec_it {
	*item_ref = if *item_ref == 'b' { 'B' } else { '-' };
	print!("{} ", *item_ref);
}
```

这里仅是将原来注释部分还原，其中，

- `slice`变量是一个 __mutable__ 字节slice reference
- `arr`和`vec`变量是 __mutable__ 
- 三处的`iter`函数调用，替换为了`iter_mut`
- `iter_mut`返回一个`IterMut`泛型值类型
- 循环中由变量`item_ref`引用的元素发生了变更

下面引用一段程序来证明原生数据的变更已经生效，

```rust
let slice = &mut [11u8, 22, 33];
for item_ref in slice.iter_mut() {
	*item_ref += 1;
}
print!("{:?}", slice);

let mut arr = [44, 55, 66];
for item_ref in arr.item_mut() {
	*item_ref += 1;
}
print!("{:?}", arr);
let mut vec = vec!['a', 'b', 'c'];
for item_ref in vec.iter_mut() {
	*item_ref = if *item_ref == 'b' { 'B' } else { '-' };
}
print!("{:?} ", vec);
```

将会打印：“`[12, 23, 34] [45, 56, 67] ['-', 'B', '-']`”。

截止目前为止，接触了四个迭代类型的函数：`chars`、`bytes`、`iter`、`iter_mut`。这种不获取迭代器，而是返回迭代器的函数，称为“迭代器生成器( __iterator generators__ )”。

## An Iterator Adapter: filter

让我们看看迭代器的一些其它用法。

例如，给一个数字数组，如何将所有的负数打印？

一个可能的方法是：

```rust
let arr = [66, -8, 43, 19, 0, -31];
for n in arr.iter() {
	if *n < 0 { print!("{} ", n); }
}
```

结果输出：“-8 -31”。

但另一种可能的方式是，

```rust
let arr = [66, -8, 43, 19, 0, -31];
for n in arr.iter().filter(|x| **x < 0) {
	print!("{} ", n);
}
```

`filter`函数定义在Rust的标准库中。它作用于一个迭代器，并接收一个闭包作为参数。正如其名，它用于“过滤”被迭代的序列，并丢弃不满足闭包定义条件的元素，剩下满足条件的元素。

闭包在每次迭代的元素时调用，例如，这里每次迭代的元素为`x`，

`filter`要求闭包参数的返回值类型必须是Boolean。

实际上，`filter`函数返回一个迭代器(`next`函数被调用时)，它由闭包返回`ture`的情况下产生，

我们注意到，我们仅关心是否为负数，即闭包内为`x < 0`即可，但为什么这里会有两个星号( * )？

首先，一个星号是明确的。因为我们已经说过，`iter`函数会产生序列元素的引用，而不是元素自身。

另外，`filter`函数中，它接收迭代器中的一个元素，并将该元素的引用传递给闭包，所以需要另一个星号( * )。因此`x`变成了引用的引用，要添加两个星号才能获取其表示的值。

我们说过`filter`函数返回另外一个迭代器。所以我们可以在`for`循环中使用，并做迭代实现。

这样一来，`filter`通过一个迭代器，返回另一个迭代器，它担当了一个“转换器transforms”的角色。这种“transformers”通常称为“迭代适配器(iterator adapters)”。术语“adapter”由电连接器得名：如果一个插头不能适合插座，使用一个适配嫁接。


## The map Iterator Adapter

`map`函数相当于一个映射，即`x:T -> y:T`，它被定义在标准库中，`map`函数不会删除元素，而是将里面的元素进行了转换，区别于`filter`函数，`filter`元素传递给闭包参数的值是一个引用，`map`传递的是一个值。

```rust
let arr = [66, -8, 43, 19, 0, -31];
for n in arr.iter().map({x} *x * 2) {
	print!("{} ", n);
}
```

## The enumerate Iterator Adapter

要对一个序列进行迭代，最传统做法是，

```rust
let arr = ['a', 'b', 'c'];
for i in 0..arr.len() {
	print!("{} {}, ", i, arr[i]);
}
```

使用迭代器，可以避免出现计数，

```rust
let arrlet arr = ['a', 'b', 'c'];
for ch in arr.iter() {
	print!("{}, ", ch);
}
```

但如果你想在新学习基础上对其进行计数，你可以，

```rust
let arr = ['a', 'b', 'c'];
for (i, ch) in arr.iter().enumerate() {
	print!("{} {}, ", i, *ch);
}
```

在第二行，循环变量实际上是一个tuple，第一次迭代时，`i`的值是0，`ch`是第一个字符数组的地址，每次迭代时，`i`和`ch`都会递增。

`enumerate`函数接收一个迭代器，并返回另外一个迭代器。该返回的迭代器，在每次迭代时都返回一个类型为`(usize, &char)`的tuple，第一个字段是一个计数器，第二个字段是原来迭代器元素的一份拷贝。

## An Iterator Consumer: `any`

给定一个字符串，如何判断是否包含某个字符？

```rust
let s = "Hello world!";
let ch = 'R';
let mut contains = false;
for c in s.chars() {
	if c == ch {
		contains = true;
	}
}
print!("\"{}\" {} '{}'.", s,
	if contains {
		"contains"
	} else {
		"does not contain"
	},
	ch
);
```

结果将打印："Hello world!" does not contain 'R'.

这里进行了字符大小写的比较。如果换为`R`，则会输出第二种情况，你可以替换为闭包实现，

```rust
let s = "Hello, world!";
let ch = 'R';
print!("\"{}\" {} '{}'.",
	s,
	if s.chars().any(|c| c == ch) {
		"contains"
	} else {
		"does not contain"
	},
	ch);
```

闭包的好处是替换掉原来使用自定义变量的做法；这个变量替换为了表达式`s.chars().any(|c| c == ch)`的实现。

函数`any()`定义在Rust标准库中，它作用在迭代器上。它的目的是确定迭代器中的任意元素是否满足布尔函数(又名“预设”predicate)。

`any()`函数必须是作用在一个迭代器上的，以及必须接收一个闭包参数。闭包内的操作，对迭代内的每个元素进行处理，当某个处理到某个元素返回`true`时，结果立即返回`true`，否则所以元素的处理结果为`false`时，函数返回`false`。

因此，函数名`any()`，顾名思义就是“任意的”满足条件。

前面的一个判断是否包含负数的例子，可以用`any()`函数来处理，

```rust
#[macro_use]
extern crate t_bang;

use t_bang::*;

fn main() {
    let arr = [45, 8, 2, 6];
    let mut arr_iter = arr.iter();
    let arr_any = arr_iter.any(|n| *n < 0);
    print!("{} ", t!(arr_iter));
    print!("{} ", arr_any);
}
```

为了代码清晰，你可以给闭包的类型加上注解，

```rust
print!("{} ", [45, 8, 2, 6].iter().any(|n: &i32| -> bool { *n < 0 }));
print!("{} ", [45, 8, -2, 6].iter().any(|n: &i32| -> bool { *n < 0 }));
```

前面说过，迭代器的迭代变量是一个reference，所以这里不能省略`&`符号，否则有类型错误。

前面介绍了几种迭代概念，

- 函数作用在非迭代器，生成一个迭代器的，称为“迭代生成器(iterator generator)”
- 函数作用在一个迭代器，返回另一个迭代器的，称为“迭代适配器(iterator adapter)”
- 函数作用在一个迭代器，但不返回迭代器的，称为“迭代消费者(iterator consumer)”

迭代器 “ __消费者__ ”，就是消费掉了数据，而不是“ __适配__ ”数据。除了这里的`any()`消费者，下面再介绍几种常见的。

## The `all` Iterator Consumer

`any()`就是“任意的”，至少有一个满足条件；对应就有`all()`，“所有的”，要求全部满足条件；

```rust
print!("{} ", [45, 8, 2, 6].iter().all(|n: &i32| -> bool { *n < 0 }));
print!("{} ", [45, 8, -2, 6].iter().all(|n: &i32| -> bool { *n < 0 }));
```

## The `count` Iterator Consumer

迭代计数器`count()`和`enumerate()`概念类似，只不过`count()`是个消费者，不会生成另一个迭代器，内部元素不会发生拷贝。

例如，你想统计一个slice，array，vector的长度，你可能会使用`len`函数。但要想知道一个字符串里面有多少个字符，你就必须扫描这个字符串，因为组成字符串的字符不会保存，除非你将它存储下来。

```rust
let s = "€èe";
print!("{} {}", s.chars().count(), s.len());
```

`count()`不接收任何参数，以及它的返回值类型总是`usize`。

## The `sum` Iterator Consumer

`sum()`函数用于迭代添加，它也是个迭代消费者，

```rust
print!("{}", [45, 8, -2, 6].iter().sum::<i32>());
```

这里可以指定它的类型参数`<i32>`，这个类型参数是可选的，可选的前提条件是：迭代器的元素类型需要是可加的，这样才能被类型推断处理；例如`[3.4].iter().sum::<f64>()`是合法的，但`[true].iter().sum::<bool>()`是不合法的，因为布尔值无法满足加法。

## The `min` and `max` Iterator Consumers

`min()`和`max()`函数用于查找最小值、最大值，它的返回类型是`Option`，其中`Some`值的作用在非空序列，`None`时则表示序列是空的。

```rust
let arr = [45, 8, -2, 6];
match arr.iter().min() {
	Some(n) => print!("{} ", n),
	_ => (),
}
match arr.iter().max() {
	Some(n) => print!("{} ", n),
	_ => (),
}
match [0; 0].iter().min() {
	Some(n) => print!("{} ", n),
	_ => print!("---"),
}
```

将会打印： -2 45 ---.

`min()`和`max()`也可作用在非数字的迭代对象上，但要满足可比较性(即该类型要有`std::cmp`)，

```rust
let arr = ["hello", "brave", "new", "world"];
match arr.iter().min() {
	Some(n) => print!("{} ", n),
	_ => (),
}
match arr.iter().max() {
	Some(n) => print!("{] ", n),
	_ => (),
}
```

## The `collect` Consumer

像`any()`、`all()`、`count()`、`sum()`、`min()`和`max()`这些迭代消费者返回都是简单一个值，但如果我们想将所有处理的元素收集到一个Vector呢，

```rust
let arr = [36, 1, 15, 9, 4];
let v = arr.iter().collect::<Vec<&i32>>();
print!("{:?}", v);
```

结果将打印："`[36, 1, 15, 9, 4]`".

这里的类型参数是必须的，不过可以改为这样写，

```rust
let arr = [36, 1, 15, 9, 4];
let v = arr.iter().collect::<Vec<_>>();
print!("{:?}", v);
```

又或者这样，

```rust
let arr = [36, 1, 15, 9, 4];
let v: Vec<_> = arr.iter().collect();
print!("{:?}", v);
```

同样，字符和字节也可以收集到一个Vector，

```rust
let s = "Hello";
println!("{:?}", s.chars().collect::<String>());
println!("{:?}", s.chars().collect::<Vec<char>>());
println!("{:?}", s.bytes().collect::<Vec<u8>>());
println!("{:?}", s.as_bytes().iter().collect::<Vec<&u8>>());
```

将会打印，

```
"Hello"
['H', 'e', 'l', 'l', 'o']
[72, 101, 108, 108, 111]
[72, 101, 108, 108, 111]
```

注意`collect()`函数不能用于静态字符串、静态数组、或静态切片，因为它要求运行期内存分配。


## Iterator Chains

假设你想要将一个数组里面的负数，平方后收集到另一个vector中，

```rust
let arr = [66, -8, 43, 19, 0, -31];
let mut v = vec![];
for i in 0..arr.len() {
	if arr[i] > 0 { v.push(arr[i] * 2); }
}
print!("{:?}", v);
```

打印输出：`[132, 86, 38]`.

一个等效实现是，

```rust
let arr = [66, -8, 43, 19, 0, -31];
let mut v = vec![];
for n in arr.iter() {
	if *n > 0 { v.push(*n * 2); }
}
print!("{:?}", v);
```

又或者，

```rust
let arr = [66, -8, 43, 19, 0, -31];
let mut v = vec![];
for n in arr
	.iter()
	.filter(|x| **x > 0)
	.map(|x| *x * 2)
{
	v.push(n);
}
print!("{:?}", v);
```

再或，

```rust
let arr = [66, -8, 43, 19, 0, -31];
let mut v = vec![];
for n in arr
	.iter()
	.filter(|x| **x > 0)
	.map(|x| *x * 2)
	.collect::<Vec<_>>();
print!("{:?}", v);
```

最后一个版本展示的编程模式是函数式语言的典型：迭代链(iterator chain)。

迭代链由几个概念属于组成，**迭代生成器(iterator generator)** + **迭代适配器(iterator adapter)** + **迭代消费者(iterator consumer)** 。

## Iterators Are "Lazy"

我们给最后一个例子加一些调试信息打印出来，

```rust
let v = [66, -8, 43, 19, 0, -31]
	.iter()
	.filter(|x| { print!("F{} ", x); **x > 0 })
	.map(|x| { print!("M{} ", x); *x * 2 })
	.collect::<Vec<_>>();
print!("{:?}", v);
```

将打印：`F66 M66 F-8 F43 M43 F19 M19 F0 F-31 [132, 86, 38]`。


运行时的操作如下，

首先调用了`iter()`准备一个迭代器，但它没有访问该数组。我们给该迭代器命名为“I”。

`filter()`的调用准备了一个迭代器，但它没有处理数据。我们给迭代器命名为“F”。

`map()`的调用准备了一个迭代器，但它没有处理数据。我们给该迭代器命名为“M”。

`collect()`的调用，向“M”请求一个元素；“M”向“F”请求一个元素；“F”向“I”请求一个元素；“I”拿到来自数组的数66，传给“F”，打印这个数，检查是否是正数，然后传递给“M”，打印，平方，再传递给`collect()`，最后推送到Vector。

接着，`collect()`接收到`Some`，继续向“M”请求另一个元素，重复这个操作，知道元素-8到达“F”时，由于是负数被过滤。所有“M”没有打印出结果。停留在“F”，因为“F”接收到`Some`，继续问“I”索取另一个元素。

该算法处理，会直到数组完成。当“I”不能在数组找到其它元素时，将一个`None`发给“F”告知它没有更多元素了。“F”收到`None`后，再将其传递给“M”，“M”再发给`collect()`，整个语句结束。

如果省略掉迭代器消费者，

```rust
[66, -8, 43, 19, 0, -31]
	.iter()
	.filter(|x| { print!("F{} ", x); **x > 0 })
	.map(|x| { print!("M{} ", x); *x * 2 });
```

它不会输出任何消息，因为它什么也没有做，编译器只会报告，

```
warning unused `std::iter::Map` witch must be used: iterator adapters are lazy and do nothing unless consumed.
```

“lazy”在计算机科学中是“惰性求值”的概念，以为这尽可能迟的处理。迭代适配器是惰性的，它仅在其它函数向其请求元素时才真正调用：即需要消费者。

如果没有数据接收器，就没有数据访问(If there is no data sink, there is no data access)。

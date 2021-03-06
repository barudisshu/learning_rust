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

考虑所有这些，为了克服所有指出的错误，“切片(slice)”的概念被引入到该语言中。它的语法参考：

```rust
fn min(arr: &[i32]) -> i32 {
	// Let's assume 'arr' is not empty.
	let mut minimum = arr[0];
	for i in 1..arr.len() {
		if arr[i] < minimum { minimum = arr[i]; }
	}
	minimum
}
print!("{}", min(&[23, 17, 12, 16, 15, 28, 17, 30]));
```

和上一小节的不同是，“`; 8`”消失了。现在`arr`参数看起来是一个数组引用，并且没有指定数组的大小。

这种类型是一个`切片引用(a reference to a slice, or slice reference)`。它的泛型形式是“`&[T]`”，`T`表示包含在数组中的任何类型。这里的“slice”表示的序列条目的子序列(sub-sequence)，就像一个数组或一个向量缓冲区。基于这个目的，一个切片引用的实现是一对值：序列的第一个条目的地址，以及条目的个数。

注意通常我们有变量类型是“切片引用(slice reference)”很少会“切”。一个slice会有类型“`[T]`”，但这种类型不能作为参数传递给一个函数，因为它的大小在编译时没有定义，函数参数的一个需求是它们在编译期定义大小。因此，我们仅能给一个函数传递`切片引用(references to slices)`，而不是`slices(切片)`。这种对象是一个指针和长度的对，因此它们占的内存为常规引用对象的两倍。

切片引用的用法和一个数组用法十分类似。主要实现的不同是，数组上的`len`函数的调用，可以通过替换为数组长度的常量进行优；而对于切片引用上的`len`函数，通过访问该对象第二个字段实现。

实际上，前一个章节我们看到跟slices和slice references可以类比：字符串缓冲区，静态字符串。

我们可以建一个相似性表格：

|undefined-length sequence of bytes	|	(address of beginning, length in bytes)	|	(address of beginning, length in bytes, number of bytes used)|
|-----------------------------------|-------------------------------------------|----------------------------------------------------------------|
|    String buffer: `str`           |    Static string: `&str`                  |    Dynamic string: `String`                                    |
|    Slice of bytes: `[ u8 ]`       |    Reference to slice of bytes: `&[u8]`   |    Vector or bytes: `Vec<u8>`                                  |


第一列是未定义长度的类型。`字符缓冲区(string buffers)`，类型是`str`，是由UTF-8字符的序列推断的未定义长度字节序列。`切片(slices)`是无符号8位数，它的类型是`[u8]`，是未定义长度的字节序列。

第二列是第一列的类型引用。`静态字符串(static strings)`，类型是`&str`，由两个字段构造：字符缓冲区的内存首地址，以及缓冲区字节的长度。`切片引用(references to slices)`是无符号8位数，类型是`&[u8]`，由两个字段构成：无符号8位数的切片的内存首地址，以及切片的长度。

第三列是动态分配的堆分配对象。其中`动态字符串(dynamic strings)`，它的类型是`String`，有三个字段构造而成：堆空间分配的字符缓冲区的内存首地址，缓冲区的字节长度，以及被用于缓冲区的字节数。对于无符号8位数的vector，类型是`Vec<u8>`，也是由三个字段构造而成：对空间分配的无符号8位数的一个切片的内存首地址，切片的长度，以及切片当前使用的字节数。

回到最后一个示例代码，注意`min`函数的调用没有发生改变。仍然将数组的引用作为参数传递。实际上，这个数组引用会隐式地转换为一个切片引用，使用数组的地址作为切片地址，数组的长度作为切片长度。

因此，程序最后语句传递给函数一个两个字段的结构：首先是包含数字23的数组元素的内存地址，其次是数字8.

使用切片，便利性增加了。因此，现在可以这样写：

```rust
fn min(arr: &[i32]) -> i32 {
	// Let's assume 'arr' is not empty.
	let mut minimum = arr[0];
	for i in 1..arr.len() {
		if arr[i] < minimum { minimum = arr[i]; }
	}
	minimum
}
print!("{} ", min(&[23, 17]));
print!("{}", min(&vec![55, 22, 33, 44]));
```

打印：“17 22”。

第一次调用仅传递两个参数，17是它们中最小的。因此，`min`函数不再局限于8个元素的数组，它可以处理任何长度不为0的数组和切片。

第二次调用展示了`min`也可以处理vector。传递给该函数的值是一个vector的引用，因为函数的参数类型是“reference to slice”，参数变成了一个切片的引用表示整个vector内容。

因此，我们已经克服了前面提到过得所有缺陷。


## Slicing

有了切片的便利，渴望一个新的可能的用法。

我们说有一个数组或一个向量，例如`vector[23, 17, 12, 16, 15, 2]`，以及一个函数以切片(slice)作为参数，例如上面看到的`min`函数，我们想用该函数处理仅数组或函数的一小段。例如，我们想在数组的第三、第四和第五元素中查找最小值。

我们需要做的是伪造一个切片表示一个数组或向量的片段，不需要整个数组和向量。

为了获得一个数组`arr`或向量`v`下标2的条目，分别可以写`arr[2]`或`v[2]`。为了获得2到5之间的所有元素，可以写`arr[2..5]`或`v[2..5]`。下面是另一种用法：

```rust
fn min(arr: &[i32]) -> i32 {
	// Let's assume 'arr' is not empty.
	let mu minimum = arr[0];
	for i in 1..arr.len() {
		if arr[i] < minimum { minimum = arr[i]; }
	}
	minimum
}
let arr = [23, 17, 12, 16, 15, 2];
let range = 2..5;
let slice_ref = &arr[range];
print!("{}", min(slice_ref));
```

打印“12”，最后4行可以合并：

```rust
fn min(arr: &[i32]) -> i32 {
	// Let's assume 'arr' is not empty.
	let mu minimum = arr[0];
	for i in 1..arr.len() {
		if arr[i] < minimum { minimum = arr[i]; }
	}
	minimum
}
print!("{} ", min(&[23, 17, 12, 16, 15, 2][2..5]));
```

这种从一个数组或一个向量获取切片(slice)的过程，称为“切分(slicing)”。

注意，和`for`循环一样，slicing不包含上限值。所以，范围`2..5`的记录包含的位置是2,3,4。从0开始计数。

切片操作可以作用在数组和向量中，也可以作用在其它切片：

```rust
let arr = [55, 22, 33, 44, 66, 7, 8];
let v = vec![55, 22, 33, 44, 66, 7,9];
let sr1 = &arr[2..5];
let sr2 = &v[2..5];
print!("{:?} {:?} {:?} {:?}", sr1, sr2, &arr1[1..2], &sr1[1]);
```

结果将输出“`[33, 44, 66] [33, 44, 66] [44] 44`”。

其中`sr1`变量是一个切片引用，它引用的是数组`arr`的第三、第四和第五个元素。

`sr2`是一个类似地切片引用，但它引用的是向量`v`的条目。


## Out-of-range Slicing

除了常规的切分(slicing)，甚至可以做某些怪异的事情：

```rust
let arr = [55, 22, 33, 44, 66];
let _r1 = 4..4; let _a1 = &arr[_r1];
let _r2 = 4..3;	//let _a2 = &arr[_r2];
let _r3 = -3i32..2;	//let _a3 = &arr[_r3];
let _r4 = 3..8;	//let _a4 = &arr[_r4];
```

该程序中，除了第一行，其它行都是以第一行作为切分数组方式声明一个range。

所有range都是合法的，但不是所有切分操作合法，所以注释掉了某些语句。

第二行是非常正确的，从位置4开始，再从位置4结束。它是一个空slice，空slice是允许的。

第三行用了一个“倒退”的切片。在编译器中它是允许的，但在运行期会导致panic，就和超范围数组访问一样。运行期出现的错误信息，会打印在控制台，`slice index that starts at 4 but ends at 3`。

第四行使用了类型限制为`i32`的一个range。它会导致编译错误，因为对于切分(slicing)操作，和序列索引一样，必须是`usize`类型。

第五行使用的range超出了`arr`的范围。它编译会通过，但在运行期会带有一个panic信息“`index 8 out of range for slice of length 5`”。

注意到，在这里，不论是array、vector还是slice，情况都一样。

## Mutable Slicing

切片是另一个序列的一段(a portion)，更改切片内容意味着更改原来序列相应的记录。

```rust
let mut arr = [11, 22, 33, 44];
{
	let sl_ref = &mut arr[1..3];
	print!("{:?}", sl_ref);
	sl_ref[1] = 0;
	print!(" {:?}", sl_ref);
}
print!(" {:?}", arr);
```

这会打印“`[22, 33] [22, 0] [11, 22, 0, 44]`”。


可变变量`sl_ref`指向一个可变切片(mutable slice)。因此，引用不变，但切片会被改变，意味着这里可以更改数组的记录。

为了获得一个可变切片的引用，基础序列(underlying sequence)必须是可变的。所以要求第一行带`mut`从句。

更改切片引用意味着什么？切片引用是引用的一种，更改该引用意味着序列片段引用的更改，这里的序列(sequence)可能是切片片段，也可能是基础序列(数组或向量)片段。

```rust
let arr = [11, 22, 33, 44];
{
	let mut sl_ref = &arr[1..3];
	print!("{:?}", sl_ref);
	sl_ref = &arr[0..1];
	print!(" {:?}", sl_ref);
}
print!("" {:?}", arr);
```

这里将打印：“`[22, 33] [11] [11, 22, 33, 44]`”。

这段程序中，变量`arr`是一个不可变数组，所以它不会更改。变量`sl_ref`是指向一个可变切片的可变引用。


## Open-Ended Ranges and Slicing

有时希望从给定的`n`开始获取一个序列的所有条目，或从`n`到最后的条目，可以这样：

```rust
let arr = [11, 22, 33, 44];
let n = 2;
let sr1 = &arr[0..n];
let sr2 = &arr[n..arr.len()];
print!("{:?} {:?}", sr1, sr2);
```

结果打印输出：“`[11, 22] [33, 44]`”。

但有更简单的写法：

```rust
let arr = [11, 22, 33, 44];
let n = 2;
let sr1 = &arr[..n];
let sr2 = &arr[n..];
print!("{:?} {:?}", sr1, sr2);
```

第三行没有上限，第四行没有下限。实际上，这些Range是不同类型：

```rust
let r1: std::ops::RangeFrom<i32> = 3..;
let r2: std::ops::RangeTo<i32> = ..12;
println("{:?} {:?} {} {}", r1, r2, std::mem::size_of_val(&r1), std::mem::size_of_val(&r2));
```

结果将打印：“`3.. ..12 4 4`”。变量`r1`的类型是`RangeFrom`，有下限没有上限。变量`r2`的类型是`RangeTo`，有上限没有下限。都占4个字节，因为它们仅需要存储`i32`类型的对象。

`RangeTo`仅用于开口切片(open-ended slicing)，而`RangeFrom`也可能用于特定的循环中。

```rust
for i in 3.. {
	if i * i > 40 { break; }
	print!("{} ", i);
}
```

结果输出：“`3 4 5 6`”。

除了上面介绍的两种，还有一种泛型类型的range：

```rust
let range: std::ops::RangeFull = ..;
let a1 = [11, 22, 33, 44];
let a2 = &a1[range];
print!("{} {:?} {:?}", std::mem::size_of_val(&range), a1, a2);
```

结果打印：“`0 [11, 22, 33, 44] [11, 22, 33, 44]`”。

因为`RangeFull`没有存储东西，所以它的大小是0。它被用于基础序列范围特别大的情形。

下面总结一下容易混淆的概念，

>`[T; n]` is an array of length `n`, represented as `n` adjacent `T` instances.
>`&[T; n]` is purely a reference to that array, represented as a thin pointer to the data.
>`[T]` is a slice, an unsized type; it can only be used through some form of indirection.
>`&[T]`, called a slice, is a sized type. It's a fat pointer, represented as a pointer to the first item and the length of the slice.

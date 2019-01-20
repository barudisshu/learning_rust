本章覆盖有：

- 静态字符串是如何实现的
- 动态字符串是如何实现的
- 如何从动态字符串添加或删除字符
- 如何在静态字符串和动态字符串之间相互转换
- 如何合并字符串

## Static Strings

我们用到的字符串是可变的(changeable)吗？

某种层面上，她们是可变的(mutable)，我们可以改变它们：

```rust
let mut a = "Hel";
print!("{}", a);
a = "lo";
print!("{}", a);
```

我们这里的改变，是更改了整个字符串的内容，不是某些字符。实际上，这里将字符串变量，指派给了一个新的字面量或字符串变量。

但如果我们想要创建一个字符串，是由算法、文件读取、或由用户输入(type)的，这怎么实现？可以确切说，这些都可以做到，并且会改变字符串的内容，它们有一个不可变的`content`，这些内容不能对其中一两个字符进行重写。基于这个原因，这些字符内容(content)称作**静态字符串(static strings)**。下面例子理清一下：

```rust
use std::mem::*;
let a: &str = "";
let b: &str = "0123456789";
let c: &str = "abcdè";
print!("{} {} {}",
	size_of_val(a),
	size_of_val(b),
	size_of_val(c));
```

结果将打印“0 10 6”。

首先，我们指定了三个变量的类型。它们的类型是`&str`，即“`str`的引用”。

`str`这个词定义在标准库中，作为一个不可修改的字节数组，表示UTF-8字符串。编译器每次解析到字面量字符串时，它存储在一个字符串的字符静态程序区域，这个区域是`str`类型的。编译器使用一个引用(reference)，来表示字面量字符串表达式在该区域的值，因此任何字符串字面量的类型都是`&str`。

在例子中，泛型函数`size_of_val`入参三个字符串变量调用。还记得该函数返回引用对象的大小。如果参数是`a`，参数类型是`&str`，该函数会返回由`a`引用的字符串缓冲区的大小，即返回类型`str`的大小。

因此，这里打印出引用字符串缓冲区`a`、`b`和`c`的大小。分别大小由`0`、`10`和`6`字节。第一个字符串为空，第二个包含10个数，与此，第三个仅包含5个字符，却打印了6.这是因为UTF-8标注的原因。这种标注，取决于字符本身，每个字符由一个或多个字节表示。ASCII字符由1个字节表示，对于“grave e”字符，即“è”，由两个字节表示。因此，整个字符串大小是6个字节。

注意由`a`、`b`和`c`变量引用的缓冲区的类型相同，都是`str`，但它们有不同长度：0，10，6。这里我们第一次看到了一个不与长度关联的类型。

这种类型不常见，但它们有某些限制。一是你不能给这种类型声明一个变量或一个函数参数。另一个明显的限制是，你不能访问这种类型的大小。

```rust
let a: str;
fn f(a: str) {}
print!("{}", std::mem::size_of::<str>());
```

上面三个语句都是不合法的。

但，要怎样才能获得缓冲区的大小？在C语言，字符串终止符被作为字符串结束标志，但Rust中没有字符串终止符。

实际上`&str`不是一个普通的Rust引用，它仅包含一个指针，但它是一对指针和长度(a pair of a pointer and a length)。指针的值是字符串缓冲区的开始地址，长度值是字符串缓冲区的字节数量。

让我们更深入探索一下这种奇怪的类型。

```rust
use std::mem::*;
let a: &str = "";
let b: &str = "0123456789"
let c: &str = "abcdè";
print!("{} {} {}; ",
	size_of_val(&a),
	size_of_val(&b),
	size_of_val(&c));
print!("{} {} {}; ",
	size_of_val(&&a),
	size_of_val(&&b),
	size_of_val(&&c));
```

该程序在一个64位系统将打印“16 16 16; 8 8 8”，在32位系统打印“8 8 8; 4 4 4”。

第一条`print`语句打印变量自身的大小，即类型`&str`。该变量得到的结果，是常规引用大小的2倍，因为它们包含一个指针对象和一个`usize`对象。所以，当我们在一个静态字符串调用`len`函数，得到的是pair的第二个值。

第二条`print`语句打印变量自身引用的大小，即类型`&&str`。它们是常规引用。

## Dynamic Strings

因此如果我们想要在运行期创建或更改字符串的内容(contents)，前面用到的`&str`类型显然不适合。

Rust同时也提供了另外一种字符串类型，`动态字符串(dynamic strings)`，它的内容可以被改变：

```rust
let mut a: String = "He".to_string();
a.push('l');
a.push('l');
a.push('o');
print!("{}", a);
```

结果将输出“Hello”。

变量`a`是一个`String`类型，它是Rust静态字符串的类型。

在Rust中没有动态字符串字面量；字符串字面量总是静态的。但一个动态字符串可能由一个静态字符串的几种方式构造得来。一种方式是在静态字符串调用`to_string`函数。这种函数名应该考虑是`to_dynamic_string`或`to_String`。但第一个名字太长，第二个违反了函数名字母大写的规则。

一个动态字符串应该能像任何静态字符串一样打印输出，如上面的例子。以及它有静态字符串做不到的能力：增长。

第二、三、四语句中向字符串尾部添加一个字符。

以及可以在一个动态字符串内部的其它位置添加、或者删除字符：

```rust
let mut a: String = "Xy".to_string(); // "Xy"
a.remove(0); // "y"
a.insert(0, 'H'); // "Hy"
a.pop(); // "H"
a.push('i'); // "Hi"
print!("{}", a);
```

结果将打印“Hi”。

变量`a`由`Xy`初始化。然后在位置0的字符被移除，剩下`y`。然后`H`插入到位置0，变成了`Hy`。然后最后一个字符pop out，剩下`H`。接着添加`i`，得到`Hi`。

## Implementation of String

Rust的静态字符串和C语言的字符串有几分类似，带有一个额外的计数器，以及一个Rust动态字符串和C++ `std::string`对象十分相像。Rust和C++动态字符串类型的主要不同是，C++字符串包含一个字符数组，而Rust动态字符串，和Rust静态字符串一样，包含的是一个由UTF-8字符串表述的字节数组；它不是包含字符数组的。

Rust语言中保留了其它一些相似的特性。静态字符串缓冲区类似于数组，即`str`类型类似于泛型类型`[u8; N]`；动态字符串类似于字节向量，即`String`类型类似于`Vec<u8>`类型。

进一步，上面我们看到的函数——`push`，`pop`，`insert`以及`remove`，还有`len`函数，在`Vector`泛型类型都有对应的同名函数。

另外，动态字符串和向量拥有相同的实现。两者都又下面三部分构成：

- 堆空间分配缓冲区的首地址包含数据条目；
- 条目的数量可能包含在分配的缓冲区；
- 条目的数量可能会在分配的缓冲区提供使用。

然而，值得注意的是，字符串的“条目”是字节，不是字符：

```rust
let mut s1 = "".to_string();
s1.push('e');
let mut s2 = "".to_string();
s2.push('è');
let mut s3 = "".to_string();
s3.push('€');
print!("{} {}; ", s1.capacity(), s1.len());
print!("{} {}; ", s2.capacity(), s2.len());
print!("{} {}", s3.capacity(), s3.len());
```

这里可能打印：“4 1; 2 2; 3 3”。意味着在一个4字节的缓冲区ASCII字符`e`占一个字节，在一个两字节缓冲区重音字符`è`占两个字节，在一个3字节缓冲区货币符号`€`占三个字节。字节数由UTF-8标准导致，而缓冲区大小则由Rust标准库的实现决定，它可能会在将来的版本改进。

让我们看看当向一个动态字符串添加若干字符时发生了啥：

```rust
let mut s = "".to_string();
for _ in 0..10 {
	println!("{:?} {} {}",
		s.as_ptr(), s.capacity(), s.len());
	s.push('a');
}
println!("{:?} {} {}: {}",
	s.as_ptr(), s.capacity(), s.len(), s);
```

在64位系统中，可能输出：

```
0x1 0 0
0x7fbf95e20020 4 1
0x7fbf95e20020 4 2
0x7fbf95e20020 4 3
0x7fbf95e20020 4 4
0x7fbf95e20020 8 5
0x7fbf95e20020 8 6
0x7fbf95e20020 8 7
0x7fbf95e20020 8 8
0x7fbf95e2a000 16 9
0x7fbf95e2a000 16 10: aaaaaaaaaa
```

函数`as_ptr`(可以读作“as pointer”)返回堆空间分配的字符串缓冲区地址。

注意到当字符串是空的，该地址简化为`1`，它不是一个有效的内存地址，因为没有给一个空字符串指派任何缓冲。

当一个ASCII字符被添加，一个4字节缓冲区被分配在一个由十六进制7fbf95e20020表述的地址上。

添加另外3个字符后，没有再分配的发生，因为缓冲区已经足够大了。

当第五个字符被添加，要求重新分配，但，由于内存紧接着的缓冲区仍然为空闲，缓冲区可以扩展8个字节简化实现。因此为了避免在新缓冲区分配上的开销，拷贝4个已用的字节，回收前面的缓冲区。

再说一遍，添加另外3个字符，不要求再分配，当第九个字符被添加，不仅缓冲区扩展到16个字节，而且它必须重新定位地址，因为，接下来8字节可能并不是空闲的。

最后，字符串用了10字节。

## Creating Strings

创建空的动态字符串有几种方式。

```rust
let s1 = String::new();
let s2 = String::from("");
let s3 = "".to_string();
let s4 = "".to_owned();
let s5 = format!("");
print!("({}{}{}{}{})", s1, s2, s3, s4, s5);
```

打印“()”。

`new`函数是`String`类型的基础构造器，类似于C++的“default constructor”。

`from`函数为`String`类型的转换构造器，类似于C++的“non-default constructor”。

函数`to_string`和`to_owned`现在是可以替换的用法。由于历史的原因有几分不同。

`format`宏是`print`宏的identical，唯一不同的是`print`是将结果发送到控制台，而`format`是返回一个`String`对象结果。

上述几种方式，除了`new`函数，都可以由一个非空静态字符串转换为动态字符串。

```rust
let s = "a,";
let s1 = String::from(s);
let s2 = s.to_string();
let s3 = s.to_owned();
//let s4 = format!(s);
//let s5 = format!("a,{}");
let s6 = format!("{}", s);
print!("({}{}{}{})", s1, s2, s3, s6);
```

输出“`(a,a,a,a,)`”。

相反，在第五行和第六行会生成编译错误。实际上，`format`宏，和`print`还有`println`宏一样，要求它们第一个参数是一个字面量，以及这个字面量包含与连续参数相同数目的占位符。

## Concatenating Strings

一个动态字符串也可以由两个静态字符串、两个动态字符串、或一个动态字符串一个静态字符串，组合得到。

```rust
let ss1 = "He";
let ss2 = "llo ";
let ds1 = ss1.to_string();
let ds2 = ss2.to_string();
let ds3 = format!("{}{}", ss1, ss2);
print!("{}", ds3);
let ds3 = format!("{}{}", ss1, ds2);
print!("{}", ds3);
let ds3 = format!("{}{}", ds1, ss2);
print!("{}", ds3);
let ds3 = format!("{}{}", ds1, ds2);
print!("{}", ds3);
```

输出“`Hello Hello Hello Hello`”。

通常，一个字符串的出现由另一个字符串取决，当然它必须是mutable的。这样可以使用`format`宏，但它是冗长和低效的：

```rust
let mut dyn_str = "Hello".to_string();
dyn_str = format!("{}{}", dyn_str, ", ");
dyn_str = format!("{}{}", dyn_str, "world");
dyn_str = format!("{}{}", dyn_str, "!");
print!("{}", dyn_str);
```

下面是一个比较好的写法：

```rust
let mut dyn_str = "Hello".to_string();
dyn_str.push_str(", ");
dyn_str.push_str("world");
dyn_str.push_str("!");
print!("{}", dyn_str);
```

函数`push_str`接收一个静态字符串，并把所有接收到的字符串push到后面。打印“Hello, world!”。

另外有一种更紧凑(compact)的写法：

```rust
let mut dyn_str = "Hello".to_string();
dyn_str += ", ";
dyn_str += "world";
dyn_str += "!";
print!("{}", dyn_str);
```

`+=`操作符，当作用在一个`String`对象时，等价于`push_str`函数。

也可以追加字符串对象或单个字符。

```rust
let comma = ", ".to_string();
let world = "world".to_string();
let excl_point = '!';
let mut dyn_str = "Hello".to_string();
dyn_str += &comma;
dyn_str.push_str(&world);
dyn_str.push(excl_point);
print!("{}", dyn_str);
```

该程序和上一个是等价的。注意到将动态字符串作为参数传递给`push_str`或`+=`时，必须预先转换为静态字符串。这个效果使用`&`操作符获得。实际上，可以用该操作符，获得一个`String`的引用，但任何一个`String`的引用，会隐式转换为一个`str`引用，如下：

```rust
let word = "bye".to_string();
let w1: &str = &word;
let w2: &String = &word;
print!("{} {}", w1, w2);
```

打印为：“bye bye”。
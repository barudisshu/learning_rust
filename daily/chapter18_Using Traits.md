本章覆盖有：

- `traits`如何避免在泛型函数调用中的不能理解的编译错误信息
- 泛型参数的边界如何被单子化(monolithic)，或如何分解为几个traits
- 如何创建函数体返回内的traits
- 如何简单地使用`self`关键字，通过"点，dot notation"操作来创建函数
- 如何迭代一个trait
- 如何定义类型别名
- 如何定义泛型迭代器
- 如何使用关联类型来简化泛型迭代器的使用
- 如何定义字节的迭代器

## The Need for Traits

假设我们要计算一个4次方根，该函数命名为“quartic_root”。以及调用标准库的`sqrt`函数，我们会写，

```rust
fn quartic_root(x: f64) -> f64 { x.sqrt().sqrt() }
let qr = quartic_root(100f64);
print!("{} {}", qr * qr * qr * qr, qr);
```

结果会打印：“100.00000000000003 3.1622776601683795”。

但我们还需要计算32位数的4次方根，于是又，

```rust
fn quartic_root_f64(x: f64) -> f64 { x.sqrt().sqrt() }
fn quartic_root_f32(x: f32) -> f32 { x.sqrt().sqrt() }
print!("{} {}",
    quartic_root_f64(100f64),
    quartic_root_f32(100f32));
```

根据前面我们所学知识，我们可以定义泛型函数来处理，于是，

```rust
fn quartic_root<Number>(x: Number) -> Number {
x.sqrt().sqrt()
}
print!("{} {}",
    quartic_root(100f64),
    quartic_root(100f32));
```

但这段代码是不合法的，生成编译错误，"no method named `sqrt` found for type `Number` in the current scope"。它意思是说，泛型类型`Number`没有这个`sqrt`函数。

在这方面，Rust不同于C++。C++可以通过模板来关联这个泛型函数，

```cpp
#include <iostream>
#include <cmath>

template <typename Number>
Number quartic_root(Number x) {
	return sqrt(sqrt(x));
}
int main() {
	std::count << quartic_root((float)100) << " " << quartic_root((double)100);
}
```

即使C++中的`NUmber`泛型类型没有这个可用的`sqrt`函数，编译器也不知这个表达式是否允许。但当调用`quartic_root`时，函数被计数，编译期生成两个具体函数`quartic_root<float>`和`quartic_root<double>`。这叫“泛型函数实例化 generic function instantiation”，或“function monomorphization”。这种实例化会检测具体的类型。

C++这种方案带来的缺陷很明显，即当出现程序错误时，譬如，

```cpp
#include <iostream>
#include <cmath>
template <typename Number>
Number quartic_root(Number x) {
	return sqrt(sqrt(x));
}
int main() {
	std::count << quartic_root("Hello");
```

编译器会实例化这个`const char*`类型的具体函数，它会生成`sqrt(const char*)`的方法签名。但没有这个函数声明，所以会导致出现变异错误。

这个缺陷带来的问题是，这个泛型类型`Number`，它所提供的具体类型的函数`sqrt`可能是由某一位开发者编写的，以及另一种具体类型的`sqrt`又是另外一位开发者编写的。可能两位开发者的`sqrt`函数签名并不一样！！

另外类似于C++这种`quartic_root`的实现，代码阅读是晦涩难懂的，因为它大部分变量、函数、类型都属于库实现(实际上有很多库...)，而不是接口。要理解它，不仅需要知道它的API使用；还需要知道它的库的实现。

## Traits to the Rescue

Rust中为了避免这种类似于C++的编译为题，提供了trait来澄清复杂错误消息的各种情况，因为它更贴近真实软件环境。

```rust
trait HasSquareRoot {
    fn sq_root(self) -> Self;
}

impl HasSquareRoot for f32 {
    fn sq_root(self) -> Self { f32::sqrt(self) }
}

impl HasSquareRoot for f64 {
    fn sq_root(self) -> Self { f64::sqrt(self) }
}

fn quartic_root<Number>(x: Number) -> Number
    where Number: HasSquareRoot {
    x.sq_root().sq_root()
}
print!("{} {}", quartic_root(100f64), quartic_root(100f32));
```

结果将打印：“3.1622776601683795 3.1622777”

第一个trait命名为“HasSquareRoot”，包含函数签名“sq_root”。一个Rust trait是一个函数签名的容器；它表示这个trait有能力使用某些函数。这里表示了`HasSquareRoot`trait可以在有“HasSquareRoot”的地方调用“sq_root”函数，或者更常规的说法是，任何满足“HasSquareRoot”trait的类型，都可以调用这个`sq_root`函数。

但究竟哪些类型满足“HasSquareRoot”？没有定义，因此接下来两个语句，使得`f32`类型和`f64`类型满足这个trait。换言之，这些`impl`语句，可以从给定的`f32`和`f64`类型调用这个`sq_root`。

这些`impl`反映了“HasSquareRoot”仅是一个程序接口，或API，它需要又具体的类型实现。所以当然地，`impl`语句的函数签名，需要跟原来的前一个方法签名一样。不同的是`impl`包含有函数实现。

Rust的trait类似于Java或C#接口，或没有方法体的抽象类。

现在有了具体的类型实现了。第四条语句定义了`quartic_root`泛型函数，参数化类型参数是`Number`。然而，这个声明有一个新的段：`where Number: HasSquareRoot`。这种从句叫做——“trait bound”，它是方法签名的一部分。它字面量的意思是，`Number`泛型类型必须实现`HasSquareRoot`特质。

代码调用函数是，这个`where`从句表示“当调用该函数，你必须确保你传递的参数化类型实现了`HashSquareRoot`trait”。例如这个的100f32和100f64，对应类型是f32和f64。这两种类型都有`hasSquareRoot`的实现，因此它们是合法参数。但如果替换为“quartic_root("Hello"));”，这里没有`&str`的“HasSquareRoot”的实现，因此违反了条约。以及会得到编译错误“the traitbound `&str: main::HasSquareRoot` is not satisfied”。

又或者你替换为“quartic_root(81i32));”，也会得到编译错误，因为“HasSquareRoot”没有`i32`类型的实现。

注意的时，`x`表达式在函数体内，它的类型仅可能是`Number`，实际类型并不清楚，所以你不能将`x.sq_root()`，写为`x.abs().sq_root()`这种，编译错误`abs()`在`Number`范围内没有定义。

## Generic Functions with No Trait Bounds

不带特质边界的泛型函数是很少见的，比如这段代码，

```rust
let mut a = 'A';
let mut b = 'B';
print!("{}, {}; ", a, b);
std::mem::swap(&mut a, &mut b);
print!("{}, {};, a, b);
```

泛型函数`swap`的方法签名是：`fn swap<T>(x: &mut T, y: &mut T)`。它不需要使用`where`从句进行trait bound。因为它直接交换了两个对象的地址。实际编码过程中，泛型函数，类型参数总是需要边界绑定的。Rust代码设计，总是强调类型安全这个概念，也是我们编写代码的原则。


## Scope of Traits

前面用了一个`sq_root`来区分标准库的`sqrt`函数，不过我们也可以将其命名为`sqrt`，

```rust
fn sqrt() {}
trait HasSquareRoot {
	fn sqrt(self) -> Self;
}
impl HasSquareRoot for f32 {
	fn sqrt(self) -> Self { f32::sqrt(self) }
}
impl HasSquareRoot for f64 {
	fn sqrt(self) -> Self { f64::sqrt(self) }
}
fn quartic_root<Number>(x: Number) -> Number
where Number: HasSquareRoot {
	x.sqrt().sqrt()
}
sqrt();
print!("{} {}",
	quartic_root(100f64),
	quartic_root(100f32));
```

同一个作用范围内是不允许有同名方法的。不过上面代码是合法的；因为它们并不作用在同一个scope。`fn sqrt()`是个本地函数，在HasSquareRoot外；`fn sqrt(self)`作用在HasSquareRoot内；`f32::sqrt`和`f64::sqrt`是个标准库调用。

## Traits with Numltiple Functions

前面的例子有个问题是，如果传入的是“-100f64”或“-100f32”，程序会打印“NaN，Not a Number”，我们想处理负数的情况，

```rust
trait HasSquareRoot {
    fn sq_root(self) -> Self;
}

impl HasSquareRoot for f32 {
    fn sq_root(self) -> Self { f32::sqrt(self) }
}

impl HasSquareRoot for f64 {
    fn sq_root(self) -> Self { f64::sqrt(self) }
}

trait HasAbsoluteValue {
    fn abs(self) -> Self;
}

impl HasAbsoluteValue for f32 {
    fn abs(self) -> Self { f32::abs(self) }
}

impl HasAbsoluteValue for f64 {
    fn abs(self) -> Self { f64::abs(self) }
}

fn quartic_root<Number>(x: Number) -> Number
    where Number: HasSquareRoot + HasAbsoluteValue {
    x.abs().sq_root().sq_root()
}
```

多种类型，可以组合不同trait，使用`+`。

## Methods

目前我们接触到的函数的调用方式有两种，一种是`f(x, y)`，另一种是`x.f(y)`。例如之前例子的`String::new()`、`String::form("")`写法，和`"abcd".to_string()`，`"abcd".len()`。一种是点操作，一种是函数调用操作。

任何函数都可以使用者两种调用方式，

```rust
print!("{},", "abcd".to_string());
print!("{},",[1,2,3].len());
let mut v1 = vec![0u8; 0];
v1.push(7u8);
print!("{:?}; ", v1);

print!("{},", std::string::ToString::to_string("abcd"));
print!("{:?},", <[i32]>::len(&[1, 2, 3]));
let mut v2 = vec![0u8; 0];
Vec::push(&mut v2, 7u8);
print!("{:?}", v2);
```

虽然可以这样做，但有scoping问题。在标准库中，有很多同名的函数`to_string`、`len`、`push`...。使用点操作，自然会选择适当的函数。但是使用函数调用，函数的范围必须显式写明。例如，`to_string`的范围在`std::string::ToString`，`len`函数的范围在`<[i32]>`，`push`的作用范围在`Vec`。

如果不写清楚，譬如这段代码，

```rust
fn double(x: i32) -> i32 {
x * 2
}
print!("{}", double(7i32));
```
```rust
fn double(x: i32) -> i32 {
x * 2
}
print!("{}", 7i32.double());
```

这里的点操作调用，会发生编译错误，它会说当前范围内，`i32`类型没有double方法。区别于方法和函数，Rust中点操作的调用，区分为方法，它仅能在有trait实现的声明的方法中调用，所以，要允许点操作，可以改为，

```rust
trait CanBeDoubled {
	fn double(self) -> Self;
}
impl CanBeDoubled for i32 {
	fn double(self) -> Self {
		self * 2
	}
}
print!("{}", 7i32.double());
```

trait的名字是任意的。通常trait仅包含一个函数，trait的名字使用Pascal-case记法。对于类型来说，像`CanBeDoubled`，从命名上看出，它表示有一个`double`函数可以获取自身`self`类型的一个值，遵循这种命名规范便于阅读理解。

当编译这段表达式时，编译器会搜索支持`i32`的`double`操作，并找到对应的方法签名。

## The "self" and "Slef" Keywords

前面一个小节，我们发现了两个关键字：“self”和“Self”。

在语句`trait CanBeDoubled { fn double(self) -> Self; }`中，`self`表示`double`方法将作用的值，`Self`表示`self`的类型。

因此，`self`是一个方法的预设参数，`Self`表示这一个参数的类型。因此，`self`和`Self`仅能被用于一个`trait`或`impl`的块内。以及，如果有方法，`self`必须是方法的第一个参数。

在`impl CanBeDoubled for i32`块内，下面6行是等价的：

```rust
fn double(self) -> Self {
fn double(self: Self) -> Self {
fn double(self: i32) -> Self {
fn double(self) -> i32 {
fn double(self: Self) -> i32 {
fn double(self: i32) -> i32 {
```

第一行和第四行给定的`self`参数带有隐式类型；只不过，`self`的类型就是`Self`，所以也可以显式指定，又因为在`impl`块内，`Self`就是`i32`，所以也可以替换为`i32`。

不过最常使用的是第一种写法，它更接近泛型编程概念。

让我们看看另一种情况，我们希望有这样一个表达式`"foobarbaz".letters_count('a')"`统计字符串中有多少个字符，

```rust
trait LettersCount {
	fn letters_count(&self, ch: char) -> usize;
}
impl LettersCount for str {
	fn letters_count(&self, ch: char) -> usize {
		let mut count = 0;
		for c in self.chars() {
			if c == ch {
				count += 1;
			}
		}
		count
	}
}
print!("{} ", "".leters_count('a'));
print!("{} ", "ddd".leters_count('a'));
print!("{} ", "ddd".leters_count('d'));
print!("{} ", "foobarbaz".leters_count('a'));
```

因为我们想用点操作，首先声明一个trait，它的名字来源于函数名。这个函数需要两个参数：字符串切片用于搜索，字符用于查找。但我们不想将字符串切片的拷贝作为参数传递；我们仅想直接传递字符串切片引用，因此我们将参数声明为`&self`，这里的`self`就是一个字符串切片，有任意长度；`&self`是一个切片引用，有一对指针的大小(字符串切片有header和content的pointer)。

返回值类型是`usize`表示非负整数。

`impl`实现了使用了命令式风格。浏览`chars()`迭代器的所有字符，出现要搜索的字符，则统计一次。

如果使用函数式风风格，可以更简短，如下，

```rust
self.chars().filter(|c| *c == ch).count()
```

## Standard Traits

在最开始的章节，我们用到了宏`print`、`println`和`format`。我们可以用`{}`占位符表示支持的类型，使用`{:?}`来进行调试。

但怎么知道某些类型支持`{}`占位符，其它类型却不支持？我自己写的类型如何实现支持这种占位符？

实际上，这些宏使用了`fmt`函数，有标准库的`std::fmt::Display`提供了trait。所有原生类型都实现了这个trait，所以你可以给自己的类型实现，

```rust
struct Complex {
	re: f64,
	im: f64,
}
impl std::fmt::Display for Compex {
	fn fmt(&self, &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{} {} {}i",
			self.re,
			if self.im >= 0. { '+' } else { '-' },
			self.im.abs()
		)
	}
}
let c1 = Complex { re: -2.3, im: 0. };
let c2 = Complex { re: -2.1, im: -5.2 };
let c3 = Complex { re: -2.2, im: 5.2 };
print!("{}, {}, {}", c1, c2, c3);
```

结果会打印：“-2.3 + 0i, -2.1 - 5.2i, -2.2 + 5.2i”。

对于原生类型的实现traits，标准库中有非常多函数提供。

## The "Terator" Trait

一个非常有趣的标准库trait是“Iterator”。让我们看看它主要解决哪方面问题。

例如编写一个函数，给定参数`range`，返回第三个元素，长度不够，则返回None。

```rust
fn get_third(r: std::ops::Range<u32>) -> Option<u32> {
	if r.len() >= 3{
		Some(r.start + 2)
	} else {
		None
	}
}
print!("{:?} {:?}", get_third(10..12), get_third(20..23));
```

将类型换成`slice`怎样，

```rust
fn get_third(s: &[f64]) -> Option<f64> {
	if s.len() >= 3 {
		Some(s[2])
	} else {
		None
	}
}
print!("{:?} {:?}", get_third(&[1.0, 2.0]), get_thrid(&[1.1, 2.1, 3.1]);
```

这两个程序非常相似。但使用的是迭代器，应该将它们写成一个泛型函数，你可能会写成

```rust
fn get_third<Iter, Item>(mut iterator: Iter) -> Option<Item> {
	iterator.next();
	iterator.next();
	iterator.next()
}
print!("{:?} {:?}", get_third(0..9), get_third([11, 22, 33, 44].iter()));
```

你会得到几个编译错误。这种想法是好的，但有几个问题，

- `iterator`变量没有边界，所以它没有`next`函数。当我们调用`get_third`函数式，我们看到参数确实是iterator，以为有`next`函数。然而，Rust需要知道泛型参数对象有哪些函数可以被调用，
- 再看`get_thrid`函数的调用，它的泛型参数`Item`不能被推断，因为没有表达式表明给这个泛型参数传递了值。

对于第一种错误，表明“迭代器”的概念没有被Rust语言定义。这个概念由Rust标准库的一个标准trait——`Iterator`定义了。我们知道迭代器都有一个`next`函数，所以任何迭代器都必须要有这个函数。

```rust
fn get_third<Iter, Item>(mut iterator: Iter) -> Option<Iterm> 
where Iter: std::iter::Iterator {
```

但仍然有第二个错误存在：怎么确定`Item`的具体类型。为了解决这个问题，需要首先介绍`type`关键字。

## The "type" Keyword

Rust中的`type`对于C语言的`typedef`关键字，它相当于一个类型的别名，

```rust
type Number = f32;
fn f1(x: Number) -> Number { x }
fn f2(x: Number) -> NUmber { x }
let a: Number = 2.3;
let b: Number = 3.4;
print!("{} {}", f1(a), f2(b));
```

使用`type`结构有两点好处：

- 简洁代码，它使用了一个有意义的名字来表示原生类型了
- 方便性，不用频繁切换类型，只需要修改type的类型即可

但`type`实际上有另一个重要用途，

## Generic Traits

前面的章节我们知道有泛型函数和泛型结构体。trait也可以由一个或多个类型参数化表示，即要求它的函数需要泛型参数的情况。这个概念和Java的接口类似，

```rust
trait Searchable<Key> {
	fn contains(&self, key: Key) -> bool;
}
fn is_present<Collection>(coll: &Collection, id: u32) -> bool
where Collection: Searchable<u32> {
	coll.contains(id)
}
```

下面是该代码的完整实现，

```rust
trait Searchable<Key> {
    fn contains(&self, key: Key) -> bool;
}
struct RecordWithId {
    id: u32,
    _descr: String,
}
struct NameSetWithId {
    data: Vec<RecordWithId>
}
impl Searchable<u32> for NameSetWithId {
    fn contains(&self, key: u32) -> bool {
        for record in self.data.iter() {
            if record.id == key {
                return true;
            }
        }
        false
    }
}
fn is_present<Collection>(coll: &Collection, id: u32) -> bool
where Collection: Searchable<u32> {
    coll.contains(id)
}

let names = NameSetWithId {
	data: vec![
	RecordWithId {
		id: 34,
		_descr: "John".to_string(),
	},
		RecordWithId {
			id: 49,
			_descr: "Jane".to_string(),
		},
	],
};
print!("{}, {}", is_present(&names, 48), is_present(&names, 49));
```

声明了`Searchable`泛型trait后，也声明了两个结构体：“RecordWithId”，表示由唯一数字标识的数据元素；“NameSetWithId”，表示一个类型为`RecordWithId`的集合。

然后，trait实现了这个集合类型。有两个方式实现：保留泛型参数，编写类似于`impl<T> Searchable<T> for NameSetWithId {`；这里是另一种实现方法，因为`contains`不仅需要指定`NameSetWithId`，还需要知道`Key`的具体类型。

定义了`is_present`函数，要是有这个函数，需要定义对应的结构体。

这个解决方案虽然生效了，但有一些缺陷。

这里，`Searchable`需要指定`Key`的类型是`u32`，另外还要指定参数化类型的值，但在`where`从句中又重复指定了一次，

考虑更复杂的情况，

```rust
trait Searchable<Key, Count> {
	fn contains(&self, key: Key) -> bool;
	fn count(&self, key: Key) -> Count;
}
struct RecordWithId {
	id: u32,
	_descr: String,
}
struct NameSetWithId {
	data: Vec<RecordWithId>,
}
impl Searchable<u32, usize> for NameSetWithId {
	fn contains(&self, key: u32) -> bool {
		for record in self.data.iter() {
			if record.id == key {
				return true;
			}
		}
		false
	}
	fn count(&self, key: u32) -> usize {
		let mut c = 0;
		for record in self.data.iter() {
			if record.id == key {
				c += 1;
			}
		}
		c
	}
}
fn is_present<Collection>(coll: &Collection, id: u32) -> bool
where Collection: Searchable<u32, usize>, {
	coll.contains(id)
}
let names = NameSetWithId {
	data: vec![
		RecordWithId {
			id: 34,
			_desrc: "John".to_string(),
		},
		RecordWithId {
			id: 49,
			_desrc: "Jane".to_string(),
	],
};
print!(
	"{}, {}; {} {}",
	names.count(48),
	names.count(49),
	is_present(&names, 48),
	is_present(&names, 49),
);
```

这里不明显地`is_present`的泛型函数签名，必须指定新的类型。但这个函数并没有使用这个类型，这个类型参数在这里没有很大意义。

## Using Associated Types to Simplify Generic Traits Use

前面以及描述了这个无实际意义的泛型参数签名的问题。一个最好的解决方案如下，

```rust
trait Searchable { //1
    type Key; //2
    type Count; //3
    fn contains(&self, key: Self::Key) -> bool;  //4
    fn count(&self, key: Self::Key) -> Self::Count; //5
}
struct RecordWithId {
    id: u32,
    _desrc: String,
}
struct NameSetWithId {
    data: Vec<RecordWithId>,
}

impl Searchable for NameSetWithId { //6
    type Key = u32; //7
    type Count = usize; //8
    fn contains(&self, key: Self::Key) -> bool {    //9
        for record in self.data.iter() {
            if record.id == key {
                return true;
            }
        }
        false
    }
    fn count(&self, key: Self::Key) -> usize {  //10
        let mut c = 0;
        for record in self.data.iter() {
            if record.id == key {
                c += 1;
            }
        }
        c
    }
}
fn is_present<Collection>(
    coll: &Collection, 
    id: <Collection as Searchable>::Key,    // 11
) -> bool
where Collection: Searchable,   //12
{
    coll.contains(id)
}
let names = NameSetWithId {
	data: vec![
		RecordWithId {
			id: 34,
			_desrc: "John".to_string(),
		},
		RecordWithId {
			id: 49,
			_desrc: "Jane".to_string(),
		},
	],
};
print!("{}, {}; {} {}",
	   names.count(48),
	   names.count(49),
	   is_present(&names, 48),
	   is_present(&names, 49));
```

首先是，“Searchable”特质不再使用泛型，而是将泛型定义在自身内部，

>这点写法和Scala的trait真的非常非常像...

因此，每次使用“Key”和“Count”类型参数时，都需要带前缀“Self::”。

这些改变的好处体现在`is_present`方法签名上。首先如其用具体的类型，这里用一个关联类型`Key`指定，这个不需要在指定`Searchable`，因为它没有泛型。

这里将类型的定义绑定在一个`trait`上，这种实现机制，对于大型软件开发更有优势。

## the "Iterator" Standard Trait Declaration

关于“`Iterator`”这个标准trait，我们说过它仅包含一个item：`next`函数签名。这样说不对的。应该说是一个泛型的item。

因为它由`type`元素签名，你可以认为它在标准库中的定义是这样的，

```rust
trait Iterator {
	type Item;
	fn next(&mut self) -> Option<Self::Item>;
}
```

这种定义强制要求具体的迭代实现要为`Item`类型定义，以及实现`next`的方法体，

下面是一种可能的实现range的方式，

```rust
trait MyIterator {
	type Item;
	fn next(&mut self) -> Option<Self::Item>;
}
struct MyRangeIterator<T> {
	current: T,
	limit: T,
}
impl MyIterator for MyRangeIterator<u32> {
	type Item = u32;
	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.limit {
			None
		} else {
			self.current += 1;
			Some(self.current - 1)
		}
	}
}
let mut range_it = MyRangeIterator {
	current: 10,
	limit: 13,
};
print!("{:?}, ", range_it.next());
print!("{:?}, ", range_it.next());
print!("{:?}, ", range_it.next());
print!("{:?}, ", range_it.next());
print!("{:?}, ", range_it.next());
print!("{:?}, ", range_it.next());
```

这里使用了`MyRangeIterator<u32>`，指定了它的具体类型，但实际上，我们不需要定义`MyIterator`特质，因为我们可以直接使用标准库的`Iterator`。

```rust
struct MyRangeIterator<T> {
	current: T,
	limit: T,
}
impl Iterator for MyRangeIterator<u32> {
	type Item = u32;
	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.limit {
			None
		} else {
			self.current += 1;
			Some(self.current - 1)
		}
	}
}
print!("{:?}; ", 
	MyRangeIterator {
			current: 10,
			limit: 13,
	}.collect::<Vec<_>>()
);
for i in (MyRangeIterator {
	current: 20,
	limit: 24,
}) {
	print!("{} ", i);
}
```

因为“MyRangeIterator”对象有实现“Iterator”特质的类型，所以它可以使用`collect`迭代器消费者。

## Using Generic Iterators

现在，回到原先那个`Item`无用的问题。我们想要实现泛型函数`get_third`，它接收任何迭代器，返回迭代器第三个元素，问题可以由下面代码解决，

```rust
fn get_third<Iter>(mut iterator: Iter) -> Option<Iter::Item>
where
	Iter: std::iter::Iterator,
{
	iterator.next();
	iterator.next();
	iterator.next()
}
print!(
	"{:?} {:?} {:?} {:?}",
	get_third(10..12),
	get_third(20..29),
	get_third([31, 32].iter()),
	get_third([41, 42, 43, 44].iter())
);
```

这里直接用`where`来绑定的`Item`的范围，这样一来，就可以访问`Item`关联的类型了，包括它的方法`next`。以及它的返回类型就是`Option<Iter::Item>`。

这段代码，对于迭代参数类型的函数编写提供了参考。实际上，标准库中已经定义了类型的迭代器消费者`nth`。所以下面写法是等价的，

```rust
print!(
	"{:?} {:?} {:?} {:?}",
	(10..12).nth(2),
	(20..29).nth(2),
	([31, 32].iter()).nth(2),
	([41, 42, 43, 44].iter()).nth(2)
);
```


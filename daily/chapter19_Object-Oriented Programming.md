本章覆盖有：

- 不使用trait，继承方式实现关联类型的函数
- Rust面向对象和C++面向对象的区别
- 那些trait可以实现哪些type，哪些不能
- 如何指定一个方法更改对象
- 构造对象的一些约定方法
- 为什么Rust不适用数据继承
- 什么是静态派遣，什么是动态派遣，如何实现，如何使用

## Inherent Implementations

前面张杰，我们看到如何解决下面的问题。你有一个结构体`Stru`，用于两个方面：用作名称空间，里面有一个函数`f1`，用作表达式`Stru::f1(500_000)`的调用；用作创建实例，实例名为`s`，这个实例可以调用`f2`方法，例如`s.f2(456).x`。

```rust
trait Tr {
    fn f1(a: u32) -> bool;
    fn f2(&self, b: u16) -> Self;
}
struct Stru {
    x: u16,
    y: u16,
}
impl Tr for Stru {
    fn f1(a: u32) -> bool {
        a == 0
    }
    fn f2(&self, b: u16) -> Self {
        if b == self.x || b == self.y {
            Stru {
                x: self.x + 1,
                y: self.y + 1,
            }
        } else {
            Stru {
                x: self.x - 1,
                y: self.y - 1,
            }
        }
    }
}
let s = Stru { x: 23, y: 456 };
print!("{} {}", Stru::f1(500_000), s.f2(456).x);
```

结果将打印：`false 24`。

首先，`Tr`被声明，带有两个方法签名，`f1`和`f2`，`Stru`结构体被声明。然后，trait`Tr`有该结构体的实现。最后实例化结构体变量，并调用这两个方法。

这种模式很常见，下面有一种简化的写法，

```rust
struct Stru {
    x: u16,
    y: u16,
}
impl Stru {
    fn f1(a: u32) -> bool {
        a == 0
    }
    fn f2(&self, b: u16) -> Self {
        if b == self.x || b == self.y {
            Stru {
                x: self.x + 1,
                y: self.y + 1,
            }
        } else {
            Stru {
                x: self.x - 1,
                y: self.y - 1,
            }
        }
    }
}
let s = Stru { x: 23, y: 456 };
print!("{} {}", Stru::f1(500_000), s.f2(456).x);
```

这段代码将trait部分移除了，但需要推断trait的名字；因此，对于`impl`语句,它直接作用于类型，所以不需要trait。这种类型有继承实现。

从面向对象的角度，它表示：我们有一个自定义类型，`Stru`，带有某些数据成员，x和y；以及某些方法，`f1`和`f2`。

C++的类似实现如下，

```cpp
#include <iostream>
int main() {
    struct Stru {
        unsigned short x;
        unsigned short y;
        static bool f1(unsigned long a) {
            return a == 0;
        }
        Stru f2(unsigned short b) const {
            return b == x || b == y ?
                Stru {
                    (unsigned short)(x + 1),
                    (unsigned short)(y + 1)
                }
            :
                Stru {
                    (unsigned short)(x - 1),
                    (unsigned short)(y - 1)
                }
            ;
        }
    };
    Stru s = { 23, 456 };
    std::cout << std::boolalpha << Stru::f1(500000) << " " << s.f2(456).x;
}
```

Rust方法中，参数以`self`开头的称为“对象方法object methods”；不以`self`开头的称为“类方法class methods”。

在一个对象方法内，`self`关键表示当前对象，类似其它面向对象语言的`self`或`this`。

要调用带有`self`参数的方法，使用点操作，如`s.f2(456)`；调用不带`self`参数的方法，使用函数调用方式，语法像`Stru::f1(500_000)`，类型名后带两个冒号，其后跟着方法名。

Rust和C++字段访问方式的不同在于，Rust中必须写`self.x`，但C++或其它语言对应可能是`this -> x`，甚至可以不写，例如Java中的字段`this.x`和`x`都一样。

Rust和其它面向对象语言的不同在于，大部分面向对象语言对当前对象的参考(`this`、`self`或`Me`)总是一个指针(pointer)或一个引用(reference)。在Rust中，方法前面中的`&self`是个引用`reference`，`self`则是当前对象的一个拷贝。

## Peculiarities of Rust Object-Orientation

Rust和其它面向对象语言还有几点不同的地方：

```rust
S::f2();
impl S { fn f1() { print!("1"); } }
impl S { }
S::f1();
impl S { fn f2() { print!("2"); } fn _f3() {} }
struct S {}
```

`impl`的实现只要在同一个范围，可以不用关心它的位置和顺序。结构体和函数也可以在调用之后再定义。不过为了便于阅读，通常会先声明，后面再使用。

在同一个范围内，只允许有一个`struct S`语句；而对于`impl S`语句可以有多个。

```rust
struct S1 {}
struct S2 {}
impl S1 {
    fn f() {}
    //ILLEGAL: fn f(a: i32) {}
}
impl S2 {
    fn f() {}
}
S1::f();
S2::f();
```

在Rust中，同一个范围不允许有同名函数。一个类型表示一个范围。因此，对于`S1`类型，不能有两个`f`的方法，即使它有不同的参数。

```rust
enum Continent {
    Africa,
    America,
    Asia,
    Europe,
    Oceania,
}
impl Contient {
    fn name(&self) -> &str {
        match *self {
            Continent::Afria => "Africa",
            Continent::America => "America",
            Continent::Asia => "Asia",
            Continent::Europe => "Europe",
            Continent::Oceania => "Oceania",
        }
    }
}
print!("{}", Continent::Asia.name());
```

在Rust中，不仅可以在结构体添加方法，其它任何定义的类型都可以，诸如枚举和元组-结构体。

但原生数据类型不能直接添加方法。

```rust
impl i32 {}
```

这段代码尝试给`i32`原生类型添加方法，即使方法体是空的，编译器会报错：“only a single inherent implementation marked with `#[lang = "i32"]` is allowed for the `i32` primitive”。意思是说，`i32`原生类型仅能有一处实现，也就是仅能在语言自身和标准库中提供。

另外也不能直接对标准库或其它库中的非原生类型添加方法，

```rust
impl Vec<i32> {}
```

对于这段代码，编译器会报错：“cannot define inherent `impl` for a type outside of the crate where the type is defined.”。这里的“crate”指一个程序或一个库。因为`Vec`泛型类型被定义在标准库，这段错误信息告诉你，`Vec`不能在标准库的外部继承实现。

对于trait，也不能在标准库或其它库的外部实现，

```rust
impl std::iter::Iterator for i32 {}
```

这段代码，编译器会报错：“only traits defined in the current crate can be implemented for arbitrary types”。意思是说，“Iterator”并没有被声明在你的代码范围内，“i32”没有声明在代码中，trait不能对该类型实现。

所以，反过来说，定义在可见范围的任何类型、任何trait都可以实现，

```rust
trait Tr {
    fn f1();
    fn f2(&self);
}
impl Tr for bool {
    fn f1() { print!("Tr::f1 "); }
    fn f2(&self) { print!("Tr::f2 "); }
}
bool::f1();
true.f2();
```

结果打印：“Tr::f1 Tr::f2”。

任何类型都可以通过trait实现代码，

```rust
struct Pair(u32, u32);
impl std::iter::Iterator for Pair {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
let mut a = Pair(23u32, 34u32);
print!("{:?}", a.next());
```

将会打印：“None”。

首先是“Pair”已经定义，以及“Iterator”定义在标准库中，被用来实现“Pair”类型。两者都在作用域可见并且不冲突。

总结！

- 如果`Ty`是一个类型，允许有`impl Ty`，要求`Ty`被声明在当前crate。
- 如果`Tr`是一个trait，允许有`impl Tr for Ty`，要求`Tr`或`Ty`被声明在当前crate，不能两者都是标准库或其它库的一部分。

|                 | `Tr`在当前rate         | `Tr`在其它crate               |
|-----------------|------------------------|-------------------------------|
| `Ty`在当前crate | `impl Tr for Ty` 允许  | `impl Tr for Ty` 允许         |
| `Ty`在其它crate | `impl Tr for Ty` 允许  | `impl Tr for Ty` 不合法       |

## Mutating Methods

Rust中，任何不带`mut`关键字的，都是immutable的。这对于虚拟参数(pseudo-argument)`self`也一样。如果想通过方法，更改其作用的对象，需要带上关键字`mut`。

```rust
struct S { x: u32 }
impl S {
    fn get_x(&self) -> u32 { self.x }
    fn set_x(&mut self, x: u32) { self.x = x; }
}
let mut s = S { x: 12 };
print!("{} ", s.get_x());
s.set_x(17);
print!("{} ", s.get_x());
```

C++的等价实现如下，

```cpp
#include <iostream>
int main() {
	struct S {
		unsigned long x;
		unsigned long get_x() const { return x; }
		void set_x(unsigned long x) { this -> x = x; }
	};
	S s = { 12 };
	std::cout << s.get_x() << " ";
	s.set_x(17);
	std::cout << s.get_x() << " ";
}
```

## Constructors

每次用到构造对象时，我们都需要指定它的所有字段。

为了独立于方法实现的方式处理对象，某些语言会提供“构造器”这个特性。Rust中也提供了一个或多个方法，不需要接收`self`参数，但有`Self`返回的实现。这类方法就是构造器，Rust中没有构造器的明确语法写法，但有一些惯例(convention)。

```rust
struct Number {
	x: f64,
}
impl Number {
	fn new() -> Number { Number { x: 0. } }
	fn from(x: f64) -> Number { Number { x: x } }
	fn value(&self) -> f64 { self.x }
}
let a = Number::new();
let b = Number::from(2.3);
print!("{} {}", a.value(), b.value());
```

`new`和`from`方法是构造器。按照惯例，不带参数的构造器命名为`new`，带有一个参数的构造器命名为`from`。然而，通常有很多构造器仅带一个参数的；这些构造器中仅有一个命名为`from`。

这种惯例在标准库中有实例，

```rust
let a = String::new();
let b = String::from("abcd");
print!("({}) ({});", a, b);
let c = Vec::<i32>::new();
let d = Vec::<u8>::from("abcd");
print!(" {:?} {:?}", c, d);
```

## Composition Instead of Inheritance

面向对象中有三种继承：数据继承、方法实现继承、方法接口继承。Rust中不支持数据继承，因为它会带来很多问题，Rust中使用组合方式来替代数据继承的实现。

假设我们有一个类型，它表示将字符文本画在屏幕上，另外要创建一个类型表示这个文本的框。为了简单，用控制台打印，替代画文本，用中括号代替这个矩形。

```rust
struct Text { characters: String }
impl Text {
    fn from(text: &str) -> Text {
        Text { characters: text.to_string() }
    }
    fn draw(&self) {
        print!("{}", self.characters);
    }
}
struct BoxedText {
    text: Text,
    first: char,
    last: char,
}
impl BoxedText {
    fn with_text_and_borders(
        text: &str, first: char, last: char) -> BoxedText {
            BoxedText {
                text: Text::from(text),
                first: first,
                last: last,
            }
        }
    fn draw(&self) {
        print!("{}", self.first);
        self.text.draw();
        print!("{}", self.last);
    }
}
let greeting = Text::from("Hello");
greeting.draw();
let boxed_greeting = 
	BoxedText::with_text_and_borders("Hi", '[', ']');
	print!(", ");
	boxed_greeting.draw();
```

第二条语句定义了两个方法：`from`，它是一个构造器；`draw`，打印输出字符内容。

现在想利用已有的结构和方法，来创建一个新的结构`BoxedText`。它是继承的一种常见方法。

在Rust中，如其使用继承，你可以创建一个`BoxedText`结构体来“包含”`Text`类型对象。然后创建对应的方法封装这个类型`with_text_and_borders`。这段代码中，代码复用出现在几个地方：

- `struct BoxedText`的第一个字段是`Text`类型，它复用了数据结构，
- `BoxedText`构造器用到了`Text::from(text)`，它复用了`Text`的构造器，
- `BoxedText`的方法体`draw`内，用到了`self.text.draw();`。它复用了`Text`的方法`draw`，

## Memory Usage of Composition

组合和继承的内存使用没有区别。它们都需要内存，

```rust
struct Base1 {
	_x: f64
}
struct Base2 {}
struct Derived1 {
	_b1: Base1,
	_b2: Base2,
}
struct Derived2 {
	_d1: Derived1,
	_other: f64,
}
use std::mem:size_of;
print!("{} {} {} {}",
	size_of::<Base1>(), size_of::<Base2>(),
	size_of::<Derived1>(), size_of::<Derived2>());
```

打印输出为：“8 0 8 16”。`Base1`是一个仅包含8字节数的结构体，所以它占8个字节；`Base2`结构体不包含任何东西，占0个字节；`Derived1`是一个包含两个结构体的结构体，一个占8，一个占0，总共占8个字节；`Derived2`是一个包含8字节结构体，以及一个8字节字段，总共占16字节。我们看到内存使用是非常高效的。

## Static Dispatch

Rust不是动态语言，所以，下面写法是不允许的，

```rust
fn draw_text(txt) {
	txt.draw();
}
```

这里希望，如果`txt`的类型是`Text`，则调用`Text`对应的`draw`方法；如果`txt`的类型是`BoxedText`，则调用`BoxedText`的方法`draw`。因为Rust是强静态语言，要实现这个方案，有两种不等价的实现方式，

```rust
trait Draw {
	fn draw(&self);
}
struct Text { characters: String }
impl Text {
	fn from(text: &str) -> Text {
		Text { characters: text.to_string() }
	}
}
impl Draw for Text {
	fn draw(&self) {
		print!("{}", self.characters);
	}
}
struct BoxedText {
	text: Text,
	first: char,
	last: char,
}
impl BoxedText {
	fn with_text_and_borders(text: &str, first: char, last: char) -> BoxedText {
		BoxedText {
			text: Text::from(text),
			first: first,
			last: last,
		}
	}
}
impl Draw for BoxedText {
	fn draw(&self) {
		print!("{}", self.first);
		self.text.draw();
		print!("{}", self.last);
	}
}
let greeting = Text::from("Hello");
let boxed_greeting = 
	BoxedText::with_text_and_borders("Hi", '[', ']');

// SOLUTION 1 //
fn draw_text<T>(txt: T) where T: Draw {
	txt.draw();
}
draw_text(greeting);
print!(", ");
draw_text(boxed_greeting);
```

这里定义了泛型函数，并使用`where`从句确定类型边界。我们需要在这里引申解析静态派遣(static dispatch)这个概念。

首先声明了`Draw`，作为一个对象，拥有drawn的能力。

然后`Text`和`BoxedText`类型被声明，有对应的方法，有两个构造函数`Text::from`和`BoxedText::with_text_and_borders`；它们的`draw`函数的实现都继承来自`Draw`。

SOLUTION 1中的方法，`draw_text`泛型方法接收类型参数`T`，`T`是任何实现了`Draw`的类型。

因此，不乱编译器计数器在哪里调用`draw_text`函数，它会决定参数的类型，并检测该类型是否实现`Draw`。如果没有对应类型，编译器报错，若有具体的类型，会生成具体版本的`draw_text`函数，泛型函数体内的`draw`方法的调用，会被替换为对应`T`的实现的`draw`的方法。

这种技术称为“静态派遣static dispatch”。在计算机科学中，`dispatch`表示有几个同名函数时，选择调用哪个函数。在这段程序中，有两个函数命名为`draw`，因此派遣从两者中选择一个。在该程序中，选择由编译器处理，在编译期，这种派遣是“静态的static”。

## Dynamic Dispatch

上面的程序可以稍作改变，改变最后几行代码，

```rust
// SOLUTION 1/bis //
fn draw_text<T>(txt: &T) where T: Draw {
	txt.draw();
}
draw_text(&greeting);
print!(", ");
draw_text(&boxed_greeeting);
```

这里把接收参数，改为了一个reference，即在方法签名的参数带上`&`，以及两处调用带上`&`。

这种方案仍然是静态派遣。因此，可以看到静态派遣工作在值传递(pass-by-value)和引用传递(pass-by-reference)上。

上面的代码可以改变下，

```rust
// SOLUTION 2 //
fn draw_text(txt: &Draw) {
	txt.draw();
}
draw_text(&greeting);
print!(", ");
draw_text(&boxed_greeting);
```

该程序保留原来的行为，但使用了另一种技术。仅改变了`draw_text`的签名，删除了`T`类型参数，删除了`where`从句，参数用`&Draw`替换了`&T`。现在，由原来的泛型函数，替换为具体的函数，它的参数是对trait的一个引用。

不同的是，一个trait不是一个类型(type)。你不能声明一个变量或一个函数参数用trait来表示它的类型。但对trait的reference是一个有效的类型。然而，它不是普通的引用。

在第一个地方，如果它是一个普通引用，它不能将引用，传递`Text`或`BoxedText`中函数的参数；但实际上，它是允许的，考虑如下，

```rust
trait Tr {}
impl Tr for bool {}
let _a: &Tr = &true;
```

这里`bool`类型实现了`Tr`trait，所以`&true`的引用的值类型是`bool`，可以初始化给变量`_a`，`_a`是`Tr`的一个引用。

相反，下面写法是不合法的，

```rust
trait Tr {}
let _a: &Tr = &true;
```

这里，`bool` 没有`Tr`的实现，因此`&true`这个对`bool`的值引用，不能被初始化为`Tr`的引用。

通常地，任何对类型`T`的引用，都可以初始化为一个实现`T`的trait的一个引用。将参数传递给函数，是一种初始化处理，因此任何对类型`T`的引用，可以作为函数参数进行传递，这个参数引用的`trait`是`T`的实现。

在第二处，如果`&Draw`是一个普通指针，`txt`是指针的类型，表达式`txt.draw()`会调用相同的函数，取决于引用对象`txt`的名字。如其说需要一个dispatch，实际我们需要的是，当`draw_text`接收一个`Text`时，`Text`类型关联的`draw`方法被调用；当`draw_tet`接收一个`BoxedText`时，`BoxedText`类型关联的`draw`方法被调用。

所以，这里的`&Draw`并不是一个普通的指针，而一个能够根据引用对象的类型，选择调用方法的指针。这是一种派遣(dispatch)，但发生在运行时，因此叫做“动态派遣(dynamic dispatch)”。

动态派遣在C++中的通过`virtual`关键字处理，尽管机制略有不同。

## Implementation of References to Traits

回到原来代码关于派遣的问题，将最后几行替换如下代码，

```rust
use st::mem::size_of_val;
print!("{} {} {}, {} {} {}, ",
	size_of_val(&greeting),
	size_of_val(&&greeting),
	size_of_val(&&&greeting),
	size_of_val(&boxed_greeting),
	size_of_val(&&boxed_greeting),
	size_of_val(&&&boxed_greeting));
fn draw_text(txt: &Draw) {
	print!("{} {} {} ",
		size_of_val(txt),
		size_of_val(&txt),
		size_of_val(&&txt));
	txt.draw();
}
draw_text(&greeting);
print!(", ");
draw_text(&boxed_greeting);
```

在64位目标机器上，会打印：“`24 8 8, 32 8 8, 24 16 8 Hello, 32 16 8 [Hi]`”。

`size_of_val`定义在标准库的一个泛型函数，接收对象的引用，返回该引用对象的字节大小。

首先，`greeting`变量被处理。它的类型是`Text`结构体，仅包含一个`String`对象。我们已经探讨过`String`对象在栈上占24个字节，附带在堆一个缓冲区。这个缓冲区不会在`size_of_val`函数的计算范围内。

接着打印`Text`引用的大小，`Text`的引用是普通引用，占8个字节。

类似地，`boxed_greeting`变量是个结构体，有两个`char`对象。每个占4个字节，一共有24 + 4 + 4 = 32字节。

对于表达式`&greeting`，有类型`&Text`，它作为参数传递给`draw_text`函数，该函数实例化`txt`参数，参数类型是`&Draw`。

由于`txt`是一种引用，所以可以由表达式`size_of_val(txt)`计算。它会返回引用对象的大小。但就是哪个才是`&Draw`的引用对象？明显，一定不是`Draw`，因为`Draw`不是类型。实际上，在编译期不能确定。它需要运行期，有初始化`txt`参数的表达式决定。首先，第一次接收的`txt`参数的引用类型是`Text`，占24个字节。

当接收的`txt`参数的引用类型是`BoxedText`时，它占32个字节，将被打印。

回到`greeting`的调用处，我们发现表达式`size_of_val(&txt)`的值是16。这很奇怪。这个表达式是求类型`&Draw`对象的大小，由类型`&Text`的对象初始化。所以，实际上用了一个常规8字节引用来初始化一个16字节的trait引用？为什么对trait的引用这么大？

实际上，任何对trait的引用有两个字段。第一个字段，是初始化引用的一个拷贝；第二个字段，是一个指针，用于选择合适“版本”的`draw`函数，或者说其它函数的动态派遣。它的名字是“虚拟表指针，virtual table pointer”。该名称来源于C++。

最后，trait的引用的引用被打印，它是个常规引用，所以占8个字节。


## Static vs. Dynamic Dispatch

我们可以用静态派遣，也可以用动态派遣，哪个更适合？

“静态static”意味着“编译期”，“动态dynamic”意味着“运行期”，静态要求更多的编译时间，以及生成更多更快的代码，但如果编译期没有足够的可用信息，动态方案是唯一可能的选择。

假设将原来的示例程序更改一下需求，要求如果字符串是“b”，则输出带边框，否则，直接输出文本。

使用静态派遣，程序最后部分会变为，

```rust
// SOLUTION 1/ter //
fn draw_text<T>(txt: T) where T: Draw {
	txt.draw();
}
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
if input.trim() == "b" {
	draw_text(boxed_greeting);
} else {
	draw_text(greeting);
}
```

当使用动态派遣，

```rust
// SOLUTION 2/bis //
fn draw_text(txt: &Draw) {
	txt.draw();
}
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
let dr: &Draw = if input.trim() == "b" {
	&boxed_greeting
} else {
	&greeting
};
draw_text(dr);
```

静态派遣要求写几个函数调用，动态派遣允许你将选择的对象派遣给变量`dr`，然后仅需要些一个函数接收这个变量。

另外，静态派遣使用了泛型方法，这个技术可以会导致代码膨胀，可能最后会变得越来越慢。


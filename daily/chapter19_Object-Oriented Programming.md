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

首先是“Pair”已经定义，以及“Iterator”定义在标准库中，被用来实现“Pair”类型。两者都在范围可见并且不冲突。

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





























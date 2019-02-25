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

|                | `Tr`在当前rate         | `Tr`在其它crate                |
|----------------|------------------------|-------------------------------|
| `Ty`在当前crate | `impl Tr for Ty` 允许  | `impl Tr for Ty` 允许         |
| `Ty`在其它crate | `impl Tr for Ty` 允许  | `impl Tr for Ty` 不合法       |

## Mutating Methods
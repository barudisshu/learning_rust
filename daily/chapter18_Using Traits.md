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






































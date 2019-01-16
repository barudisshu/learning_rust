本章覆盖有：

- 为什么需要匿名函数，如何编写匿名函数，如何访问它定义的变量
- 这些“闭包”，如何声明和调用

## The Need for "Disposable" Functions

Rust对数组的升序实现，

```rust
let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
arr.sort();
print!("{:?}", arr);
```

结果将输出：“`[0, 1, 4, 7, 8, 10, 12, 45]`”。

但却没有包函数提供降序；你需要调用`sort_by`函数，将它的一个引用传递给一个比较函数。这种函数接受两个记录，并返回一个indication：

```rust
let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
use std::cmp::Ordering;
fn desc(a: &i32, b: &i32) -> Ordering {
	if a < b { Ordering::Greater }
	else if a > b { Ordering::Less }
	else { Ordering::Equal }
}
arr.sort_by(desc);
print!("{:?}", arr);
```

`desc`函数返回了一个标准库中定义的类型：

```rust
enum Ordering { Less, Equal, Greater }
```

这种方式生效，但有几点诟病。

首先，`desc`函数定义仅用于一处。标准库函数`sort_by`接收一个函数入参。这个入参需要是一个匿名函数，这个函数也仅用于一处。

另外，虽然类型规范对于变量声明是可选的，但对于参数和函数的返回值是必须的。这些规范，可以像函数名一样，方便地在其它语句或程序调用。但当你需要写一个函数仅在它声明的地方调用，这种规范几乎是匿名的。因此，声明和调用一个行内匿名的、由参数和返回值推断的类型的函数，将会是一个便利的特性。

另一个诟病是需要给函数体花括号闭合。通常函数会包含几条语句，因此不是所有匿名的带上花括号闭合。相反，匿名函数通常只有一条单一表达式，可以不用写闭合。

## Capturing the Environment

本章我们所陈述的所有内容，对于其它大多数语言也是适用的，包括C语言。但Rust函数有一个额外的不寻常限制：它不能访问任何外部声明的变量。你可以访问`static`的，你可以访问`constants`的，但不能访问栈分配的变量(也就是用`let`声明的变量)。例如，下面例子是不合法的：

```rust
let two = 2.;
fn print_double(x: f64) {
	print!("{}", x * two);
}
print_double(17.2);
```

编译出错：“can't capture dynamic environment in an fn item.”

“dynamic environment”意味着一系列变量在函数调用时才生效。所以，它是“dynamic”的，这些变量核能在某些语句生效，在其它语句失效。“capture the environment”意味着能够访问这些变量。

相反，下面是有效的：

```rust
const TWO: f64 = 2.;
fn print_double(x: f64) {
	print!("{}", x * TWO);
}
print_double(17.2);
```

或者这样写

```rust
static TWO: f64 = 2.;
fn print_double(x: f64) {
	print!("{}", x * TWO);
}
print_double(17.2);
```

这种限制有一个很好的理由：外部变量可以有效地进入函数的程序接口，但是从函数签名中看不出来，因此它对理解代码产生误导。

但当一个函数仅能在它定义的地方调用，访问外部变量并不能降低理解难度，因为这些外部变量在声明语句已经生效。

因此，我们特性的需求是：一个行内匿名函数，带类型推断；一个单一表达式作为函数体；可以捕获任何有效变量。

## Closures

闭包，说白了就是引用了自由变量的函数。这个被引用的自由变量将和这个函数一同存在，即使离开了创造它的环境也不例外。

闭包的调用出现在它定义的地方。实际上，你也可以定义一个闭包，尽管类型规范可行，实际上典型使用闭包的场景并不多。下面是使用闭包实现排序的一种方式：

```rust
let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
use std::cmp::Ordering;
let desc = |a: &i32, b: &i32| -> Ordering {
	if a < b { Ordering::Greater }
	else if a > b { Ordering::Less }
	else { Ordering::Equal }
};
arr.sort_by(desc);
print!("{:?}", arr);
```

跟前面不同的是：

- 使用了`let`关键字代替`fn`。
- 闭包名后面带有`=`号。
- 函数的参数由`(`和`)`，在闭包中变为`|`(管道)标志。
- 闭包声明带有分号`;`。

至此，我们说过，闭包声明和调用都在同一个地方，类型和大括号(braces)是可选的。因此，上面可以简化一下：

```rust
let mut arr = [4, 8, 1, 20, 0, 45, 12, 7];
use std::cmp::Ordering;
arr.sort_by(|a, b|
	if a < b { Ordering::Greater }
	else if a > b { Ordering::Less }
	else { Ordering:: Equal });
print!("{:?}", arr);
```









































本章覆盖

- 概念关于 "value" "object" "variable"
- 概念关于 "mutability"
- 关于初始化(initialization) 和 再赋值(re-assignment)
- 避免 unused variables 的警告
- 概念关于 "Boolean expression"
- 哪一种类型检查是由编译器执行的
- 某些运算符是如何同时执行运算和指派的
- 如何调用Rust标准库中的函数

## Associating Names to Values

```rust
let number = 12;
let other_number = 53;
print1("{} {}", number, 47);
```

12 作为`变量`的值输出，47作为字面量输出。

## Mutable Variables

```rust
let mut number = 12;
print!("{}", number);
number = 53;
print!(" {}", number);
```

`mut`是Rust的关键字，mutable的缩写。

## Uninitialized Variables

```rust
let number;
number = 12;
print!("{}", number);
```

如果没有指派number的值，将发生编译错误。

```rust
let number1;
let number2 = 22;
number1 = number2;
print!("{}", number1);
```

上述语句是合法的，先初始化number2，再初始化numer1。

```rust
let number1;
print!("{}", number1);
number1 = 12;
```

上述语句将发生编译错误。

## The Leading Underscore

Rust在编译时遇到 unsed variables时，会提示

```bash
let number = 12;
  |         ^^^^^^ help: consider using `_number` instead
  |
  = note: #[warn(unused_variables)] on by default
```
不妨遵循警告，改为 `let _number = 12;`。警告消失了！！！ 这种前面加下划线的方式，可以用于沉默这类的警告。

```rust
let _ = 12;
```

上述语句有另一层意思。它不声明一个变量。单一下划线字符不是一个有效的标识，它是一个占位符，表示 "don't-care"标识。因为它没有任何意义。所以下述程序不是有效的：

```rust
let _ = 12;
print!("{}", _);
```

## Boolean Values

```rust
let truth = true;
let falsity = false;
print!("{} {}", truth, falsity);
```

```rust
let truth = 5 > 2;
let falsity = -12.3 >= 10.;
print!("{} {} {}", truth, falsity, -50 < 6);
```

## Boolean Expressions

```rust
let truth = true;
let falsity = false;
println!("{} {}", ! truth, ! falsity);
println!("{} {} {} {}", falsity && falsity, falsity && truth, truth && falsity, truth && truth);
println!("{} {} {} {}", falsity || falsity, falsity || truth, truth || falsity, truth || truth);
```

## Type Consistency in Assignments

```rust
let num n = 1;
print!("{}", n);
n = 2;
print!(" {}", n);
n = 3;
print!(" {}", n);
```

如果改为 `n = 3.14;`将抛出类型错误，

## Change of Type and of Mutablility

```rust
let mut n = 1;
print!("{}", n);
n = 2;
print!(" {}", n);
let n = 3.14;
print!(" {}", n);
```

`let n = 3.14`表示 re-declares，n的值被重新声明。因此n的类型变更为浮点型。

下述语句会发生编译错误，请找出原因：

```rust
let mut _n = 1;
_n = 2;
let _n = 3.14;
_n = 5.9;
```

重声明(re-declaration)带来新的变量，因此：

```rust
let x = 120; print!("{}", x);
let x = "abcd"; print!("{}", x);
let mut x = true; print!("{} ", x);
x = false; print!("{}", x);
```

## Assignment Arithmetic Operators

```rust
let mut a = 12;
a = a + 1;
a = a - 4;
a = a * 7;
a = a / 6;
print!("{}", a);
```

```rust
let mut a = 12;
a += 1;
a -= 4;
a *= 7;
a /= 6;
print!("{}", a);
```

## Using the Functions of the Standard Library

Rust安装时提供了官方标准库。和C不同，不需要使用`#include`指示源码文件。默认地，不需要引入任何模块即可直接使用。

```rust
print!("{} {}", str::len("abcde"), "abcde".len());
```

在Rust有两种形式调用函数，即使用 `::` 或 `.`无参形式。




























本章覆盖有：

- 如何编写16进制、10进制、2进制字面量
- 使用下划线的数字字面量
- 如何使用指数计数法表示超大或超小的数
- 10个原生类型、两个浮点类型；以及它们的范围；使用场景
- 如何指定数字字面量的具体类型和非约束类型(以`u`开头）
- 如何将一个数字类型转换为另一个数字类型
- 其他原生类型： 布尔、字符、元组(tuple)
- 类型推导是如何工作的
- 如何表述数组(array)和向量(vector)的类型
- 如何指派名字到编译期的常亮
- 如何用编译器发现表达式的类型


## Non-Decimal Numeric Bases

十进制记数法叫做"decimal notation" 或 "base-ten notation"，一个数10在不同的形式表示不同：

```rust
let hexadecimal = 0x10;
let decimal = 10;
let octal = 0o10;
let binary = 0b10;
print!("{} {} {} {}", hexadecimal, decimal, octal, binary);
```

输出结果为："16 10 8 2"

- 数字字面量以0x开头(`x`就是 "he`x`adecimal")，表示16进制记数法。
- 数字字面量以0o开头(`o`就是 "`o`ctal")，表示10进制记数法。
- 数字字面量以0b开头(`b`就是 "`b`inary")，表示2进制记数法。
- 其他任何情况都表示十进制记数法。

虽然表示形式不一样，它们的类型都是整数类型，因此你可以：

```rust
let hexadecimal = 0x10;
let octal = 0o10;
let binary = 0b10;
let mut n  = 10;
print!("{} ", n);
n = hexadecimal;
print!("{} ", n);
n = octal;
print!("{} ", n);
n = binary;
print!("{} ", n);
```

实际上，浮点型数字仅能以十进制形式表示。

上述这种表达仅限于源代码中。对于编译器来说，所有数字类型都是以二进制形式表示。

对于十六进制的数字表述不区分大小写，诸如`0xAEf5b`和`0xaeF5B`是相同的。

但是数值基础(前缀）必须是小写，诸如`0X4`，`0O4`，`0B4`是不合法。


## Underscore in Numeric Literals

我们看到，编写一个"one billion"， 1000000000。可读性非常差。前面说过，下划线表示任意类型，并且被编译器忽略。因此你可以使用下划线字符进行区分。因此`3___4_.56_`是个合法的数字，它等效于`34.56`。下划线通常用于对数字分组：

```rust
let hexadecimal = 0x_00FF_F7A3;
let decimal = 1_234_567;
let octal = 0o_777_205_162;
let binary = 0b_0110_1001_1111_0001;
print!("{} {} {} {}",
    hexadecimal, decimal, octal, binary);
```

## The Exponential Notation

浮点型数字可以用指数记数法表示：

```rust
let one_thousand = 1e3;  // e^3 = 10 ^3
let one_million = 1e6;
let thirtheen_billions_and_half = 13.5e9;
let twelve_millionths = 12e-6; // 0.000012
```

e前面的称为"尾数(mantissa)",e后面跟着的称为"指数(exponent)"。它们都被声明为十进制数。


## The Various Kinds of Signed Integer Numbers

Rust中有10种整数类型，2种浮点类型。这所以这样定义是为了达到："高效"。我们知道，数字类型越多，带来的优势也越多，因为它可以针对具体的形式定义。例如，我们仅存储0到200的数字，显然用一个32位的对象存储显得内存有点浪费，因为你可以用一个8位表示。

Rust提供了8位、16位、32位以及64位的整数类型。

```rust
let a: i8 = 5;
let b: i16 = 5;
let c: i32 = 5;
let d: i64 = 5;
print!("{} {} {} {}", a, b, c, d);
```

## Unsigned Integer Number Types


和C语言类似，Rust中有Unsigned 整数类型。

```rust
let a: u8 - 5;
let b: u16 = 5;
let c: u32 = 5;
let d: u64 = 5;
print!("{} {} {} {}", a, b, c, d);
```



















































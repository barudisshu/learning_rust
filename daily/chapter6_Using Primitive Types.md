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

前面看到，整型和浮点型数值不能加减处理，因为它们有不同类型。类似地，不同位数的整型也不能相加：

```rust
let a: i8 = 5;
let b: i16 = 5;
print!("{}", a + b);
```

编译将报“mismatched types”。

或许会有人疑惑，为什么都已8、16、32、64作为整数的位，而不是19之类的，主要基于3个原因，以效率为目的：

- 处理器架构以2进制为单位转换。
- 内存管理和分配机制。
- 类型越多，机器码也越多，一门语言需要有限的类型来适应CPU缓存。

最后一项，不是说类型越少越好，主要取决于CPU指令集。

## Unsigned Integer Number Types

如果定义一个对象包含一个范围0~200的整数，用哪种类型合适？上小节介绍，使用最小类型范围最好。`i8`类型是最小类型的，但它仅能表示-128~+127之间的数。目前为止所学的类型，我们必须使用`i16`。

但这不是最理想的，因为所有包含在0~255的值，包括极限值(extremes)，如果我们解析它们，仅能使用8个位表示。这种解析早已包含在所有现代处理器的机器语言中，如果不这样用会有点可惜。

和C语言类似，Rust中允许使用4种数字类型，它们都是无符号类型，并且都是非负数。

```rust
let a: u8 = 5;
let b: u16 = 5;
let c: u32 = 5;
let d: u64 = 5;
print!("{} {} {} {}", a, b, c, d);
```

这里解析下，“u”，表示 “unsigned(无符号类型)”，标示它是一个unsinged integer number。“u”后面跟的数字表示该对象用了多少位；例如，“a”变量使用8位，它可以表述256个值。因此，成为一个unsigned数字，该值表述为0到255的整数，包括极值。

另一点好处是，一般要检查一个整数x是否在\[0,n)，我们需要写` 0 <=x && x < n`。但如果x是一个__unsigned__ number，我们可以直接简化为`x < n`。

注意变量“a”，“b”，“c”，“d”也是4种类型。

## Target-Dependent Integer-Number Types

除了上面介绍的8种整型类型，Rust种还有一种不定类型。它的位数取决于编译器所在的操作系统的位数。

在16位计算机，它是unsinged 16-bit integger。
在32位计算机，它是unsinged 32-bit integger。
在64位计算机，它是unsinged 64-bit integger。

实际上，Rust并不支持16位系统，仅支持32位和64位。

为了决定不同系统这种依赖问题，Rust包含有`isize`类型和`usize`类型：

```rust
let arr = [11, 22, 33];
let i: usize = 2;
print!("{}", arr[i]);
```

这里的 `usize`，“u”表明是一个无符号整数，“size”表示类型根据设备的长度决定。

机器码是32位系统的，`usize`类型就是`u32`；机器码是64位系统的，`usize`类型就是`u64`。

也就是说，`usize`类型是一个unsigned integer，并且跟内存地址长度一致。

```rust
let arr = [11, 22, 33];
let i: usize = 2;
print!("{}", arr[i]);
let i: isize = 2;
print!("{}", arr[i]);
let i: u32 = 2;
print!("{}", arr[i]);
let i: u64 = 2;
print!("{}", arr[i]);
```

会有3个编译错误，实际上，仅`usize`类型被运行作为array的下标。相似地，vector也仅能用`usize`。

为了对称，Rust提供了`isize`类型，它是一个 __signed__ 整型，跟系统内存地址长度一致。

## Type Inference

















































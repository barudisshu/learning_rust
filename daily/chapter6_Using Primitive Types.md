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

前面几章，我们声明变量而没有指定它们的类型，以及我们讨论了整型、浮点型等。

本章，我们开始讨论变量类型声明。

考虑如果没有指定类型，变量是否仍然有一个指定类型，或有一个泛型类型？

```rust
let a = [0];
let i = 0;
println!("{}", a[i]);
```

这段程序是有效的，为什么？不是说对于数组的下标，仅`usize`才是有效的吗？

事实上，每个变量和表达式总是有一个明确的(well-defined)类型。但并不需要显式指定。通常情况下，编译器能够根据变量或表达式的上下文进行演绎，更常规的说法——推断(infer)。

例如，这里的变量“i”，有值0，编译器便知道“i”的类型一定是一个整数类型，但还没有确定是哪一种，因为在Rust中有10种整数类型。这个变量的类型是一个`泛型的(generic)`、`无约束的(unconstrained)`整型数。

当编译器意识到该变量被用于数组下标，得知仅有`usize`类型才能作为数组下标，编译器赋予变量“i”的类型`usize`。

又如，

```rust
let i = 0;
let _j: u16 = i;
```

编译器最先得知“i”的类型是“unconstrained integer number”，然后得知变量“_j”被显式声明为`u16`，因为“i”被用作初始化“_j”，便可得知“i”的真实类型。

相反，下面会出现语法错误

```rust
let i = 0;
let _j: u16 = i;
let _k: i16 = i;
```

错误出现在第3行，“expected i16, found u16”。

相反，下面是有效的

```rust
let i = 0;
let _j: u16 = i;
let _k = i;
```

注意，在编译期，每个成功的编译，每个变量只有一个具体的、约束的类型。

如果编译器不能推断一个变量的类型，会生成一个编译错误。

实际上，如果编译器推断出类型仅是一个整数，但不能确定是哪个位数的整数，默认选择`i32`作为整数类型。例如：

```rust
let i = 8;
let j = 8_000_000_000;
print!("{} {}", i, j);
```

程序将打印：“8 -589934592”。

这里两个变量都是`i32`类型。由于第二个变量太大，超出了数值范围。和C语言类似，整型字面量溢出不会产生错误，但是会有编译警告。

## The Type Inference Algorithm

编译器每次都为每个变量和表达式推断一个具体类型。它是使用了下面的算法。

如果类型是显式指定，类型就是它指定的。

如果变量或表达式的变量没有指定，变量或表达式所在的语句或声明仅限某一特定类型，该类型就是该变量或表达式的类型。


## Floating-Point Numberic Types

Rust中仅有两种浮点类型

```rust
let a: f64 = 4.6;
let b: f32 = 3.91;
println!("{} {}", a, b);
```

`f64`是64位浮点数，`f32`是32位浮点数。“f”就是“floating-point”。该类型对应C语言的“double”和“float”类型。

到目前为止，Rust已没有更多数字类型了。

下面看看这段代码：

```rust
let a = 4.6;
let mut _b: f32 = 3.91e5;
_b = a;
```

前面说过，Rust会进行类型推断，“a”的类型推断为`f32`。所以这段代码是有效的。

Rust中，默认的浮点类型是`f64`，所以，如果没有最后一段代码，“a”的类型是`f64`。

## Explicit Conversions

Rust每次编译都提供类型检查，要在不同类型间处理计算，可以使用`as`关键字显式转换。

```rust
let a: i16 = 12;
let b: u32 = 4;
let c: f32 = 3.7;
print!("{}", a as i8 + b as i8 + c as i8);
```

输出结果是“19”。对于浮点值 3.7，小数点部分会被舍弃计算。

下面代码中，由于显式转换超出了数值范围，数值发生溢出。

```rust
let a = 500 as i8;
let b = 100_000 as u16;
let c = 10_000_000_000 as u32;
print!("{} {} {}", a, b, c);
```

结果将打印 "-12 34464 1410065408"。

## Type Suffixes of Numberic Literals

Rust中声明一个数字变量有几种方式：

```rust
let _a: i16 = -150;
let _b = -150 as i16;
let _c = -150 + _b = _b;
let _d = -150i16;
```

除了上面说到的用`as`关键字，你也可以使用后缀的方式`-150i16`，为了代码清晰，你可以加下划线表示`-150_i16`或`5__u32`

对于浮点数，`-4f32`或`0_f32`是32位浮点数。如果没有小数位，小数点可以省略。

## All the Numberic Types

下面例子列出了Rust所有数据类型：

```rust
let _: i8 = 127;
let _: i16 = 32_767;
let _: i32 = 2_147_483_647;
let _: i64 = 9_223_372_036_854_775_807;
let _: isize = 100; // The maximum value depends on the target architecture
let _: u8 = 255;
let _: u16 = 65_535;
let _: u32 = 4_294_967_295;
let _: u64 = 18_446_744_073_709_551_615;
let _: usize = 100; // The maximum value depends on the target architecture
let _: f32 = 1e38;
let _: f64 = 1e308;
```

下面列出Rust的内建整型类型：

|   Type   |  Occupied bytes  |    Minimum value          |    Maximum value                       |
|:--------:|:----------------:|:-------------------------:|:--------------------------------------:|
|  i8      |   1              |    $-128$                 |    $+127$                              |
|  i16     |   2              |    $-32768$               |    $+32767$                            |
|  i32     |   4              |    $-2147483648$          |    $+2147483647$                       |
|  i64     |   8              |    $-2^63$                |    $+2^63 - 1$                         |
|  isize   |   4 or 8         |    on a 32-bit target:    |    on a 320bit target:                 |
|          |                  |    $-2147483648$;         |    $+2147483647$;                      |
|          |                  |    on a 64-bit target:    |    on a 64-bit target:                 |
|          |                  |    $-2^63$                |    $+2^63-1$                           |
|  u8      |   1              |    $0$                    |    $+255$                              |
|  u16     |   2              |    $0$                    |    $+65535$                            |
|  u32     |   4              |    $0$                    |    $+4294967295$                       |
|  u64     |   8              |    $0$                    |    $+2^64-1$                           |
|  usize   |   4 or 8         |    $0$                    |    on a 32-bit target: $+4294967295$;  |                         |
|          |                  |                           |    on a 64-bit target: $+2^64-1$;      |                |


前面说过，Rust中仅有两种浮点类型：

- `f32`，32位，等同于C语言的float类型。
- `f64`，64位，等同于C语言的double类型。

## Booleans and Characters

除了数字类型，Rust还定义了其它一些原生类型：

```rust
let a: bool = true; print!("[{}]", a);
let b: char = 'a'; print!("[{}]", b);
```

用法和C语言类似。但有点不同，由于Rust用的是Unicode字符，所以Rust中的`char`是4个字节的，C中只有一个字节。

字符字面量用单引号表示，它可以表述非ASCII的字符：

```rust
let e_grave = 'è';
let japanese_character = 'さ';
println!("{} {}", e_grave, japanese_character);
```

和C语言不同，`bool`和`char`类型不能看做数字处理，因此下面语句是错误的：

```rust
let _a = 'a' + 'b';
let _b = false + true;
```

但可以显式转换：

```rust
print!("{} {} {} {} {}", true as u8, false as u8,
'A' as u32, 'à' as u32, '€' as u32);
```

事实上，每个字符在Unicode有对应的编码，因此你可以将数字显式转换为字符：

```rust
for i in 0..256 {
println!("{}: [{}]", i, i as u8 as char);
}
```

但语句bool型，不能实现数字到bool的转换，实际上，只需要带上 `truth == 0`这样的表达式就可以了，没有必要在Rust中实现这种类型转换。

注意，在char类型转换中，必须是unsigned的。

## The Empty Tuple

Rust中还有一个奇怪的原生类型，在Rust中叫“()”，圆括号。该类型只有一个值，它的值和类型一个写法，也是“()”。它和C语言的void类型类似，又或者雷同于JavaScript中的“undefined”。为了有个好听的名字，被称为"empty tuple"。

这种类型会出现在几个情况：

```rust
let a: () = ();
let b = { 12; 87; 283 };
let c = { 12; 87; 283; };
let d = {};
let e = if false { };
let f = while false { };
print!("{:?} {:?} {:?} {:?} {:?} {:?}", a, b, c, d, e, f);
```

输出为："() 283 () () () ()".

第一行好理解，值类型和值都是“()”，所以输出结果为 “()”。

第二行开始，涉及好几个概念，

首先像“12” 、 “87”这些简单的数字被用作表达式。当然，这里的表达式什么也没做，它被编译为机器码。

第二个概念是，语句块的值，由它最后一个表达式定义，如果有这么一个表达式，比如这里的第二行，最后一个表达式是283，所以变量“b”的值是283，由于没有指定类型，默认是`i32`.

第三行中，由于语句块结束，最后一个表达式是空，或者说是void、undefined。所以“c”的值是“()”，C的类型也是“()”。

第四行同理，

第五行中，它是一个条件语句。由于没有“else”分支，“else {}”被隐式处理。因此，该表达式被看作为`let e = if false {} else {}`。

第六行，不管是`while`、`loop`还是`for`，实际上，它自身的值总是`()`，你可以理解为在使用`while`的构造函数。诸如这种写法，编译会报错：

```rust
let l:() = while false {
	return 1;
};
println!("{:?}", l);
```


## Array and Vector Types

前面说过，如果我们改变容器条目的类型，数组或向量的类型相应也被改变；但如果改变容器条目的数量，仅改变数组的类型，向量的类型并没有改变。

如果你要显式更变数组或向量的类型，你可以：

```rust
let _array1: [char; 3] = ['x', 'y', 'z'];
let _array2: [f32; 200] = [0f32; 200];
let _vector1: Vec<char> = vec!['x', 'y', 'z'];
let _vector2: Vec<i32> = vec![0; 5000];
```

注意array和vector的写法；array用`[ ; ]`；vector用`Vec<>`。可以看到，数组的类型包含元素的类型和长度，向量仅包含类型。


## Constants

下面程序是不合法的：

```rust
let n = 20;
let _ = [0; n];
```

这是因为数组的长度必须在编译期确定，尽管“n”是immutable的。但某种意义上说，n会在运行时被修改或覆盖。所以不能指定数组的大小。

下面程序是合法的：

```rust
const N: usize = 20;
let _ = [0; N];
```

关键字`const`允许我们声明一个在编译期有唯一值，且运行期不会被改变。这里要求指定具体的类型。

Rust的常量和C++的`const`对应。

## Discovering the Type of an Expression


























本章覆盖有：

- 如何从有限集定义枚举变量
- 如何用枚举实现组合类型
- 如何用`match`模式匹配处理枚举
- 如何用`match`处理其他数据类型，诸如整数、字符、字符串
- 如何用布尔值泛化`match`结构


## Enumerations

与其下面的写法：

```rust
const EUROPE: u8 = 0;
const ASIA: u8 = 1;
const AFRICA: u8 = 2;
const AMERICA: u8 = 3;
const OCEANIA: u8 = 4;

let continent = ASIA;

if continent == EUROPE { print!("E"); }
else if continent == ASIA { print!("As"); }
else if continent == AFRICA { print!(Af"); }
else if continent == AMERICA { print!("Am"); }
else if continent == OCEANIA { print!("O"); }
```

不如用下面等价写法：

```rust
enum Continent {
	Europe,
	Asia,
	Africa,
	America,
	Oceania,
}

let contin = Contient::Asia;

match contin {
	Continent:: Europe => print!("E"),
	Continent:: Asia => print!("As"),
	Continent:: Africa => print!("Af"),
	Continent:: America => print!("Am"),
	Continent:: Oceania => print!("O"),
}
```

`enum`关键字后面指定了一个新的类型Continent。这种类型称为枚举(enumerative)，因为它包含一系列条目，每个条目有唯一关联数字。在这个例子中，类型Continent允许的值是Europe，Asia，Africa，America和Oceania，内部由值0u8，1u8，2u8，3u8，4u8表示。

创建了枚举类型后，就可以创建该类型的对象。例如这里的 contin 变量，类型是 Continent。

枚举对象仅有一个值，这个值称为“变量(variants)”。

注意，变量的值必须由它的类型区分，例如 Continent::Asia。

```rust
enum T {A, B, C, D};
let n: i32 = T::D;
let e: T = 1;
```

第二行会发生编译错误，第二三行都提示mismatched types错误。尽管期望是`i32`的枚举值`main::T`，但枚举类型并不能隐式转换为数值类型，数值类型也不能隐式转换为枚举类型。

## The match Construct

Rust中，match语句是基本工具。类似于C语言的switch，但在很多层面不同。

首先，match关键字不需要闭合括号。

其次，多个case，称为“手臂(arms)”，用作模式，跟随符号“=>”，接着跟着一个表达式。这些手臂用逗号分隔。

不管是声明枚举类型，还是在“`match`”语句，最后一个条目的逗号可写可不写。通常的做法是，如果是每行一个item，就带上逗号；如果是整行罗列，最后一个逗号会省略：

```rust
enum CardinalPoint { North, South, West, East };
```

match语句的行为是。

首先，`match`后的语句先被执行，因此获取一个值，例如这里是Continent::Asia。

然后，这个值会被用于比较五个pattern的每个手臂(arm)，按照行顺序进行比较。如果匹配，右边的语句被执行，语句结束。

注意每个手臂右边的必须是一个单一表达式。

只要求表达式是个有效语句就可以了，并且带上分号。例如

```rust
let a = 7.2;
12;
true;
4 > 7;
5.7 + 5. *a;
```

上述这些代码是有效的，只是什么也没做。










































































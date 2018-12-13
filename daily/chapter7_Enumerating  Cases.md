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

事实上，只要带上分号，任何表达式都能成为有效的语句。例如

```rust
let a = 7.2;
12;
true;
4 > 7;
5.7 + 5. *a;
```

上述这些代码是有效的，只是什么也没做。

但是，有些语句并不是有效的表达式。例如，“let a = 3; ”、“fn empty() {}”，

```rust
match contin {
	Continent::Europe => let a = 7;,
	Continent::Asia => let a = 7,
	Continent::Africa => f aaa() {},
	Continent::America => print!("Am"),
	Continent::Oceania => print!("O"),
}
```

前面三个case会报错，因为 “=>”后面跟的不是有效的表达式。

若果我们要执行多个表达式，可以使用语句块方式实现：

```rust
enum Continent {
	Europe,
	Asia,
	Africa,
	America,
	Oceania,
}
let mut contin = Continent::Asia;
match contin {
	Continent::Europe => {
		contin = Continent::Asia;
		print!("E");
	}
	Continent::Asia => {
		let a = 7;
	}
	Continent::Africa => print!("Af"),
	Continent::America => print!("Am"),
	Continent::Oceania => print!("O"),
}
```

## Relational Operators and Enums

枚举不能用“==”操作符作比较。实际上，下面程序是不合法的：

```rust
enum CardinalPoint { North, South, West, East };
let direction = CardinalPoint::South;
if direction == CardinalPoint::North { }
```

编译器报“binary operation `==` cannot be applied to type `main::CardinalPoint`”。因此，你应该用 match 语句。

不仅是“==”操作符，其它二进制操作符都是错误的：

```rust
enum CardinalPoint { North, South, West, East };
if CardinalPoint::South < CardinalPoint::North { }
```

## Handling All the Cases

下面代码编译出错：

```rust
enum CardinalPoint { North, South, West, East };
let direction = CardinalPoint::South;
match direction {
	CardinalPoint::North => print!("NORTH"),
	CardinalPoint::South => print!("SOUTH"),
}
```

会获得一个错误“non-exhaustive patterns: `West` and `East` not covered”。编译器会抱怨说，四个枚举值，仅两个被考虑了，另外两个West和East没有考虑进去。这是由于Rust要求显式处理所有的情况。

要解决这个问题有两种做法，一是匹配所有的情况；

```rust
enum CardinalPoint { North, South, West, East };
let direction = CardinalPoint::South;
match direction {
	CardinalPoint::North => print!("NORTH"),
	CardinalPoint::South => print!("SOUTH"),
	CardinalPoint::East => {},
	CardinalPoint::West => {},
}
```

二是使用下划线，做默认情况处理；

```rust
enum CardinalPoint { North, South, West, East };
let direction = CardinalPoint::South;
match direction {
	CardinalPoint::North => print!("NORTH"),
	CardinalPoint::South => print!("SOUTH"),
	_ => {},
}
```

注意，下划线语句会匹配任何值，所以要将它放在最后。


## Using match with Numbers

match结构，除了可以用于枚举，也可以用于其他数据类型：

```rust
match "value" {
	"val" => print!("value "),
	_ => print!("other "),
}
match 3 {
	3 => print!("three "),
	4 => print!("four "),
	5 => print!("five "),
	_ => print!("other "),
}
match '.' {
	':' => print!("colon "),
	'.' => print!("point "),
	_ => print!("other "),
}
```

结果将输出：“other three point”。


## Enumerations with Data

Rust的枚举类型，并没有前面看到的这么简单：

```rust
enum Result {
	Success(f64),
	Failure(u16, char),
	Uncertainty,
}

// let outcome = Result::Success(23.67);
let outcome = Result::Failure(1200, 'X');

match outcome {
	Result::Success(value) => print!("Result: {}", value),
	Result::Failure(error_code, module) => print!("Error n. {} in module {}", error_code, module),
	Result::Uncertainty => {},
}
```

结果将输出：“Error n. 1200 in module X”。

若替换为注解行，结果将输出为“Result: 23.67”。

对比于C语言，Rust的枚举类型包含枚举和组合的特性。

```c
#include <stdio.h>
int main() {
    enum eResult {
        Success,
        Failure,
        Uncertainty
    };
    struct sResult {
        enum eResult r;
        union {
            double value;
            struct {
                unsigned short error_code;
                char module;
            } s;
        } u;
    } outcome;

/*
outcome.r = Success;
outcome.u.value = 23.67;
*/
outcome.r = Failure;
outcome.u.s.error_code = 1200;
outcome.u.s.module = 'X';

switch (outcome.r) {
	case Success:
		printf("Result: %g", outcome.u.value);
		break;
	case Failure:
		printf("Error n. %d in module %c",
			   outcome.u.s.error_code,
			   outcome.u.s.module);
		break;
	case Uncertainty:
		break;
	}
return 0;
}
```

在“match”语句中，模式匹配的参数，例如Result::Success(value)中的value，会被看做是该scope下的变量，以及该变量的类型，由这个手臂(arm)声明。

当手臂(arm)满足case，这个变量的值就被初始化。例如value的值是23.67。并且用于手臂右边的作用范围。

如果不需要这个变量，为避免编译告警，可以：

```rust
enum Result {
	Success(f64),
	Failure(u16, char),
	Uncertainty,
}

let outcome = Result::Success(23.67);

match outcome {
	Result::Success(_) => print!("OK"),
	Result::Failure(error_code, module) =>
		print!("Error n. {} in module {}", error_code, module),
	Result::Uncertainty => {},
}
```

## "match" Expressions

类似于“if”表达式，“match”也有表达式：

```rust
enum CardinalPoint { North, South, West, East };
let direction = CardinalPoint::South;
print!("{}", match direction {
	CardinalPoint::North => 'N',
	CardinalPoint::South => 'S',
	_ => '*',
});
```

结果会输出：“S.”。

这里有个要求，就是手臂右边的值，必须类型是一样的。如果将第三个case改为“_ => {}”。会发生“match arms have incompatible types”。因为是一个match表达式，所以它只能有指定的一种类型。

## Use of Guards in match Constructs

假设我们要区分整数的类别：负数、0、1、正数：

```rust
for n in -2..5 {
	println!("{} is {}.", n, match n {
		0 => "zero",
		1 => "one",
		_ if n < 0 => "negative",
		_ => "plural",
	});
}
```
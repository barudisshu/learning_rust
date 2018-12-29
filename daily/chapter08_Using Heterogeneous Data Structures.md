本章覆盖有：

异构数据结构，就是不同类型的组合，包括有，

- Tuples
- Structs
- Tuple-Structs

用于不同类型的分组对象。本章最后，会介绍一些代码风格的规约。

## The Tuples

数组和向量可以包含多个条目元素，每个条目必须是相同类型。如果你希望一个对象存储不同类型的几个子对象，可以如下实现：

```rust
let data = (10000000, 183.19, 'Q');
let copy_of_data = data;
print!("{}, {}, {}", data.0, copy_of_data.1, data.2);
```

结果将打印：“10000000, 183.19, Q”。

“data”是一个组合对象，由三个对象组成。这三个对象有不同类型：整型、浮点型、字符。

这类对象称为：`tuple`。

tuple的声明和array类似。唯一不同是tuple用的是小括号，数组用的是方括号。

tuple的条目称为字段(field)。

tuple的类型可以显式指明：

```rust
let data: (i32, f64, char) = (10000000, 183.19, 'Q');
```

你可以用点符号(dot-notation)访问tuple的field的值。

类似地，tuple可以是mutable的：

```rust
let mut data = (10000000, 183.19, 'Q');
data.0 = -5;
data.2 = 'x';
print!("{}, {}, {}", data.0, data.1, data.2);
```

输出为：“-5, 183.19, x”。

类似于数组，tuple可以有任意个字段(field)。如果一个field都没有，这个tuple的类型是“()”。

前面已经介绍过“empty tuples”。下面解析下：

一个不同的地方是，tuples不同通过变量下标进行访问：

```rust
let array = [12, 13, 14];
let tuple = (12, 13, 14);
let i = 0;
print!("{}", array[i]);
print!("{}", tuple.i);
```

最后一行是不合法的，因为不能在运行时确定tuple的field的值。即使用const来标识也不行，因为tuple.i表述的是它的field，语法上认为`<expr>.<integer>`，解析器会认为取其field，实际上可看做是一个方法调用，主要是为了语义上的一致性。

> 为什么Rust这样定义：
>实际上，可以参考[源码](https://github.com/AndrewScull/rust/blob/1cc8b6ec664f30b43f75551e95299d943c8a4e6a/src/libcore/tuple.rs)得知，tuple在宏中，满足`<expr>.<integer>`，后面的数字被看做identifier处理了。
>可以参考[rfc](https://github.com/rust-lang/rfcs/blob/master/text/0184-tuple-accessors.md)这样设计的原因。


## The Structs

Tuple在条目较少时比较好用。当字段过多了，会带来一些障碍：

```rust
let data = (10, 'x', 12, 183.19, 'Q', false, -9);
print!("{}", data.2 + data.6);
```

上述代码，并不能一眼看出输出结果是3。另外，任何tuple字段是按顺序定义的，当类型足够多时，会变得难于阅读：

```rust
let data1 = (10, 'x', 12, 183.19, 'Q', false, -9);
let mut data2: (u16, char, i16, f64, bool, char, i16);
data2 = data1;
```

另外，对于tuple来说，添加或删除一个filed，data.2就可能偏移到data.3。这样处理也很麻烦。

因此，Rust给出了一个有具体类型的结构，相当于C语言的构造体：

```rust
struct SomeData {
	integer: i32,
	fractional: f32,
	character: char,
	five_bytes: [u8; 5],
}
let data = SomeData {
	integer: 10_000_000,
	fractional: 183.19,
	character: 'Q',
	five_bytes: [9, 0, 250, 60, 200],
};

print!("{}, {}, {}, {}",
	data.five_bytes[3], data.integer,
	data.fractional, data.character);
```

结果打印：60， 10000000， 183.19， Q。

Rust中的结构体，是有`struct`关键字声明的一个block，它声明了一个“SomeData”类型。任何该类型的对象，都有且仅有固定的字段，以及每个字段的类型也是固定的。

要声明一个该类型的变量“data”，以及初始化这个类型对象。直接用“name”带上对应字段即可。

要访问struct-object的值，用法和tuple一样，用dot-notation访问。

上述代码类似于C语言：

```c
#include <stdio.h>
int main() {
	struct SomeData {
		int integer;
		float fractional;
		char character;
		unsigned char five_bytes[5];
	};
	struct SomeData data = {
		10000000,
		183.19,
		'Q',
		{9, 0, 250, 60, 200},
	};
	printf("%d, %d, %g, %c",
		data.five_bytes[3], data.integer,
		data.fractional, data.character);
	return 0;
}
```

下面和C语言的写法进行对比下。

C中字段用分号隔开，Rust中用逗号，

Rust中，字段带有类型，放在后面；C放在前面，

C的字段类型可以简写，“int a, b;”；但是在Rust中，每个字段都要指定类型“a: i32, b: i32”。

C中，初始化很简单，类似于Rust的tuple；但是Rust的结构体初始化，必须指定字段名。

不论C还是Rust，都是点标记符(dot-notation)调用。

如果声明的是一个可变(mutable)变量，同样可以用点标记符(dot-notation)变更字段的值。


```rust
struct SomeData {
	integer: i32,
	fractional: f32,
}
let mut data = SomeData {
	integer: 10,
	fractional: 183.19,
};
data.fractional = 8.2;
print!("{}， {}", data.fractional, data.integer);
```

结果将输出：“8.2，10”。

和tuple一样，strcut也可以是空。

## The Tuple-Structs

到目前为止，有两类结构化的容器：

- tuple，类型没有名称，不能提前声明，字段没有名称；
- struct，类型有名称，必须提前声明，字段有名称。

另外还需要一种结构：类型有名称，需要提前声明，但字段没有名称。这种基于tuple和struct形式的结构体。称为：“`tuple-structs`”。

```rust
struct SomeData (
	i32,
	f32,
	char,
	[u8; 5],
);
let data = SomeData (
	10_000_000,
	183.19,
	'Q',
	[9, 0, 250, 60, 200],
);
print!("{}, {}, {}, {}", 
	data.2, data.0, data.1, data.3[2]);
```

这种结构感觉很别扭，因为是用圆括号作语句块，并且没有指定字段的名称，初始化像struct，访问字段方式像tuple。

区别于tuple和struct，`tuple-structs`这种结构不能为空。

实际应用中，tuple-struct并不常用。


## Lexical Convertions

到目前为止，我们认识了一部分Rust结构体(不是全部！)，接下来要阐述一些词法规则。这些规则是如此根深蒂固，甚至于违反这些规则会令编译器抛出警告。

```rust
const MAXIMUM_POWER: u16 = 600;
enum VehicleKind {
	Motorcycle,
	Car,
	Truck,
}
struct VehicleData {
	kind: VehicleKind,
	registration_year: u16,
	registration_month: u8,
	power: u16,
}
let vehicle = VehicleData {
	kind: VehicleKind::Car,
	registration_year: 2003,
	registration_month: 11,
	power: 120,
};
if vehicle.power > MAXIMUM_POWER {
	println!("Too powerful");
}
```

上述例子阐述了一些规则要求：

- 常理的名称要大些，并且用下划线划分词组。
- 类型的名称由应用代码或标准库定义，枚举变量的名称由一组关联词汇粘合，要求命名首字母大写。
- 其它(诸如关键字`let`、原生类型`u8`)名称则由小写字母，下划线划分词组。

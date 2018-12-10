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













































































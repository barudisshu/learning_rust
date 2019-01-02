本章覆盖有：

- 如何定义单独的函数，使得调用者可以处理不同的数据类型
- 泛型函数如何使用类型推导来避免指定具体类型
- 如何在struct、tuple-struct、enum实现各种泛型类型
- 如何使用两个常见的枚举类型，Option和Result
- 标准函数对Option和Result的处理

## Need of Generic Functions

Rust提供一个静态类型检查，所以定义函数的参数必须是确切的类型，例如`fn square_root(x: f32) -> f32`，调用时必须传递一个确切的参数，例如`square_root(45.2f32)`，或显示转换为它所需要的参数`square_root(45.2f64 as f32)`。你不能传递不同的类型。

这不论是写代码、还是调用代码都带来了不方便。甚至于Rust有很多不同的整型类型，当写一个函数，每次都要处理选择哪一种类型。例如，当你用`i16`类型作为参数，但每次都传递了`i32`类型，这要求我们重新定义我们的函数。

另外，函数会被作为模块在其它地方调用，这也不能满足每个使用者。

## Defining and Using Generic Functions

最笨的办法是编写泛型函数：

```rust
// Library code
fn f<T>(ch: char, num1: T, num2: T) -> T {
    if ch == 'a' { num1 }
    else { num2 }
}

let a: i16 = f::<i16>('a', 37, 41);
let b: f64 = f::<f64>('b', 37.2, 41.1);
println!("{} {}", a, b);
```

泛型参数类型定义在函数名后面，由尖括号囊括，它就是函数的类型参数声明。它表述声明的不是一个具体函数，而是一个泛型函数，该函数的泛型参数类型，仅在编译期确定。

参数`T`的定义范围仅限于函数定义内。实际上仅会出现在3个地方，函数签名，以及函数语句体内部调用，不会在其它地方出现。

泛型函数的调用，需要带上类型参数，类型参数就是具体的类型替换这个`T`即可。如这里的`f::<i16>`、`f::<f64>`。

C语言没有泛型类型的概念，但C++有：它使用了函数模板。


## Inferring the Parametric Types

上面的代码可以进一步简化，

```rust
// Library code
fn f<T>(ch: char, num1: T, num2: T) -> T {
    if ch == 'a' { num1 }
    else { num2 }
}

let a: i16 = f('a', 37, 41);
let b: f64 = f('b', 37.2, 41.1);
println!("{} {}", a, b);
```

这里把`::<i16>`和`::<f64>`移除了，以及得到等价的结果。实际上，编译器在解析一个泛型函数的调用时，使用了参数的类型来确定类型。

说白了，参数化类型(parametric type)是由泛型函数调用的表达式推断(inferred)出来的。

当然，被使用的类型必须是一致的：

```rust
fn f<T>(a: T, _b: T) -> T { a }
let _a = f(12u8, 13u8);
let _b = f(12i64, 13i64);
let _c = f(12i16, 13u16);
let _d: i32 = f(12i16, 13i16);
```

最后一个出现编译错误，虽然参数传递了相同的类型，但函数返回的类型不匹配变量声明。

如果有几个不同的类型需要参数化，你可以指定泛型函数的多个类型参数。

```rust
fn f<Param1, Param2>(_a: Param1, _b: Param2) {}
f('a', true);
f(12.56, "Hello");
f((3, 'a'), [5, 6, 7]);
```

## Defining and Using Generic Structs

参数化类型也适用于声明泛型结构体和泛型元组-结构体(tuple-struct)：

```rust
struct S<T1, T2> {
	c: char,
	n1: T1,
	n2: T1,
	n3: T2,
}
let _s = S { c: 'a', n1: 34, n2: 782, n3: 0.02 };

struct SE<T1, T2> (char, T1, T1, T2);
let _se = SE ('a', 34, 782, 0.02);
```

结构体和元组-结构体的类型参数声明，和泛型函数的声明一样，都是在名称后面带上类型参数声明。

当然也可以显式指定类型参数的具体类型：

```rust
struct S<T1, T2> {
	c: char,
	n1: T1,
	n2: T1,
	n3: T2,
}
let _s = S::<u16, f32> { c: 'a', n1: 34, n2: 782, n3: 0.02 };
struct SE<T1, T2> (char, T1, T1, T2);
let _se = SE::<u16, f32> ('a', 34, 782, 0.02);
```

C语言没有泛型结构体，但C++提供了：类模板和结构模板。


## Genericity Mechanics

要理解泛化的内部机制，应该先看编译器这个角色。实际上，概念上来说，泛型代码的编译出现在几个阶段。

让我们跟随编译机制的基本概念，提供一下代码：

```rust
fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }
let x = swap(3i16, 4u16);
let y = swap(5f32, true);
print!("{:?} {:?}", x, y);
```

第一阶段，源码被浏览，每次编译器查找一个泛型函数声明(这里的是`swap`函数)，编译器加载该函数的数据结构，检查泛型代码有没有语法错误。

第二阶段，源码再次被浏览，编译器统计泛型函数的调用次数，并加载该函数的结构关联关系——泛型声明的内部响应和使用进行关联，这阶段在类型检查和响应合法的前提下进行。


在这两个阶段下，编译器获得了一个泛型函数`swap`，一个`main`函数，以及`swap`函数的引用。

第三阶段，所有泛型函数的调用被读取(这里的例子，swap调用了两次)。每次调用，确定一个具体的响应类型的定义。这个具体类型可能在调用的地方显式获取，又或者有参数的表达式推断出来。例如这里例子，第一处调用swap，参数`T1`关联的类型是`i16`，`T2`关联的类型是`u16`；以此类推...

确定了具体泛型参数类型后，一个确定类型版本的函数被生成。每个泛型函数的调用，都会替换到具体生成的函数上。

例如，内部生成的Rust代码为：

```rust
fn swap_i16_u16(a: i16, b: u16) -> (u16, i16) { (b, a) }
fn swap_f32_bool(a: f32, b: bool) -> (bool, f32) { (b, a) }
let x = swap_i16_u16(3i16, 4u16);
let y = swap_f32_bool(5f32, true);
print!("{:?} {:?}", x, y);
```

可以看到，调用了两处，就生成了两个具体的函数。

第四阶段，是编译这些生成的代码。

```rust
fn swap<T1, T2>(a: T1, b: T2) -> (T2, T1) { (b, a) }
let x = swap('A', 4.5);
let y = swap('g', -6.);
print!("{:?} {:?}", x, y);
```

如果调用的参数类型一致，仅会生成一个版本，因为所有调用的参数类型都相同。

上面会生成，

```rust
fn swap_char_f64(a: char, b: f64) -> (f64, char) { (b, a) }
let x = swap_char_f64('A', 4.5);
let y = swap_char_f64('g', -6.);
print!("{:?} {:?}", x, y);
```

上面得出，一段程序，会生成几个具体版本的机器代码：

- 对于编译非泛型代码，这种多阶段编译会有几分慢。
- 生成的代码针对每个特定的调用进行了高度优化，因此不需要对类型转换或决策，运行时的性能都是优化的。
- 每次调用使用不同的数据类型时，会产生大量的机器码。这会导致一个现象，“代码膨胀(code bloat)”，面对这个事实，为了优化性能，最好不要在单一进程使用过多不同的类型，具体代码使用具体的类型，这回给CPU缓存带来负担。

对于泛型结构体和泛型元组-结构体也一样原理。


## Generic Arrays and Vectors

关于array和vector不是新的事物。最开始的章节已经介绍了泛型类型。

实际上，array是Rust语言的一部分，vector作为结构体定义在Rust标准库。


## Generic Enums

在Rust中，enum也可以泛化。

```rust
enum Result1<SuccessCode, FailureCode> {
	Success(SuccessCode),
	Failure(FailureCode, char),
	Uncertainty,
}
let mut _res = Result1::Success::<u32,u16>(12u32);
_res = Result1::Uncertainty;
_res = Result1::Failure(0u16, 'd');
```

上面代码是合法的。下面代码在最后一行发生编译错误，因为第一个参数Failure的类型是`u32`，但根据初始化，它实际上应该是`u16`

```rust
enum Result1<SuccessCode, FailureCode> {
    Success(SuccessCode),
    Failure(FailureCode, char),
    Uncertainty,
}
let mut _res = Result1::Success::<u32,u16>(12u32);
_res = Result1::Uncertainty;
_res = Result1::Failure(0u32, 'd');
```

泛型枚举在Rust标准库中被大量用到。

Rust标准库中enum最常被用于解决下面常见问题。如果一个函数可以失败(`fail`)，失败时应该做什么？

例如，当vector包含条目，函数`pop`移除vector最后一个条目，并返回被删除的记录。若vector是空的，表达式`vec![0;0].pop()`应该怎样处理？

某些语言不定义这种行为，让其报错或导致一个不可预测的结果。Rust尽可能避免这种未定义的行为。

某些语言抛出一个异常，由闭合块或当前函数的调用方处理，或报错误。Rust中不适用异常这一概念。

某些语言会返回一个指定的`null`值。但vector可以包含几乎所有类型，这些类型没有`null`值。

下面是Rust的解决办法：

```rust
let mut v = vec![11, 22, 33];
for _ in 0..5 {
	let item: Option<i32> = v.pop();
	match item {
		Some(number) => print!("{}, ", number),
		None => print!("#, "),
	}
}
```

结果将输出：“33, 22, 11, #, #, ”。

`pop`函数作用于`Vec<T>`类型对象，并返回一个`Option<T>`类型的值。

该泛型类型被定义在Rust的标准库中：

```rust
enum Option<T> {
	Some(T),
	None,
}
```

它是一个optional的T类型，表示有，或者无。


## Error Handling

Rust标准库也定义了一个泛型枚举来处理函数不能返回正确类型的情况：

```rust
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
	if denominator == 0. {
		Err(format!("Divide by zero"))
	} else {
		Ok(nmerator / denominator)
	}
}
print!("{:?}, {:?}", divide(8., 2.), divide(8., 0.));
```

这回输出 `Ok(4), Err("Divide by zero")`。

`Result`类型和`Option`类型类似，其中`Option`表示有或无，`Result`表述了一种异常情况。

它在标准库的定义为：

```rust
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

我们使用了debug输出结果信息，在生产环境不建议这样做，可以改为下面这种形式：

```rust
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0. {
        Err(format!("Divide by zero"))
    } else {
        Ok(numerator / denominator)
    }
}

fn show_divide(num: f64, den: f64) {
    match divide(num, den) {
        Ok(val) => println!("{} / {} = {}", num, den, val),
        Err(msg) => println!("Cannot divide {} by {}: {}", num, den, msg),
    }
}
show_divide(8., 2.);
show_divide(8., 0.);
```

结果将输出：

```
8 / 2 = 4
Cannot divide 8 by 0: Divide by zero
```

## Enum Standard Utility Functions

`Option`和`Result`标准泛型类型以一种灵活、高效的方式，允许我们捕获real-world code出现的所有情况；然而，使用`match`语句来获取结果有点不方便。

因此，标准库包含一些工具类函数，以方便`Option`和`Result`类型的使用。

```rust
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
	if denominator == 0. {
		Err(format!("Divide by zero"))
	} else {
		Ok(numerator / denominator)
	}
}
let r1 = divide(8., 2.);
let r2 = divide(8., 0.);
println!("{} {}", r1.is_ok(), r2.is_ok());
println!("{} {}", r1.is_err(), r2.is_err());
println!("{}", r1.unwrap());
println!("{}", r2.unwrap());
```

程序首先输出：

```
true false
false true
4
```

然后给出一个panic信息：“thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "Divide by zero"'”。

`is_ok`函数返回`true`如果变式为Ok，`is_err`返回`true`如果变式为Err。`is_err()`等价于`! is_ok()`。

当作用于一个Ok变式，`unwrap`函数返回`Ok`变式的值，否则出现panics。该函数的意思是“我知道在一个Ok变式中可以wrap这个值，因此我只想获取这个容器的值，摈除它的转换；若不是Ok变式，会出现一个不可覆盖的错误，因此我会立即终止该程序”。该代码编译没有出错，运行时会出现panic错误。

对于`Option`枚举也有`unwrap`函数，要输出一个Vec的所有制，你可以：

```rust
let mut v = vec![11, 22, 33];
for _ in 0..v.len() {
	print!("{}, ", v.pop().unwrap())
}
```

结果会输出：“33, 22, 11,”。`unwrap`的调用会获取`Ok`内部的值。我们避免了在一个空vector调用`pop()`；否则，`pop()`返回一个`None`，`unwrap()`会出现panick。

`unwrap`函数常被用于`quick-and-dirty`Rust程序中，即错误不要求处理的情况。
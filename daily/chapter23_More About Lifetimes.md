本章覆盖有：

- 简单、自由(free)函数和方法需要避免写入生命周期指示器(lifetime specifiers)，因为它们是被推断的
- 为什么包含引用的结构体(structs)、元组-结构体(tuple-structs)、枚举(enums)需要生命周期指示器(lifetime specifiers)
- 如何为结构体(structs)、元组-结构体(tuple-structs)、枚举(enums)编写生命周期指示器(lifetime specifiers)
- 为什么包含指向泛型参数的结构体需要生命周期边界(协变、逆协变)

## Lifetime Elision

上一章节，我们看到每个函数签名，都必须确定引用返回值，即要么是`'static`的lifetime specifier，要么是关联的函数参数的lifetime specifier。

有时避免编写这种烦人的注解。

```rust
trait Tr {
    fn f(x: &u8) -> &u8;
}
```

这代码是被允许的。返回的值是个引用，它没用指定生命周期，但它也不是`'static`的，因此这个隐式生命周期指示器(implicit lifetime specifier)必须是函数参数中的其中一个。因为函数参数仅有一个，因此它的lifetime specifier无可厚非就是该参数关联的lifetime specifier。换言之，这个声明函数等效于下面的写法：

```rust
trait Tr {
    fn f<'a>(x: &'a u8) -> &'a u8;
}
```

甚至下面的声明也是有效的：

```rust
trait Tr {
    fn f(b: bool, x: (u32, &u8)) -> &u8;
}
```

因为仅有一个引用，因此可以推断出返回值指向的对象。

下面的代码也是有效的：

```rust
trait Tr {
    fn f(x: &u8) -> (&u8, &f64, bool, &Vec<String>);
}
```

这种情况，返回值有好几个引用，但参数的引用仍然仅有一种。

参数引用类型仅有一种时，你甚至可以省略其返回值类型，而用在其它类型中指定。

```rust
trait Tr {
    fn f<'a>(x: &'a u8) -> (&u8, &'a f64, bool, &'static Vec<String>);
}
```

这里的返回值包含三个引用：第一个没有指定，第二字段有`'a`，第三个用了`'static`生命周期指示器。然而，参数中仍然仅有一个引用，因此返回的第一个字段的引用有一个隐式`'a`生命周期指示器。

这种省略机制称为“**lifetime elision**”。为了简化语法，生命周期指示器遇到这种仅有一个非静态(non-static)值时，可以 __删去(elided)__ 。

## Lifetime Elision with Object-Oriented Programming

考虑下面：

```rust
trait Tr {
    fn f(&self, y: &u8) -> (&u8, &f64, bool, &Vec<String>);
}
```

因为参数有两个引用，所以前面的规则不起作用。然而，当一个方法返回某些引用时，多数情况下这些引用会租借当前的对象，即由`&self`指向的值。因此，为了简化表达式，前面的代码被看做是下面的等价：

```rust
trait Tr {
	fn f<'a>(&'a self, y: &u8) -> (&'a u8, &'a f64, bool, &'a Vec<String>);
}
```

目前，你可以为特定引用重载行为。这种情况，你希望返回值的第二个字段生命周期关联参数`y`，你需要写，

```rust
trait Tr {
	fn f<'a>(&self, y: &'a u8) -> (&u8, &'a f64, bool, &Vec<String>);
}
```

这里，tuple中第二个字段指向的对象，存活时间必须要少于`y`指向的对象，以及第一个字段和第四个字段指向的对象，存活时间必须要少于`self`指向的对象。

当然，这个规则同样作用于`&mut self`参数。


## The Need of Lifetime Specifiers for Structs

上一个章节中，我们看到这种代码是有效的：

```rust
let x: i32 = 12;
let _y: &i32 = &x;
```

这是因为，尽管`_y`持有对`x`的引用，它存活少于`x`。

相反，下面是不合法的，

```rust
let _y: &i32;
let x: i32 = 12;
_y = &x;
```

因为`_y`先于`x`声明，所以它会在`x`后销毁，它存活长于`x`。

我们也看到了，对于函数签名，需要有合适的注解(即lifetime specifier)，来满足有效的租借检查(borrow checker)。

除了上一章介绍这些情况，对于结构体(struct)来说，包含某些引用的issue下也相似。

下面代码看起来是合法的(实际上不是)：

```rust
struct S {
	_b: bool,
	_ri: &i32,
}
let x: i32 = 12;
let _y: S = S { _b: true, _ri: &x };
```

下面这种很明显是不合法：

```rust
struct S {
	_b: bool,
	_ri: &i32,
}
let _y: S;
let x: i32 = 12;
_y = s { _b: true, _ri: &x };
```

尽管`S`的字段`_ri`持有一个`x`的引用，但它存活长于`x`。

这种情况可以非常简单分析出来，但在一个real-world程序中可能是：

```rust
// In some library code:
struct S {
	_b: bool,
	_ri: &i32,
}
fn create_s(ri: &i32) -> {
	s { _b: true, _ri: ri }
}
// In application code:
fn main() {
	let _y: S;
	let x: i32 = 12;
	_y = create_s(&x);
}
```

该程序代码是无效的，因为，当调用`create_s`时，指向`x`的引用会被存储在`_y`对象内部，但`_y`存活长于`x`(`_y`先于`x`声明)。

但程序员若不查看函数体的代码，又怎么知道`create_s`函数将参数的引用对象作为返回存储了？让我们看看下面的有效程序，它和上面这个类似，

```rust
// In some library code:
struct S {
	_b: bool,
	_ri: &'static i32,
}
fn create_s(ri: &i32) -> S {
	static ZERO: i32 = 0;
	static ONE: i32 = 1;
	S {
		_b: true,
		_ri: if *ri > 0 { &ONE } else { &ZERO },
	}
}

// In application code:
fn main() {
	let _y: S;
	let x: i32 = 12;
	_y = create_s(&x);
}
```

代码中，函数`create_s`仅是用`ri`参数来初始化结构体的`_ri`字段。因此参数的值没有被存储在结构体内。`_ri`会包含一个静态值，即`ZERO`或`ONE`，静态变量不会被销毁。

这里的`create_s`函数签名虽然和上一个代码一样；但前一个代码是不合法的，因为参数被存储在结构体的字段中，而这里，参数通过`*ri`使用后，scope就已经结束了，并被回收掉。

因此，如果没有lifetime specifier，编程者被强迫去阅读函数`create_s`的方法体的内容，这样才能得知该函数是否会存储参数的引用对象。这种设计是糟糕的。

为了让编程者避免需要分析`create_s`函数来得知对象的声明周期是否合法，有必要有更进一步的生命周期注解(lifetime annotations)。

因此，类似于函数，仅是对于结构体函数，必须显式指定它们字段的每个引用的生命周期。

这解析了上面代码中，形式上看起来是有效的代码片段，实际上会产生“missing lifetime specifier”的编译错误。


## Possible Lifetime Specifiers for Structs

实际上，对于一个结构体的引用字段的生命周期，Rust编译器仅允许两种可能：

- 该字段仅指向静态对象(static objects)。
- 或者指向的对象，虽然不是静态的，但预先存在整个结构体对象中，并且存活长于该结构体。

第一种情况已经介绍过，

```rust
struct S { _b: bool, _ri: &'static i32 }
```

该结构体包含有一个引用，不过它是`'static`引用，所以该引用不能被指派给任何租借的引用值。因此不会有生命周期的讨论问题，只要`_ri`字段被分配的是静态引用。

相反，对于第二种情况，下面有效代码阐述，

```rust
// In some library code:
struct s<'a> { _b: bool, _ri: &'a i32 }
fn create_s<'b>(ri: &'b i32) -> S<'b> {
	s{ _b: true, _ri: ri }
}
// In application code:
fn main() {
	let x: i32 = 12;
	let _y: S;
	_y = create_s(&x);
}
```

由于仅包含一个参数，所以可以通过lefetime elision由其隐式推断，

```rust
fn create_s(ri: & i32) -> S {
	s{ _b: true, _ri: ri }
}
```



































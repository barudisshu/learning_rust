本章覆盖有：

- 如何定义程序/函数,如何调用
- 同名函数
- 函数的参数如何传递，by-value还是by-reference
- 函数的返回值
- 如何提前退出一个函数
- 对象的引用如何控制

## Defining and Invoking a Function

函数的定义比较简单，以`fn`关键字开头，函数名+参数+块。

```rust
fn foo() {
    // your code here
}
```

这个块是函数的“body”，对这个body的处理称为函数署名“signature”。

同大多数语言一样，main方法/函数式程序的入口，它由机器码调用。

和C有点不同的是，函数的定义，可以在其它函数的body内直接定义。

```rust
fn f1() { print!("1"); }
fn main() {
    f1();
    fn f2() { print!("2"); }
    f2(); f1(); f2();
}
```

注意，块定义的函数作用范围仅在其块内。


## Functions Defined After Their Use

下面代码是不合法的：

```rust
a;
let a = 3;
```

因为变量在定义前被调用了；对于函数，只要在作用范围内，顺序不受影响，下面是有效的：

```rust
f();
fn f() {};
```

## Functions Shadowing Other Functions

前面说过，变量声明如果同名，变量会被投影。但是函数却不能这样：

```rust
fn f() {}
fn f() {}
```

这样写会有编译错误：“the name `f` is defined multiple times”。

只有在不同的块里面，才可以定义多个同名的函数：

```rust
{
    fn f() { print!("a"); }
    f(); f();
}
{
    fn f() { print!("b"); }
    f(); f();
}
```

结果将输出：“aab”。

并且不能再语句块外部调用：

```rust
{
    fn f() { }
}
f();
```

编译器会抱怨：“cannot find function `f` in this scope”。

最后，函数投影，仅能投影语句块外部定义的。下面是完整例子：

```rust
fn f() { print!("1"); }
fn main() {
    f(); // Prints 2
    {
        fn(); // Prints 3
        fn f() { print!("3"); }
    }
    f(); // Prints 2
    fn f() { print!("2"); }
}
```

实际上，按照编译器的思路，当调用`fn()`时，会先找临近的块作用范围，如果有，则调用最近路径上的函数。如果没有，则往下一个节点寻找(类似于树的深度搜索)。

## Passing Arguments to a Function

可以将参数传递给函数，

```rust
fn print_sum(addend1: f64, addend2: f64) {
	println!("{} + {} = {}", addend1, addend2, addend1 + andend2);
}
print_sum(3., 5.);
print_sum(3.2, 5.1);
```

3 + 5 = 8
3.2 + 5.1 = 8.3

现在先来理解下圆括号(parentheses)的用法！在一个函数定义，闭合着参数定义列表；该闭合参数在函数调用时传递。

函数参数的定义和变量的定义类似。

因此，下面代码会被解析：

```rust
{
	let addend1: f64 = 3.; let addend2: f64 = 5.;
	println!("{} + {} = {}", addend1, addend2,
		addend1 + addend2);
}
{
	let addend1: f64 = 3.2; let addend2: f64 = 5.1;
	println!("{} + {} = {}", addend1, addend2,
		addend1 + addend2);
}
```

函数参数定义和变量定义的主要不同是，函数参数的定义，类型是必须的，它不能依赖于类型推断。

类型推断总是被用于编译器检查，确保接收的参数是实际参数的声明类型。正是这样，下面代码

```rust
fn f(a: i16) {}
f(3.);
f(3u16);
f(3i16);
f(3);
```

会发生错误，因为浮点类型不能传递给一个整型参数；第二个函数调用也一样，u16类型的值不能传递给一个i16的参数。

最后两个函数调用是运行的。实际上，第三处调用传递了正确的参数类型；第四处调用，它被类型推断，传递了unconstrained integer类型。


## Passing Arguments by Value

注意参数不是简单传递对象的新名，它不是别名；实际上他们是对象的拷贝。该拷贝在函数调用时被创建，函数结束并return给调用者时拷贝被销毁。下面例子阐明这个概念：

```rust
fn print_double(mut x: f64) {
	x *= 2.;
	print!("{}", x);
}
let x = 4.;
print_double(x);
print!(" {}", x);
```

结果输出：“8 4”。
变量名“x”被声明并初始化，然后传递给函数“print_double”，该函数包含一个参数“x”，函数执行正确后返回给调用者。

实际上，不是 __变量__ 传递给函数，二是变量的 __值__ 。它称为 __值传递__ ，和C语言类似。变量“x”的值被用于初始化一个新变量，这个新变量为函数的参数。新的变量被修改，并在函数体内打印，最后由函数结束销毁。该函数的调用者的变量并没有发生改变。

注意这里的函数签名“print_double”，在参数“x”前有关键字“`mut`”。它允许函数体内第一条语句；至此，该语句仅改变函数参数的值，函数外部的变量并没有发生改变，所以外部的“x”不需要用`mut`关键字修饰。


## Returning a Value from a Function

函数若要给调用者返回一个结果：

```rust
fn double(x: f64) -> f64 { x * 2. }
print!("{}", double(17.3));
```

返回值实际上是函数体自身。因为函数体是个块，所以它的值就是最后一个表达式的值，否则就是一个空tuple ()。

函数的返回类型，在C语言写在函数名前面，在Rust则写在后面，并由符号“`->`”隔离。

如果没有指定返回类型，默认是空tuple，即前面说的“()”：

```rust
fn f1(x: i32) {}
fn f2(x: i32) -> () {}
```

函数体的类型，必须与函数签名指定的类型相同，或者无符号类型可以约束的类型。因此下面代码是合法的：

```rust
fn f1() -> i32 { 4.5; "abc"; 73i32 }
fn f2() -> i32 { 4.5; "abc"; 73 }
fn f3() -> i32 { 4.5; "abc"; 73 + 100 }
```

下面代码不合法：

```rust
fn f1() -> i32 { 4.5; "abc"; false }
fn f2() -> i32 { 4.5; "abc"; () }
fn f3() -> i32 { 4.5; "abc"; {} }
fn f4() -> i32 { 4.5; "abc"; }
```

## Early Exit


要让一个函数从某条中间语句结束，可以使用`return`关键字返回，

```rust
fn f(x: f64) -> f64 {
	if x <= 0. { return 0.; }
	x + 3.
}
print!("{} {}", f(1.), f(-1.));
```

`return`关键字和C语言类似，不同的是最后一个语句可以不写。

```rust
fn f(x: f64) -> f64 {
	if x <= 0. { return 0.; }
	return x + 3.;
}
print!("{} {}", f(1.), f(-1.));
```

上面的写法不是严谨的，

```rust
fn f(x: f64) -> f64 {
if x <= 0. { 0. }
else { x + 3. }
}
print!("{} {}", f(1.), f(-1.));
```

如果函数签名指定的是空tuple，可以有多种写法：

```rust
fn f(x: i32) {
	if x <= 0 { return; }
	if x == 4 { return (); }
	if x == 7 { return {}; }
	print!("{}", x);
}
f(5);
```

任何函数调用可被看做一个有效语句：

```rust
fn f() -> i32 { 3 }
f();
```

这里，返回值被忽略并立即销毁。

相反，如果返回值被使用，如下，

```rust
fn f() -> i32 { 3 }
let _a: i32 = f();
```

它必须是一个正确的类型。


## Returning Several Values

可以使用tuple返回多个值：

```rust
fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
	(dividend / divisor, dividend % divisor)
}
print!("{:?}", divide(50, 11));
```

结果输出“(4, 6)”

或者你可以返回一个enum, struct, tuple struct, array, vector：

```rust
enum E { E1, E2 }
struct S { a: i32, b: bool }
struct TS (f64, char);
fn f1() -> E { E::E2 }
fn f2() -> S { S { a: 49, b: true } }
fn f3() -> TS { TS (4.7, 'w') }
fn f4() -> [i16; 4] { [7, -2, 0, 19] }
fn f5() -> Vec<i64> { vec![12000] }
print!("{} ", match f1() { E::E1 => 1, _ => -1 });
print!("{} ", f2().a);
print!("{} ", f3().0);
print!("{} ", f4()[0]);
print!("{} ", f5()[0]);
```

结果输出“"-1 49 4.7 7 12000”。

下面解析下。

函数`f1`调用返回一个枚举E2，并用于匹配E1，没有匹配，返回默认值-1。
函数`f2`调用返回一个结构对象，并访问该结构的field。
函数`f3`调用返回一个tuple-struct，通过数字identifiered访问field。
函数`f4`调用返回一个数组，并获取数组下标的值。
函数`f4`调用返回一个向量，并获取向量下标的值。


## How to Change a Variable of the Caller

假设我们要对数组作平方处理：

```rust
let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
for i in 0..10 {
	if arr[i] < 0 { arr[i] *= 2; }
}
print!("{:?}", arr);
```

现在要将其封装成一个函数：

```rust
fn double_negatives(mut a: [i32; 10]) {
	for i in 0..10 {
		if a[i] < 0 { a[i] *= 2; }
	}
}
let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
double_negatives(arr);
print!("{:?}", arr);
```

结果仅输出“\[5, -4, 9, 0, -7, -1, 3, 5, 3, 1\].”。并没有达到预期。

前面说个，函数的参数是变量的一个拷贝，因此没有办法直接修改外部变量。你可以：


```rust
fn double_negatives(mut a: [i32; 10]) -> [i32; 10] {
	for i in 0..10 {
		if a[i] < 0 { a[i] *= 2; }
		}
	a
}
let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
arr = double_negatives(arr);
print!("{:?}", arr);
```

这种方法有点遗憾：数据被拷贝了两次，第一次发生在函数调用上，第二次发生在覆盖赋值上。这种拷贝会造成额外的计算消耗，并且可以避免的。


## Passing Arguments by Reference

函数可以传递被传参数的地址，这个方式称为引用传递，

```rust
fn double_negatives(a: &mut [i32; 10]) {
	for i in 0..10 {
		if (*a)[i] < 0 { (*a)[i] *= 2; }
	}
}
let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
double_negatives(&mut arr);
print!("{:?}", arr);
```

该代码没有发生数据拷贝的现象。

这种方式称为引用传递(by reference)。它的语法和C语言的类似。

这里出现了符号“`&`”和“`*`”，在Rust中表示的意义和C语言一样。符号“`&`”表示“`对象的(内存)地址...`”，符号“`*`”表示“`(内存)地址的对象...`”。

函数`double_negatives`的参数类型是`&mut [i32; 10]`，在类型前面带一个`&`符号，指明它是类型对象的地址(也叫指针、引用)。因此，`a`在这里表示的是“长度为10的32位整型数组的地址(指针、引用)”。

在函数体内，我们并不关心这个地址(指针、引用)，而是这个内存地址的对象，所以我们需要用`*`访问该对象。通常，给定一个引用`a`，表达式`*a`标识为该对象的引用。

函数方法内的第二行，`*a`标识为内存地址之上的对象，这个对象就是数组，因此可以用下标访问。

围绕表达式`*a`的圆括号是必须的，因为方括号优先于星号，所以`*a[i]`会被当做`*(a[i])`处理，并且这里会发生编译错误。

使用这类参数，函数`double_negatives`仅接收一个数组的地址，因此可以对该数组进行读写操作。

声明该函数后，要调用该函数。数组必须声明以及初始化为可变(mutable)，因为它内部会发生改变。

注意，调用函数需要显式带上`mut`关键字一并传递。

实际上，该函数可以简化成下面等价的版本：

```rust
fn double_negatives(a: &mut [i32; 10]) {
	for i in 0..10 {
		if a[i] < 0 { a[i] *= 2; }
	}
}
```

我们删除了两个星号，以及圆括号，

我们说过这里的`a`不是一个数组，而是数组的地址，那么这里的`a[i]`表达式应该不合法才对。目前Rust对这类地址作了如下简化：

每当一个引用被不正确使用，即它是一个非引用值时，Rust会假装前面有一个星号(asterisk)。也就是Rust对其进行了反向引用(dereference)，所以这里把内存地址(指针、引用)替代为引用自身。

由此产生的语法就是C++的引用，Rust的不同之处在于，星号(asterisk)可以写、可以省略，但C++必须在指针前面写，必须不能在引用前面写。


## Using References

引用主要用于函数参数上，但也可能用在其他地方：

```rust
let a = 15;
let ref_a = &a;
print!("{} {} {}", a, *ref_a, ref_a);
```

结果输出：“15 15 15”。

前面阐述过，`ref_a`是一个内存地址，`*ref_a`标识为该内存地址上的对象引用，最后一个语句，编译器尝试直接输出`ref_a`时，会反向获取该引用的对象，并输出结果。

使用引用，你可以做一些技巧性的事情：

```rust
let a = &&&7;
print!("{} {} {} {}", ***a, **a, *a, a);
```

这里的`a`就变成了7的引用的引用的引用... 所以输出结果就变成了引用的引用的引用的对象... 但这里你不能再加多星号(asterisk)，否则会发生编译错误，因为反向引用只发生了3层。

最后一个表达式是隐式的、顺序性的、重复了上面几个步骤。

什么情况下星号才能被省略，

```rust
fn main() { a(); b(); c(); d(); }

fn a() {
    let v = 1;
    let x = &v;
    println!("a {}", *x);
    println!("a {}", 1 + *x);
}

fn b() {
    let v = 1;
    let x = &v;
    println!("b {}", x);
    println!("b {}", 1 + x);
}

fn c() {
    let mut v = 1;
    let mut x = &mut v;
    println!("c {}", *x);
    println!("c {}", 1 + *x);
}

fn d() {
    let mut v = 1;
    let mut x = &mut v;
    println!("d {}", x);
    println!("d {}", 1 + x); // error
}
```

最后一个会出错：`the trait bound \`_: std::ops::Add<&mut _>\` is not satisfied [E0277]`。

这是因为对于`Add`trait的操作，内部实现了`i32 + &32`(`&i32 + &i32`)的处理，但是，对于immutable reference没有扩展有`&&i32` 或 `&mut i32`的实现。

## Mutablility of References

如何在reference上使用mut关键字：

```rust
let mut a: i32 = 10;
let mut b: i32 = 20;
let mut p: &mut i32 = &mut a;	// line 3
print!("{} ", *p);
*p += 1;	// line 5
print!("{} ", *p);
p = &mut b;	// line 7
print!("{} ", *p);
*p += 1;	// line 9
print!("{} ", *p);
```

结果输出为：“10 11 20 21”。

注意第5行和第9行，`*p`是mutable的。所以`p`的类型不能是`&i32`，必须是`&mut i32`。这里的`p`有很多处`mut`，表示了不同的意义：第一个`mut`表示`p`是一个可变的变量，可以被修改，第二个`mut`作为了它的类型，表示它的引用类型可以修改。
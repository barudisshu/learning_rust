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
























































































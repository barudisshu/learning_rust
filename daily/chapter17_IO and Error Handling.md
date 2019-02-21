本章覆盖有：

- 如何通过命令行参数启动程序
- 如何给操作系统返回一个状态码
- 如何获取和设置进程环境变量
- 如何处理运行时错误
- 如何在控制台读取键盘输入信息并打印输出
- 原生类型如何转换为字符串
- 如何读写二进制文件
- 如何按行读取文本文件

## Command-Line Arguments

通过命令行输入的最基本形式是，

```rust
let command_line: std::env::Args = std::env::args();
for argument in command_line {
	println!("[{}]", argument);
}
```

该程序被编译用来创建一个文件，它通过命令行“./main first second”，它将输出：

```rust
[./main]
[first]
[second]
```

标准库中定义的`args`返回命令行参数的迭代。这种迭代器的类型是`Args`，它产生`String`值。第一个产生的值是程序名，它用路径访问。其它则是程序参数。

任何空白块会被移除，所以如果你想保留，可以用引号，`./main " first argument" "second argument "`，它将打印：

```rust
[./main]
[ first argument]
[second argument ]
```

该程序可以简化为，

```rust
for a in std::env::args() {
	println!("[{}]", a);
}
```

## Process Return Code

退出程序的最基本形式是返回码，

```rust
std::process::exit(107);
```

当调用`exit`函数是程序立即退出，并返回启动进程数字107。

在类Unix系统中，可以通过`echo $?`得到上一次输入内容，要在Windows，则输入`echo %errorlevel%`。


## Environment Variables

另外一种最常见的输入/输出的形式是环境变量，

```rust
for var in std::env::vars() {
	println!("[{}]=[{}]", var.0, var.1);
}
```

该程序将给逐行打印输出每个变量。然后，要读或写这些特殊环境变量，可以，

```rust
print!("[{:?}]", std::env::var("abcd"));
std::env::set_var("abcd", "This is the value");
print!(" [{:?}]", std::env::var("abcd"));
```

该程序可能输出：`[Err(NotPresent)] [Ok("This is the value")]`。首先是，环境变量`abcd`不存在，因此调用`var`函数时，返回`Result`类型的一个`Err`值，这种错误的特定类型是枚举`NotPresent`。

因为在当前程序中又给这个环境变量设置了值，即使用了`set_var`函数。所以，下一次获取时，得到内部变量`Ok`类型的值。

一段类似的程序如下，

```rust
print!("{}",
	if std::env::var("abcd").is_ok() {
		"Already defined"
	} else {
		"Undefined"
	}
);
std::env::set_var("abcd", "This is the value");
print!(", {}.", match std::env::var("abcd") {
	Ok(value) => value,
	Err(err) => format!("Still undefined: {}", err),
})
```

结果将打印：`Undefined, This is the value.`。


## Reading from the Console

对于面向命令行的编程，最典型地方式是从控制台输入，读取其输入信息。这种输入可能被另一段进程重定向为读取一个文件的方式。

```rust
let mut line = String::new();
println!("{:?}", std::io::stdin().read_line(&mut line));
println!("[{}]", line);
```

当该段程序启动时，它将等待你从键盘的输入，直到回车。例如，键入“Hello”，然后按回车，将会打印，

```
Ok(6)
[Hello
]
```

`stdin`函数为对当前进程的标准输入流返回一个句柄(handle)。这个句柄上，可以调用`read_line`函数。它等待标准输入流的行尾(end-of-line)或卷尾(end-of-file)字符的输入，并读取当前输入缓冲区提供的所有字符。这种读取可能失败，因为同时可能会有其它进程在读取。

如果读取成功，读取到的字符丢到一个字符串对象，指派给`line`变量，作为参数的形式，接收这个可变对象的引用，`read_line`函数返回一个`Ok`结果对象，该对象的数据是按字节的数字读取的。注意这个数字是6，所以“Hello"是5个字节，但还包含一个行尾(end-of-line)控制字符。实际上，当`line`变量被输出后，中括号另起一样输出，因为行尾(end-of-line)字符在最后一行被打印出来了。

如果`read_line`函数不能从标准输入流读取字符串，它返回一个`Err`结果对象，以及不会更改变量`line`的值。

让我们看看标准输入流读取几行时发生了什么，

```rust
let mut text = format!("First: ");
let inp = std::io::stdin();
inp.read_line(&mut text).unwrap();
text.push_str("Second: ");
inp.read_line(&mut text).unwrap();
println!("{}: {} bytes", text, text.len());
```

运行该程序，键入“eè€”，回车，键入“Hello”，回车，将打印，

```
First: eè€
Second: Hello
: 28 bytes
```

首先，注意到字符串输入了三行，因为它包含两个行尾字符串。另外，它包含7字节的ASCII字符串“First: ”，以及8字节的ASCII字符串“Second: ”。“Hello”也是一个ASCII字符串，包含5个字节。另外“eè€”字符串包含6个字节，所以我们一共有7+6+1+8+5+1=28字节。

然后，让我们看看`text`变量的文本如何构建起来的。注意`read_line`函数将键入行追加到参数指定的对象上，而不是重写它。`text`变量由“First: ”初始化。然后，在第三行，首次键入的行被追加到文本中。然后，在第四行，字符串字面量“Second：”追加到变量。最后，在第五行，第二次键入的行再次被追加的内容中。

其三，注意到当函数`read_line`从缓冲区读取时，它会清空缓冲区，这样再次读取缓冲区时不会重复读取缓冲区的内容。

其四，注意每次调用`read_line`后，后面都会调用`unwrap`，但它的返回值可以忽略。

所以这个调用可以省略，

```rust
let mut text = format!("First: ");
let inp = std::io::stdin();
inp.read_line(&mut text);
text.push_str("Second: ");
inp.read_line(&mut text).unwrap();
println!("{}: {} bytes", text, text.len());
```

然而，当这段程序被编译，编译输出，对于两处调用的`read_line`，会警告：`"unused `std::result::Result` which must be used".`。它意味着`read_line`返回个`Result`的值，以及这个值被忽略或不被使用。Rust中认为忽略`Result`类型的值是危险的，因为这种类型可能表示一个运行时错误，所以程序逻辑不会统计这种错误。这在生产环境是危险的，但它也不适用于调试代码，因为它隐藏了你需要寻找的错误。

因此，在调试代码时，最好总是在最后加上`.unwrap()`从句。

在生产环境代码，问题并不是那么简单。


## Proper Runtime Error Handling

在真实软件世界，大部分函数调用返回一个`Result`类型值。这类函数称为“不可靠，fallible”函数，即正常返回`Ok`，异常情况返回`Err`。

在C++、Java以及其他面向对象语言中，标准错误的处理技术基于“异常”这一概念，并有`throw`、`try`、`catch`这些关键字。在Rust中没有这些东西；所有错误处理基于`Result`类型，以及`match`语句匹配。

假设，典型地，你写一个函数`f`，要实现它的功能，会调用几个不可靠函数，`f1`、`f2`、`f3`和`f4`。这些函数可能会返回错误，或者成功结果。希望如果某个函数失败，应该立即将错误信息返回给`f`函数，若是成功，则传递给下一个函数继续执行。

一个可能的写法是，

```rust
fn f1(x: i32) -> Result<i32, String> {
	if x == 1 {
		Err(format!("Err. 1"))
	} else {
		Ok(x)
	}
}
fn f2(x: i32) -> Result<i32, String> {
	if x == 2 {
		Err(format!("Err. 2"))
	} else {
		Ok(x)
	}
}
fn f3(x: i32) -> Result<i32, String> {
	if x == 3 {
		Err(format!("Err. 3"))
	} else {
		Ok(x)
	}
}
fn f4(x: i32) -> Result<i32, String> {
	if x == 4 {
		Err(format!("Err. 4"))
	} else {
		Ok(x)
	}
}
fn f(x: i32) -> Result<i32, String> {
	match f1(x) {
		Ok(result) => {
			match f2(result) {
				Ok(result) => {
					match f3(result) {
						Ok(result) => f4(result),
						Err(err_msg) => Err(err_msg),
					}
				}
				Err(err_msg) => Err(err_msg),
			}
		Err(err_msg) => Err(err_msg),
		}
	}
}
match f(2) {
	Ok(y) => println!("{}", y),
	Err(e) => println!("Error: {}", e),
}
match f(4) {
	Ok(y) => println!("{}", y),
	Err(e) => println!("Error: {}", e),
}
match f(5) {
	Ok(y) => println!("{}", y),
	Err(e) => println!("Error: {}", e),
}
```

结果将打印：

```
Error: Err. 2
Error: Err. 4
5
```

这种写法肯定不方便，可以替换为行内写法，

```rust
fn f(x: i32) -> Result<i32, String> {
	let result1 = f1(x);
	if result1.is_err() { return result1; }
	let result2 = f2(result1.unwrap());
	if result2.is_err() { return result2; }
	let result3 = f3(result2.unwrap());
	if result3.is_err() { return result3; }
	f4(result3.unwrap())
}
```

这种写法是将结果写入临时变量中，结果值通过`is_err`函数检测。失败则返回，成功则`unwrap`出真实结果。

下面是另一种等价`f`的实现，

```rust
fn f(x: i32) -> Result<i32, String> {
	f4(f3(f2(f1(x)?)?)?)
}
```

这里的问号是一个特定的宏(macro)，诸如“`e?`”的表达式，如果“`e`”是泛型类型“`Result<T,E>`”，宏展开为表达式“`match e { Some(v) => v, _ => return e }`”；相反，如果“`e`”是泛型类型“`Option<T>`”，宏展开为表达式“`match e { Ok(v) => v, _ => return e }`”。换言之，这种宏语法将“`Some`”或“`Ok`”的参数，进行转换，或返回包含的函数的一个值。

它仅能作用于类型为“`Result<T,E>`”或“`Option<T>`”的表达式中，所以也仅能作用于有恰当返回值类型的函数内部。如果闭合函数返回值类型是“`Result<T1,E>`”，问号宏仅能作用于类型“`Result<T2,E>`”的表达式，其中“`T2`”可以和“`T1`”不同，但“`E`”必须相同；以及，如果闭合函数返回值类型是“`Option<T1>`”，问号宏仅能作用于类型“`Option<T2>`”的表达式。

因此，要构建一个稳健的错误处理模式。每个函数包含对一个不可靠(fallible)函数的调用，应该是一个fallible函数或使用“`match`”语句处理“`Result`”结果值。在最先的一种示例代码中，每个不可靠函数的调用，都应该用问号宏来传递错误条件。“`main`”函数不可能是falliable的，所以在调用链中，应该用“`match`”语句处理“`Result`”的值。


## Writing to the Console

我们一直用“`print`”或“`println`”宏来打印消息。然而，你也可以直接用库函数将信息输出到控制台。

```rust
use std::io::Write;
//ILLEGAL: std::io::stdout().write("Hi").unwrap();
//ILLEGAL: std::io::stdout().write(String::from("Hi")).unwrap();
std::io::stdout().write("Hello ".as_bytes()).unwrap();
std::io::stdout().write(String::from("world").as_bytes()).unwrap();
```

结果将打印：“`Hello world`”。

“`stdout`”标准库函数返回一个句柄处理当前进程的标准输出流，“`write`”可以实现。

“`write`”函数不能直接打印静态字符串，也不能打印动态字符串，当然数字、常见组合对象也不能。

































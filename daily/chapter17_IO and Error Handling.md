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

































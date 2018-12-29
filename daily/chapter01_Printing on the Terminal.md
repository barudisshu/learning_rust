## Hello, World!

学习如何在终端输出：

```rust
fn main() {
    print!("Hello, world!");
}
```

运行将输出 `"Hello, world!"`，上述包含8个语法，

- `print`： Rust标准库定义宏
- `!`： 它表示前面的名字是一个宏(macro)，没有该标志，`print`会标识为一个函数(function)。在Rust标准库中没有该函数，因此会得到一个编译错误。一个宏(macro)是和函数类似的 - 它是某个以名字关联的Rust代码。要使用这个宏名，你将该代码嵌入到该指示的位置。
- `(`： 宏参数列表的开始。
- `"`： 字符串字面量开始。
- `Hello, world!`： 字符串字面量的内容。
- `"`： 字符串字面量的结束。
- `)`： 宏参数列表的结束。
- `;`： 语法结束。

字符串字面量(literal string)中的，“string” 表示 “有限字符序列，可能包含空格和符号”。字面量("literal")表示 “源代码直接标识的值”。字符串(literal string) 就是 “源代码中具体指示的有限序列字符”。

`print`宏在接收到参数时，插入代码并打印到终端。

Rust区分大小写。

## Printing Combinations of Literal Strings

与其使用单一的字符字面量，你可以打印多个。

```rust
print!("{}, {}!", "Hello", "world");
```
在这里，`{}`称为占位符。它跟C语言类似。

```c
printf("%s, %s!", "Hello", "world");
```

占位符的个数必须跟参数对应，否则出现编译错误。相反，C语言并不会提升为编译错误，而是让编译的程序崩溃。

## Printing Serveral Lines of Text

一个语句可以打印多行

```rust
print!("First line\nSecond line\nThird line\n");
```

输出：

First line
Second line
Third line

可以使用另外一个宏，`println`，它表示换行输出。

```rust
println!("text of the line");
```

## Printing Integer Numbers

```rust
print!("My number: {}", 140);
```
下面输出和上述等价。因为 000140 将被转换为二进制格式，0被忽略。

```rust
print!("My number: {}", 000140);
```

## Command-Line Script

`rustc`命令有一个缺陷：它按顺序打印出所有错误。当你的代码包含很多语法错误时，编译器会打印大量的错误信息，你需要回滚到最先开始的那段。

面对这种情况，可以使用命令行脚本的方式：

在Linux系统，你可以：

```bash
clear
rustc $* --color always 2>&1 | more
```

在Windows中，你可以将下面三行添加到`.BAT`文件：

```bat
@echo off
cls
rustc %* --color always 2>&1 | more
```

保存为`rs` (windows 为 `rs.bat`)，编译文件 `main.rs`时：

```bash
rs main.rs
```

它将错误信息，通过管道 `|` 交由命令 `more`处理。最终变为一个Linux文本处理。

注意，`rs`命令会跟系统(freeBSD Unix)中的 `reshape a data array`命令冲突。建议改名。

## Comments

Rust 的注释包含有：

- 单行注释： 以 `//`开头
- 多行注释： 以 `/*` 开始，以 `*/`结束。

Rust的注释和C类型，区别是Rust的注释可以嵌套。

```rust
/* This is /* a valid*/
comment, event /* if /* it contains
comments */ inside */itself. */

/* This */ instead is not allowed in Rust,
while in C is tolerated (but it may generate a warning).*/
```





























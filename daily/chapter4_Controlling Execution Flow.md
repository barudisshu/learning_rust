本章覆盖有：

- if语句
- while语句
- for语句
- 变量的有效性范围

## Conditional Statements (if)

```rust
let n = 4;
if n > 0 { print!("posistive");}
```

和其它语言不同的是：

- 条件部分必须是Boolean类型。诸如`if 4 { print!("four"); }`是不允许的。
- 不推荐用括号将条件部分括起来，编译器会报一条警告。
- 条件语句后面跟着 block是必须的。诸如 `if 4 > 0 print!("four");` 或 `if (4 > 0) print!("four");`是不允许的。

```rust
let n = 0;
if n > 0 {
    print!("number is");
    print!(" positive");
} else {
    print!("non positive");
}

条件表达式支持嵌套。

## Conditional Expressions

```rust
let n = 4;
print!("{}", 
    if n > 1000 {
        "big"
    } else if n > 0 {
        "small"
    } else if n < 0 {
        "negative"
    } else {
        "neither positive nor negative"
    }
);
```

条件表达式可用于语句中。类似于C-like语言的`?:`

```c
#include<stdio.h>
int main(int argc, char **argv) {
    int n = 4;
    printf("%s", 
        n > 1000 ?
            "big" :
        n > 0 ?
            "small":
        n < 0 ?
            "negative":
            "neither positive nor negative");
    )
}
```

## Conditioned Loops (while)

打印1到10的整数，平方的方式，你可以：

```rust
let mut i = 1;
while i <= 10 {
    print!("{} ", i * i);
    i += 1;
}
```
可以使用`continue`和`break`，下面写法等价：

```rust
let mut i = 0;
while i < 50 {
    i += 1;
    if i % 3 == 0 {continue; }
    if i * i > 400 {break;}
    print!("{} ", i * i)
}
```

## Infinite Loops(loop)

```rust
let mut i = 1;
while true {
    let ii = i * i;
    if ii >= 200 {break;}
    print!("{} ", ii);
    i += 1;
}
```

当然编译器会抱怨说："denote infinite loops with `loop {... } `"。

```rust
let mut i = 1;
loop {
    let ii = i * i;
    if ii >= 200 {break;}
    print!("{} ", ii);
    i += 1;
}
```

## Couting Loops (for)

```rust
for i in 1..11 {
    print!("{} ", i * i);
}
```

如果循环部分中有变量同名，循环语句内的变量被忽略：

```rust
let index = 8;
for index in 0..4 {print!("{} "， index); }
print!(":{}", index);
```

结果将输出： "0 1 2 3 :8"。

让我们看看下面这个例子：

```rust
let mut limit = 4;
for i in 1..limit {
    limit -= 1;
    print!("{} ", i);
}
print!(":{}", limit);
```

## Variables Scopes

变量范围推荐使用语句块的方式。

```rust
print!("1");
 {
     print!("2");
     print!("3");
     {
         print!("4");
         {
             print!("5");
             {{}}
             print!("6");
         }
     }
     print!("7");
 }
 ```

 变量的范围是有意义的，如果你执行下面的代码，它实际上会得出编译错误：

 ```rust
 { let i = 10;}
 print!("{} ", i);
 ```

 对于条件语句`if`，`while`和`for`，变量的范围仍然支持：

 ```rust
 let mut _i = 1;
 if true { let _i = 2; }
 print!("{} ", _i);

 while _i > 0 { _i -= 1; let _i = 5; }
 print!("{} ", _i);
 ```

 
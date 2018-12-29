本章覆盖有：

- 如何定义相同类型的序列，包含固定长度(arrays)和可变长度(vectors)
- 如何初始化arrays和vector
- 如何向arrays和vector写和读
- 如何向一个vector添加条目或删除条目
- 如何创建多维数组
- 如何打印或复制整个数组或向量


## Arrays

存储字符串变量：

```rust
let x = ["English", "This", "sentence", "a", "in", "is"];
print!("{} {} {} {} {} {}", x[1], x[5], x[3], x[2], x[4], x[0]);
```

获取数组的元素个数，你可以：

```rust
let a = [true, false];
let b = [1, 2, 3, 4, 5];
print!("{}, {}.", a.len(), b.len());
```

数组中的元素要求类型一致，像下面的写法是错误的：

```rust
let x = ["This", 4];
or
let x = [4, 5.]:
```

下面的写法也是错误的：

```rust
let mut x = ["a"];
x[0] = 3;
x[-1] = "b";
x[0.] = "b";
x[false] = "b";
x["0"] = "b";
```

第二条语句是错误的，因为声明的类型不一致。

下面的情况是有效的

```rust
let x = ["a"];
let _y = x[1];
```

## Mutable Arrays

```rust
let mut x = ["This", "is", "a", "sentence"];
x[2] = "a nice";
print!("{} {} {} {}.", x[0], x[1], x[2], x[3]);
```

通过`mut`关键字声明数组可变。

- `x`变量是可变的。
- 新赋值的类型和`x`相同
- 下标是非负整数。

## Arrays of Specified Size

```rust
let mut x = [4.; 5000];
x[2000] = 3.14;
print!("{}, {}", x[1000], x[2000]);
```

这里指定了变量数组的大小为5000，所有元素的值都是4.

练习，运用初始化大小的方式，计算斐波那契数列

```rust
let mut fib = [1; 15];
for i in 2..fib.len() {
    fib[i] = fib[i - 2] + fib[i - 1];
}
for i in 0..fib.len() {
    print!("{}, ", fib[i]);
}
```

## Multidimensional Arrays

```rust
let mut x = [[[[23; 4]; 6]; 8]; 15];
x[14][7][5][3] = 56;
print!("{}, {}", x[0][0][0][0], x[14][7][5][3]);
```
这里将打印输出：“23，56”。

我们看看多维数组的长度，

```rust
let mut x = [[[[0; 4]; 6]; 8]; 15];
 print!("{}, {}, {}, {}.", x.len(), x[0].len(), x[0][0].len(), x[0][0][0].len());
```

数组的最大限制是必须在编译其指定长度。

```rust
let length = 6;
let arr = [0; length];
```

该代码会提示“attempt to use a non-constant value in a constant”，实际上表达式`lenght`是一个变量，从概念上来说它不是一个编译时常量，即使它是可变的，即使它被初始化为一个常量。

## Vectors

Rust标准库提供了Vec类型，代表`vector`。

```rust
let x = vec!["This", "is"];
print!("{} {}. Length: {}.", x[0], x[1], x.len());
```

结果将输出“This is. Lenght: 2.”


Vec类型的长度是不固定的，因此，

```rust
let mut x = vec!["This", "is"];
print!("{}", x.len());
x.push("sentence");
print!(" {}", x.len());
x[0] = "That";
for i in 0..x.len() {
    print!(" {}", x[i]);
}
```

输出为：“2 3 4 That is a sentence."

 和数组array不同，vector可以定义长度，或者在执行时改变它的长度。因为vector的长度在运行时是个变量，它不属于类型。因为Rust中的所有类型在编译期都是定义的：

 ```rust
     let length = 5000;
    let mut y = vec![4.; length];
    y[6] = 3.14;
    y.push(4.89);
    print!("{}, {}, {}", y[6], y[4999], y[5000]);
```

因为Rust要求类型在运行时是预先定义好的，所以，下面的不是合法的代码：

```rust
let mut _x = vec!["a", "b", "c"];
_x = vec![15, 16, 17];
```

## Other Operations on Vectors

标准库中为vector提供了许多操作，下面常见一些例子：

```rust
let mut x= vec!["This", "is", "a", "sentence"];
x.insert(1, "line");
x.insert(2, "contains");
x.remove(3);
x.push("about Rust");
x.pop();
for i in 0..x.len() {print!("{} ", x[i]); }
```


结果将输出"This line contains a sentence。"

上述显示的`vector.push(item);`，实际上和`insert(vector.len(), item);`等价；`vector.pop()`和`vector.remove(vector.len() - 1)`等价。

同样，基于类型安全，下面的写法是错误的。

```rust
let mut _x = vec!["This", "is", "a", "sentence"];
_x.insert("line", 1);
```

## Empty Arrays and Vectors

Rust中不支持没有预先定义的类型操作，假如有一个函数`f`接收两个参数,

```rust
f(["help", "debug"], vec![0, 4, 15]);
```

当传递空array和vector时，编译器将报错，因为它不能决定数组或向量的类型。那怎么定义一个空数组和向量。

```rust
let _a = [];
```

这样写编译器会报"type annotations needed"和 "cannot infer type."

但是这样写却是可以的

```rust
let _a = [""; 0];
```

类型地，向量

```rust
let _a = vec![true; 0];
let _b = vec![false; 0];
```

因此，我们的函数调用可以表示为这样：

```rust
f([""; 0], vec![0; 0]);
```


## Debug Print

如何打印数组和向量呢？因为`print`和`println`是个宏调用，它可以接收各种类型地输出。然而，对于数组或向量却不适用：

```rust
print!("{} {}", [1, 2, 3], vec![4, 5]);
```

因为没有定义适当的输出格式，因此要改为：

```rust
print!("{:?} {:?}", [1, 2, 3], vec![4, 5]);
```

将会输出： "[1, 2, 3] [4, 5]"。

`:?` 表示的是`print`(`println`)宏将生成一个debug format的响应数据。 因此不管是任何变量，你都可以用`{:?}`处理。


## Copying Arrays and Vectors


如果你想要复制整个数组或向量，你不需要逐个浏览每个元素：

```rust
let mut a1 = [4, 56, -2];
let a2 = [7, 81, 12500];
print!("{:?} ", a1);
a1 = a2;
print!("{:?}", a1);
```

向量也是一样：


```rust
let mut a1 = vec![4, 56, -2];
let a2 = vec![7, 81, 12500];
print!("{:?} ", a1);
a1 = a2;
print!("{:?}", a1);
```


需要注意的是，Array的复制必须指定长度和类型，如下则报错：

```rust
let mut a1 = [4, 56, -2];
let a2 = [7, 81];
print!("{:?} ", a1);
a1 = a2;
print!("{:?}", a1);
```


































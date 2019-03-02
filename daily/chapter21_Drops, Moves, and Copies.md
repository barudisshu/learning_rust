本章覆盖有：

- 为什么决定性的(deterministic)、隐式(implicit)的对象销毁是Rust的一大亮点
- 对象所有者(ownership)的概念
- 为什么自定义销毁可能有用，怎么创建
- 三种赋值语义：共享(share)、拷贝(copy)、移动(move)
- 为什么隐式共享对软件正确性是糟糕的
- 为什么对象的移动(move semantics)比起拷贝(copy semantics)可能有更好的性能
- 为什么某些类型需要拷贝(copy semantics)，某些不需要，怎么区分
- 为什么某些类型需要是不可复制的(non-cloneable)，怎么区分

## Deterministic Destruction

前面，我们看到有几种内存分配对象的方式，这些分配都是在stack和heap发生：

- 临时表达式，分配在stack；
- 变量(包括数组)，分配在stack；
- 函数和闭包的参数，分配在stack；
- `Box`对象，引用分配在stack，引用的对象分配在heap；
- 动态字符串和集合(包括vector）,header分配在stack，数据分配在heap。

对象“__真实__”的瞬时分配是很难预测的，因为它取决于编译器优化。因此，我们考虑“__概念__”上的瞬时分配情况。

概念上，当对应表达式第一次出现在代码时发生stack分配，因此：

- 临时表达式，变量，数组在它们第一次出现在代码时被分配；
- 函数和闭包的参数，在函数/闭包被调用时被分配；
- `Box`对象，动态字符串，集合header，在代码第一出现时被分配。

heap的当需要这些数据时，进行heap分配。因此：

- `Box`对象的分配，由`Box::new`函数触发；
- 动态字符串的字符分配，在字符被添加到该字符串时触发；
- 集合内容的分配，出现在有数据被添加到集合时。

上面这些跟大多数编程语言没不同之处。那么数据的销毁在什么时候发生？

概念上，在Rust中，当这些数据不再可被访问时，自动销毁。因此：

- 临时表达式被回收，当它在语句的结束位置(即，在下一个`;`位置或当前scope的结束位置)；
- 变量(包括数组)被回收，当它在scope的声明结束；
- 函数/闭包的参数的回收，出现在函数/闭包体结束；
- `Box`对象的回收，在当前scope的声明结束；
- 动态字符串的字符被回收，出现在从字符串中删除该字符时，或者整个字符串删除时；
- 集合中的条目被回收，出现在从集合中删除该条目时，或者整个集合被删除时。

这一概念使得Rust和大部分语言区分开来。任何语言都有临时对象或栈分配(stack-allocated)对象，这种对象是自动回收的。但堆分配(heap-allocated)对象的回收，不同语言各不相同。

在某些语言中，诸如Pascal，C，C++，heap上的对象通常仅能显式地调用类似`free`或`delete`这些函数进行回收。另一些语言，诸如Java，JavaScript，C+，Python，堆上的对象不可访问时，并没有立即进行回收，而是有一个定期的行程，用来查找heap不可达对象，并回收这些对象。这种机制称为“垃圾回收”，因为它类似城市的清理系统：它定期清理城镇，当有垃圾堆积。

因此，在C++和类似语言中，heap回收既是`决定性的，deterministic`，也是`显式的，explicit`。决定性的，因为它在源代码的定义位置，以及是显式的，因为它要求程序员编写指定的回收语句。决定性的好处在于，可能有更好的性能，程序员可能更好地控制。但显式的却不好，因为不能避免出现错误的回收，丑陋的bug结果。

相反，在Java和类似语言中，heap回收既是`非决定性的，non-deterministic`，也是`隐式的，implicit`。非决定性的，因为它出现未知的执行瞬时，以及是隐式的，因为它不需要指定回收语句。非决定性是糟糕的，但隐式的美好的。

区别于这两种技术，在Rust中，通常，heap的回收既是`决定性的，deterministic`，也是`隐式的，implicit`，这是Rust比起其它语言更大的优势。

这种可能性的实现，是因为遵循了基于“`所有者，ownership`”的概念，

## Ownership

首先介绍术语“**`to own`**”。在计算机科学中，对于一个标识符或一个对象A，拥有(to own)对象B，意味着A可以对B进行回收，它有两个意义：

- Only A can deallocate B.
- When A becomes unreachable, A must deallocate B.

在Rust中没有显示的回收机制，因此这种定义可以复述为“A owns B means that B is deallocated  when and only when A becomes unreachable”。

```rust
let mut a = 3;
a = 4;
let b = vec![11, 22, 33, 44, 55];
```

该程序，变量`a`拥有一个对象初始化值3，因为当`a`离开了它的scope，变成不可访问，该初始化值为3的对象被回收。我们也可以这样说“`a`是一个对象的所有者，它由值3初始化”。尽管，我们不能说“`a`拥有3”，因为3是一个值，不是对象；仅对象才能被拥有(owned)。在内存中，有很多对象值是3的，`a`拥有其中一个。在第二条语句中，该对象的值变为4；但它的拥有者没有变。

在最后一条语句中，`b`由一个5个元素的vector初始化。这个vector由一个头(header)和一个数据缓冲区(data buffer)；header的实现由一个结构体三个filed表示：一个执行data buffer的指针，两个数(capacity、len)；数据缓冲区包含5个条目，另外可能有额外的空间。这里我们可以说“`b`拥有vector的header，以及一个指针，该指针包含数据缓冲区的拥有者头(header)”。实际上，当`b`离开了它的scope，vector的头被回收；当vector的header被回收后，它包含的指针不可访问；当前的vector表示为一个空，因此缓冲区的条目被回收。

不是每个引用所有者是一个对象，

```rust
let a = 3;
{
	let a_ref = &a;
}
print!("{}", a);
```

这里的`a_ref`变量拥有一个引用，但这个引用什么都没有。实际上，在这个嵌入块的结束位置，`a_ref`变量离开了它的scope，该引用被回收，但引用对象，即这个包含值3的对象，没有立即被回收，因为它必须在最后一条语句打印输出。

为了确保每个对象自动回收，Rust中有一个简单规则，在每个执行的瞬时，每个对象有且仅能有一个“owner”。当这个owner被回收，该对象自身被回收。如果一个对象有几个owner，这个对象可能被回收几次，这是不被允许的。如果对象没有owner，该对象从不被回收，这种情况叫做“内存泄露，memory leak”。


## Destructors

我们看到对象的创建有两步：给对象分配内存，初始化这个内存空间的值。对于复杂对象，初始化是如此复杂，通常需要使用一个函数实现。这个函数叫“构造器”，用来“构造”一个新的对象。

我们刚看到，当一个对象被回收，会发生一些复杂情况。如果在heap中一个对象引用另一个对象，一个级联(cascade)的回收可能会发生。因此，对象的“销毁”可能需要由一个函数处理，称作“destructor，焚烧炉，销毁装置”。

通常销毁器是属于标准库的一部分，但有时你可能需要在对象回收时做一些cleanup code操作，所以你需要写一个destructor。

```rust
struct CommunicationChannel {
    address: String,
    port: u16,
}
impl Drop for CommunicationChannel {
    fn drop(&mut self) {
        println!("Closing port {}:{}", self.address, self.port);
    }
}
impl CommunicationChannel {
    fn create(address: &str, port: u16) -> CommunicationChannel {
        println!("Operning port {}:{}", address, port);
        CommunicationChannel {
            address: address.to_string(),
            port: port,
        }
    }
    fn send(&self, msg: &str) {
        println!("Sent to {}:{} the message '{}'", self.address, self.port, msg);
    }
}
let channel = CommunicationChannel::create("usb4", 879);
channel.send("Message 1");
{
	let channel = CommunicationChannel::create("eth1", 12000);
	channel.send("Message 2");
}
channel.send("Message 3");
```

该程序将打印：

```
Operning port usb4:879
Sent to usb4:879 the message 'Message 1'
Operning port eth1:12000
Sent to eth1:12000 the message 'Message 2'
Closing port eth1:12000
Sent to usb4:879 the message 'Message 3'
Closing port usb4:879
```




















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

第二条语句声明新的类型CommunicationChannel用于实现`Drop`。这个trait有一个特有的方法`drop`，它会在对象被回收时自动被调用，因此它是一个“destructor”。通常，给一个类型创建一个销毁器，为该类型实现这个`Drop`trait即可。因为任何没有被定义的trait，不能在程序外部实现。

第三条语句是一个语句块，为结构体定义了两个方法：`create`构造器，`send`方法。

最后是应用代码。创建了一个CommunicationChannel，这个创建会打印一行内容。接着调用了send方法，打印第二行内容。接着是内嵌语句块，创建了另一个channel，打印第三、四行内容。

嵌套语句块内的变量名跟存在的变量名相同，这会导致变量投影(shadow)。

接着嵌套语句结束。这发生率内部变量被销毁，因此它的`drop`方法被调用，于是打印第五行。

现在，嵌套语句块结束后，第一个变量再次可见。`send`方法再次调用，打印一行。

最后，变量被销毁，打印最后一行。

在Rust中，内存早已由语言和标准库释放掉了，因此没有必要像C语言那样调用`free`函数，或像C++那样调用`delete`。但其他资源不会自动释放。因此销毁器(destructor)对于那些副作用的实现非常有用：诸如文件处理，通讯处理，GUI窗口，图形资源等，标准库中早已为资源的处理的任何类型提供了`Drop`实现。

销毁器可以更好地理解内存的管理。

```rust
struct S ( i32 );
impl Drop for S {
	fn drop(&mut self) {
		println!("Dropped {}", self.0);
	}
}
let _a = S (1);
let _b = S (2);
let _c = S (3);
{
	let _d = S (4);
	let _e = S (5);
	let _f = S (6);
	println!("INNER");
}
println!("OUTER");
```

结果打印：

```
INNER
Dropped 6
Dropped 5
Dropped 4
OUTER
Dropped 3
Dropped 2
Dropped 1
```

注意到对象的销毁的顺序跟构造顺序相反，

```rust
struct S ( i32 );
impl Drop for S {
	fn drop(&mut self) {
		println!("Dropped {}", self.0);
	}
}
let _ = S (1);
let _ = S (2);
let _ = S (3);
{
	let _ = S (4);
	let _ = S (5);
	let _ = S (6);
	println!("INNER");
}
println!("OUTER");
```

结果将打印：

```
Dropped 1
Dropped 2
Dropped 3
Dropped 4
Dropped 5
Dropped 6
INNER
OUTER
```

因为只有占位符，因此所有对象都是临时的。临时对象在它们语句结束位置就销毁了，即统计到分号(`;`)立即销毁。

上面的程序和下面的是等价的，

```rust
struct S ( i32 );
impl Drop for S {
	fn drop(&mut self) {
		println!("Dropped {}", self.0);
	}
}
s (1);
S (2);
S (3);
{
	S (4);
	S (5);
	S (6);
	println!("INNER");
}
println!("OUTER");
```

## Assignment Semantics

下面程序做了什么？

```rust
let v1 = vec![11, 22, 33];
let v2 = v1;
```

概念上，

首先，`v1`的标头(header)被分配到了栈。然后，`v1`的内容，会在堆为该内容分配一个缓冲区，`v1`的元素之被拷贝到这个缓冲区。然后标头(header)被初始化，作为引用指向新分配的堆缓冲。

然后，`v2`的标头被分配在栈。接着，用`v1`的值初始化`v2`。但，这是如何实现的？

通常至少有三种方式实现这种操作：

- **`Share semantics`**：`v1`的标头被拷贝到`v2`的标头，其它不发生任何操作。因此，可以用`v1`，也可以用`v2`，它们都同时指向相同的堆缓冲区；因此，它们指向同样的内容，不是相等的，而是唯一的。这种术语的常见于垃圾回收语言，比如Java。

- **`Copy semantics`**：分配另外的堆缓冲。它和`v1`使用的缓冲区有同样的大小，并将先存的缓冲区内容拷贝到新的缓冲区。然后`v2`的标头被初始化指向新分配的缓冲区。因此，两个变量指向两个不同的缓冲区并且初始化的内容相同。这种实现，是C++的默认机制。

- **`Move semantics`**：`v1`的标头被拷贝到`v2`的标头，其它不发生任何操作。因此，`v2`可以使用，它的标头指向原先`v1`分配的堆缓冲区，但`v1`不能再被使用。这种实现，是Rust的默认机制。

```rust
let v1 = vec![11, 22, 33];
let v2 = v1;
print!("{}", v1.len());
```

该代码产生编译错误：“use of moved value: `v1`”。当`v1`的值指派给`v2`是，变量`v1`终止并退出。再次使用是不被编译器允许的。

先看看，为什么Rust不实现**share semantics**。首先，如果变量是可变的，这种语义(semnatics)会有几分迷惑。在共享术语(share semantics)，通过一个变量更改一个条目，这个条目也可以被其它变量更改和访问。这不是直觉，可能是bug的根源。因此，共享术语(share senantics)仅在只读数据(read-only data)能被接收。

但这里有个大问题，对于内存回收。如果使用共享术语，`v1`和`v2`都将会拥有同一个单一的数据缓冲区，因此当他们被回收时，同样的堆缓冲区会被回收两次。一个缓冲区不能被分配两次，而不导致内存损耗以及引起程序崩溃(program malfunction)。要解决这个问题，语言本身需要在scope结束时不对变量使用的内进行回收，而是凭借GC处理。

相反，拷贝语义(copy semantics)和移动语义(move semantics)都是正确的。实际上，Rust规则上把回收看做是任何对象必须有且仅有一个owner。当使用拷贝语义时，原来的vector缓冲区还是原来的owner，即变量`v1`的标头，新创建的缓冲区，有新的owner引用，即`v2`的标头。另一方面，当使用移动语义时，原来单一vector缓冲区更改它的owner：分配之前，缓冲区的所有者是`v1`的标头reference，分配之后，所有者更改为`v2`的标头reference。在分配之前，`v2`的标头并不存在，分配之后，`v1`的标头不再存在。

那为什么Rust不实现拷贝语义(copy semantics)？

实际上，某些情况下，使用拷贝语义更合适，另一些情况下，使用移动语义更适合。甚至C++，从2011年开始，允许同时拷贝语义和移动语义。

```cpp
#include <iostream>
#include <vector>
int main() {
	auto v1 = std::vector<int> {11, 22, 33};
	const auto v2 = v1;
	const auto v3 = move(v1);
	std::count << v1.size() << " " << v2.size() << " " << v3.size();
}
```

这段C++程序会打印：0 3 3。`v1`首先被拷贝到`v2`，然后移动到`v3`。C++标准函数`move`会清空vector但不会让其undefined。因此，在最后，`v2`有三个元素的拷贝，`v3`就是原来的`v1`，`v1`变为空。

Rust中也允许拷贝语义和移动语义。

```rust
let v1 = vec![11, 22, 33];
let v2 = v1.clone();
let v3 = v1;
// ILLEGAL: print!("{} ", v1.len());
print!("{} {}", v2.len(), v3.len());
```

将会打印：3 3。

这段程序和C++类似，但不能再访问`v1`了，因为它被移动了。因为C++的默认语义是拷贝语义(copy semantics)，所以需要调用`move`标准函数来进行对象移动；而Rust的默认语义是移动语义(move semantics)，所以需要调用标准函数`clone`进行拷贝。

另外，`v1`虽已被移动，但仍然可访问，只不过内容为空，Rust中被移动的变量不可再被访问。

## Copying vs. Moving Performance

Rust偏向于移动语义的选择是从性能方面考量的。对于拥有堆缓冲区的对象，比如vector，移动比拷贝要快，因为移动的仅是header，然而如果是拷贝一个vector，要求分配和初始化一个可能的堆缓冲区，它最终会被回收。

在C++中，被移动的对象意味着不在被使用了，但语言为了对遗留代码做后向兼容(backward-compatible)，被移动的对象仍然可以访问，这可能会给开发者再次使用该对象的机会。另外，清空一个被移动的vector有较小的消耗，即当一个vector被销毁，会检测它是否为空，这也有较小消耗。Rust被设计避免手动移动对象，因此不会有不正当的移动vector，因为编译器知道vector被移动了，可以产生更好的代码。

我们可以通过下面代码度量性能的影响，这并不简单，因为编译优化器会移除loop内的工作。

下面代码使用了拷贝语义。

```rust
use std::time::Instant;
fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
    let t = t2 - t1;
    t.as_secs() as f64 * 1000. + t.subsec_nanos() as f64 / 1e6
}
const N_ITER: usize = 100_000_000;
let start_time = Instant::now();
for i in 0..N_ITER {
	let v1 = vec![11, 22];
	let mut v2 = v1.clone();    // Copy semantics is used
	v2.push(i);
	if v1[1] + v2[2] == v2[0] {
		print!("Error");
	}
}
let finish_time = Instant::now();
print!("{} ns per iteration\n", elapsed_ms(start_time, finish_time) * 1e6 / N_ITER as f64);
```

下面是C++的等价实现，

```cpp
#include <iostream>
#include <vector>
#include <ctime>
int main() {
	const int n_iter = 100000000;
	auto start_time = clock();
	for (int i = 0; i < n_iter; ++i) {
		auto v1 = std::vector<int> { 11, 22 };
		auto v2 = v1;	// Copy semantics is used
		v2.push_back(i);
		if (v2[1] + v2[2] = v2[0]) { std::cout << "Error"; }
	}
	auto finish_time = clock();
	std::cout << (finish_time - start_time) * 1.e9 / CLOCKS_PER_SEC / n_iter << " ns per iteration\n";
}
```

下面Rust程序使用了移动术语，

```rust
use std::time::Instant;
fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
    let t = t2 - t1;
    t.as_secs() as f64 * 1000. + t.subsec_nanos() as f64 / 1e6
}
const N_ITER: usize = 100_000_000;
let start_time = Instant::now();
for i in 0..N_ITER {
	let v1 = vec![11, 22];
	let mut v2 = v1;    // Move semantics is used
	v2.push(i);
	if v1[1] + v2[2] == v2[0] {
		print!("Error");
	}
}
let finish_time = Instant::now();
print!("{} ns per iteration\n", elapsed_ms(start_time, finish_time) * 1e6 / N_ITER as f64);
```

C++的等价实现为，

```cpp
#include <iostream>
#include <vector>
#include <ctime>
int main() {
	const int n_iter = 100000000;
	auto start_time = clock();
	for (int i = 0; i < n_iter; ++i) {
		auto v1 = std::vector<int> { 11, 22 };
		auto v2 = move(v1);	// Move semantics is used
		v2.push_back(i);
		if (v2[1] + v2[2] = v2[0]) { std::cout << "Error"; }
	}
	auto finish_time = clock();
	std::cout << (finish_time - start_time) * 1.e9 / CLOCKS_PER_SEC / n_iter << " ns per iteration\n";
}
```

下面是编译优化后的大致的时间损耗，

|                 |  Rust  |  C++  |
|-----------------|--------|-------|
| Copy semantics  |  157   |  87   |
| Move semantics  |  67    |  67   |

不管是在C++还是Rust中，移动术语都要比拷贝术语要快。在这方面，移动术语两者都差不多，拷贝术语方面C++要比Rust好很多。

## Moving and Destroying Objects

所有这些概念不单是对vector，任何有缓冲区引用的对象都适用，譬如`String`或`Box`。

```rust
let s1 = "abcd".to_string();
let s2 = s1.clone();
let s3 = s1;
// ILLEGAL: print!("{} ", s1.len());
print!("{} {}", s2.len(), s3.len());
```

这和C++类似，

```cpp
#include <iostream>
#include <string>
int main() {
	auto s1 = std::string { "abcd" };
	const auto s2 = s1;
	const auto s3 = move(s1);
	std::cout << s1.size() << " " << s2.size() << " " << s3.size();
}
```

前面说过，被移动的对象不能再访问，因此`s1`访问时会导致编译错误；而对于C++，原来的`s1`会置为空，所以会输出0 4 4。

对于`Box`类型，

```rust
let i1 = Box::new(12345i16);
let i2 = i1.clone();
let i3 = i1;
// ILLEGAL: print!("{} ", i1);
print!("{} {}", i2, i3);
```

对应的C++，

```cpp
#include <iostream>
#include <memory>
int main() {
	auto i1 = std::unique_ptr<short> {
		new short(12345)
	};
	const auto i2 = std::unique_ptr<short> {
		new short(*i1)
	};
	const auto i3 = move(i1);
	std::cout << (bool)i1 << " " << (bool)i2 << " " << (bool)i3 << " " << *i2 << " " << *i3;
}
```

Rust程序会输出12345 12345，任何访问`i1`都会导致编译错误。C++会输出0 1 1 12345 12345。因为仅`i1`是null，它被移动到`i3`了。

仅当他们被用于初始化一个变量，给一个有值的变量重新赋值，对象不被移动，

```rust
let v1 = vec![false; 3];
let mut v2 = vec![false; 2];
v2 = v1;
v1;
```

以及给函数参数传递值时，

```rust
fn f(v2: Vec<bool>) {}
let v1 = vec![false; 3];
f(v1);
v1;
```

以及指派的对象在当前没有实际堆分配时，

```rust
let v1 = vec![false; 0];
let mut v2 = vec![false; 0];
v2 = v1;
v1;
```

编译上面任何三条程序，最后一个语句都会导致“use of a moved value”的编译错误，

尤其是，在程序最后，`v1`被移动到`v2`，即使他们都为空，因此它们没有堆空间被使用。为什么？因为移动规则由编译器提供，因此它在运行期必须是独立的内容。

下面的代码，最后一行也会导致编译错误，

```rust
struct S {}
let s1 = S {};
let s2 = s1;
s1;
```

编译器可以确切知道这个引用不会指向heap，但仍然编译报错。为什么Rust不为该类型使用拷贝语义？

它的基本原理是这样的。用户定义的类型`S`现在没有引用内存，但在将来软件维护的时候，指向堆的引用可能会被添加，即`S`可能会被作为字段(field)等。因此，如果为`S`实现了拷贝语义，当程序源被更改，一个`String`、`Box`、或集合，直接或间接地添加到`S`，会导致很多错误。因此，作为规则，最后保留移动语义。

## Need for Copy Semantics

我们看到很多类型使用移动语义，包括vector，动态字符串，boxes，结构体... 下面的程序是合法的，

```rust
let i1 = 123;
let _i2 = i1;
let s1 = "abc";
let _s2 = s1;
let r1 = &i1;
let _r2 = r1;
print!("{} {} {}", i1, s1, r1);
```





































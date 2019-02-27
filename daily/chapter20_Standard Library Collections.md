本章覆盖有：

- 如何度量运行部分代码所花费的时间
- 基于性能因素，如何使用哪种类型的集合
- 集合中的各种操作中哪个更好：顺序扫描(sequential scan)、两端插入和删除(insertion and removal of items at both ends)、删除最大项(removal of the largest item)、搜索(search)、键搜索(search by key)、保持顺序(keeping items sorted)

## Collections

数组、向量、结构体、元组结构体、元组、枚举属于数据类型，它们的对象可能包含几个其它对象。然而，对于结构体、元组-结构体、元组、枚举，每个包含的对象，需要指定声明类型和构造类型，它们不能存储多个对象。相反数组和向量的数据类型会包含多个对象，这一类对象称为“集合”。

数组和向量是最佳集合：内存高效，读取速度快，CPU缓存高效，能通过索引快速访问内部元素。当然，在某些情况不尽是高效的，这种情况下需要使用其它集合。Rust标准库提供了各种各样的集合类型：`VecDeque<T>`、`LinkedList<T>`、`BinaryHeap<T>`、`BTreeSet<T>`、`BTreeMap<K,V>`、`HashSet<T>`和`HashMap<K,V>`。

说到集合，数组是一个单独案例，因为它完全是栈分配的，以及在编译期已经定义了大小。而对于其它集合，包括vector，元素个数可变，它将header存储在stack，数据部分则存储在堆。这种称为“动态数据集dynamically-sized collections”。

## Measuring Execution Time

集合的选取更多地由它的性能决定，先绕开这方面内容，看看如何精确度量不同Rust代码所花费的性能。

对于软件开发者来说性能是很重要的一方面。单个函数的运行，高级语言都要求命令处理花费至毫秒和秒级别，像Rust这类低级语言，都要求毫秒甚至纳秒。

在Rust标准库中，有几个函数可以度量源代码消耗的时间，

```rust
use std::time::Instant;

fn elapsed_ms(t1: Instant, t2: Instant) -> f64 {
    let t = t2 - t1;
    t.as_secs() as f64 * 1000. + t.subsec_nanos() as f64 / 1e6
}
let time0 = Instant::now();
for i in 0..10_000 {
	println!("{}", i);
}
let time1 = Instant::now();
println!("{}", elapsed_ms(time0, time1));
```

程序会打印0到9999的整数，然后将所花费的毫秒数输出。

所花费的时间跟计算机能力有关，当然也跟编译器的优化有关。

前面章节说到，可以用`rustc`来编译源代码文件，但是这个命令并没有编译器优化，只是单纯生成机器码用于调试，它不是高效的。

如果你对性能感兴趣，可以带上编译参数`-O`。省略这个参数，所有优化都是禁用的。

因此，这章示例可以通过下面命令行编译优化，

```rust
rustc -O main.rs
```

要度量一个时间，你应该用`Instant`类的`now`函数。这个类型定义在Rust的标准库中。


## Performing Arbitrary Insertions and Removals

回到原来的集合处理。下面程序是非常高效的，

```rust
const SIZE: usize = 100_000_000;
let t0 = Instant::now();
let mut v = Vec::<usize>::with_capacity(SIZE);
let t1 = Instant::now();
for i in 0..SIZE {
	v.push(i);
}
let t2 = Instant::now();
for _ in 0..SIZE {
	v.pop();
}
let t3 = Instant::now();
print!("{} {} {}", elapsed_ms(t0, t1), elapsed_ms(t1, t2), elapsed_ms(t2, t3));
```

记得添加`-O`进行编译。

程序将打印三段数字，它是由编译器、或者有操作系统所决定。

假设你本机输出的数据是：“0.002667 454.516057 87.302678”。

这意味着创建一个vector为这个`usize`对象分配“房间”，它占64位系统800M，以及少于3毫秒的消耗时间。要将一千万的值塞入这个空间，不使用内存派遣，少于1秒的损耗时间，同时还要删除所有数据，花费1/10秒的时间。

如果不加`-O`参数编译，你会发现它花费时间非常大。

相反，下面的程序非常低效，

```rust
const SIZE: usize = 100_000_000;
let t0 = Instant::now();
let mut v = Vec::<usize>::with_capacity(SIZE);
let t1 = Instant::now();
for i in 0..SIZE {
	v.insert(0, i);
}
let t2 = Instant::now();
for _ in 0..SIZE {
	v.remove(0);
}
let t3 = Instant::now();
print!("{} {} {}", elapsed_ms(t0, t1), elapsed_ms(t1, t2), elapsed_ms(t2, t3));
```

它会打印：“0.00178 2038.879344 2029.447851”。

要创建一个800KB的vector，花费少于2毫秒的时间，但是插入数据却花费了多于2秒的时间，以及差不多等同的时间用来删除数据。这里发现插入动作，比前面花费更多的时间。

导致两者的差异很好解析。

从栈顶添加元素，只需要确保有足够的空间，然后将数据拷贝到缓冲区，增加元素个数。对于计算机来说，处理这些时间少于5纳秒，包括迭代器的移位动作。

同样，对于从栈顶删除元素，确保vector不为空，然后递减元素，花费不到1纳秒的时间。

相反，从vector的开始部分插入元素，首先你需要将地址进行转换，每次有新的元素过来，都要释放地址空间。虽然转换很快，随着元素个数的增加，要插入首位置的元素也越来越多。

类似的，要从首位置移除元素，需要将所有元素都转换一遍，不仅仅是首位置，

从计算复杂度表示，栈顶(尾部)插入或删除元素是`O(K)`复杂度，它是常量复杂度(constant complexity)；而对于从栈尾(首部)插入或删除元素是`O(N)`复杂度，它是线性复杂度(linear complexity)。

即使是在中间部分插入或删除数据，性能可能会稍微好一点，但仍然比在栈顶插入或删除数据要慢。


## Queues

如果是在首部和尾部同时有插入或删除动作，这个vector不会是一个优化集合。典型情况类似于Queue，它在尾部插入元素，在首部萃取元素，

```rust
const SIZE: usize = 40_000;
let t0 = Instant::now();
let mut v = Vec::<usize>::new();
for i in 0..SIZE {
	v.push(i);
	v.push(SIZE + i);
	v.remove(0);
	v.push(SIZE * 2 + i);
	v.remove(0);
}
let t1 = Instant::now();
while v.len() > 0 {
	v.remove(0);
}
let t2 = Instant::now();
print!("{} {}", elapsed_ms(t0, t1), elapsed_ms(t1, t2));
```

可能的输出会是：“561.189636 276.056133”。

代码中，创建了一个空的vector，用了4千次循环，将三个数插入栈顶，以及在栈顶删除两个元素。第二个循环体内，每次从栈顶删除元素。第一个循环片段花费了大约半秒的时间，第二个循环体花费了大约四分之一秒。实际上，大部分时间都用在了萃取元素上，插入其实是非常快的。

我们希望插入元素总是发生在栈尾，萃取(extract)元素总是在栈顶，

```rust
const SIZE: usize = 40_000;
let t0 = Instant::now();
let mut v = Vec::<usize>::new();
for i in 0..SIZE {
	v.insert(0, i);
	v.insert(0, SIZE + i);
	v.pop();
	v.insert(0, SIZE * 2 + i);
	v.pop();
}
let t1 = Instant::now();
while v.len() > 0 {
	v.pop();
}
let t2 = Instant::now();
print!("{} {}", elapsed_ms(t0, t1), elapsed_ms(t1, t2));
```

结果可能打印：“790.365012 0.000112”。

现在插入很慢，删除却很快。可是总共花费的时间并没有提升多少。我们尝试用`VecDeque`类型，

```rust
const SIZE: usize = 40_000;
let t0 = Instant::now();
let mut v = std::collections::VecDeque::<usize>::new();
for i in 0..SIZE {
	v.insert(0, i);
	v.insert(0, SIZE + i);
	v.pop();
	v.insert(0, SIZE * 2 + i);
	v.pop();
}
let t1 = Instant::now();
while v.len() > 0 {
	v.pop();
}
let t2 = Instant::now();
print!("{} {}", elapsed_ms(t0, t1), elapsed_ms(t1, t2));
```

结果打印：“0.40793 0.050257”。

整段程序花费不到半毫秒的时间，这里需要显式声明`VecDeque`的类型，它是“vector-like double-ended queue”的缩写，“queue”表示的是“sequential collection into which items are inserted at one end and from which items are extracted at the other end”。“double-ended”表示在尾部插入元素，也可以在尾部萃取元素，不带有penalty。“vector-like”表示具有vector相似的操作。

要在vector栈顶添加或删除元素，可以简单使用`push`和`pop`，而对于`双端队列，double-ended queue`，需要理解两端的实现是等效的，插入元素可以用`push_front`和`push_back`，也可以在两端用`pop_front`和`pop_back`删除元素。虽然`VecDeque`类型支持`insert`和`remove`函数，但不被推荐使用，因为它不是高效的。

给出的队列非常高效，为什么我们总是用它，而是选择用vector？

原因是vector更普遍的操作是迭代、元素访问，这种时间损耗一直保持为常量因素。

```rust
const SIZE: usize = 40_000;
let mut v = Vec::<usize>::new();
let mut vd = std::collections::VecDeque::<usize>::new();
let t0 = Instant::now();
for i in 0..SIZE {
	v.push(i);
}
let t1 = Instant::now();
for i in 0..SIZE {
	vd.push_back(i);
}
let mut count = 0;
let 2 = Instant::now();
for i in v.iter() {
	count += i;
}
let t3 = Instant::now();
for i in vd.iter() {
	count += i;
}
let t4 = Instant::now();
print!("{} {} {} {} {}", count,
	elapsed_ms(t0, t1), elapsed_ms(t1, t2),
	elapsed_ms(t2, t3), elapsed_ms(t3, t4));
```

可能打印：“1599960000 0.230073 0.203979 0.013144 0.035295”。

会发现，`Vec`和`VecDeque`几乎花费相当的时间，但对于扫描整个集合元素，`Vec`效率要高出两倍。

## Linked Lists

对于某些应用，可能会频繁地在中间位置插入或删除元素。这种情况，向量(vector)和对象(queue)不是高效的，所以需要引入新的集合类型——`LinkedList`。

然而，如果你需要在一个集合进行大量操作，譬如添加或删除很多个条目，会比`Vec`或`VecDeque`要快很多，它会创建一个新的临时集合，并由临时集合替换原来的集合。

`LinkedList`的使用，不应该用于那些需要频繁读取的位置做插入或删除操作。

## Binary Heaps

访问集合还有另外一种方式，即所谓的“优先队列”。它出现在仅有两个函数的地方：插入元素和萃取元素。但每个元素都有优先值，萃取(extract)元素需要根据优先级获取。使用Vector时，可以类似下面这种方式包含这种行为，

```rust
fn add(v: &mut Vec<i32>, a: i32) {
	v.push(a);
	v.sort();
}
let a = [48, 18, 20, 35, 17, 13, 39, 12, 42, 33, 29, 27, 50, 16];
let mut v = Vec::<i32>::new();
for i in 0..a.len() / 2 {
	add(&mut v, a[i * 2]);
	add(&mut v, a[i * 2 + 1]);
	print!("{} ", v.pop().unwrap());
}
while ! v.is_empty() {
	print!("{} ", v.pop().unwrap());
}
```

每次将数组元素添加到vector时，vector都进行一次排序，它的元素保持升阶的顺序。所以vector的值的萃取也总可以得到最大值。

下面是另一种等价实现，只不过是在萃取前进行排序，

```rust
fn extract(v: &mut Vec<i32>) -> OPtion<i32> {
	v.sort();
	v.pop();
}
let a = [];
```

















本章覆盖有：

- “租借，borrowing”和“生命周期，lefetime”的概念
- 折磨系统软件的租借典型错误是哪些
- 如何通过租借checker，Rust的严格语法来避免这种典型错误
- 如何通过插入语句块来约束租借的作用域(scope)
- 为什么函数返回的引用需要生命周期指示符(specifiers)
- 如何给函数使用生命周期指示符(lifetime specifiers)，它们表示什么
- 租借checker的任务是什么

## Ownership and Borrowing

上一章介绍到，当将变量`a`赋值给`b`时，会有两种情况：如果它们类型是可拷贝的(copyable)，它就实现了`Copy`特质(当然也肯定实现了`Clone`)；如果它们的类型不可拷贝(non-copyable)，则没有实现`Copy`(`Clone`可能实现，也可能没有)。

第一种情况，用到拷贝语义(copy semantics)。意味着，在赋值过程中，当`a`保留它对象的所有权(ownership)，一个新的对象被创建，初始化值等同于`a`的值，以及`b`获得这些新的对象的所有权。当`a`和`b`离开它的作用域时，它们拥有的对象被销毁(又叫dropped)。

相反，第二种情况，用到移动语义(move semantics)。意味着，在赋值过程中，`a`将它的所有权移交给了`b`，不会有新对象的创建，`a`不再可访问。当`b`离开它的作用域时，它拥有的对象被销毁。当`a`离开它的作用域，不发生任何事情。

所有这些保证了合适的内存管理，只要没有引用被使用到。

但看看这个代码，

```rust
let n = 12;
let ref_to_n = &n;
```

第一条语句后，变量`n`拥有一个数。

第二条语句后，变量`ref_to_n`拥有一个引用，该引用指向同一个由`n`引用的数。它是一个所有权吗？

它不能作为一个所有权，因为这个数早已经由`n`所拥有，如果同时被这个引用所“拥有”，它将会被销毁两次。因此，类似这样的引用不能“拥有”对象。

表达式`n`和`*ref_to_n`指向同一个对象，但仅`n`拥有这个对象。变量`ref_to_n`可以访问这个对象，但不是“拥有”这个对象。这种概念称为**租借，borrowing**。我们说`ref_to_n`借了`n`拥有的数。这种租借，开始于引用指向该对象，结束于该对象的销毁。

关于可变性(mutability)，有两种类型的borrowing：

```rust
let mut n = 12;
let ref1_to_n = &mut n;
let ref2_to_n = &n;
```

这段程序中，`ref1_to_n`将`n`拥有的值，租借为 __mutably__ 的值，以及`ref2_to_n`租借为 __immutably__ 的值。第一种是可变租借(mutable borrowing)，第二种是不可变租借(immutable borrowing)。可变租借仅能从可变变量中获取。

## Object Lifetimes

注意到，“作用域，scope”的概念作用于编译期的变量，而不是运行期的对象。对应运行期对象的概念叫“生命周期，lifetime”。在Rust中，一个对象的生命周期，指的是一系列执行指令，从执行指令的创建，到执行指令的销毁。在该时间段，该对象叫做“存活，to live，to be alive”。

当然，作用域和生命周期存在一定关系，但它们不是同一个概念。例如：

```rust
let a;
a = 12;
print!("{}", a);
```

该程序中，变量`a`的作用域开始于第一行，而`a`拥有的对象的生命周期开始于第二行。通常认为，变量`a`的作用域开始于它的声明，对象的生命周期开始于该对象接收一个值。

即使是变量作用域(scope)的结束，跟对象生命周期(lifetime)的结束也不是同时发生的，

```rust
let mut a = "Hello".to_string();
let mut b = a;
print!("{}, ", b);
a = "world".to_string();
print!("{}!", a);
b = a;
```

结果将输出：“Hello, world!”。

在第一条语句，变量`a`被声明以及初始化。因此`a`的作用域开始，接着`a`拥有的对象被创建，`a`的生命周期开始。

在第二条语句，变量`b`被声明，由`a`移动对象进行初始化。因此，`b`的作用域开始，`a`的作用域被悬挂(suspended)，因为它被移动了，所以它不可再被访问。`b`拥有的对象不用创建，因为它就是先前创建的对象。

在第三条语句，`b`被访问。

在第四条语句，变量`a`通过`new`构造器，指派新的值。这里，`a`恢复(resume)它的作用域(scope)，因为它的作用域还没有结束。一个新的对象被创建，该对象的生命周期开始。前面由于变量`a`被“移动”了，所以它不“拥有”任何对象。所以这里的语句类似于一个初始化。

在第五条语句，`a`(拥有对象)可被访问了。

在第六条语句，`a`再次被移动到`b`，它的作用域再次被悬挂(suspended)。相反，`b`一直是活动的(active)，它拥有的对象由移动的`a`替换，因此，原先的对象在这里被销毁，以及生命周期结束。如果该对象实现了`Drop`，在这里，它的`drop`方法会被调用。

最后，`b`和`a`陆续退出它们的作用域。变量`b`拥有一个对象，该对象被销毁，以及结束它的生命周期。相反，变量`a`被“移动”了，不再拥有任何对象，也就不会有销毁对象的发生。

## Errors Regarding Borrowing

C和C++程序编写总是被各种错误困扰，而Rust则通过设计来避免这一类问题。Rust的一种常见错误是“use after move”，前面介绍过。另一种错误如下，

```rust
let ref_to_n;
{
    let n = 12;
    ref_to_n = &n;
    print!("{} ", *ref_to_n);
}
print!("{}", *ref_to_n);
```

首先，变量`ref_to_n`被声明，但没有被初始化。然后，在语句块内，可变变量`n`被声明并初始化，它分配一个数在栈上，值为12。

然后，原先的变量，用一个指向`n`的引用进行初始化，它租借(borrow)了这个对象。

接着，变量`ref_to_n`指向的对象，即值为12的对象，打印输出。

接着，语句块结束，内部变量`n`结束了它的作用域，它的对象被销毁。

接着，变量`ref_to_n`指向的对象再次被打印。但该对象原先被`n`“拥有”，它现在不存在了！

幸运的是，Rust编译器拒绝该代码，产生错误信息“`n` does not live long enough”。该消息表示，变量`n`死了(dying)，但仍然有指向它“拥有”的对象的引用，它应该活更长一些；至少应该跟租借它对象的租借方一样长。

顺便，C或C++对应的代码如下，

```c
#include <stdio.h>
int main() {
    int* ref_to_n;
    {
        int n = 12;
        ref_to_n &n;
        printf("%d ", *ref_to_n);
    }
    printf("%d", *ref_to_n);
    return 0;
}
```

这段程序可被C和C++编译器接受。结果会打印“12”，之后的行为会变得不可预测。

这类程序错误，我们称之为“use after drop”。

有另一种可避免的Rust错误，

```rust
let mut v = vec![12];
let ref_to_first = &v[0];
v.push(13);
print!("{}", ref_to_first);
```

对应的C语言实现是，

```c
#include <stdio.h>
#include <stdlib.h>
int main() {
    int* v = malloc(1 * sizeof (int));
    v[0] = 12;
    const int* ref_to_first = &v[0];
    v = realloc(v, 2 * sizeof (int));
    v[1] = 13;
    printf("%d", *ref_to_first);
    free(v);
}
```

以及C++的实现是，

```cpp
#include <iostream>
#include <vector>
int main() {
    std::vector<int> v { 12 };
    const int& ref_to_first = v[0];
    v.push_back(13);
    std::cout << ref_to_first;
}
```

不用多说，最后两个程序会被各自的编译器接受，即使它们的行为是未定义的。相反，Rust编译器会拒绝并抛出错误信息“cannot borrow `v` as mutable because it is also borrowed as immutable”。让我们看看该程序有什么错误。

首先，可变(mutable)变量`v`，用一个仅带数12的vector声明和初始化。

然后，变量`ref_to_first`用原先指向`v`的引用声明和初始化。引用指向的值包含数12。

然后，另一个数13被添加到vector。但这种插入会导致缓冲区的重新分配。即使在这个例子，`ref_to_first`仍会继续指向旧的对象，不是有效的内存地址。

最后，这个旧的内存地址，可能是错误的，读取的内存地址的值打印后，是不可预测的结果。

由于从一个vector插入或添加元素时，所有指向vector的引用都“失效”，导致这种错误的出现。通常，这种错误属于广义的错误类别，也就是一个数据结构，通过几种路径、或别名访问，当数据结构被其中一个别名修改时，不能被另一个别名使用。

我们将这类编程错误命名为“use after change by an alias”。


## How to Prevent "Use After Drop" Errors

Rust防止使用“脱落，dropped”对象的技术是简单的。

把该对象看做是被一个变量指向，遵循栈分配的规则，会按照变量声明的反向顺序被脱落，而不是初始化的反向顺序。

```rust
struct X(char);
impl Drop for X {
	fn drop(&mut self) {
		print!("{}", self.0);
	}
}
let _a = X('a');
let _b;
let _c = X('c');
_b = X('b');
```

该程序会打印“cba”。这三个对象按照顺序“acb”被构造，但三个对象的分配顺序是“abc”，因此回收以及脱落的按相反的顺序。

为了避免使用脱落对象，所有变量，租借其它变量拥有的对象，必须在该变量的 **后面** 声明。

例如，

```rust
let n = 12;
let mut _r;
let m = 13;
_r = &m;
_r = &n;
```

这段代码会产生错误信息：“`m` does not live long enough”。这是因为`_r`同时从`m`和`n`进行borrow，虽然不在同一时刻指向两者，但它在`m`之前声明了。要更正这段代码，改为如下，

```rust
let n = 12;
let m = 13;
let mut _r;
_r = &m;
_r = &n;
```

这段代码是合法的，当`m`和`n`拥有的对象被dropped时，不会再有指向它们的引用。

## How to Prevent "Use After Change by an Alias" Errors

由于其它变量引起对象的改变，导致当前变量使用该对象出现错误。要避免这种错误，使用的规则有几分复杂。

首先，要求考虑任何语句会读这个对象，不会有写操作，就像是该对象的一个 __临时不可变租借(temporary immutable borrowing)__；任何语句变更这个对象，就像是该对象的一个 __临时可变租借(temporary mutable borrowing)__。这种租借的出现和结束在该语句的内部进行。

然后，租借开始于，获取指向该对象的引用，并分配给一个变量；结束于，该变量的作用域(scope)结束。

下面是一个例子，

```rust
let a = 12;
let mut b = 13;
print!("{} ", a);
{
	let c = &a;
	let d = &mut b;
	print!("{} {} ", c, d);
}
b += 1;
print!("{}", b);
```

结果会打印：“12 12 13 14”。

在第3行和最后一行，一个可变租借开始并结束。在第5行，一个不可变租借开始，第6行，一个可变租借开始；它们在语句块末尾结束。在第9行，一个可变租借开始并结束。

这种规则要求，同一时刻，可变租借(即，&mut )的出现不能和其它租借并存。

换句话说，同一时刻：

- 没有租借
- 或，一个单一的可变(mutable)租借
- 或，一个单一的不可变(immutable)租借
- 或，几个不可变(immutable)租借

不能有：

- 几个可变(mutable)租借
- 不能有一个单一的可变(mutable)租借和一个或多个不可变(immutable)租借


## Listing the Possible Cases of Multiple Borrowings

下面罗列六种允许的情况。

第一种：

```rust
let a = 12;
let _b = &a;
let _c = &a;
```

有两个不可变租借，直到结束为止被持有。

第二种：

```rust
let mut a = 12;
let _b = &a;
print!("{}", a);
```

第三种：

```rust
let mut a = 12;
a = 13;
let _b = &a;
```

一个不可变租借，紧随后一个临时可变租借。

第四种：

```rust
let mut a = 12;
a = 13;
let _b = &mut a;
```

一个可变租借，紧随后一个临时可变租借。

第五种：

```rust
let mut a = 12;
print!("{}", a);
let _b = &a;
```

一个不可变租借，紧随后一个临时不可变租借。

第六种：

```rust
let mut a = 12;
print!("{}", a);
let _b = &mut a;
```

一个可变租借，紧随后一个临时不可租借。

下面是六种不合法情况。

第一种：

```rust
let mut a = 12;
let _b = &mut a;
let _c = &a;
```

编译出错“cannot borrow `a` as immutable because it is also borrowed as mutable”。

第二种：

```rust
let mut a = 12;
let _b = &a;
let _c = &mut a;
```

编译出错“cannot borrow `a` as mutable because it is also borrowed as immutable”。

第三种：

```rust
let mut a = 12;
let _b = &mut a;
let _c = &mut a;
```

编译出错“cannot borrow `a` as mutable more than once at a time”。

第四种：

```rust
let mut a = 12;
let _b = &a;
a = 13;
```

编译出错“cannot assign to `a` because it is borrowed”。

第五种：

```rust
let mut a = 12;
let _b = &mut a;
a = 13;
```

编译出错“cannot assign to `a` because it is borrowed”。

第六种：

```rust
let mut a = 12;
let _b = &mut a;
print!("{}", a);
```

编译出错“cannot borrow `a` as immutable because it is also borrowed as mutable”。

梳理一下。对于当前尚未被borrowed的对象来说，允许的操作是：

1. 仅可被不可变租借(immutablely borrowed)数次，然后可以由所有者(owner)和任何租借方(borrower)读取。
2. 仅可被可变租借(mutably borrowed)一次，然后有且仅能由这个租借方(borrower)读取和修改。


## Using a Bock to Restrict Borrowing Scope

当一个对象的租借结束后，对象对其它租借又变得可用了。任何类型的租借都可限制在语句块内。

```rust
let mut a = 12;
{
	let b = &mut a;
	*b += 1;
}
let c = &mut a;
*c += 2;
```

这是允许的，因为租借发生在第三行，语句块结束后，租借也结束了，因此第7行的`a`可以用于其它租借。

类似地，对于函数也一样，在函数结束后，对象再次变得可用。

```rust
let mut a = 12;
fn f*b: &mut i32) {
	*b += 1;
}
f(&mut a);
let c = &mut a;
*c += 2;
```

这种规则，适用于Rust确保可以自动地决定内存的回收，避免不合法的引用；这种规则允许Rust实现无数据竞争(data-race-free)的并发编程。


## The Need of Lifetime Specifiers for Returned References

先看这段代码：

```rust
let v1 = vec![11u8, 22];
let result;
{
	let v2 = vec![33u8];
	result = {
		let _x1: &Vec<u8> = &v1;
		let _x2: &Vec<u8> = &v2;
		_x1
	}
}
print!("{:?}", *result);
```





























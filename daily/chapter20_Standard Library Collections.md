本章覆盖有：

- 如何度量运行部分代码所花费的时间
- 基于性能因素，如何使用哪种类型的集合
- 集合中的各种操作中哪个更好：顺序扫描(sequential scan)、两端插入和删除(insertion and removal of items at both ends)、删除最大项(removal of the largest item)、搜索(search)、键搜索(search by key)、保持顺序(keeping items sorted)

## Collections

数组、向量、结构体、元组结构体、元组、枚举属于数据类型，它们的对象可能包含几个其它对象。然而，对于结构体、元组-结构体、元组、枚举，每个包含的对象，需要指定声明类型和构造类型，它们不能存储多个对象。相反数组和向量的数据类型会包含多个对象，这一类对象称为“集合”。

数组和向量是最佳集合：内存高效，读取速度快，CPU缓存高效，能通过索引快速访问内部元素。当然，在某些情况不尽是高效的，这种情况下需要使用其它集合。Rust标准库提供了各种各样的集合类型：`VecDeque<T>`、`LinkedList<T>`、`BinaryHeap<T>`、`BTreeSet<T>`、`BTreeMap<K,V>`、`HashSet<T>`和`HashMap<K,V>`。



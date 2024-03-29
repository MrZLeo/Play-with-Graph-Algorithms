# 玩儿转图论算法——Rust实现
本项目代码为@liuyubobobo 在慕课网的玩转图论算法课程的Rust语言实现。

⚠️  :

1. 对同一种算法，例如BFS、DFS，我尝试了很多不同的写法，例如纯函数过程、算法类以及全局变量；对于特定的实现细节，也有各种不同的尝试，尽可能地使用了Rust方方面面的特性和技巧。最前面的几个章节中，代码风格更接近C++/JAVA，这是因为作为初学者，我更习惯性地采用了熟悉的代码风格。随着我对Rust的深入学习，偏后续的章节的算法实现会更有Rust的代码风格。

2. Rust标准库对于Set提供了两个容器，分别是HashSet和BTreeSet。其中，BTreeSet采用了b-tree来实现，而非其他语言广泛采用的red-black tree，这是由于Rust官方进行的benchmark测试得到的结果显示，b-tree的实现能更好地利用cache，提高内存的利用率，进而最终的效率高于red-black tree。本仓库的所有代码实现为了更好的性能，均使用了HashSet。

> B 树表示缓存效率与实际最小化搜索中执行的工作量之间的根本折衷。从理论上讲，二元搜索树 (BST) 是排序的 map 的最佳选择，因为完全平衡的 BST 执行查找元素 (log2n) 所需的理论上最小的比较量。 但是，实际上，完成此操作的方式对于现代计算机体系结构而言效率非常低。 特别是，每个元素都存储在其自己的单独堆分配节点中。 这意味着每个插入都会触发堆分配，并且每个比较都应该是缓存未命中。 由于在实践中这些都是非常昂贵的事情，因此我们至少不得不重新考虑 BST 战略。
>
> 相反，B 树使每个节点在连续数组中包含 B-1 到 2B-1 元素。通过这样做，我们将分配数量减少了 B 倍，并提高了搜索中的缓存效率。但是，这确实意味着平均而言，搜索将不得不进行更多的比较。 比较的精确数量取决于所使用的节点搜索策略。为了获得最佳的缓存效率，可以线性搜索节点。为了进行最佳比较，可以使用二进制搜索来搜索节点。作为一种折衷，也可以执行线性搜索，该搜索最初仅检查每个 第i个 元素以选择 i。
>
> 当前，我们的实现仅执行简单的线性搜索。这在比较便宜的元素的小节点上提供了出色的性能。但是，未来我们将进一步探索基于 B 的选择以及可能的其他因素来选择最佳搜索策略。使用线性搜索，搜索随机元素预计会进行 B * log(n) 次比较，这通常比 BST 差。
>
> 但是，实际上，性能非常好。

3. 代码的实现基于课程，并不能保证绝对的性能或与课程官方的JAVA代码实现完全一致，也不代表实现上不存在冗杂，尤其是前几章的代码实现。此外，有一些小节的代码重复度较高，重复实现意义不大，因此是没有的，缺少这部分代码不会影响整体的学习。非常欢迎对代码的修改和PR！

4. Rust的类型系统和以往我们学习的语言稍有不同，首先，Rust是很少能够隐式转型的，其次，Rust只允许使用usize作为数组等容器的索引，同时返回容器数量、统计结果等等的API都会返回usize。而我们常常使用int作为值的默认选择，这就意味着需要转型的情况非常多，举个例子：

```rust
let mut res = vec![];
let mut cur = end;
while cur != start {
    res.push(cur);
    cur = pre[cur];         // error: cur is i32, here we need usize
    cur = pre[cur as usize];  // correct: use cur as usize
}
res.push(cur);
```

与此相关的情况会大量出现，这让我们需要深入思考，为什么会产生这样的问题？答案很简单，Rust这么做是有他的理由的：i32可以是负数，这对于索引而言是不合理的。那么，为什么我们需要大量的直接强制转型呢？这个问题的意思是，如果我们能保证他一定是正数，用usize就好了；如果我们不能保证，那应该需要一小段判断的逻辑。综合来说，就是：大量的强制转型是不合理的，因为既然我们允许大量的强制转型存在，那么这里的数一定是正数。所以我们不应该随意的选择所谓的i32，而是应该根据实际需求来选择。

实际上，C++也是使用usize来表示诸如vector长度之类的值的，如果我们深入了解的话，C++使用了size_t的值类型来表示，本质上就是usize，但是C++默认的隐式转型替我们隐去了这些东西。

对于图来说，我们课程中设计的点的范围是从0开始到g.v()。因此必定是正数，所以，我们应该选择usize来表示点，而非i32。

5. 项目采用MIT协议，允许随意使用。

## 代码运行
要运行每一章节的示例代码：
```bash
cargo run
```

运行测试：
```bash
cargo test
```

运行特定的测试集：
```bash
cargo test <name_of_test>
```

运行测试的同时显示必要的输出信息：
```bash
cargo test -- --nocapture
```

## 课程源码目录

| **第一章 欢迎大家学习《玩儿图论算法》** | [无代码] |
| :--- | :---: |
| 1-1 欢迎大家学习《玩儿转图论算法》 | [无代码] |
| 1-2 课程学习的更多说明 | [无代码] |
| 1-3 课程编程环境的搭建 | [无代码] |
| **第二章 图论基础** | [章节 Rust 源码](02-Graph-Basics/) |
| 2-1 图的分类 | [无代码] |
| 2-2 图的基本概念 | [无代码] |
| 2-3 图的基本表示：邻接矩阵 | [Rust](02-Graph-Basics/03-Adjacency-Matrix/src/) |
| 2-4 更多图的方法 | [Rust](02-Graph-Basics/04-Other-Methods-in-Graph/src/) |
| 2-5 图的基本表示：邻接表 | [无代码] |
| 2-6 邻接表的实现 | [Rust](02-Graph-Basics/06-Adjacency-List/src/) |
| 2-7 邻接表的问题和改进 | [无代码] |
| 2-8 实现邻接表的改进 | [Rust](02-Graph-Basics/08-Adjacency-Set/src/) |
| 2-9 图的基本表示的比较 | [Rust](02-Graph-Basics/09-Graph-Representation-Comparation/src/) |
| **第三章 图的深度优先遍历** | [章节 Rust 源码](03-Graph-DFS/) |
| 3-1 数据结构遍历的意义 | [无代码] |
| 3-2 从树的深度优先遍历到图的深度优先遍历 | [无代码] |
| 3-3 DFS逻辑的微观解读 | [无代码] |
| 3-4 实现图的深度优先遍历 | [Rust](03-Graph-DFS/04-Graph-DFS-Implementation/src/) |
| 3-5 图的深度优先遍历的改进 | [Rust](03-Graph-DFS/05-Graph-DFS-Improvement/src/) |
| 3-6 更多关于图的深度优先遍历 | [无代码] |
| 3-7 【文字】邻接矩阵的深度优先遍历 | [见慕课网代码仓] |
| 3-8 【文字】创建统一的图接口 | [见慕课网代码仓] |
| 3-9 【文字】深度优先遍历的非递归实现 | [见慕课网代码仓] |
| **第四章 图的深度优先遍历的应用** | [章节 Rust 源码](04-Graph-DFS-Applications/)  |
| 4-1 图的联通分量的个数 | [Rust](04-Graph-DFS-Applications/01-Connected-Components-Count/src/) |
| 4-2 DFS 中的一个小技巧 | [Rust](04-Graph-DFS-Applications/02-Connected-Components/src/) |
| 4-3 求解联通分量 | [Rust](04-Graph-DFS-Applications/03-More-about-Connected-Components/src/) |
| 4-4 单源路径问题 | [无代码] |
| 4-5 单源路径问题的编程实现 | [Rust](04-Graph-DFS-Applications/05-Single-Source-Path/src/) |
| 4-6 【文字】单源路径问题的小优化 | [见慕课网代码仓] |
| 4-7 【文字】所有点对路径问题 | [见慕课网代码仓] |
| 4-8 提前结束递归：路径问题的另一个优化 | [Rust](04-Graph-DFS-Applications/08-Path-Improvement/src/) |
| 4-9 无向图的环检测问题 | [Rust](04-Graph-DFS-Applications/09-Cycle-Detection/src/) |
| 4-10 二分图检测 | [无] |
| 4-11 实现二分图检测 | [Rust](04-Graph-DFS-Applications/11-Bipartition-Detection/src/) |
| 4-12 本章小结和更多拓展 | [无] |
| 补充内容 1 所有路径问题 | - |
| **第五章 图的广度优先遍历** | [章节 Rust 源码](05-Graph-BFS/)  |
| 5-1 从树的广度优先遍历到图的广度优先遍历 | [无] |
| 5-2 图的 BFS 的实现 | [Rust](05-Graph-BFS/02-BFS/src/) |
| 5-3 使用 BFS 求解路径问题 | [Rust](05-Graph-BFS/03-Single-Source-Path-BFS/src/) |
| 5-4 【文字】更多关于使用 BFS 求解路径问题 | [见慕课网代码仓] |
| 5-5 【文字】使用 BFS 求解联通分量问题 | [见慕课网代码仓] |
| 5-6 【文字】使用 BFS 求解环检测问题 | [见慕课网代码仓] |
| 5-7 【文字】使用 BFS 求解二分图检测问题 | [见慕课网代码仓] |
| 5-8 BFS 的重要性质 | [无] |
| 5-9 无权图的最短路径 | [Rust](05-Graph-BFS/09-Unweighted-Shortest-Path/src/) |
| 5-10 DFS 和 BFS 的神奇联系，与本章小结 | [无] |
| **第六章 图论问题建模和 Floodfill** | [章节 Rust 源码](06-Graph-Modelling-and-Floodfill/) |
| 6-1 算法笔试中图论问题的书写 | [Rust](06-Graph-Modelling-and-Floodfill/01-Leetcode-Graph-Basics/src/) |
| 6-2 图的建模和二维数组中的小节 | 无 |
| 6-3 实现图的建模问题 | [Rust](06-Graph-Modelling-and-Floodfill/03-Graph-Construction/src/) |
| 6-4 Flood Fill | [Rust](06-Graph-Modelling-and-Floodfill/04-Flood-Fill/src/) |
| 6-5 更多关于 Flood Fill 的问题 | [无] |
| 6-6 【文字】联通性和并查集 | [见慕课网代码仓] |
| 6-7 【文字】在 Leetcode 上的更多优化 | [见慕课网代码仓] |
| **第七章 图论搜索和人工智能** | [章节 Rust 源码](07-AI-Search-and-BFS/) |
| 7-1 算法笔试中的 BFS 问题 | [Rust](07-AI-Search-and-BFS/01-Leetcode-BFS/src/) |
| 7-2 图论建模的核心：状态表达 | [无] |
| 7-3 实现转盘锁问题 | [Rust](07-AI-Search-and-BFS/03-States/src/) |
| 7-4 一道智力题 | [Rust](07-AI-Search-and-BFS/04-Water-Puzzle-Uncompleted/src/) |
| 7-5 代码实现一道智力题 | [Rust](07-AI-Search-and-BFS/05-Water-Puzzle-Completed/src/) |
| 7-6 Leetcode 上一个困难的问题 | [Rust](07-AI-Search-and-BFS/06-Sliding-Puzzle-Uncompleted/src/) |
| 7-7 实现滑动谜题 | [Rust](07-AI-Search-and-BFS/07-Sliding-Puzzle-Completed/src/) |
| 7-8 图论搜索和人工智能 | [无] |
| 补充代码1 过河问题代码参考 | - |
| **第八章 桥，割点和图的遍历树** | [章节 Rust 源码](08-Bridges-and-Cut-Points/) |
| 8-1 什么是桥 | [无] |
| 8-2 如何寻找桥 | [无] |
| 8-3 完整模拟寻找桥算法 | [无] |
| 8-4 实现寻找桥算法 | [Rust](08-Bridges-and-Cut-Points/04-Bridges-Algorithm/src/) |
| 8-5 图的遍历树 | [无] |
| 8-6 割点和寻找割点的算法 | [无] |
| 8-7 实现寻找割点的算法 | [Rust](08-Bridges-and-Cut-Points/07-Cut-Points-Algorithm/src/) |
| 8-8 本章小结：寻找桥和割点 | [无] |
| **第九章 哈密尔顿回路和状态压缩** | [章节 Rust 源码](09-Hamilton-Loop-and-Path/) |
| 9-1 哈密尔顿回路和 TSP | [无] |
| 9-2 求解哈密尔顿回路的算法 | [无] |
| 9-3 实现哈密尔顿回路算法 | [Rust](09-Hamilton-Loop-and-Path/03-Hamilton-Loop/src/) |
| 9-4 哈密尔顿回路算法的一个优化 | [Rust](09-Hamilton-Loop-and-Path/04-Hamilton-Loop-Optimization/src/) |
| 9-5 【文字】哈密尔顿路径算法 | [见慕课网代码仓] |
| 9-6 Leetcode 上的哈密尔顿问题 | [Rust](09-Hamilton-Loop-and-Path/06-Unique-Paths-III/src/) |
| 9-7 状态压缩 | [无] |
| 9-8 基于状态压缩的哈密尔顿回路算法 | [Rust](09-Hamilton-Loop-and-Path/08-State-Compression/src/) |
| 9-9 记忆化搜索 | [Rust](09-Hamilton-Loop-and-Path/09-Memory-Search/src/) |
| 9-10 哈密尔顿回路和哈密尔顿路径小结 | [无] |
| **第十章 欧拉回路与欧拉路径** | [章节 Rust 源码](10-Euler-Loop-and-Euler-Path/) |
| 10-1 什么是欧拉回路 | [无] |
| 10-2 欧拉回路的存在性 | [无] |
| 10-3 实现欧拉回路存在的判断 | [Rust](10-Euler-Loop-and-Euler-Path/03-Euler-Loop-Detection/src/) |
| 10-4 求解欧拉回路的三种算法 | [无] |
| 10-5 Hierholzer 算法模拟 | [无] |
| 10-6 实现 Hierholzer 算法 | [Rust](10-Euler-Loop-and-Euler-Path/06-Hierholzer-Algorithm/src/) |
| 10-7 欧拉路径和更多欧拉回路相关问题 | [无] |
| 补充代码 1 欧拉路径 | - |
| 补充代码 2 欧拉回路的递归实现 | - |
| **第十一章 最小生成树** | [章节 Rust 源码](11-Minimum-Tree-Spanning/) |
| 11-1 带权图及实现 | [Rust](11-Minimum-Tree-Spanning/01-Weighted-Graph/src/) |
| 11-2 Map 的遍历 | [Rust](11-Minimum-Tree-Spanning/02-Weighted-Graph-Completed/src/) |
| 11-3 最小生成树和 Kruuskal 算法 | [无] |
| 11-4 切分定理 | [无] |
| 11-5 Kruskal 算法的实现 | [Rust](11-Minimum-Tree-Spanning/05-Kruskal-Algorithm/src/) |
| 11-6 并查集动态环检测 | [Rust](11-Minimum-Tree-Spanning/06-Kruskal-Algorithm-Completed/src/) |
| 11-7 Prim 算法原理 | [无] |
| 11-8 Prim 算法的实现 | [Rust](11-Minimum-Tree-Spanning/08-Prim-Algorithm/src/) |
| 11-9 Prim 算法的优化 | [Rust](11-Minimum-Tree-Spanning/09-Prim-Algorithm-Optimized/src/) |
| 11-10 关于最小生成树问题的更多讨论 | [无] |
| **第十二章 最短路径算法** | [章节 Rust 源码](12-Shortest-Path/) |
| 12-1 有权图的最短路径问题 | [无] |
| 12-2 Dijkstra 算法的原理和模拟 | [无] |
| 12-3 实现 Dijkstra 算法 | [Rust](12-Shortest-Path/03-Dijkstra-Algorithm/src/) |
| 12-4 Dijkstra 算法的优化 | [Rust](12-Shortest-Path/04-Dijkstra-Algorithm-Optimized/src/) |
| 12-5 更多关于 Dijkstra 算法的讨论 | [Rust](12-Shortest-Path/05-More-about-Dijkstra/src/) |
| 12-6 Bellman-Ford 算法 | [无] |
| 12-7 负权环 | [无] |
| 12-8 实现 Bellman-Ford 算法 | [Rust](12-Shortest-Path/08-Bellman-Ford-Algorithm/src/) |
| 12-9 更多关于 Bellman-Ford 算法的讨论 | [Rust](12-Shortest-Path/09-More-about-Bellman-Ford/src/) |
| 12-10 Floyed 算法原理 | 无 |
| 12-11 实现 Floyed 算法 | [Rust](12-Shortest-Path/11-Floyed-Algorithm/src/) |
| 12-12 更多关于最短路径问题的讨论 | [无] |
| **第十三章 有向图算法** | [章节 Rust 源码](13-Directed-Graph/) |
| 13-1 有向图的实现 | [Rust](13-Directed-Graph/01-Directed-Graph/src/) |
| 13-2 有向图算法 | [Rust](13-Directed-Graph/02-Directed-Graph-Algorithms/src/) |
| 13-3 有向图环检测和 DAG | [Rust](13-Directed-Graph/03-Directed-Cycle-Detection/src/) |
| 13-4 有向图的度：入度和出度 | [Rust](13-Directed-Graph/04-Directed-Graph-Degrees/src/) |
| 13-5 有向图求解欧拉回路 | [Rust](13-Directed-Graph/05-Directed-Euler-Loop/src/) |
| 13-6 拓扑排序 | [无] |
| 13-7 拓扑排序的实现 | [Rust](13-Directed-Graph/07-Topological-Sort/src/) |
| 13-8 另一个拓扑排序算法 | [无] |
| 13-9 另一个拓扑排序的实现 | [Rust](13-Directed-Graph/09-Another-Topological-Sort-Implementation/src/) |
| 13-10 有向图的强连通分量 | [无] |
| 13-11 Kosaraju 算法 | [无] |
| 13-12 Kosaraju 算法的实现 | [Rust](13-Directed-Graph/12-SCC/src/) |
| 13-13 有向图算法小节 | [无] |
| **第十四章 网络流模型和最大流问题** | [章节 Rust 源码](14-Network-Flows/) |
| 14-1 网络流模型和最大流问题 | [无] |
| 14-2 Ford-Fulkerson 思想 | [无] |
| 14-3 Edmonds-Karp 算法 | [无] |
| 14-4 最大流算法的基本架构 | [Rust](14-Network-Flows/04-Max-Flow/src/) |
| 14-5 实现 Edmonds-Karp 算法 | [Rust](14-Network-Flows/05-Edmonds-Karp-Algorithm/src/) |
| 14-6 Edmonds-Karp 算法的测试和更多讨论 | [Rust](14-Network-Flows/06-Edmonds-Karp-Algorithm-Completed/src/) |
| 14-7 最大流问题建模 | [Rust](14-Network-Flows/07-Baseball-Match/src/) |
| 14-8 本章小结和更多相关讨论 | [无] |
| **第十五章 匹配算法** | [章节 Rust 源码](15-Matching-Algorithm/) |
| 15-1 最大匹配和完全匹配 | [无] |
| 15-2 使用最大流算法解决匹配问题 | [无] |
| 15-3 实现使用最大流算法解决匹配问题 | [Rust](15-Matching-Algorithm/03-Solving-Matching-Problem-in-Max-Flow/src/) |
| 15-4 从 Leetcode 上一个 Hard 问题看匹配算法建模 | [Rust](15-Matching-Algorithm/04-LCP-4/src/) |
| 15-5 匈牙利算法 | [无] |
| 15-6 匈牙利算法的实现 | [Rust](15-Matching-Algorithm/06-Hungarian-BFS/) |
| 15-7 基于递归实现的匈牙利算法 | [Rust](15-Matching-Algorithm/07-Hungarian-DFS/) |
| 15-8 匹配问题小结 | [无] |
| **第十六章 更广阔的图论算法世界** | [更新中，敬请期待] |
| 16-1 大家加油！ | [无] |

---

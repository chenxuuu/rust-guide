# 生成随机值

## 应用场景

生成随机数最显著的应用场景莫过于一些随机业务，如公司年会的抽奖、各类彩票业务、邮箱新用户生成密钥等。

但对于非随机业务来说，随机数的生成也有罕见。多见于瞬时负载压力，数据大批量处理等方面的业务。比如：

- 多客户端多线程并发类似的业务。在高峰期间，会导致应用服务器、数据库服务器的瞬时负载率超高。将会造成任务处理的延迟，必须进行优化以降低负载。在这种情况下，有一种解决方案即为分析梳理业务，进行随机休眠处理，这样可将执行散列到不同的时间段。
- 数据预热业务。当有大量数据不能离线预热，必须要线上预热的时候，势必造成资源的严重紧张，甚至打垮服务器。所以采用随机预热方式，使数据逐渐预热。预热成功后，在取消随机预热。
- 批量的缓存处理。当大批量缓存同时建立，又批量失效，导致缓存建立不分散，对服务端瞬时产生压力。可以通过将部分缓存失效时间随机延长几分钟即可，分散批量建立和失效的压力。
- 也可用于重复提交方面的解决方案、浏览器缓存处理等等。

生成随机数的应用场景非常广阔，在此不一一赘述。而当前技术潮流中，和生成随机数功能结合最紧密的，莫过于大红大紫的区块链技术。

在区块链及其数字证券领域中，随机数是关键的技术要点，其直接与区块链及其数字证券领域中最重要和最基本的安全保障方面息息相关。

## crate 介绍

### rand

rand crate 是 Rust 生态中的一个随机数生成工具。其提供了如下功能：

- 生成随机数；
- 将生成的随机数进行类型转换、分发；
- 一些与随机性相关的算法。

rand crate 是由一系列 crate 组成的，rand crate 提供了主用户界面。如果需要额外的分发类型，可以使用 rand_distr crate 或者 statrs crate 来补充。具体结构关系如表 3.1-1 所示。

```
getrandom ┐
          └ rand_core ┐
                      ├ rand_chacha ┐
                      ├ rand_hc     ┤
                      ├ rand_pcg    ┤
                      └─────────────┴ rand ┐
                                           ├ rand_distr
                                           └ statrs
```

表 3.1-1

### rand_distr

rand_distr crate 实现了诸多概率分布类型，如均匀分布、正态分布（Normal distribution）、柯西分布（Cauchy distribution）等。rand_distr crate 是 `rand::distributions` 模块的一个超级集合，提供了以下概率分布：

- 线性增长相关（例如误差、偏移量等）：
  - `正态（Normal）`分布，以`标准正态（StandardNormal）`为原语
  - `柯西（Cauchy）`分布
- 伯努利试验相关（给定概率的`是/否`事件）：
  - `二项（Binomial）`分布
- 指数增长相关（例如价格、收入、人口等）：
  - `对数正态（LogNormal）`分布
- 给定条件下独立事件的发生率相关：
  - `帕雷托（Pareto）`分布
  - `泊松（Poisson）`分布
  - `指数（Exponential）`分布，以结构体 `Exp1` 为原语
  - `韦布尔（Weibull）`分布
- 伽马分布、导出分布：
  - `伽马（Gamma）`分布
  - `卡方（ChiSquared）`分布
  - `学生-T（StudentT）`分布
  - `费歇尔-F（FisherF）`分布
- 三角学分布：
  - `Beta` 分布
  - `三角（Triangular）`分布
- 多元概率分布：
  - `狄利克雷（Dirichlet）`分布
  - `UnitSphere` 分布
  - `UnitBall` 分布
  - `UnitCircle` 分布
  - `UnitDisc` 分布
<!-- - 基于权重的指数样本：
  - `基于权重的别名指数（WeightedAliasIndex）`分布 -->
- 其它分布：
  - `逆高斯（InverseGaussian）`分布
  - `正态逆高斯（NormalInverseGaussian）`分布

如上表 3.1-1 所示，rand_distr crate 是对 rand crate 的补充。

## 实例实践

{{#include randomness/rand.md}}
{{#include randomness/rand-range.md}}
{{#include randomness/rand-dist.md}}
{{#include randomness/rand-custom.md}}
{{#include randomness/rand-passwd.md}}
{{#include randomness/rand-choose.md}}

{{#include ../links.md}}

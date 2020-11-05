# 生成随机值

## crate 介绍

### rand

rand crate 是 Rust 生态中的一个随机数生成工具。其提供了如下功能：

- 生成随机数；
- 将生成的随机数进行类型转换、分发；
- 一些与随机性相关的算法。

rand crate 是由一系列 crate 组成的，rand crate 提供了主用户界面。如果需要额外的分发类型，可以使用 rand_distr crate 或者 statrs crate 来补充。

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

rand_distr crate 是对 rand crate 的补充。

## 实例实践

{{#include randomness/rand.md}}

{{#include randomness/rand-range.md}}

{{#include randomness/rand-dist.md}}

{{#include randomness/rand-custom.md}}

{{#include randomness/rand-passwd.md}}

{{#include randomness/rand-choose.md}}

{{#include ../links.md}}

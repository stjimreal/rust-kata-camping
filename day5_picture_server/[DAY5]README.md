<!--
 * @Date: 2021-10-03 22:44:18
 * @LastEditors: LIULIJING
 * @LastEditTime: 2021-10-03 22:49:21
-->

# [DAY5] 做一个图片服务器有多难

## 思考题: 添加一个新功能

步骤如下：
1. 首先添加新的 proto，定义新的 spec  
2. 然后为 spec 实现 SpecTransform trait 和一些辅助函数  
3. 最后在 Engine 中使用 spec

如果要换图片引擎？
1. 添加新的图片引擎，像 Photon那样，实现 Engine trait 以及为每种 spec 实现 SpecTransform Trait  
2. 在 `main.rs` 里使用新的引擎


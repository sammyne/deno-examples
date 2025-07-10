# 基于 OpState 持久化数据

## 两种形式
1. 初始化 JsRuntime 时注入自定义的状态数据，关键点为 `extension` 宏设置以下两个选项
  - `options` 自定义状态数据的类型；
  - `state` 初始化自定义状态数据，并根据需求将其注入其第一个参数 `OpState`
2. 调整 `op2` 修饰的函数，将其第一个参数设置为 `OpState` 类型，在函数内借助 `put`/`try_borrow` 等接口更新状态数据；

## 温馨提示
- 普通 JS 文件不能直接调用 `Deno.core.ops` 的接口

## 参考文献
- https://github.com/denoland/deno_core/blob/0.352.0/core/examples/op2.rs


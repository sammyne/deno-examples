# 快速开始

## 温馨提示
- 拓展的名称前缀不为 `ext:` 时，包装 Rust op 的 JS 桩代码不能通过 `ext:core/ops` 导入 deno-core 内置的 `ops`；
  - 可借助 `Deno.core.ops` 模块直接绕过这个限制
- 拓展的名称前缀为 `ext:` 时，包装 Rust op 的 JS 桩代码能通过 `ext:core/ops` 导入 deno-core 内置的 `ops`；
- 名称前缀为 `ext:` 不能被非 `ext:` 前缀的 ES 模块导入
- 普通 JS 文件不能直接调用 `Deno.core.ops` 的接口

## 参考文献
- https://github.com/denoland/deno_core/tree/0.352.0/core/examples/snapshot

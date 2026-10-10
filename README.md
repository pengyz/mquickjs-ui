# mquickjs-ui

[mquickjs-rs-sdk](https://github.com/pengyz/mquickjs-rs-sdk) 的 **UI 绑定层与轻量应用框架**——
面向 OpenVela（NuttX + LVGL），组件经 **RIDL 定义、胶水自动生成**，LVGL C API
经 **vtable 后端注入**（本 crate 零 LVGL 链接依赖）。

## 现状（v0.1 spike）

- 组件类：`Label` / `Button`（RIDL 定义，`new` 即创建）
- 事件：`onClick(callback())` —— 经 mquickjs-rs 同步回调桥（many-shot、异常受控、GC 安全）
- 后端：`LvglBackend` vtable（create/set_text/set_on_click），宿主 app 注入
- OpenVela 侧 LVGL 后端 + 演示 app：见适配仓 `mquickjs-openvela`（`mqjs_lvgldemo`）

## 构建

兄弟检出约定：`mquickjs-rs-sdk` 与本仓同级（OpenVela 树内为
`external/mquickjs-rs-sdk` 与 `external/mquickjs-ui`）。

```bash
cargo test        # mock 后端端到端测试
```

生成产物已入库（`src/generated/`，对 .ridl 字节确定）；再生成：

```bash
cargo run -q -p ridl-tool --manifest-path ../mquickjs-rs-sdk/Cargo.toml -- \
  module src/mquickjs_ui.ridl src/generated
```

## 演进路线

1. ✅ v0.1：Label/Button 绑定 + 回调桥（本版本）
2. LVGL 后端补全：对齐/样式/更多 widget；`LV_EVENT_DELETE` 句柄失效机制
3. 声明式组件树（快应用 lite）：JS 提交 UI 树对象 → 运行时映射 LVGL
4. 应用框架化：生命周期、页面栈、系统 Feature

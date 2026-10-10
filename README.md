# mquickjs-ui

[mquickjs-rs-sdk](https://github.com/pengyz/mquickjs-rs-sdk) 的 **UI 绑定层与轻量应用框架**——
面向 OpenVela（NuttX + LVGL），组件经 **RIDL 定义、胶水自动生成**，LVGL C API
经 **vtable 后端注入**（本 crate 零 LVGL 链接依赖）。

## 现状（v0.1 spike，B0 已实机验收）

- 组件类：`Label` / `Button`（RIDL 定义，`new` 即创建）
- 事件：`onClick(callback())` —— 经 mquickjs-rs 同步回调桥（many-shot、异常受控、GC 安全）
- 回程：**显式携带 `JSContext*`**（`mqjs_ui_dispatch_click(ctx, handle)`，
  `ContextToken::from_js_ctx`）——eval 返回后 TLS current 栈为空，事件回程
  不依赖 TLS；回调执行期间桥以 `with_current` 压栈（回调体内
  `ContextToken::current()` 语义可用，stdlib console 即依赖此）
- 生命周期：宿主在 `LV_EVENT_DELETE` 调 `mqjs_ui_callback_release(ctx, handle)`
  注销回调桥槽位（B0 风险点 d 实测通过）
- 后端：`LvglBackend` vtable（create/set_text/set_on_click），C 侧经
  `mqjs_ui_backend_attach` 一次性注入
- OpenVela 侧 LVGL 后端 + 演示 app：适配仓 `apps/system/mqjs/lvgldemo/`
  （sim:mqjs_lvgl，四风险点实测见 demo 仓
  `docs/planning/2026/2026-10-10-b0-lvgl-spike-report.md`）

## 构建

兄弟检出约定：`mquickjs-rs-sdk` 与本仓同级（OpenVela 树内为
`external/mquickjs-rs-sdk` 与 `external/mquickjs-ui`）。

```bash
./scripts/run-tests.sh   # mock 后端端到端测试（见下：需要 app 框架产物）
```

生成产物已入库（`src/generated/`，对 .ridl 字节确定）；再生成：

```bash
cargo run -q -p ridl-tool --manifest-path ../mquickjs-rs-sdk/Cargo.toml -- \
  module src/mquickjs_ui.ridl src/generated
```

### 为什么 `cargo test` 不能裸跑

生成的 glue 经 `mquickjs_rs::ridl_js_class_id` 解析类 id（编译期常量），
该模块由 mquickjs-rs 的 build 脚本解析 **app 聚合的
`mquickjs_ridl_register.h`** 生成——SDK 自身框架（demo app 聚合）不含
`mquickjs-ui` 的类。测试二进制还链接框架的 ridl 变体 C stdlib（强
`js_stdlib`，见 `build.rs`）。因此测试需要一个**包含本 crate 类 id 的
app 框架产物**：

- OpenVela 两仓布局下 `scripts/run-tests.sh` 自动发现
  `../../apps/system/mqjs/gen`（其聚合按构造包含 mquickjs-ui；框架产物由
  该 app 仓 SYNC.md 第④步生成，lib/ 与 debug 份不入库需本机构建一次）；
- 任意其它聚合：`MQUICKJS_UI_FRAMEWORK_DIR=<gen目录> ./scripts/run-tests.sh`。

纯构建（`cargo build`，如作为 OpenVela app 依赖被 adapter 静态库消费）
无此要求——链接职责属镜像与测试目标（`build.rs` 按 `MQJS_ENGINE_LINK`
门控）。

## 演进路线

1. ✅ v0.1：Label/Button 绑定 + 回调桥 + 生命周期（B0 spike 验收通过）
2. B1：布局/样式最小面、Container 类、脚本文件装载、更多 widget
3. 声明式组件树（快应用 lite）：JS 提交 UI 树对象 → 运行时映射 LVGL
4. 应用框架化：生命周期、页面栈、系统 Feature

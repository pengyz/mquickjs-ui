//! # mquickjs-ui — OpenVela 轻量 UI 绑定层（LVGL via RIDL）
//!
//! 组件类（Label/Button）由 RIDL 定义、胶水自动生成；LVGL C API 经
//! **vtable 后端注入**（[`backend::attach`]）——本 crate 无 LVGL 链接依赖，
//! 任何镜像可链接；真实后端由宿主 app 启动时注册（见 OpenVela 适配仓
//! `mqjs_lvgldemo`）。
//!
//! 事件链路：`btn.onClick(fn)` → 回调桥（mquickjs-rs CallbackRegistry，
//! JSGCRef 持有 + 同步 many-shot + 异常受控）→ 后端 `set_on_click(obj,
//! handle)`；LVGL 事件触发时后端调用 [`backend::dispatch_click`] 回到 JS。

mquickjs_rs::ridl_include_module!();

pub mod backend;
pub mod impls;

pub use mquickjs_rs::CallbackHandle;

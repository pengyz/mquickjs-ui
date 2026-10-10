//! LVGL 后端 vtable：组件 impl 经此调用平台能力，本 crate 保持零 LVGL
//! 链接依赖。宿主 app（LVGL 侧）启动时调用 [`attach`] 注入一次（进程级
//! 单后端，泄漏式存放——一次性初始化，随进程存活）。
//!
//! 点击事件回程：后端 C trampoline（LVGL 事件回调）调用本 crate 导出的
//! [`mqjs_ui_dispatch_click`]（extern "C"），经 mquickjs-rs 回调桥回到 JS。

use core::ffi::c_char;
use core::ffi::c_void;
use core::sync::atomic::{AtomicPtr, Ordering};

/// LVGL 平台后端（C ABI 函数指针表）。
#[repr(C)]
pub struct LvglBackend {
    /// 在默认屏创建 `lv_label`，返回对象指针。
    pub create_label: unsafe extern "C" fn(text: *const c_char) -> *mut c_void,
    /// 在默认屏创建 `lv_button`（内含 label），返回对象指针。
    pub create_button: unsafe extern "C" fn(text: *const c_char) -> *mut c_void,
    /// 设置 widget 文本。
    pub set_text: unsafe extern "C" fn(obj: *mut c_void, text: *const c_char),
    /// 注册点击事件：`handle` 为回调桥句柄；后端在 CLICKED 事件里回调
    /// `mqjs_ui_dispatch_click(handle)`。
    pub set_on_click: unsafe extern "C" fn(obj: *mut c_void, handle: u32),
}

static BACKEND: AtomicPtr<LvglBackend> = AtomicPtr::new(core::ptr::null_mut());

/// 注入平台后端（进程级一次性；重复注入以最后一次为准）。
pub fn attach(backend: LvglBackend) {
    let boxed = Box::new(backend);
    BACKEND.store(Box::into_raw(boxed), Ordering::Release);
}

/// 访问已注入的后端；未注入返回 `None`（调用方降级为 no-op + 日志）。
pub fn with<R>(f: impl FnOnce(&LvglBackend) -> R) -> Option<R> {
    let ptr = BACKEND.load(Ordering::Acquire);
    if ptr.is_null() {
        return None;
    }
    // SAFETY: attach 之后指针恒有效（泄漏式存放，从不释放）。
    Some(f(unsafe { &*ptr }))
}

/// 导出给 C 后端 trampoline：点击事件回程（LVGL → 回调桥 → JS）。
///
/// # Safety
/// `handle` 必须是此前 `onClick` 注册时经 `CallbackHandle::raw()` 传入的
/// 句柄；宿主需保证本函数仅在 JS 线程（引擎所在线程）调用。
#[no_mangle]
pub unsafe extern "C" fn mqjs_ui_dispatch_click(handle: u32) {
    dispatch_click(handle);
}

/// Rust 侧触发入口（与 extern "C" 版本同语义）。
pub fn dispatch_click(handle: u32) {
    let Some(h) = mquickjs_rs::context::ContextToken::current() else {
        eprintln!("mquickjs-ui: dispatch_click but no live context");
        return;
    };
    if let Err(e) = h.callbacks().invoke(
        mquickjs_rs::CallbackHandle::from_raw(handle),
        &[],
    ) {
        // 回调异常已由 registry 清除；此处仅保证错误可见。
        eprintln!("mquickjs-ui: callback failed: {e}");
    }
}

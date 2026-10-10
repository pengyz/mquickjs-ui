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

/// C 侧后端装配入口：宿主 app 用四个 C 函数指针注入 vtable（与
/// [`attach`] 同语义，供无 Rust 绑定的 LVGL 宿主一次性调用）。
///
/// # Safety
/// 四个函数指针必须非空，且其 C 实现满足 `LvglBackend` 字段注释的契约
/// （线程约定：全部只在 LVGL/JS 共用线程上调用）。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqjs_ui_backend_attach(
    create_label: unsafe extern "C" fn(text: *const c_char) -> *mut c_void,
    create_button: unsafe extern "C" fn(text: *const c_char) -> *mut c_void,
    set_text: unsafe extern "C" fn(obj: *mut c_void, text: *const c_char),
    set_on_click: unsafe extern "C" fn(obj: *mut c_void, handle: u32),
) {
    attach(LvglBackend {
        create_label,
        create_button,
        set_text,
        set_on_click,
    });
}

/// 导出给 C 后端 trampoline：点击事件回程（LVGL → 回调桥 → JS）。
///
/// # Safety
/// - `ctx` 必须是存活的 `JSContext*`（由 `mquickjs_rs::Context::new` 创建、
///   user_data 已设置——即经 adapter 三件套创建的 context），且本函数仅在
///   该 context 销毁前调用；
/// - `handle` 必须是此前 `onClick` 注册时经 `CallbackHandle::raw()` 传入的
///   句柄；宿主需保证本函数仅在 JS 线程（引擎所在线程）调用。
///
/// 为什么显式携带 `ctx` 而不是依赖 `ContextToken::current()`：后者读的是
/// `enter_current()` 维护的 TLS 栈，而 LVGL 事件回调发生在 eval 返回之后
/// （TLS 栈为空）——显式传 ctx 是事件回程唯一可靠的 context 来源。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqjs_ui_dispatch_click(ctx: *mut c_void, handle: u32) {
    dispatch_click(ctx, handle);
}

/// Rust 侧触发入口（与 extern "C" 版本同语义）。
pub fn dispatch_click(ctx: *mut c_void, handle: u32) {
    invoke_callback(ctx, handle);
}

/// 导出给 C 后端：widget 销毁（LV_EVENT_DELETE）时注销回调句柄，释放
/// 回调桥的 JSGCRef 槽位。ctx 约束同 [`mqjs_ui_dispatch_click`]。
///
/// 句柄不存在时静默返回（幂等；重复 DELETE/双注册场景不 panic）。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn mqjs_ui_callback_release(ctx: *mut c_void, handle: u32) {
    let Some(token) = (unsafe {
        mquickjs_rs::context::ContextToken::from_js_ctx(ctx as *mut _)
    }) else {
        eprintln!("mquickjs-ui: callback_release but ctx has no RIDL user_data");
        return;
    };
    token
        .callbacks()
        .unregister(mquickjs_rs::CallbackHandle::from_raw(handle));
}

fn invoke_callback(ctx: *mut c_void, handle: u32) {
    // SAFETY: 见 mqjs_ui_dispatch_click 的 Safety 契约。
    let Some(token) = (unsafe {
        mquickjs_rs::context::ContextToken::from_js_ctx(ctx as *mut _)
    }) else {
        eprintln!("mquickjs-ui: dispatch_click but ctx has no RIDL user_data");
        return;
    };
    // 回调执行期间把该 context 压入 TLS current 栈：回调体内经
    // `ContextToken::current()` 取 context 的实现（如 stdlib console 的
    // print_js_values）才能工作——eval 路径有 enter_current 语义，事件
    // 回程必须由桥补齐同样的语义，否则静默 no-op。
    token.with_current(|| {
        if let Err(e) = token
            .callbacks()
            .invoke(mquickjs_rs::CallbackHandle::from_raw(handle), &[])
        {
            // 回调异常已由 registry 清除；此处仅保证错误可见。
            eprintln!("mquickjs-ui: callback failed: {e}");
        }
    });
}

//! Mock 后端端到端：不依赖 LVGL，验证「RIDL 组件 → 回调桥 → JS」全链路。

use std::ffi::c_char;
use std::ffi::c_void;
use std::sync::Mutex;

/// 记录后端调用序列（mock）。
fn log() -> &'static Mutex<Vec<String>> {
    static LOG: std::sync::OnceLock<Mutex<Vec<String>>> = std::sync::OnceLock::new();
    LOG.get_or_init(|| Mutex::new(Vec::new()))
}

fn record(s: String) {
    log().lock().unwrap().push(s);
}

extern "C" fn mock_create_label(text: *const c_char) -> *mut c_void {
    let c = unsafe { std::ffi::CStr::from_ptr(text) };
    record(format!("create_label:{}", c.to_string_lossy()));
    0x1000 as *mut c_void // 伪对象指针
}

extern "C" fn mock_create_button(text: *const c_char) -> *mut c_void {
    let c = unsafe { std::ffi::CStr::from_ptr(text) };
    record(format!("create_button:{}", c.to_string_lossy()));
    0x2000 as *mut c_void
}

extern "C" fn mock_set_text(obj: *mut c_void, text: *const c_char) {
    let c = unsafe { std::ffi::CStr::from_ptr(text) };
    record(format!("set_text:{:p}:{}", obj, c.to_string_lossy()));
}

extern "C" fn mock_set_on_click(obj: *mut c_void, handle: u32) {
    record(format!("set_on_click:{:p}:{}", obj, handle));
    // C 后端在此注册 LVGL 事件；mock 在测试里手工触发 dispatch。
}

/// 强引用 stdlib 胶水的 console C 入口：build.rs 以 --whole-archive 链入
/// 测试二进制的框架 ridl stdlib 归档（`js_stdlib` 表）引用这些符号，而
/// 依赖 rlib 的成员只有被静态引用才会进入链接（apps/demo 的等价物是其
/// emit() 产出的 register.o 对象；本 crate 是 library，测试负责自证）。
#[used]
static _CONSOLE_SYMBOL_KEEPALIVE: [unsafe extern "C" fn(
    *mut mquickjs_rs::mquickjs_ffi::JSContext,
    *mut mquickjs_rs::mquickjs_ffi::JSValue,
    core::ffi::c_int,
    *mut mquickjs_rs::mquickjs_ffi::JSValue,
) -> mquickjs_rs::mquickjs_ffi::JSValue; 3] = [
    stdlib::js_global_singleton_console_log,
    stdlib::js_global_singleton_console_error,
    stdlib::js_global_singleton_console_get_enabled,
];

#[test]
fn component_binding_end_to_end_via_mock_backend() {
    // 1) 注入 mock 后端
    mquickjs_ui::backend::attach(mquickjs_ui::backend::LvglBackend {
        create_label: mock_create_label,
        create_button: mock_create_button,
        set_text: mock_set_text,
        set_on_click: mock_set_on_click,
    });

    // 2) 引擎 + JS：创建组件、注册点击回调（真实回调桥）
    let mut ctx = mquickjs_rs::Context::new(1024 * 1024).expect("create ctx");
    ctx.eval(
        r#"
        globalThis.log = [];
        var label = new Label();
        label.setText("Hello mquickjs-ui");
        var btn = new Button();
        btn.setText("Click me");
        btn.onClick(function () {
            globalThis.log.push("clicked");
        });
    "#,
    )
    .expect("eval ui script");

    // 3) mock 后端记录了创建调用：构造器以默认文本创建，setText 单独下发
    //    （vtable 契约：create_* 只收默认文本，文本更新走 set_text）。
    {
        let guard = log().lock().unwrap();
        assert!(
            guard.iter().any(|s| s == "create_label:label"),
            "label create missing: {:?}",
            *guard
        );
        assert!(
            guard.iter().any(|s| s == "create_button:button"),
            "button create missing: {:?}",
            *guard
        );
        assert!(
            guard
                .iter()
                .any(|s| s == "set_text:0x1000:Hello mquickjs-ui"),
            "label set_text missing: {:?}",
            *guard
        );
    }

    // 4) 触发点击（模拟 LVGL 事件回程）→ JS 回调执行 → log 记录
    //    句柄从 set_on_click 的 mock 记录里取（真实后端由 C 侧保存）；
    //    ctx 显式携带（事件回程发生在 eval 返回后，TLS current 栈为空，
    //    与真实 LVGL trampoline 的调用形态一致）。
    let (ctx_ptr, handle) = {
        let guard = log().lock().unwrap();
        let handle = guard
            .iter()
            .find_map(|s: &String| {
                s.strip_prefix("set_on_click:")
                    .map(|rest| rest.split(':').nth(1).unwrap().to_string())
            })
            .expect("set_on_click recorded");
        (ctx.token().ctx as *mut c_void, handle.parse::<u32>().unwrap())
    };
    mquickjs_ui::backend::dispatch_click(ctx_ptr, handle);

    let hits = ctx.eval("String(globalThis.log.length)").unwrap();
    assert_eq!(hits, "1", "JS callback should have fired once");

    // 5) 注销路径（LV_EVENT_DELETE 对应）：再次 dispatch 必须 no-op，
    //    且不触碰已释放的槽位。
    // SAFETY: ctx_ptr 是存活 context 的 JSContext*（本测试的契约）。
    unsafe { mquickjs_ui::backend::mqjs_ui_callback_release(ctx_ptr, handle) };
    mquickjs_ui::backend::dispatch_click(ctx_ptr, handle);

    let hits = ctx.eval("String(globalThis.log.length)").unwrap();
    assert_eq!(hits, "1", "released handle must not fire again");
}

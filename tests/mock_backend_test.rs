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

    // 3) mock 后端记录了创建调用
    let log = log().lock().unwrap();
    assert!(
        log.iter().any(|s| s == "create_label:Hello mquickjs-ui"),
        "label create missing: {:?}",
        *log
    );
    assert!(
        log.iter().any(|s| s == "create_button:Click me"),
        "button create missing: {:?}",
        *log
    );
    drop(log);

    // 4) 触发点击（模拟 LVGL 事件回程）→ JS 回调执行 → log 记录
    //    句柄从 set_on_click 的 mock 记录里取（真实后端由 C 侧保存）。
    let handle = {
        let l = log().lock().unwrap();
        l.iter()
            .find_map(|s| s.strip_prefix("set_on_click:").map(|rest| {
                rest.split(':').nth(1).unwrap().to_string()
            }))
            .expect("set_on_click recorded")
    };
    mquickjs_ui::dispatch_click(handle.parse::<u32>().unwrap());

    let hits = ctx.eval("String(globalThis.log.length)").unwrap();
    assert_eq!(hits, "1", "JS callback should have fired once");
}

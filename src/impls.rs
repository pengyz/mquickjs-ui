//! 组件 impl：RIDL 生成 trait 的实现体，经 vtable 调用平台后端。
//!
//! 后端未注入时全部方法安全降级为 no-op + 日志（如 sim 无 LVGL 的
//! `js` builtin 里组件类照常可用，只是不绘制）。

use crate::api::{ButtonClass, LabelClass};
use crate::backend;
use core::ffi::c_void;
use std::ffi::CString;

fn cstr(s: &str) -> CString {
    // 内嵌/开发者字符串含 NUL 属于编程错误：截断即可（不 panic 保 UI 线程）。
    let mut v = s.as_bytes().to_vec();
    v.retain(|&b| b != 0);
    v.push(0);
    unsafe { CString::from_vec_unchecked(v) }
}

// ---------------------------------------------------------------------------
// Label
// ---------------------------------------------------------------------------

pub struct LabelImpl {
    obj: *mut c_void,
}

impl LabelImpl {
    fn dead(&self) -> bool {
        self.obj.is_null()
    }
}

pub fn label_constructor() -> Box<dyn LabelClass> {
    let obj = backend::with(|b| unsafe { (b.create_label)(cstr("label").as_ptr()) });
    Box::new(LabelImpl {
        obj: obj.unwrap_or(core::ptr::null_mut()),
    })
}

impl LabelClass for LabelImpl {
    fn set_text(&mut self, text: String) {
        if self.dead() {
            eprintln!("mquickjs-ui: Label::setText on null object (backend missing?)");
            return;
        }
        let c = cstr(&text);
        backend::with(|b| unsafe { (b.set_text)(self.obj, c.as_ptr()) });
    }
}

// ---------------------------------------------------------------------------
// Button
// ---------------------------------------------------------------------------

pub struct ButtonImpl {
    obj: *mut c_void,
}

pub fn button_constructor() -> Box<dyn ButtonClass> {
    let obj = backend::with(|b| unsafe { (b.create_button)(cstr("button").as_ptr()) });
    Box::new(ButtonImpl {
        obj: obj.unwrap_or(core::ptr::null_mut()),
    })
}

impl ButtonClass for ButtonImpl {
    fn set_text(&mut self, text: String) {
        if self.obj.is_null() {
            eprintln!("mquickjs-ui: Button::setText on null object");
            return;
        }
        let c = cstr(&text);
        backend::with(|b| unsafe { (b.set_text)(self.obj, c.as_ptr()) });
    }

    fn on_click<'ctx>(
        &mut self,
        _env: &mut mquickjs_rs::Env<'ctx>,
        cb: mquickjs_rs::CallbackHandle,
    ) {
        if self.obj.is_null() {
            eprintln!("mquickjs-ui: Button::onClick on null object");
            return;
        }
        backend::with(|b| unsafe { (b.set_on_click)(self.obj, cb.raw()) });
    }
}

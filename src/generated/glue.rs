// Auto-generated Rust glue code for RIDL interfaces
#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use mquickjs_rs::mquickjs_ffi::{JSContext, JSValue};

// Mode-agnostic prelude (std and no_std consumers): CString/c_int/Vec/Box are
// re-exported from alloc/core by mquickjs-rs, so the generated glue never
// depends on the std prelude (bare-metal RIDL, see mquickjs-rs::glue_prelude).
use mquickjs_rs::glue_prelude::*;

#[allow(unused_imports)]
use crate::impls::*;

// Module initializer API
pub fn initialize_module() {
    __ridl_symbols::ensure_symbols();
}

// Fill per-context RIDL extension slots for this module.
// Called by the app-level aggregated ridl_context_init.
//
// This API must not reference any app crate types.
pub fn ridl_module_context_init(w: &mut dyn mquickjs_rs::ridl_runtime::RidlSlotWriter) {
    let _ = w;
}

// Erased ctx slot vtables consumed by the app-side aggregated ridl_context_init.
//
// - singleton slots:
//   - Module must provide a Rust-level creator: `crate::impls::create_<name>_singleton() -> Box<dyn Trait>`.
//   - Glue exports the C ABI creator: `#[no_mangle] extern "C" fn ridl_create_<name>_singleton() -> *mut c_void`.
//     Return value is an erased *thin* pointer produced by `Box::into_raw(Box<Box<dyn Trait>>)`.
// - class proto slots: module provides `crate::ridl_create_proto_<class>() -> <Impl as Class>::Proto`

mod __ridl_symbols {
    #[allow(unused_imports)]
    use super::*;

    // Keep glue symbols linked.
    // NOTE: we intentionally use `extern` + function pointers here to avoid depending on
    // any generated module layout (`crate::generated::...`).
    unsafe extern "C" {
    }

    pub fn ensure_symbols() {
    }
}

#[inline]
fn js_throw_type_error(ctx: *mut JSContext, msg: &str) -> JSValue {
    let cstr = CString::new(msg).unwrap_or_else(|_| CString::new("TypeError").unwrap());
    // mquickjs exposes JS_ThrowTypeError as a macro; bindings expose JS_ThrowError.
    // JS_CLASS_TYPE_ERROR is stable in this fork.
    unsafe {
        mquickjs_rs::mquickjs_ffi::JS_ThrowError(
            ctx,
            mquickjs_rs::mquickjs_ffi::JSObjectClassEnum_JS_CLASS_TYPE_ERROR,
            cstr.as_ptr(),
        )
    }
}

// Call into crate-local implementations.
// The module crate is expected to provide `crate::impls::*`.

// Glue implementations for functions

// Glue implementations for singletons

// Interface implementations

// -----------------------------------------------------------------------------
// RIDL class glue (constructors/finalizers/methods/properties/proto properties)
// -----------------------------------------------------------------------------

// Store a pointer to a Box<dyn Trait> (thin pointer), so it can round-trip through c_void safely.
// Contract: JSValue receiver must be validated via JS_GetClassID prior to dereferencing.

#[allow(dead_code)]
#[inline]
unsafe fn ridl_boxed_label_to_js(
    ctx: *mut mquickjs_rs::mquickjs_ffi::JSContext,
    inst: Box<dyn crate::api::LabelClass>,
) -> mquickjs_rs::mquickjs_ffi::JSValue {
    // Create JS object of the RIDL class.
    let obj = unsafe {
        mquickjs_rs::mquickjs_ffi::JS_NewObjectClassUser(
            ctx,
            mquickjs_rs::ridl_js_class_id::JS_CLASS_GLOBAL_LABEL,
        )
    };
    if (obj as u32) & ((1u32 << (mquickjs_rs::mquickjs_ffi::JS_TAG_SPECIAL_BITS as u32)) - 1)
        == (mquickjs_rs::mquickjs_ffi::JS_TAG_EXCEPTION as u32)
    {
        return obj;
    }

    let holder: Box<Box<dyn crate::api::LabelClass>> = Box::new(inst);
    let holder_ptr: *mut Box<dyn crate::api::LabelClass> = Box::into_raw(holder);

    unsafe {
        mquickjs_rs::mquickjs_ffi::JS_SetOpaque(ctx, obj, holder_ptr as *mut core::ffi::c_void);
    }

    // NOTE: boxed-return path does not install JS-only instance fields.
    obj
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn js_global_class_label_constructor(
    ctx: *mut mquickjs_rs::mquickjs_ffi::JSContext,
    this_val: *mut mquickjs_rs::mquickjs_ffi::JSValue,
    argc: c_int,
    argv: *mut mquickjs_rs::mquickjs_ffi::JSValue,
) -> mquickjs_rs::mquickjs_ffi::JSValue {
    let _ = this_val;
    let _ = argc;
    let _ = argv;

    // Create JS object of the RIDL class.
    let obj = unsafe {
        mquickjs_rs::mquickjs_ffi::JS_NewObjectClassUser(
            ctx,
            mquickjs_rs::ridl_js_class_id::JS_CLASS_GLOBAL_LABEL,
        )
    };
    if (obj as u32) & ((1u32 << (mquickjs_rs::mquickjs_ffi::JS_TAG_SPECIAL_BITS as u32)) - 1)
        == (mquickjs_rs::mquickjs_ffi::JS_TAG_EXCEPTION as u32)
    {
        return obj;
    }

    // Contract: module crate provides `crate::impls::label_constructor(...)`.
    let inst: Box<dyn crate::api::LabelClass> =
        crate::impls::label_constructor()
        ;

    let holder: Box<Box<dyn crate::api::LabelClass>> = Box::new(inst);
    let holder_ptr: *mut Box<dyn crate::api::LabelClass> = Box::into_raw(holder);

    unsafe {
        mquickjs_rs::mquickjs_ffi::JS_SetOpaque(ctx, obj, holder_ptr as *mut core::ffi::c_void);
    }

    // Install JS-only instance fields declared in RIDL.

    obj
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn js_global_class_label_finalizer(
    ctx: *mut mquickjs_rs::mquickjs_ffi::JSContext,
    opaque: *mut core::ffi::c_void,
) {
    // Finalizer must not call JS APIs.
    let _ = ctx;

    let holder_ptr = opaque as *mut Box<dyn crate::api::LabelClass>;
    if !holder_ptr.is_null() {
        unsafe {
            let holder: Box<Box<dyn crate::api::LabelClass>> = Box::from_raw(holder_ptr);
            drop(holder);
        }
    }
}

// NOTE: 此处**不再生成** class 的 `gc_mark` FFI。
//
// 原因一（致命 ABI 缺陷）：该函数此前被生成成 **4 参**
//   `(_ctx, _obj, opaque, mf)`
// 而引擎契约是 **3 参** `(ctx, opaque, mf)`
//   （`mquickjs.h:232` 的 `JSCMark`；调用点 `mquickjs.c:12174`）。
// 寄存器错位后 `opaque` 实参收到的是 `&mf`（栈地址、非空），
// 被当作 `Box<dyn Trait>` fat pointer 解引用 → 虚表跳转 → SIGSEGV。
// 复现见 tests/gc_traced.rs::traced_node_reachable_during_gc_invokes_gc_mark。
// 参见 docs/knowledge/gotcha_mquickjs_gc_mark_signature.md。
//
// 原因二（已冗余）：`Traced<T>` 现在基于引擎的 `JSGCRef`，该链由引擎在
// mark 与重定位两个阶段自动扫描，无需再由 class 回调手动标记。
// 因此注册表中的 gc_mark 槽位恒为 NULL。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn js_global_class_label_set_text(
    ctx: *mut mquickjs_rs::mquickjs_ffi::JSContext,
    this_val: *mut mquickjs_rs::mquickjs_ffi::JSValue,
    argc: c_int,
    argv: *mut mquickjs_rs::mquickjs_ffi::JSValue,
) -> mquickjs_rs::mquickjs_ffi::JSValue {
    let _ = argc;
    let _ = argv;
    let this_val = unsafe { *this_val };
    // Validate receiver.
    let cid = unsafe { mquickjs_rs::mquickjs_ffi::JS_GetClassID(ctx, this_val) };
    if cid < mquickjs_rs::mquickjs_ffi::JSObjectClassEnum_JS_CLASS_USER as i32 {
        return js_throw_type_error(ctx, "invalid receiver");
    }

    let holder_ptr = unsafe { mquickjs_rs::mquickjs_ffi::JS_GetOpaque(ctx, this_val) }
        as *mut Box<dyn crate::api::LabelClass>;
    if holder_ptr.is_null() {
        return js_throw_type_error(ctx, "missing opaque");
    }
        if argc < 1 { return js_throw_type_error(ctx, "missing argument: text"); }
    let _v = unsafe { *argv.add(0) };
    let v: JSValue = _v;
    if unsafe { mquickjs_rs::mquickjs_ffi::JS_IsString(ctx, v) } == 0 { return js_throw_type_error(ctx, "invalid string argument: text"); }
    let mut ptr_buf = mquickjs_rs::mquickjs_ffi::JSCStringBuf { buf: [0u8; 5] };
    let ptr_ptr = unsafe { mquickjs_rs::mquickjs_ffi::JS_ToCString(ctx, v, &mut ptr_buf as *mut _) };
    if ptr_ptr.is_null() { return js_throw_type_error(ctx, "invalid string argument: text"); }
    let ptr: *const core::ffi::c_char = ptr_ptr;
    let s = unsafe { core::ffi::CStr::from_ptr(ptr) };
    let text: String = s.to_string_lossy().into_owned();


    // NOTE: 此处**没有**异步分支。
    //
    // 异步方法仅支持 singleton；class 方法上的 @nonCancellable / @timeout
    // 会在 RIDL 校验阶段被拒绝（validator::validate_async_decorator_scope），
    // 因此这里只生成同步路径。
    //
    // （此前这里曾生成异步 glue，但它是从未被编译过的死代码：既按位拷贝了
    //   内含 Mutex 的 AsyncTaskManager（UB），又把非 Send 的 opaque 指针
    //   跨线程传入 worker。）
    // Synchronous method
    let inst: &mut dyn crate::api::LabelClass = unsafe { &mut **holder_ptr };
    let result = inst.set_text(text
    );
    let _ = result;
let _ = ctx;
let _ = argc;
let _ = argv;
mquickjs_rs::mquickjs_ffi::JS_UNDEFINED
}

// Store a pointer to a Box<dyn Trait> (thin pointer), so it can round-trip through c_void safely.
// Contract: JSValue receiver must be validated via JS_GetClassID prior to dereferencing.

#[allow(dead_code)]
#[inline]
unsafe fn ridl_boxed_button_to_js(
    ctx: *mut mquickjs_rs::mquickjs_ffi::JSContext,
    inst: Box<dyn crate::api::ButtonClass>,
) -> mquickjs_rs::mquickjs_ffi::JSValue {
    // Create JS object of the RIDL class.
    let obj = unsafe {
        mquickjs_rs::mquickjs_ffi::JS_NewObjectClassUser(
            ctx,
            mquickjs_rs::ridl_js_class_id::JS_CLASS_GLOBAL_BUTTON,
        )
    };
    if (obj as u32) & ((1u32 << (mquickjs_rs::mquickjs_ffi::JS_TAG_SPECIAL_BITS as u32)) - 1)
        == (mquickjs_rs::mquickjs_ffi::JS_TAG_EXCEPTION as u32)
    {
        return obj;
    }

    let holder: Box<Box<dyn crate::api::ButtonClass>> = Box::new(inst);
    let holder_ptr: *mut Box<dyn crate::api::ButtonClass> = Box::into_raw(holder);

    unsafe {
        mquickjs_rs::mquickjs_ffi::JS_SetOpaque(ctx, obj, holder_ptr as *mut core::ffi::c_void);
    }

    // NOTE: boxed-return path does not install JS-only instance fields.
    obj
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn js_global_class_button_constructor(
    ctx: *mut mquickjs_rs::mquickjs_ffi::JSContext,
    this_val: *mut mquickjs_rs::mquickjs_ffi::JSValue,
    argc: c_int,
    argv: *mut mquickjs_rs::mquickjs_ffi::JSValue,
) -> mquickjs_rs::mquickjs_ffi::JSValue {
    let _ = this_val;
    let _ = argc;
    let _ = argv;

    // Create JS object of the RIDL class.
    let obj = unsafe {
        mquickjs_rs::mquickjs_ffi::JS_NewObjectClassUser(
            ctx,
            mquickjs_rs::ridl_js_class_id::JS_CLASS_GLOBAL_BUTTON,
        )
    };
    if (obj as u32) & ((1u32 << (mquickjs_rs::mquickjs_ffi::JS_TAG_SPECIAL_BITS as u32)) - 1)
        == (mquickjs_rs::mquickjs_ffi::JS_TAG_EXCEPTION as u32)
    {
        return obj;
    }

    // Contract: module crate provides `crate::impls::button_constructor(...)`.
    let inst: Box<dyn crate::api::ButtonClass> =
        crate::impls::button_constructor()
        ;

    let holder: Box<Box<dyn crate::api::ButtonClass>> = Box::new(inst);
    let holder_ptr: *mut Box<dyn crate::api::ButtonClass> = Box::into_raw(holder);

    unsafe {
        mquickjs_rs::mquickjs_ffi::JS_SetOpaque(ctx, obj, holder_ptr as *mut core::ffi::c_void);
    }

    // Install JS-only instance fields declared in RIDL.

    obj
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn js_global_class_button_finalizer(
    ctx: *mut mquickjs_rs::mquickjs_ffi::JSContext,
    opaque: *mut core::ffi::c_void,
) {
    // Finalizer must not call JS APIs.
    let _ = ctx;

    let holder_ptr = opaque as *mut Box<dyn crate::api::ButtonClass>;
    if !holder_ptr.is_null() {
        unsafe {
            let holder: Box<Box<dyn crate::api::ButtonClass>> = Box::from_raw(holder_ptr);
            drop(holder);
        }
    }
}

// NOTE: 此处**不再生成** class 的 `gc_mark` FFI。
//
// 原因一（致命 ABI 缺陷）：该函数此前被生成成 **4 参**
//   `(_ctx, _obj, opaque, mf)`
// 而引擎契约是 **3 参** `(ctx, opaque, mf)`
//   （`mquickjs.h:232` 的 `JSCMark`；调用点 `mquickjs.c:12174`）。
// 寄存器错位后 `opaque` 实参收到的是 `&mf`（栈地址、非空），
// 被当作 `Box<dyn Trait>` fat pointer 解引用 → 虚表跳转 → SIGSEGV。
// 复现见 tests/gc_traced.rs::traced_node_reachable_during_gc_invokes_gc_mark。
// 参见 docs/knowledge/gotcha_mquickjs_gc_mark_signature.md。
//
// 原因二（已冗余）：`Traced<T>` 现在基于引擎的 `JSGCRef`，该链由引擎在
// mark 与重定位两个阶段自动扫描，无需再由 class 回调手动标记。
// 因此注册表中的 gc_mark 槽位恒为 NULL。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn js_global_class_button_set_text(
    ctx: *mut mquickjs_rs::mquickjs_ffi::JSContext,
    this_val: *mut mquickjs_rs::mquickjs_ffi::JSValue,
    argc: c_int,
    argv: *mut mquickjs_rs::mquickjs_ffi::JSValue,
) -> mquickjs_rs::mquickjs_ffi::JSValue {
    let _ = argc;
    let _ = argv;
    let this_val = unsafe { *this_val };
    // Validate receiver.
    let cid = unsafe { mquickjs_rs::mquickjs_ffi::JS_GetClassID(ctx, this_val) };
    if cid < mquickjs_rs::mquickjs_ffi::JSObjectClassEnum_JS_CLASS_USER as i32 {
        return js_throw_type_error(ctx, "invalid receiver");
    }

    let holder_ptr = unsafe { mquickjs_rs::mquickjs_ffi::JS_GetOpaque(ctx, this_val) }
        as *mut Box<dyn crate::api::ButtonClass>;
    if holder_ptr.is_null() {
        return js_throw_type_error(ctx, "missing opaque");
    }
        if argc < 1 { return js_throw_type_error(ctx, "missing argument: text"); }
    let _v = unsafe { *argv.add(0) };
    let v: JSValue = _v;
    if unsafe { mquickjs_rs::mquickjs_ffi::JS_IsString(ctx, v) } == 0 { return js_throw_type_error(ctx, "invalid string argument: text"); }
    let mut ptr_buf = mquickjs_rs::mquickjs_ffi::JSCStringBuf { buf: [0u8; 5] };
    let ptr_ptr = unsafe { mquickjs_rs::mquickjs_ffi::JS_ToCString(ctx, v, &mut ptr_buf as *mut _) };
    if ptr_ptr.is_null() { return js_throw_type_error(ctx, "invalid string argument: text"); }
    let ptr: *const core::ffi::c_char = ptr_ptr;
    let s = unsafe { core::ffi::CStr::from_ptr(ptr) };
    let text: String = s.to_string_lossy().into_owned();


    // NOTE: 此处**没有**异步分支。
    //
    // 异步方法仅支持 singleton；class 方法上的 @nonCancellable / @timeout
    // 会在 RIDL 校验阶段被拒绝（validator::validate_async_decorator_scope），
    // 因此这里只生成同步路径。
    //
    // （此前这里曾生成异步 glue，但它是从未被编译过的死代码：既按位拷贝了
    //   内含 Mutex 的 AsyncTaskManager（UB），又把非 Send 的 opaque 指针
    //   跨线程传入 worker。）
    // Synchronous method
    let inst: &mut dyn crate::api::ButtonClass = unsafe { &mut **holder_ptr };
    let result = inst.set_text(text
    );
    let _ = result;
let _ = ctx;
let _ = argc;
let _ = argv;
mquickjs_rs::mquickjs_ffi::JS_UNDEFINED
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn js_global_class_button_on_click(
    ctx: *mut mquickjs_rs::mquickjs_ffi::JSContext,
    this_val: *mut mquickjs_rs::mquickjs_ffi::JSValue,
    argc: c_int,
    argv: *mut mquickjs_rs::mquickjs_ffi::JSValue,
) -> mquickjs_rs::mquickjs_ffi::JSValue {
    let _ = argc;
    let _ = argv;
    let this_val = unsafe { *this_val };
    // Validate receiver.
    let cid = unsafe { mquickjs_rs::mquickjs_ffi::JS_GetClassID(ctx, this_val) };
    if cid < mquickjs_rs::mquickjs_ffi::JSObjectClassEnum_JS_CLASS_USER as i32 {
        return js_throw_type_error(ctx, "invalid receiver");
    }

    let holder_ptr = unsafe { mquickjs_rs::mquickjs_ffi::JS_GetOpaque(ctx, this_val) }
        as *mut Box<dyn crate::api::ButtonClass>;
    if holder_ptr.is_null() {
        return js_throw_type_error(ctx, "missing opaque");
    }
    let Some(h) = (unsafe { mquickjs_rs::context::ContextToken::from_js_ctx(ctx) }) else {
        return js_throw_type_error(ctx, "missing ctx user_data (call ridl_context_init)");
    };

    let scope = h.enter_scope();
    let mut env = mquickjs_rs::Env::new(&scope);
        if argc < 1 { return js_throw_type_error(ctx, "missing argument: cb"); }
    let _v = unsafe { *argv.add(0) };
    let v: JSValue = _v;
    if unsafe { mquickjs_rs::mquickjs_ffi::JS_IsFunction(ctx, v) } == 0 { return js_throw_type_error(ctx, "invalid callback argument: cb"); }
    let cb_val = scope.value(v);
    let cb_fn = match cb_val.try_into_function(&scope) { Ok(f) => f, Err(_) => return js_throw_type_error(ctx, "invalid callback argument: cb") };
    let cb: mquickjs_rs::CallbackHandle = h.callbacks().register(&scope, cb_fn);


    // NOTE: 此处**没有**异步分支。
    //
    // 异步方法仅支持 singleton；class 方法上的 @nonCancellable / @timeout
    // 会在 RIDL 校验阶段被拒绝（validator::validate_async_decorator_scope），
    // 因此这里只生成同步路径。
    //
    // （此前这里曾生成异步 glue，但它是从未被编译过的死代码：既按位拷贝了
    //   内含 Mutex 的 AsyncTaskManager（UB），又把非 Send 的 opaque 指针
    //   跨线程传入 worker。）
    // Synchronous method
    let inst: &mut dyn crate::api::ButtonClass = unsafe { &mut **holder_ptr };
    let result = inst.on_click(
        &mut env, cb
    );
    let _ = result;
let _ = ctx;
let _ = argc;
let _ = argv;
mquickjs_rs::mquickjs_ffi::JS_UNDEFINED
}

// -----------------------------------------------------------------------------
// RIDL named callback trampolines（切片 2 同步回调桥的 C 事件入口）
//
// 每个具名 `callback <name>(<params>);` 生成一个 C 可调用入口：
//     void mqjs_cb_<name>_invoke(JSContext *ctx, uint32_t handle, <c params>);
//
// - C 声明只放 mquickjs_ridl_api.h（运行时 C TU 的唯一公共入口）；
//   register.h 供 ROM host tool，不声明 —— 吸取 gc_mark 双头声明教训。
// - `handle` 是 Rust impl 从 RIDL 方法收到的 CallbackHandle
//   （经 `.raw()` 存入 C user_data 后原样传回）。
// - v1 无回调返回值（设计非目标），trampoline 返回 void；
//   JS 异常不穿透 C（长跳禁令）：invoke 已清扫 pending exception。
//
// 【异常/失败可见性策略（切片 1 审查遗留的落位）】
// - std 构建：`eprintln!` 输出到 stderr —— 与 async_bridge、Context::drop
//   的既有日志惯例一致；
// - no_std 构建：生成 glue 层无日志设施（引擎 printf 桥属 stdlib 模块的
//   impl 层，不是 glue 的依赖面），失败静默。invoke 保证返回 Err 时 ctx
//   无残留异常、可继续工作，引擎状态不受影响；裸机集成方如需可观测性，
//   应在 impl 层自持句柄并直接调用 registry（可拿到 Result）。
// -----------------------------------------------------------------------------
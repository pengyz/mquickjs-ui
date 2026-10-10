// Auto-generated Rust API (traits/types) for RIDL module: mquickjs_ui
//
// IMPORTANT:
// - This file is generated into OUT_DIR and is not meant to be edited.
// - User implementations should live in the module crate sources (e.g. src/impls.rs or ../<module>_impl.rs).
// - This file must remain free of JSValue conversion glue. Keep it as pure Rust declarations.

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

// Mode-agnostic prelude (std and no_std consumers), see mquickjs-rs::glue_prelude.
use mquickjs_rs::glue_prelude::*;

// Complex type declarations generated from RIDL.

// -----------------------------------------------------------------------------
// RIDL using aliases
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// RIDL enum declarations
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// RIDL struct declarations
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// RIDL class API (traits + proto FFI declarations)
// -----------------------------------------------------------------------------

// Class Label
//
// NOTE: keep API traits object-safe and context-agnostic.
// Any JS-context borrowing must stay at method level (via `Env<'_>`).
pub trait LabelClass {
    fn set_text<'ctx>(
        &mut self,
        text: String
    ) -> ();
}

// Class Button
//
// NOTE: keep API traits object-safe and context-agnostic.
// Any JS-context borrowing must stay at method level (via `Env<'_>`).
pub trait ButtonClass {
    fn set_text<'ctx>(
        &mut self,
        text: String
    ) -> ();
    fn on_click<'ctx>(
        &mut self,
        env: &mut mquickjs_rs::Env<'ctx>,
        cb: mquickjs_rs::CallbackHandle
    ) -> ();
}

// -----------------------------------------------------------------------------
// RIDL class opaque structs (GC-traced fields)
// -----------------------------------------------------------------------------

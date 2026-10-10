//! 双模式产物装配（对齐 ridl-modules/stdlib 的先例）：
//! 1. `MQUICKJS_RIDL_TOOL` 设置 → 调用宿主工具生成（SDK 开发流，权威）；
//! 2. 未设置 → 拷贝仓内 `src/generated/{api,glue}.rs`（对同一 .ridl 字节确定），
//!    让独立构建零宿主工具依赖。

use std::{env, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=src/mquickjs_ui.ridl");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    if let Some(ridl_tool) = env::var("MQUICKJS_RIDL_TOOL")
        .ok()
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
    {
        println!("cargo:rerun-if-changed={}", ridl_tool.display());
        let status = Command::new(&ridl_tool)
            .arg("module")
            .arg("src/mquickjs_ui.ridl")
            .arg(&out_dir)
            .status()
            .unwrap_or_else(|e| panic!("failed to run {}: {e}", ridl_tool.display()));
        if !status.success() {
            panic!("ridl-tool failed (exit={:?})", status.code());
        }
        return;
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let generated = manifest_dir.join("src/generated");

    for file in ["api.rs", "glue.rs"] {
        println!("cargo:rerun-if-changed=generated/{file}");
        let src = generated.join(file);
        let dst = out_dir.join(file);
        std::fs::copy(&src, &dst)
            .unwrap_or_else(|e| panic!("failed to copy {} -> {}: {e}", src.display(), dst.display()));
    }
}

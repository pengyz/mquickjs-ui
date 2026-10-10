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

    // RIDL 变体 C stdlib 链接（leaf 职责，先例 apps/demo/build.rs）：
    // 测试/独立二进制目标需要强 `js_stdlib`（其表引用本 crate 类胶水的 C
    // 入口），经 --whole-archive 链接框架 ridl 变体归档。
    //
    // - MQJS_ENGINE_LINK=external（OpenVela adapter 消费形态）：镜像 C 侧
    //   以源码级集成提供 stdlib（mqjs_stdlib_impl.c 编进镜像），这里**不得**
    //   链接框架归档（U7 重复符号防护；staticlib 目标本无链接步，参数若
    //   传播反而会污染下游链接）。
    // - 其余（standalone cargo test）：需要框架产物（含本 crate 类 id 的
    //   mquickjs_ridl_register.h 派生的 libmquickjs_stdlib_ridl.a）。产物
    //   缺失时不 panic（library crate 的纯构建无需链接），仅提示——测试
    //   链接失败的恢复方法见 scripts/run-tests.sh。
    println!("cargo:rerun-if-env-changed=MQJS_ENGINE_LINK");
    if env::var("MQJS_ENGINE_LINK").as_deref() != Ok("external") {
        let lib = mquickjs_ridl_glue::ridl_stdlib_lib_path();
        if lib.exists() {
            mquickjs_ridl_glue::emit_native_stdlib_link();
        } else {
            println!(
                "cargo:warning=mquickjs-ui: RIDL native stdlib not found at {} \
                 -- `cargo test` will fail to link (js_stdlib); \
                 see scripts/run-tests.sh for the framework setup",
                lib.display()
            );
        }
    }
}

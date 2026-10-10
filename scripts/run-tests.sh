#!/usr/bin/env bash
# mquickjs-ui standalone test runner.
#
# cargo test needs a RIDL **framework** whose mquickjs_ridl_register.h contains
# this crate's class ids (JS_CLASS_GLOBAL_LABEL/BUTTON): the generated glue
# resolves class ids through mquickjs-rs, which parses that header from the
# framework include dir, and the test binary links the framework's ridl-variant
# stdlib (js_stdlib). The SDK's own framework (built from its demo app) does
# NOT contain mquickjs-ui classes.
#
# Framework discovery order:
#   1. $MQUICKJS_UI_FRAMEWORK_DIR — a directory containing mquickjs.build.toml
#      whose gen/target/mquickjs-build/framework/<triple>/<mode>/ridl artifacts
#      were generated from an app aggregate that includes mquickjs-ui.
#   2. The OpenVela two-repo layout sibling: <this repo>/../apps/system/mqjs/gen
#      (repo manifest places mquickjs-ui at external/mquickjs-ui). The
#      framework artifacts there are produced by the app's SYNC.md flow (step ④
#      `mquickjs-build build --ridl-register-h <app>/gen/mquickjs_ridl_register.h`),
#      which includes mquickjs-ui classes by construction.
#
# Usage: scripts/run-tests.sh [-- <extra cargo args>]
set -euo pipefail

UI_DIR="$(cd "$(dirname "$0")/.." && pwd)"

if [ -n "${MQUICKJS_UI_FRAMEWORK_DIR:-}" ]; then
    FRAMEWORK_DIR="$MQUICKJS_UI_FRAMEWORK_DIR"
elif [ -d "$UI_DIR/../../apps/system/mqjs/gen" ]; then
    FRAMEWORK_DIR="$UI_DIR/../../apps/system/mqjs/gen"
else
    echo "error: no framework found. Set MQUICKJS_UI_FRAMEWORK_DIR to a gen/ dir" >&2
    echo "whose aggregate includes mquickjs-ui (see scripts header / README)." >&2
    exit 1
fi

BUILD_TOML="$FRAMEWORK_DIR/mquickjs.build.toml"
if [ ! -f "$BUILD_TOML" ]; then
    echo "error: $BUILD_TOML not found" >&2
    exit 1
fi

echo "== framework: $FRAMEWORK_DIR"
cd "$UI_DIR"
MQUICKJS_BUILD_TOML="$BUILD_TOML" \
MQUICKJS_RIDL_TARGET_DIR="$FRAMEWORK_DIR/target" \
    cargo test "$@"

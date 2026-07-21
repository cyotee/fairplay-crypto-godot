#!/usr/bin/env bash
# Build Godot HTML5 GDExtension side modules (nothreads + optional threads).
# Requires: emcc on PATH, rustup nightly + rust-src + wasm32-unknown-emscripten
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
RUST="$ROOT/rust"
BIN="$ROOT/addons/manamesh_fairplay/bin"
CRATE_WASM_NAME="manamesh_fairplay_godot"

# Prefer rustup's cargo so `+nightly` / -Zbuild-std work (Homebrew cargo may not)
export PATH="${HOME}/.cargo/bin:/usr/local/bin:${PATH}"
CARGO="${CARGO:-$(command -v cargo)}"
if ! "$CARGO" +nightly --version >/dev/null 2>&1; then
  if [[ -x "${HOME}/.cargo/bin/cargo" ]]; then
    CARGO="${HOME}/.cargo/bin/cargo"
  else
    echo "ERROR: need rustup cargo that supports +nightly" >&2
    exit 1
  fi
fi
echo "cargo: $CARGO ($("$CARGO" +nightly --version 2>/dev/null || true))"

cd "$RUST"

if ! command -v emcc >/dev/null 2>&1; then
  echo "ERROR: emcc not on PATH" >&2
  exit 1
fi

echo "emcc: $(emcc --version | head -1)"
echo "Building nothreads web side module (recommended default for broader hosting)..."

# SIDE_MODULE requires PIC object code (libsecp256k1-sys C sources)
export CFLAGS="${CFLAGS:-} -fPIC"
export CFLAGS_wasm32_unknown_emscripten="${CFLAGS_wasm32_unknown_emscripten:-} -fPIC"
export CXXFLAGS="${CXXFLAGS:-} -fPIC"
# Force rebuild of C deps with PIC
"$CARGO" +nightly clean -p secp256k1-sys 2>/dev/null || true

# Nothreads build — uses .cargo/config.toml rustflags
"$CARGO" +nightly build \
  -p manamesh_fairplay_godot \
  --release \
  --features nothreads \
  -Zbuild-std \
  --target wasm32-unknown-emscripten

SRC_WASM="$RUST/target/wasm32-unknown-emscripten/release/${CRATE_WASM_NAME}.wasm"
if [[ ! -f "$SRC_WASM" ]]; then
  # cdylib may be named lib*.wasm
  ALT="$RUST/target/wasm32-unknown-emscripten/release/lib${CRATE_WASM_NAME}.wasm"
  if [[ -f "$ALT" ]]; then
    SRC_WASM="$ALT"
  else
    echo "ERROR: wasm not found after build" >&2
    ls -la "$RUST/target/wasm32-unknown-emscripten/release/" | head -40
    exit 1
  fi
fi

mkdir -p "$BIN"
# godot-rust expects {crate}.wasm for nothreads
cp -f "$SRC_WASM" "$BIN/${CRATE_WASM_NAME}.wasm"
# Also keep path referenced by .gdextension historically
cp -f "$SRC_WASM" "$BIN/manamesh_fairplay_godot.web.wasm32.wasm"

echo "Wrote:"
ls -la "$BIN/${CRATE_WASM_NAME}.wasm" "$BIN/manamesh_fairplay_godot.web.wasm32.wasm"

# Optional threads build (may fail on some emcc versions; non-fatal)
if [[ "${BUILD_THREADS:-0}" == "1" ]]; then
  echo "Building threads web side module..."
  export RUSTFLAGS="-C link-args=-pthread -C target-feature=+atomics -C link-args=-sSIDE_MODULE=2 -C llvm-args=-enable-emscripten-cxx-exceptions=0 -Z default-visibility=hidden -Z link-native-libraries=no"
  "$CARGO" +nightly build \
    -p manamesh_fairplay_godot \
    --release \
    -Zbuild-std \
    --target wasm32-unknown-emscripten
  unset RUSTFLAGS
  TSRC="$RUST/target/wasm32-unknown-emscripten/release/${CRATE_WASM_NAME}.wasm"
  [[ -f "$TSRC" ]] || TSRC="$RUST/target/wasm32-unknown-emscripten/release/lib${CRATE_WASM_NAME}.wasm"
  cp -f "$TSRC" "$BIN/${CRATE_WASM_NAME}.threads.wasm"
  echo "Wrote threads wasm"
fi

echo "WEB_BUILD_OK"

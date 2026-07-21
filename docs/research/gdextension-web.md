# HTML5 / web GDExtension status

## Status: **built and exported**

| Item | Status |
|------|--------|
| emcc | 6.0.3 (Homebrew) |
| Export templates | 4.7.1.stable (includes web_dlink_nothreads_*) |
| Side-module wasm | `addons/manamesh_fairplay/bin/manamesh_fairplay_godot.wasm` |
| Web export | `export/web/` with `gdextensionLibs: ["manamesh_fairplay_godot.wasm"]` |
| Thread mode | **nothreads** (broader hosting; no COOP required for threads) |

## Build

```bash
# Requires: emcc on PATH, rustup nightly + rust-src, ~/.cargo/bin before Homebrew cargo
./scripts/build_web.sh

# Export (Godot 4.7.1 + export templates installed)
godot --headless --path . --export-release "Web" export/web/index.html
```

Critical flags:

- `-sSIDE_MODULE=2` via `.cargo/config.toml`
- `CFLAGS=-fPIC` for libsecp256k1-sys under Emscripten side modules
- `cargo +nightly -Zbuild-std --features nothreads`
- godot crate features: `experimental-wasm`, `lazy-function-tables`, `experimental-wasm-nothreads` (via feature)

## Local serve

```bash
cd export/web && python3 -m http.server 8060
# open http://127.0.0.1:8060/
```

## Notes

godot-rust recommends emcc 3.1.74; Homebrew 6.0.3 worked for the side-module link with `-fPIC`.

# HTML5 / web GDExtension status

## Requirement

Milestone A treats HTML5 as a hard gate for free web multiplayer.

## Environment at implementation time

- Emscripten (`emcc`) **not installed**
- Godot export templates directory empty
- Only Rust target: `x86_64-apple-darwin`

## Status

Desktop GDExtension loads and runs crypto (see smoke logs).  
**HTML5 side-module build was not completed** in this environment; capture install/build failure rather than invent success.

## How to complete later

1. Install Emscripten SDK and Godot 4.7 web export templates.
2. Follow [godot-rust export-web](https://godot-rust.github.io/book/toolchain/export-web.html) for `wasm32-unknown-emscripten` side module (`SIDE_MODULE=2`).
3. Place WASM under `addons/manamesh_fairplay/bin/` matching `.gdextension` `web.*` keys.
4. Export project with Extension Support enabled; verify one `commit_die_face` call in browser.

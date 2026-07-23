# Platforms

| Target | Status | Notes |
|--------|--------|-------|
| Editor (Godot 4.7.x) | **Supported** | GDExtension loads after project import |
| Desktop macOS (universal) | **Supported** | Headless smoke: commit–reveal |
| Desktop Linux x86_64 | **Supported** | CI-built `.so` + smoke |
| Desktop Windows x86_64 | **Supported** | CI-built `.dll` + smoke |
| HTML5 / Web | **Preview** | WASM side module + export path; web CI is best-effort |
| Android / iOS | **Not yet** | Planned after desktop/web polish |

## Prebuilt binaries

Release assets and `addons/manamesh_fairplay/bin/`:

| File | Platform |
|------|----------|
| `libmanamesh_fairplay_godot.macos.universal.dylib` | macOS arm64 + x86_64 |
| `libmanamesh_fairplay_godot.linux.x86_64.so` | Linux x86_64 |
| `manamesh_fairplay_godot.windows.x86_64.dll` | Windows x86_64 |
| `manamesh_fairplay_godot.wasm` | Web (nothreads / default) |
| `manamesh_fairplay_godot.threads.wasm` | Web with Godot thread support |

Mapped in `addons/manamesh_fairplay/manamesh_fairplay.gdextension`.

## Why not C# as the crypto core?

Godot 4 **cannot export C# projects to HTML5**. This library uses **Rust + GDExtension** so the same crypto stack can target desktop and web for free multiplayer builds. C# games can still call the GDExtension from GDScript or via the same native class if desired; there is no separate C# crypto core.

## Godot version floor

- **Minimum:** Godot **4.4+** (`compatibility_minimum` in the `.gdextension` file is 4.3; develop against 4.4+)
- **CI / smoke:** Godot **4.7.1**

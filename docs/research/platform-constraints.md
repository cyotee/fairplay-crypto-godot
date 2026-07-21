# Research: Godot platform constraints (Phase 0)

| Field | Value |
|-------|--------|
| **Date** | 2026-07-20 |
| **Purpose** | Ground architecture and Milestone A platform scope |
| **Status** | Decisions locked in PRD §0 / IMPLEMENTATION_PLAN §12 |

---

## 1. Godot version (as of research date)

- Latest stable download page lists **Godot 4.7.1** (standard and **.NET** builds).
- Stakeholder floor: **Godot 4.4+**; develop on latest 4.7.x.

---

## 2. C# / .NET platform support (Godot 4)

Official docs ([C#/.NET index](https://docs.godotengine.org/en/stable/tutorials/scripting/c_sharp/index.html), [C# basics](https://docs.godotengine.org/en/stable/tutorials/scripting/c_sharp/c_sharp_basics.html)):

| Platform | C# in Godot 4 |
|----------|----------------|
| Desktop (Win/Linux/macOS) | Supported |
| Android | Experimental (since 4.2) |
| iOS | Experimental (since 4.2); export from macOS; NativeAOT-related limits |
| **Web / HTML5** | **Not supported** |

Exporting for the web docs state explicitly: *Projects written in C# using Godot 4 currently cannot be exported to the web.* Use Godot 3 for C# web, or do not use C# for web targets.

Background: Godot 4 moved from Mono embedding to modern .NET hosting; web export for C# was not restored ([godot#70796](https://github.com/godotengine/godot/issues/70796)). Blog: [Platform state in C# for Godot 4.2](https://godotengine.org/article/platform-state-in-csharp-for-godot-4-2/) (web section still reflects “not supported”; docs remain authoritative).

**.NET versions:**

- Godot **4.4+**: GodotSharp packages target **.NET 8** ([announcement](https://godotengine.org/article/godotsharp-packages-net8/)).
- Docs note higher .NET requirements for some Android export paths (e.g. .NET 9+ in later 4.x docs).

**Implication for this product:** A **C#-primary crypto core** cannot meet a **HTML5 Milestone A** requirement. Stakeholder decision: **full pivot to Rust GDExtension**.

---

## 3. GDScript ↔ C# interop (if C# were used)

[Cross-language scripting](https://docs.godotengine.org/en/stable/tutorials/scripting/cross_language_scripting.html):

- GDScript can `load` a `.cs` script, `.new()`, and call methods if the C# class **derives `GodotObject`**, class name matches filename, and file is in the csproj.
- GDScript **cannot** inherit C# and vice versa.
- Pure C# types without GodotObject are not a clean GDScript entrypoint.

**Superseded product choice:** C# GodotObject bridge is no longer the primary facade; **GDExtension classes + GDScript wrappers** replace it. C# may still call into GDExtension secondarily.

---

## 4. GDExtension and HTML5

- Godot web export can load **GDExtensions** when **Extension Support** is enabled; extensions must be **compiled for web** ([exporting for web](https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_web.html)).
- godot-rust documents web export: WASM built as an Emscripten **side module** (`SIDE_MODULE=2`, etc.) — [godot-rust book: Export to Web](https://godot-rust.github.io/book/toolchain/export-web.html).
- COOP/COEP / cross-origin isolation may be required when extension/thread features are enabled.

**Implication:** HTML5 is **feasible in principle** for Rust GDExtension; Phase 0 must **prove** load + one crypto op in a web export, not only desktop.

---

## 5. Mobile (deferred)

- C# mobile is experimental; GDExtension mobile is a separate multi-arch build matrix.
- Stakeholder: **skip mobile until post-publish** for Milestone A.

---

## 6. Crypto library notes (historical C# options)

| Option | Notes | Product choice |
|--------|-------|----------------|
| NBitcoin.Secp256k1 | Pure managed; used in sibling Unity/C# fairplay-crypto | N/A after Rust pivot |
| Secp256k1.Net | Native libsecp256k1 wrapper; faster; multi-arch binaries | Rejected for primary (C# path abandoned) |
| **Rust secp256k1 / k256 / libsecp256k1 bindings** | Fits GDExtension | **Selected direction**; exact crate in Phase 0 bake-off |

---

## 7. Locked outcomes from this research + Q&A

1. Architecture: **Rust + GDExtension** (not C# core).  
2. Milestone A platforms: **Editor + desktop + HTML5**; mobile later.  
3. API: **GDScript first-class**; C# second-class via extension.  
4. Godot floor: **4.4+**, develop on **4.7.x**.  
5. No .NET requirement for primary consumers.  

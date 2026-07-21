# Platform matrix

| Target | Status | Notes |
|--------|--------|-------|
| Editor (4.7.1) | **Pass** | GDExtension loads after import |
| Desktop macOS x86_64 | **Pass** | Headless smoke commit–reveal |
| HTML5 (nothreads dlink) | **Pass (build+export)** | Side-module wasm built; web export includes `manamesh_fairplay_godot.wasm` in `gdextensionLibs` |
| Android / iOS | A2 later | |

## HTML5 rebuild

```bash
./scripts/build_web.sh
godot --headless --path . --export-release "Web" export/web/index.html
```

# cdrsdk

Rust SDK for **CorelDRAW** automation, built on the COM `IDispatch` late-binding interface.

## Features

-  Type-safe wrapper around CorelDRAW's `IVG*` COM interfaces
-  Late binding via [`wincom`](https://crates.io/crates/wincom) — no `.tlb` registration needed
-  ~25 domains: Application / Document / Page / Layer / Shape / Curve / Fill / Outline / Text / Effect / Color / Print / UI / ...
-  RAII for COM resources — no manual `VariantClear`
-  Enums for all `cdr*` / `clr*` / `Prn*` / `pdf*` / `cui*` constants

## Requirements

- **Windows only** (COM is a Windows technology)
- **Rust ≥ 1.75**
- **CorelDRAW 2018+** installed and running

## Quick Start

```rust
use cdrsdk::prelude::*;
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // COM must be initialized on this thread
    unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()? };

    // Bind to a running CorelDRAW instance
    let app = IvgApplication::new("26")
        .expect("CorelDRAW not running or not installed");

    println!("version = {:?}", app.version());

    // Create a document and draw a rectangle
    let doc = app.create_document().expect("create_document");
    let layer = doc.active_layer().expect("active_layer");
    let rect = layer
        .create_rectangle2(50.0, 50.0, 100.0, 80.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");

    // Fill it with orange
    let color = app.create_rgb_color(255, 128, 0).expect("rgb");
    if let Some(fill) = rect.fill() {
        fill.apply_uniform_fill(&color);
    }

    doc.close_without_saving();

    unsafe { CoUninitialize() };
    Ok(())
}
```

## Module Overview

| Module | Purpose |
|---|---|
| `app` | `Application` object — entry point |
| `document` | Documents, metadata, data fields |
| `page` | Pages, spreads, page sizes |
| `layer` | Layers and shape factories |
| `shape` | Shapes (rectangle / ellipse / polygon / bitmap / text / ...) |
| `curve` | Curve / sub-path / node / segment |
| `geometry` | Point / vector / rect / transform / snap point |
| `color` | Color, color context, color profile |
| `fill` | Uniform / fountain / pattern / texture / hatch fills |
| `outline` | Outline, line style, arrow heads |
| `text` | Text object, range, frame, characters / words / lines |
| `effect` | Blend / contour / shadow / extrude / lens / ... |
| `style` | Style, style sheet |
| `view` | View, active view, window |
| `tree` | Object manager tree |
| `symbol` | Symbol, symbol definition, symbol library |
| `structs` | Parameter structures (`IVGStruct*`) |
| `print` | Printers, print job, separations, PDF settings |
| `import_export` | Import / export filters |
| `ui` | UI framework (`ICUI*`) |
| `misc` | Clipboard, ruler, grid, GMS, ... |
| `enums` | All `cdr*` / `clr*` / `Prn*` / `pdf*` / `cui*` enums |

## Design Notes

### Late Binding

All calls go through `IDispatch::Invoke` by name. This means:

-  No `.tlb` file needed
-  Works across CorelDRAW versions without recompilation
-  Slower than direct vtable calls
-  No compile-time check on property/method names

### Threading

COM objects are apartment-bound. All wrapper types are `!Send` / `!Sync`.
To use from multiple threads, create a separate `IvgApplication` per thread.

### Error Handling

Most methods return `Option<T>` or `bool`. For raw `HRESULT`, use `.raw()`.

## Known Limitations

- Methods with **multiple out-parameters** (e.g. `Curve.GetBoundingBox`)
  return only the first value or `None`, because `IDispatch` late binding
  cannot retrieve multiple out-params.
- `Application.ConvertUnits` is reimplemented in pure Rust (`geometry::units`)
  because the COM version requires an out-param.
- Some rarely-used property names may be incorrect — please file an issue.

## Version Compatibility

| cdrsdk | CorelDRAW | Status |
|---|---|---|
| 0.1.x | 2018 – 2024 (v20 – v26) | Tested on v26 |

## License

- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
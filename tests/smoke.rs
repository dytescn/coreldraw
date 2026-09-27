//! 入口冒烟测试
//!
//! 若本机未安装 / 未运行 CorelDRAW，测试会打印 `skip:` 并正常结束。
//!
//! 运行方式：
//! ```text
//! cargo test --test smoke -- --nocapture
//! ```

mod common;

use cdrsdk::prelude::*;
use common::ComGuard;

/// 尝试用常见版本号绑定 CorelDRAW。
///
/// 顺序从新到旧，覆盖 2018 ~ 2024 各代版本。
/// 尝试绑定 CorelDRAW。
///
/// ProgID 格式为 `CorelDRAW.Application` 或 `CorelDRAW.Application.{version}`，
/// 其中 `{version}` 是主版本号（如 `26`），**不带 `.0`**。
fn bind_app() -> Option<IvgApplication> {
    // 先试不带版本号（默认安装版本）
    if let Some(app) = IvgApplication::new("") {
        eprintln!("bound CorelDRAW (default)");
        return Some(app);
    }
    // 再逐个试主版本号
    for v in &["26", "25", "24", "23", "22", "21", "20"] {
        if let Some(app) = IvgApplication::new(v) {
            eprintln!("bound CorelDRAW version = {v}");
            return Some(app);
        }
    }
    None
}

// =============================================================
// 应用
// =============================================================

#[test]
fn smoke_app_basic() {
    let _com = ComGuard::init().expect("CoInitialize failed");

    let app = match bind_app() {
        Some(a) => a,
        None => {
            eprintln!("skip: CorelDRAW not running or not installed");
            return;
        }
    };

    println!("version       = {:?}", app.version());
    println!("version_major = {:?}", app.version_major());
    println!("version_minor = {:?}", app.version_minor());
    println!("version_build = {:?}", app.version_build());
    println!("setup_path    = {:?}", app.setup_path());
    println!("program_path  = {:?}", app.program_path());
    println!("config_path   = {:?}", app.config_path());
    println!("visible       = {:?}", app.visible());
}

#[test]
fn smoke_app_documents() {
    let _com = ComGuard::init().expect("CoInitialize failed");

    let app = match bind_app() {
        Some(a) => a,
        None => {
            eprintln!("skip: CorelDRAW not running or not installed");
            return;
        }
    };

    if let Some(docs) = app.documents() {
        println!("documents.count = {:?}", docs.count());
        for (i, d) in docs.all().iter().enumerate() {
            println!("  [{}] name = {:?}", i + 1, d.name());
        }
    }

    if let Some(doc) = app.active_document() {
        println!("active_document.name = {:?}", doc.name());
        println!("active_document.full_file_name = {:?}", doc.full_file_name());
        println!("active_document.unit = {:?}", doc.unit());
    } else {
        println!("no active document");
    }
}

#[test]
fn smoke_app_page_layer_shape() {
    let _com = ComGuard::init().expect("CoInitialize failed");

    let app = match bind_app() {
        Some(a) => a,
        None => {
            eprintln!("skip: CorelDRAW not running or not installed");
            return;
        }
    };

    if let Some(page) = app.active_page() {
        println!("active_page.name = {:?}", page.name());
        println!("active_page.size = {:?}", page.get_size());
        println!("active_page.orientation = {:?}", page.orientation());
    }

    if let Some(layer) = app.active_layer() {
        println!("active_layer.name = {:?}", layer.name());
        println!("active_layer.visible = {:?}", layer.visible());
    }

    if let Some(shape) = app.active_shape() {
        println!("active_shape.type = {:?}", shape.shape_type());
        println!("active_shape.name = {:?}", shape.name());
        println!("active_shape.bounding_box = {:?}", shape.bounding_box().map(|r| {
            (r.x(), r.y(), r.width(), r.height())
        }));
    }

    if let Some(range) = app.active_selection_range() {
        println!("selection.count = {:?}", range.count());
    }
}

// =============================================================
// 颜色工厂
// =============================================================

#[test]
fn smoke_create_colors() {
    let _com = ComGuard::init().expect("CoInitialize failed");

    let app = match bind_app() {
        Some(a) => a,
        None => {
            eprintln!("skip: CorelDRAW not running or not installed");
            return;
        }
    };

    if let Some(rgb) = app.create_rgb_color(255, 128, 0) {
        println!("RGB  = ({:?}, {:?}, {:?})",
            rgb.rgb_red(), rgb.rgb_green(), rgb.rgb_blue());
        println!("RGB  hex = {:?}", rgb.hex_value());
        println!("RGB  name = {:?}", rgb.name(true));
    }

    if let Some(cmyk) = app.create_cmyk_color(0, 50, 100, 0) {
        println!("CMYK = ({:?}, {:?}, {:?}, {:?})",
            cmyk.cmyk_cyan(), cmyk.cmyk_magenta(),
            cmyk.cmyk_yellow(), cmyk.cmyk_black());
    }

    if let Some(gray) = app.create_gray_color(128) {
        println!("Gray = {:?}", gray.gray());
    }
}

// =============================================================
// 结构体工厂
// =============================================================

#[test]
fn smoke_struct_factories() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = match bind_app() {
        Some(a) => a,
        None => { eprintln!("skip: ..."); return; }
    };

    if let Some(font) = app.create_font_properties() {
        // 先设后读
        font.set_size(24.0);
        font.set_name("Arial");
        println!("font.name = {:?}", font.name());
        println!("font.size = {:?}", font.size());

        // 改一个值，确认可写
        font.set_size(48.0);
        println!("after set 48, font.size = {:?}", font.size());
    }

    if let Some(space) = app.create_space_properties() {
        // 先设后读
        let _ = space.set_character_spacing(100.0);
        println!("space.char_spacing = {:?}", space.character_spacing());
    }
}
// =============================================================
// 几何工厂
// =============================================================

#[test]
fn smoke_geometry() {
    let _com = ComGuard::init().expect("CoInitialize failed");

    let app = match bind_app() {
        Some(a) => a,
        None => {
            eprintln!("skip: CorelDRAW not running or not installed");
            return;
        }
    };

    if let Some(rect) = app.create_rect(10.0, 20.0, 100.0, 50.0) {
        println!("rect = ({:?}, {:?}, {:?}, {:?})",
            rect.x(), rect.y(), rect.width(), rect.height());
        println!("rect.left/right/top/bottom = {:?}/{:?}/{:?}/{:?}",
            rect.left(), rect.right(), rect.top(), rect.bottom());
    }

    if let Some(sp) = app.create_snap_point(100.0, 200.0) {
        println!("snap point position = {:?}", sp.get_position());
    }
}

// =============================================================
// 单位转换
// =============================================================

#[test]
fn smoke_units() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = match bind_app() {
        Some(a) => a,
        None => { eprintln!("skip: CorelDRAW not running or not installed"); return; }
    };

    println!("current unit = {:?}", app.unit());

    for (val, from, to, label) in [
        (1.0, 0, 1, "1 inch → mm"),
        (1.0, 0, 3, "1 inch → pt"),
        (1.0, 0, 4, "1 inch → cm"),
        (25.4, 1, 0, "25.4 mm → inch"),
        (2.54, 4, 0, "2.54 cm → inch"),
        (100.0, 4, 1, "100 cm → mm"),
    ] {
        println!("{label} = {:?}", app.convert_units(val, from, to));
    }
}
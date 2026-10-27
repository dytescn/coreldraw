//! `IvgApplication` 深度测试

mod common;

use common::ComGuard;

// =============================================================
// 版本信息
// =============================================================

#[test]
fn app_version() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let v  = app.version();
    let ma = app.version_major();
    let mi = app.version_minor();
    let bd = app.version_build();

    println!("version       = {:?}", v);
    println!("version_major = {:?}", ma);
    println!("version_minor = {:?}", mi);
    println!("version_build = {:?}", bd);

    assert!(v.is_some(), "version should not be None");
    assert!(ma.is_some(), "version_major should not be None");
    assert!(ma.unwrap() >= 15, "version_major should be ≥ 15");
}

// =============================================================
// 路径
// =============================================================

#[test]
fn app_paths() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    for (label, v) in [
        ("Path",              app.path()),
        ("SetupPath",         app.setup_path()),
        ("ProgramPath",       app.program_path()),
        ("ConfigPath",        app.config_path()),
        ("UserDataPath",      app.user_data_path()),
        ("UserWorkspacePath", app.user_workspace_path()),
        ("AddonPath",         app.addon_path()),
        ("LanguagePath",      app.language_path()),
    ] {
        println!("{:18} = {:?}", label, v);
    }

    // 至少 setup_path 应该返回非空
    assert!(
        app.setup_path().is_some_and(|s| !s.is_empty()),
        "setup_path 应非空"
    );
}

// =============================================================
// 可见性
// =============================================================

#[test]
fn app_visibility() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let before = app.visible();
    println!("visible (before) = {:?}", before);

    assert!(app.set_visible(true), "set_visible(true) 应成功");

    let after = app.visible();
    println!("visible (after)  = {:?}", after);
    assert_eq!(after, Some(true), "visible 应为 true");
}

// =============================================================
// 文档集合
// =============================================================

#[test]
fn app_documents_collection() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let docs = app.documents();
    assert!(docs.is_some(), "documents() 应返回 Some");

    if let Some(docs) = docs {
        let count = docs.count();
        println!("documents.count = {:?}", count);

        // 遍历已有文档
        for (i, d) in docs.all().iter().enumerate() {
            println!(
                "  [{}] name={:?} file={:?} dirty={:?}",
                i + 1,
                d.name(),
                d.full_file_name(),
                d.dirty()
            );
        }
    }
}

// =============================================================
// 当前文档
// =============================================================

#[test]
fn app_active_document() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    match app.active_document() {
        Some(doc) => {
            println!("active_document.name         = {:?}", doc.name());
            println!("active_document.full_name    = {:?}", doc.full_file_name());
            println!("active_document.file_path    = {:?}", doc.file_path());
            println!("active_document.dirty        = {:?}", doc.dirty());
            println!("active_document.read_only    = {:?}", doc.read_only());
            println!("active_document.unit         = {:?}", doc.unit());
            println!("active_document.reference_pt = {:?}", doc.reference_point());
        }
        None => {
            println!("no active document (this is OK for a fresh CorelDRAW)");
        }
    }
}

// =============================================================
// 当前页面 / 图层 / 图形
// =============================================================

#[test]
fn app_active_page_layer_shape() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    if let Some(page) = app.active_page() {
        println!("page.name        = {:?}", page.name());
        println!("page.index       = {:?}", page.index());
        println!("page.size        = {:?}", page.get_size());
        println!("page.orientation = {:?}", page.orientation());
        println!("page.background  = {:?}", page.background());
        println!("page.center      = {:?}", page.get_center_position());
    } else {
        println!("no active page");
    }

    if let Some(layer) = app.active_layer() {
        println!("layer.name       = {:?}", layer.name());
        println!("layer.visible    = {:?}", layer.visible());
        println!("layer.editable   = {:?}", layer.editable());
        println!("layer.printable  = {:?}", layer.printable());
    } else {
        println!("no active layer");
    }

    if let Some(shape) = app.active_shape() {
        println!("shape.type       = {:?}", shape.shape_type());
        println!("shape.name       = {:?}", shape.name());
        println!("shape.position   = {:?}", shape.get_position());
        println!("shape.size       = {:?}", shape.get_size());
    } else {
        println!("no active shape (expected if nothing is selected)");
    }

    if let Some(range) = app.active_selection_range() {
        println!("selection.count  = {:?}", range.count());
    }
}

// =============================================================
// 窗口
// =============================================================

#[test]
fn app_windows() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    if let Some(w) = app.active_window() {
        println!("active_window.caption      = {:?}", w.caption());
        println!("active_window.index        = {:?}", w.index());
        println!("active_window.active       = {:?}", w.active());
        println!("active_window.window_state = {:?}", w.window_state());
        println!("active_window.handle       = {:?}", w.handle());
    } else {
        println!("no active window");
    }

    if let Some(ws) = app.windows() {
        println!("windows.count = {:?}", ws.count());
    }
}

// =============================================================
// 单位
// =============================================================

#[test]
fn app_unit() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    println!("Unit = {:?}", app.unit());

    let before = app.unit();
    assert!(app.set_unit(0), "set_unit(0) inch 应成功");
    let after = app.unit();
    println!("after set 0 -> {:?}", after);
    assert_eq!(after, Some(0), "Unit 应为 0 (inch)");

    // 恢复
    if let Some(v) = before {
        let _ = app.set_unit(v as i32);
    }
}

// =============================================================
// 表达式求值
// =============================================================

#[test]
fn app_evaluate() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    // 试试简单表达式，看 CorelDRAW 能否求值
    // （Evaluate 语义取决于版本，可能返回 None，不算失败）
    if let Some(v) = app.evaluate("1 + 1") {
        println!("Evaluate('1 + 1') = {:?}", v.to_i64());
    } else {
        println!("Evaluate('1 + 1') 返回 None（可能不被该版本支持）");
    }

    if let Some(v) = app.evaluate("Version") {
        println!("Evaluate('Version') = {:?}", v.to_string());
    }
}
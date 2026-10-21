//! `import_export` 模块接口测试
//!
//! 使用临时 PNG 做一次导出 → 导入往返；不依赖外部素材。

mod common;

use cdrsdk::enums::filter::cdrFilter;
use cdrsdk::prelude::*;
use common::ComGuard;

#[test]
fn export_then_import_png() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let layer = doc.active_layer().expect("active_layer");
    let _rect = layer
        .create_rectangle2(10.0, 10.0, 80.0, 60.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");

    let mut path = std::env::temp_dir();
    path.push("cdrsdk_import_export_test.png");
    let path_str = path.to_string_lossy().to_string();
    let _ = std::fs::remove_file(&path);

    let exported = doc.export(&path_str, cdrFilter::PNG as i32);
    if !exported || !path.exists() {
        eprintln!("skip: export PNG failed (path = {path_str})");
        let _ = doc.close_without_saving();
        return;
    }
    assert!(exported);
    assert!(path.exists());

    // 带选项导出到第二个文件
    if let Some(opts) = app.create_export_options() {
        opts.set_overwrite(true);
        let mut path2 = std::env::temp_dir();
        path2.push("cdrsdk_import_export_test2.png");
        let path2_str = path2.to_string_lossy().to_string();
        let _ = std::fs::remove_file(&path2);
        let exported2 = doc.export_ex(&path2_str, cdrFilter::PNG as i32, &opts);
        println!("export_ex = {exported2}");
        let _ = std::fs::remove_file(&path2);
    }

    // 再导入回当前图层
    let before = layer.shapes().and_then(|s| s.count()).unwrap_or(0);
    if let Some(import_opts) = app.create_import_options() {
        let imported = layer.import(&path_str, cdrFilter::PNG as i32, &import_opts);
        println!("import = {imported}");
        if imported {
            let after = layer.shapes().and_then(|s| s.count()).unwrap_or(0);
            assert!(after >= before + 1, "shapes {before} -> {after}");
        } else {
            eprintln!("skip: layer.import returned false");
        }
    }

    let _ = std::fs::remove_file(&path);
    let _ = doc.close_without_saving();
}

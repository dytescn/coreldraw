//! `IvgStyle` 一族接口测试
//!
//! 覆盖：样式表、样式集合、从图形创建样式、样式属性与子对象。

mod common;

use cdrsdk::prelude::*;
use common::ComGuard;

#[test]
fn style_sheet_read_and_create_from_shape() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let sheet = doc.style_sheet().expect("document.style_sheet()");

    // 默认样式集合
    if let Some(styles) = sheet.styles() {
        let count = styles.count().unwrap_or(0);
        println!("styles = {count}");
        if let Some(first) = styles.first() {
            assert!(first.name().is_some());
            assert!(first.display_name().is_some());
            if let Some(fill) = first.fill() {
                assert!(fill.fill_type().is_some());
            }
            if let Some(outline) = first.outline() {
                assert!(outline.outline_type().is_some() || outline.width().is_some());
            }
        }
    }

    // 所有样式 / 对象默认值
    let all = sheet.all_styles();
    assert!(all.is_some());
    assert!(sheet.object_defaults().is_some());

    // 从图形创建样式
    let layer = doc.active_layer().expect("active_layer");
    let rect = layer
        .create_rectangle2(0.0, 0.0, 50.0, 30.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");

    if let Some(created) = sheet.create_style_from_shape(&rect, "cdrsdk-test", "sdk-style", true) {
        assert!(created.count().unwrap_or(0) >= 1);
        if let Some(style) = sheet.find_style("sdk-style") {
            assert_eq!(style.name().as_deref(), Some("sdk-style"));
            assert!(style.category_name().is_some());
            assert!(style.to_style_string().is_some());
            assert!(style.is_style_set().is_some());
            assert!(style.derived_styles().is_some());

            if let Some(fill) = style.fill() {
                assert!(fill.fill_type().is_some());
            }
            if let Some(outline) = style.outline() {
                assert!(outline.outline_type().is_some() || outline.width().is_some());
            }
        } else {
            eprintln!("skip: FindStyle did not return the created style");
        }
    } else {
        eprintln!("skip: CreateStyleFromShape unavailable");
    }

    let _ = doc.close_without_saving();
}

//! `misc` 模块接口测试
//!
//! 覆盖：网格、标尺、对象属性包、URL、透明度、克隆链接，以及离线的后缀名映射。

mod common;

use cdrsdk::enums::filter::cdrFilter;
use cdrsdk::misc::filetypes;
use cdrsdk::prelude::*;
use common::ComGuard;

#[test]
fn misc_grid_and_rulers() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");

    let grid = doc.grid().expect("document.grid()");
    assert!(grid.visible().is_some());
    assert!(grid.grid_type().is_some());
    assert!(grid.spacing_x().is_some());
    assert!(grid.spacing_y().is_some());
    let snap = grid.snap().unwrap_or(false);
    assert!(grid.set_snap(snap)); // 原值写回，避免污染全局设置

    let rulers = doc.rulers().expect("document.rulers()");
    assert!(rulers.h_units().is_some());
    assert!(rulers.v_units().is_some());
    let h = rulers.h_units().unwrap_or(0) as i32;
    let v = rulers.v_units().unwrap_or(0) as i32;
    assert!(rulers.set_h_units(h));
    assert!(rulers.set_v_units(v));

    let _ = doc.close_without_saving();
}

#[test]
fn misc_shape_properties_url_transparency_clone() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let layer = doc.active_layer().expect("active_layer");
    let rect = layer
        .create_rectangle2(0.0, 0.0, 50.0, 30.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");

    // 属性包
    let props = rect.properties().expect("shape.properties()");
    assert!(props.count().is_some());
    assert!(props.exists("cdrsdk-missing", 0).is_some());

    // URL
    let url = rect.url().expect("shape.url()");
    assert!(url.set_address("https://example.com/cdrsdk"));
    assert_eq!(url.address().as_deref(), Some("https://example.com/cdrsdk"));
    assert!(url.set_alt_comment("cdrsdk test"));
    assert_eq!(url.alt_comment().as_deref(), Some("cdrsdk test"));

    // 透明度
    let transparency = rect.transparency().expect("shape.transparency()");
    let _ = transparency.transparency_type();
    let _ = transparency.frozen();

    // 克隆链接
    let clone = rect.clone(20.0, 0.0).expect("shape.clone()");
    let link = clone.clone_link().expect("clone.clone_link()");
    assert!(link.clone_parent().is_some());
    assert!(link.fill_linked().is_some());
    assert!(link.outline_linked().is_some());
    assert!(link.shape_linked().is_some());
    assert!(link.transform_linked().is_some());

    let _ = doc.close_without_saving();
}

#[test]
fn misc_filetypes_offline() {
    assert_eq!(filetypes::extension_for(cdrFilter::PNG), "png");
    assert_eq!(filetypes::extension_for(cdrFilter::JPEG), "jpg");
    assert_eq!(filetypes::extension_for(cdrFilter::CMX6), "cmx");
    assert_eq!(filetypes::extension_for(cdrFilter::PDF), "pdf");

    assert_eq!(filetypes::filter_from_extension("png"), Some(cdrFilter::PNG));
    assert_eq!(filetypes::filter_from_extension(".JPG"), Some(cdrFilter::JPEG));
    assert_eq!(filetypes::filter_from_extension("unknown-ext"), None);
}

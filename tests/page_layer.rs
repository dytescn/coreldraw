//! `IvgPage` / `IvgLayer` 接口测试

mod common;

use common::ComGuard;

#[test]
fn page_properties_and_create() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let page = doc.active_page().expect("active_page");

    assert!(page.name().is_some());
    let (w, h) = page.get_size().expect("page.get_size()");
    assert!(w > 0.0 && h > 0.0);
    assert_eq!(page.index(), Some(1));
    assert!(page.orientation().is_some());
    assert!(page.background().is_some());
    assert!(page.bounding_box().is_some());

    assert!(page.set_size(200.0, 150.0));
    let (w, h) = page.get_size().expect("page.get_size() after set");
    assert!((w - 200.0).abs() < 0.01 && (h - 150.0).abs() < 0.01);

    let before = doc.pages().and_then(|p| p.count()).unwrap_or(0);
    if let Some(_page2) = doc.create_page() {
        let after = doc.pages().and_then(|p| p.count()).unwrap_or(0);
        assert!(after >= before + 1, "pages {before} -> {after}");
    } else {
        eprintln!("skip: Document.CreatePage unavailable on this build");
    }

    assert!(doc.pages().and_then(|p| p.first()).is_some());
    assert!(doc.pages().and_then(|p| p.last()).is_some());

    let _ = doc.close_without_saving();
}

#[test]
fn layer_create_rename_and_delete() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let before = doc.active_page_layers().and_then(|l| l.count()).unwrap_or(0);

    let layer = doc.create_layer("cdrsdk-test-layer").expect("create_layer");
    let name = layer.name().unwrap_or_default();
    assert_eq!(name, "cdrsdk-test-layer");
    assert!(layer.set_name("cdrsdk-test-layer-2"));
    assert_eq!(layer.name().as_deref(), Some("cdrsdk-test-layer-2"));

    assert!(layer.activate());
    assert!(layer.set_visible(true));
    assert_eq!(layer.visible(), Some(true));
    assert_eq!(layer.printable(), Some(true));
    assert!(layer.shapes().is_some());
    assert!(layer.page().is_some());

    let layers = doc.active_page_layers().expect("active_page_layers");
    assert!(layers.find("cdrsdk-test-layer-2").is_some());
    let after = layers.count().unwrap_or(0);
    assert!(after >= before + 1, "layers {before} -> {after}");

    let _ = doc.close_without_saving();
}

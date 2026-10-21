//! `IvgView` / `IvgTreeNode` 接口测试

mod common;

use common::ComGuard;

#[test]
fn view_zoom_and_collection() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");

    // Document.Views 在不同版本/文档状态下可能为空，存在性检查即可。
    if let Some(views) = doc.views() {
        let count = views.count().unwrap_or(0);
        println!("document views = {count}");
        if count >= 1 {
            assert!(views.item_by_index(1).is_some());
        }
    }

    let window = app.active_window().expect("active_window");
    let view = window.active_view().expect("window.active_view()");
    assert!(view.zoom().is_some());
    assert!(view.set_zoom(120.0));
    assert!(view.zoom().unwrap_or(0.0) > 0.0);
    assert!(view.to_fit_page());

    let _ = doc.close_without_saving();
}

#[test]
fn tree_node_from_shape() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let layer = doc.active_layer().expect("active_layer");
    let rect = layer
        .create_rectangle2(0.0, 0.0, 40.0, 30.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");

    let node = rect.tree_node().expect("shape.tree_node()");
    assert!(node.name().is_some());
    assert!(node.node_type().is_some());
    assert!(node.shape().is_some());
    assert!(node.parent().is_some() || node.is_group_child().is_some());
    assert!(node.handle().is_some());

    if let Some(manager) = doc.tree_manager() {
        let _ = manager.selected_node_count();
    } else {
        eprintln!("skip: Document.TreeManager unavailable");
    }

    if let Some(layer_node) = layer.tree_node() {
        assert!(layer_node.node_type().is_some());
    } else {
        eprintln!("skip: Layer.TreeNode unavailable");
    }

    let _ = doc.close_without_saving();
}

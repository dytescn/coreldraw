//! `IvgShape` / 具体图形类型接口测试
//!
//! 覆盖：几何属性、变换、复制/删除、转曲线、矩形/椭圆/多边形的类型化访问。

mod common;

use cdrsdk::prelude::*;
use common::ComGuard;

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.01
}

fn setup() -> Option<(ComGuard, IvgApplication, IvgDocument, IvgLayer)> {
    let com = ComGuard::init().expect("CoInitialize failed");
    let app = common::bind_app()?;
    let doc = app.create_document().expect("create_document");
    let layer = doc.active_layer().expect("active_layer");
    Some((com, app, doc, layer))
}

#[test]
fn shape_geometry_and_transform() {
    let _serial = common::serial();
    let Some((_com, _app, doc, layer)) = setup() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let rect = layer
        .create_rectangle2(10.0, 20.0, 100.0, 50.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");

    assert_eq!(rect.shape_type(), Some(cdrShapeType::RectangleShape as i64));
    let (w, h) = rect.get_size().expect("get_size");
    assert!(approx(w, 100.0) && approx(h, 50.0), "size = {w}x{h}");

    let bbox = rect.bounding_box().expect("bounding_box");
    assert!(bbox.width().unwrap_or(0.0) > 0.0);

    assert!(rect.set_size(80.0, 40.0));
    let (w, h) = rect.get_size().expect("get_size after set");
    assert!(approx(w, 80.0) && approx(h, 40.0));

    assert!(rect.move_by(5.0, -3.0));
    let (x, y) = rect.get_position().expect("get_position");
    assert!(x.is_finite() && y.is_finite());

    assert!(rect.rotate(15.0));
    assert!(approx(rect.rotation_angle().unwrap_or(0.0), 15.0));
    assert!(rect.clear_transformations());
    assert!(rect.transformation_matrix().is_some());

    let _ = doc.close_without_saving();
}

#[test]
fn shape_duplicate_delete_and_selection() {
    let _serial = common::serial();
    let Some((_com, _app, doc, layer)) = setup() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let rect = layer
        .create_rectangle2(0.0, 0.0, 40.0, 30.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");

    assert!(rect.set_name("sdk-test-rect"));
    assert_eq!(rect.name().as_deref(), Some("sdk-test-rect"));

    let before = layer.shapes().and_then(|s| s.count()).unwrap_or(0);
    let dup = rect.duplicate(20.0, 0.0).expect("duplicate");
    let after = layer.shapes().and_then(|s| s.count()).unwrap_or(0);
    assert!(after >= before + 1, "shapes {before} -> {after}");
    assert!(dup.delete());

    assert!(rect.add_to_selection());
    assert!(rect.remove_from_selection());

    let _ = doc.close_without_saving();
}

#[test]
fn shape_specific_types_and_convert_to_curves() {
    let _serial = common::serial();
    let Some((_com, _app, doc, layer)) = setup() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let rect = layer
        .create_rectangle2(0.0, 0.0, 50.0, 50.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");
    assert!(rect.rectangle().is_some());

    let ellipse = layer
        .create_ellipse2(80.0, 25.0, 25.0, 15.0, 0.0, 360.0, false)
        .expect("create_ellipse2");
    assert_eq!(ellipse.shape_type(), Some(cdrShapeType::EllipseShape as i64));
    assert!(ellipse.ellipse().is_some());

    let polygon = layer
        .create_polygon2(140.0, 25.0, 20.0, 6, 0.0, 0.0, false, 0)
        .expect("create_polygon2");
    assert!(polygon.polygon().is_some());

    assert!(ellipse.convert_to_curves());
    let curve = ellipse.curve().expect("ellipse.curve() after convert");
    assert!(curve.length().unwrap_or(0.0) > 0.0);

    let _ = doc.close_without_saving();
}

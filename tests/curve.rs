//! `IvgCurve` 一族接口测试
//!
//! 覆盖：由线段创建曲线、长度 / 节点 / 线段 / 子路径、复制与转回图形。

mod common;

use cdrsdk::prelude::*;
use common::ComGuard;

#[test]
fn curve_from_segment_and_collections() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let layer = doc.active_layer().expect("active_layer");

    let shape = layer
        .create_curve_segment2(0.0, 0.0, 20.0, 30.0, 40.0, -30.0, 60.0, 0.0)
        .expect("create_curve_segment2");
    let curve = shape.curve().expect("shape.curve()");

    assert!(curve.length().unwrap_or(0.0) > 0.0);
    assert!(!curve.is_closed().unwrap_or(true));

    let nodes = curve.nodes().expect("curve.nodes()");
    assert!(nodes.count().unwrap_or(0) >= 2, "nodes = {:?}", nodes.count());
    let first = nodes.first().expect("nodes.first()");
    assert!(first.get_position().is_some());
    assert!(first.index().is_some());

    let segments = curve.segments().expect("curve.segments()");
    assert!(segments.count().unwrap_or(0) >= 1);
    let seg = segments.first().expect("segments.first()");
    assert!(seg.length().unwrap_or(0.0) >= 0.0);

    let sub_paths = curve.sub_paths().expect("curve.sub_paths()");
    assert!(sub_paths.count().unwrap_or(0) >= 1);
    assert!(sub_paths.first().is_some());

    assert!(curve.area().is_some());
    assert!(curve.bounding_box().is_some());
    assert!(curve.is_clockwise().is_some());

    let _ = doc.close_without_saving();
}

#[test]
fn curve_copy_and_to_shape() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let layer = doc.active_layer().expect("active_layer");

    let shape = layer
        .create_curve_segment(0.0, 0.0, 60.0, 0.0, 10.0, 20.0, 10.0, -20.0)
        .expect("create_curve_segment");
    let curve = shape.curve().expect("shape.curve()");

    let copy = curve.get_copy().expect("curve.get_copy()");
    assert!(copy.length().unwrap_or(0.0) > 0.0);
    assert!(curve.copy_assign(&copy));

    let as_shape = layer.create_curve(&copy).expect("layer.create_curve()");
    assert_eq!(as_shape.shape_type(), Some(cdrShapeType::CurveShape as i64));

    let _ = doc.close_without_saving();
}

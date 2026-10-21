//! `IvgFill` / `IvgOutline` 接口测试

mod common;

use cdrsdk::prelude::*;
use common::ComGuard;

#[test]
fn fill_uniform_no_fill_and_fountain() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let layer = doc.active_layer().expect("active_layer");
    let rect = layer
        .create_rectangle2(0.0, 0.0, 60.0, 40.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");
    let fill = rect.fill().expect("shape.fill()");

    let orange = app.create_rgb_color(255, 128, 0).expect("rgb");
    assert!(fill.apply_uniform_fill(&orange));
    assert_eq!(fill.fill_type(), Some(cdrFillType::UniformFill as i64));
    let c = fill.uniform_color().expect("uniform_color");
    assert_eq!(c.rgb_red(), Some(255));
    assert_eq!(c.rgb_green(), Some(128));
    assert_eq!(c.rgb_blue(), Some(0));

    let red = app.create_rgb_color(255, 0, 0).expect("red");
    let blue = app.create_rgb_color(0, 0, 255).expect("blue");
    let fountain = fill
        .apply_fountain_fill(
            &red,
            &blue,
            cdrFountainFillType::LinearFountainFill as i32,
            90.0,
            256,
            0,
            50,
            cdrFountainFillBlendType::DirectFountainFillBlend as i32,
            0.0,
            0.0,
        )
        .expect("apply_fountain_fill");
    assert!(fountain.fountain_type().is_some());
    assert_eq!(fill.fill_type(), Some(cdrFillType::FountainFill as i64));

    assert!(fill.apply_no_fill());
    assert_eq!(fill.fill_type(), Some(cdrFillType::NoFill as i64));

    let _ = doc.close_without_saving();
}

#[test]
fn outline_properties() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let layer = doc.active_layer().expect("active_layer");
    let rect = layer
        .create_rectangle2(0.0, 0.0, 60.0, 40.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");
    let outline = rect.outline().expect("shape.outline()");

    assert!(outline.set_width(2.0));
    assert!((outline.width().unwrap_or(0.0) - 2.0).abs() < 0.01);
    assert!(outline.set_outline_type(cdrOutlineType::Outline as i32));
    assert_eq!(outline.outline_type(), Some(cdrOutlineType::Outline as i64));

    let black = app.create_rgb_color(0, 0, 0).expect("black");
    assert!(outline.set_color(&black));
    assert!(outline.color().is_some());

    assert!(outline.set_line_caps(cdrOutlineLineCaps::Round as i32));
    assert_eq!(
        outline.line_caps(),
        Some(cdrOutlineLineCaps::Round as i64)
    );
    assert!(outline.set_behind_fill(true));
    assert_eq!(outline.behind_fill(), Some(true));

    assert!(outline.set_no_outline());
    assert_eq!(outline.outline_type(), Some(cdrOutlineType::NoOutline as i64));

    let _ = doc.close_without_saving();
}

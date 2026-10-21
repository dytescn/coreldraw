//! `IvgEffect` 一族接口测试
//!
//! 覆盖：投影、调和、轮廓效果，以及 `IvgEffects` 集合与清除效果。
//! 某些效果依赖具体图形上下文，创建失败时打印 `skip:` 而不是 fail。

mod common;

use cdrsdk::prelude::*;
use common::ComGuard;

fn setup() -> Option<(ComGuard, IvgApplication, IvgDocument, IvgLayer)> {
    let com = ComGuard::init().expect("CoInitialize failed");
    let app = common::bind_app()?;
    let doc = app.create_document().expect("create_document");
    let layer = doc.active_layer().expect("active_layer");
    Some((com, app, doc, layer))
}

#[test]
fn effect_drop_shadow_and_effects_collection() {
    let _serial = common::serial();
    let Some((_com, app, doc, layer)) = setup() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let rect = layer
        .create_rectangle2(20.0, 20.0, 60.0, 40.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");
    let black = app.create_rgb_color(0, 0, 0).expect("black");

    let effect = rect.create_drop_shadow(
        cdrDropShadowType::DropShadowFlat as i32,
        50,
        0,
        5.0,
        5.0,
        &black,
        cdrFeatherType::FeatherInside as i32,
        cdrEdgeType::EdgeLinear as i32,
        0.0,
        0.0,
        0,
        cdrMergeMode::MergeNormal as i32,
    );

    if let Some(effect) = effect {
        assert_eq!(effect.effect_type(), Some(cdrEffectType::DropShadow as i64));
        let shadow = effect.drop_shadow().expect("effect.drop_shadow()");
        assert!(shadow.offset_x().is_some());
        assert!(shadow.opacity().is_some());
        assert!(shadow.shadow_type().is_some());

        let effects = rect.effects().expect("shape.effects()");
        assert!(effects.count().unwrap_or(0) >= 1);
        assert!(effects.drop_shadow_effect().is_some());
        if let Some(current) = rect.effect() {
            assert_eq!(current.effect_type(), Some(cdrEffectType::DropShadow as i64));
        } else {
            eprintln!("skip: Shape.Effect unavailable");
        }

        assert!(rect.clear_effect(cdrEffectType::DropShadow as i32));
    } else {
        eprintln!("skip: CreateDropShadow unavailable for this shape");
    }

    let _ = doc.close_without_saving();
}

#[test]
fn effect_blend_and_contour() {
    let _serial = common::serial();
    let Some((_com, app, doc, layer)) = setup() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    // ---- 调和 ----
    let a = layer
        .create_ellipse2(10.0, 40.0, 12.0, 12.0, 0.0, 360.0, false)
        .expect("ellipse a");
    let b = layer
        .create_ellipse2(110.0, 40.0, 12.0, 12.0, 0.0, 360.0, false)
        .expect("ellipse b");

    let blend = a.create_blend(
        &b,
        5,
        0,
        cdrBlendMode::BlendSteps as i32,
        0.0,
        0.0,
        false,
        None,
        false,
        0,
        0,
        false,
    );
    if let Some(effect) = blend {
        assert_eq!(effect.effect_type(), Some(cdrEffectType::Blend as i64));
        if let Some(blend) = effect.blend() {
            assert!(blend.steps().is_some() || blend.spacing().is_some());
        }
    } else {
        eprintln!("skip: CreateBlend unavailable");
    }

    // ---- 轮廓 ----
    let rect = layer
        .create_rectangle2(10.0, 100.0, 40.0, 30.0, 0.0, 0.0, 0.0, 0.0)
        .expect("rect");
    let black = app.create_rgb_color(0, 0, 0).expect("black");
    let contour = rect.create_contour(
        cdrContourDirection::Outside as i32,
        2.0,
        3,
        0,
        &black,
        &black,
        &black,
        0,
        0,
        0,
        0,
        5.0,
    );
    if let Some(effect) = contour {
        assert_eq!(effect.effect_type(), Some(cdrEffectType::Contour as i64));
        if let Some(contour) = effect.contour() {
            assert!(contour.offset().is_some() || contour.steps().is_some());
        }
    } else {
        eprintln!("skip: CreateContour unavailable");
    }

    let _ = doc.close_without_saving();
}

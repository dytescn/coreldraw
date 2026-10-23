//! Shape 域接口：工厂 + 常用属性操作。
//!
//! 图形名称在 CorelDRAW 里默认不唯一（多个矩形都叫 `Rectangle`），
//! 因此创建接口会写入一个唯一名称（`name` 参数或自动生成 `ffi_<n>`），
//! 后续接口按名称定位图形。

use std::sync::atomic::{AtomicU64, Ordering};

use cdrsdk::prelude::*;
use serde_json::json;

use crate::com::with_app;
use crate::info::{fill_info, outline_info, shape_info};
use crate::json::{get_f64, get_str, opt_bool, opt_f64, opt_i32, opt_str, ApiError, ApiResult};
use crate::resolve::{resolve_doc, resolve_layer, resolve_shape};

static SHAPE_SEQ: AtomicU64 = AtomicU64::new(1);

fn unique_name(input: &serde_json::Value) -> String {
    opt_str(input, "name")
        .unwrap_or_else(|| format!("ffi_{}", SHAPE_SEQ.fetch_add(1, Ordering::SeqCst)))
}

fn finish(shape: &IvgShape, name: &str) -> ApiResult {
    let _ = shape.set_name(name);
    Ok(shape_info(shape))
}

pub fn create_rectangle(input: &serde_json::Value) -> ApiResult {
    let name = unique_name(input);
    let x = opt_f64(input, "x").unwrap_or(0.0);
    let y = opt_f64(input, "y").unwrap_or(0.0);
    let width = opt_f64(input, "width").unwrap_or(100.0);
    let height = opt_f64(input, "height").unwrap_or(100.0);
    let radius = opt_f64(input, "radius").unwrap_or(0.0);

    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let layer = resolve_layer(&doc, input)?;
        let shape = layer
            .create_rectangle2(x, y, width, height, radius, radius, radius, radius)
            .ok_or_else(|| ApiError::new("create_failed", "CreateRectangle2 failed"))?;
        finish(&shape, &name)
    })
}

pub fn create_ellipse(input: &serde_json::Value) -> ApiResult {
    let name = unique_name(input);
    let cx = get_f64(input, "cx")?;
    let cy = get_f64(input, "cy")?;
    let rx = opt_f64(input, "rx").unwrap_or(25.0);
    let ry = opt_f64(input, "ry").unwrap_or(25.0);

    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let layer = resolve_layer(&doc, input)?;
        let shape = layer
            .create_ellipse2(cx, cy, rx, ry, 0.0, 360.0, false)
            .ok_or_else(|| ApiError::new("create_failed", "CreateEllipse2 failed"))?;
        finish(&shape, &name)
    })
}

pub fn create_polygon(input: &serde_json::Value) -> ApiResult {
    let name = unique_name(input);
    let x = opt_f64(input, "x").unwrap_or(0.0);
    let y = opt_f64(input, "y").unwrap_or(0.0);
    let width = opt_f64(input, "width").unwrap_or(100.0);
    let height = opt_f64(input, "height").unwrap_or(100.0);
    let sides = opt_i32(input, "sides").unwrap_or(5);
    let star = opt_bool(input, "star").unwrap_or(false);

    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let layer = resolve_layer(&doc, input)?;
        let shape = layer
            .create_polygon(
                x,
                y,
                x + width,
                y + height,
                sides,
                1,
                1,
                star,
                1,
                1,
            )
            .ok_or_else(|| ApiError::new("create_failed", "CreatePolygon failed"))?;
        finish(&shape, &name)
    })
}

pub fn create_text(input: &serde_json::Value) -> ApiResult {
    let name = unique_name(input);
    let text = get_str(input, "text")?;
    let font = opt_str(input, "font").unwrap_or_else(|| "Arial".to_string());
    let size = opt_f64(input, "size").unwrap_or(24.0) as f32;
    let x = opt_f64(input, "x").unwrap_or(0.0);
    let y = opt_f64(input, "y").unwrap_or(0.0);

    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let layer = resolve_layer(&doc, input)?;
        let shape = layer
            .create_artistic_text(x, y, text, 0, 0, font, size, 0, 0, 0, 1)
            .ok_or_else(|| ApiError::new("create_failed", "CreateArtisticText failed"))?;
        finish(&shape, &name)
    })
}

pub fn info(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let shape = resolve_shape(&doc, input)?;
        Ok(shape_info(&shape))
    })
}

pub fn move_by(input: &serde_json::Value) -> ApiResult {
    let dx = get_f64(input, "dx")?;
    let dy = get_f64(input, "dy")?;
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let shape = resolve_shape(&doc, input)?;
        if !shape.move_by(dx, dy) {
            return Err(ApiError::new("move_failed", "Shape.MoveBy returned false"));
        }
        Ok(shape_info(&shape))
    })
}

pub fn set_size(input: &serde_json::Value) -> ApiResult {
    let width = get_f64(input, "width")?;
    let height = get_f64(input, "height")?;
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let shape = resolve_shape(&doc, input)?;
        if !shape.set_size(width, height) {
            return Err(ApiError::new("resize_failed", "Shape.SetSize returned false"));
        }
        Ok(shape_info(&shape))
    })
}

pub fn rotate(input: &serde_json::Value) -> ApiResult {
    let angle = get_f64(input, "angle")?;
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let shape = resolve_shape(&doc, input)?;
        if !shape.rotate(angle) {
            return Err(ApiError::new("rotate_failed", "Shape.Rotate returned false"));
        }
        Ok(shape_info(&shape))
    })
}

pub fn set_fill(input: &serde_json::Value) -> ApiResult {
    let r = opt_i32(input, "r").unwrap_or(255).clamp(0, 255);
    let g = opt_i32(input, "g").unwrap_or(0).clamp(0, 255);
    let b = opt_i32(input, "b").unwrap_or(0).clamp(0, 255);

    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let shape = resolve_shape(&doc, input)?;
        let color = app
            .create_rgb_color(r, g, b)
            .ok_or_else(|| ApiError::new("color_failed", "CreateRGBColor failed"))?;
        if !shape.apply_uniform_fill(&color) {
            return Err(ApiError::new(
                "fill_failed",
                "Shape.ApplyUniformFill returned false",
            ));
        }
        let fill = shape
            .fill()
            .ok_or_else(|| ApiError::new("fill_failed", "Shape.Fill unavailable"))?;
        Ok(json!({
            "shape": shape.name(),
            "fill": fill_info(&fill),
        }))
    })
}

pub fn set_outline(input: &serde_json::Value) -> ApiResult {
    let width = opt_f64(input, "width");
    let r = opt_i32(input, "r").unwrap_or(0).clamp(0, 255);
    let g = opt_i32(input, "g").unwrap_or(0).clamp(0, 255);
    let b = opt_i32(input, "b").unwrap_or(0).clamp(0, 255);

    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let shape = resolve_shape(&doc, input)?;
        let outline = shape
            .outline()
            .ok_or_else(|| ApiError::new("outline_failed", "Shape.Outline unavailable"))?;
        if let Some(width) = width {
            if !outline.set_width(width) {
                return Err(ApiError::new(
                    "outline_failed",
                    "Outline.Width assignment failed",
                ));
            }
        }
        let color = app
            .create_rgb_color(r, g, b)
            .ok_or_else(|| ApiError::new("color_failed", "CreateRGBColor failed"))?;
        if !outline.set_color(&color) {
            return Err(ApiError::new(
                "outline_failed",
                "Outline.Color assignment failed",
            ));
        }
        Ok(json!({
            "shape": shape.name(),
            "outline": outline_info(&outline),
        }))
    })
}

pub fn select(input: &serde_json::Value) -> ApiResult {
    let selected = opt_bool(input, "selected").unwrap_or(true);
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let shape = resolve_shape(&doc, input)?;
        if !shape.set_selected(selected) {
            return Err(ApiError::new(
                "select_failed",
                "Shape.Selected assignment failed",
            ));
        }
        Ok(json!({
            "name": shape.name(),
            "selected": shape.selected(),
        }))
    })
}

pub fn delete(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let shape = resolve_shape(&doc, input)?;
        let name = shape.name();
        if !shape.delete() {
            return Err(ApiError::new("delete_failed", "Shape.Delete returned false"));
        }
        Ok(json!({
            "deleted": name,
            "shapes": doc.active_page().and_then(|p| p.shapes()).and_then(|s| s.count()),
        }))
    })
}

//! View 域接口（活动视图的缩放 / 视口）。

use cdrsdk::prelude::IvgApplication;
use cdrsdk::view::IvgActiveView;
use serde_json::json;

use crate::com::with_app;
use crate::json::{get_f64, ApiError, ApiResult};

fn active_view(app: &IvgApplication) -> Result<IvgActiveView, ApiError> {
    let window = app
        .active_window()
        .ok_or_else(|| ApiError::new("no_window", "Application.ActiveWindow unavailable"))?;
    window
        .active_view()
        .ok_or_else(|| ApiError::new("no_view", "Window.ActiveView unavailable"))
}

fn view_info(view: &IvgActiveView) -> serde_json::Value {
    let area = view.get_view_area();
    json!({
        "type": view.view_type(),
        "origin": {"x": view.origin_x(), "y": view.origin_y()},
        "zoom": view.zoom(),
        "area": area.map(|(x, y, w, h)| json!({"x": x, "y": y, "width": w, "height": h})),
    })
}

/// `view_info` 接口：信息。
///
/// 选择器约定：`doc` / `page` / `layer` / `name` 为 1-based 索引或名称，省略时取活动对象。
pub fn info(_: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let view = active_view(app)?;
        Ok(view_info(&view))
    })
}

/// `view_set_zoom` 接口：设置缩放。
///
/// 入参字段：`zoom`。
///
/// 选择器约定：`doc` / `page` / `layer` / `name` 为 1-based 索引或名称，省略时取活动对象。
pub fn set_zoom(input: &serde_json::Value) -> ApiResult {
    let zoom = get_f64(input, "zoom")?;
    with_app(|app| {
        let view = active_view(app)?;
        if !view.set_zoom(zoom) {
            return Err(ApiError::new("zoom_failed", "ActiveView.Zoom assignment failed"));
        }
        Ok(view_info(&view))
    })
}

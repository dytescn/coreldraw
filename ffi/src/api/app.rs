//! Application 域接口。

use serde_json::json;

use crate::com::with_app;
use crate::info::{color_info, documents_list};
use crate::json::{get_f64, get_i32, get_str, opt_i32, ApiError, ApiResult};

/// 插件自检：不需要 CorelDRAW，也不触发 COM。
pub fn health(_: &serde_json::Value) -> ApiResult {
    Ok(json!({
        "plugin": "coreldraw",
        "sdk": "cdrsdk",
        "version": env!("CARGO_PKG_VERSION"),
        "interfaces": crate::INTERFACES,
    }))
}

pub fn info(_: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        Ok(json!({
            "version": app.version(),
            "version_major": app.version_major(),
            "version_minor": app.version_minor(),
            "version_build": app.version_build(),
            "path": app.path(),
            "config_path": app.config_path(),
            "setup_path": app.setup_path(),
            "program_path": app.program_path(),
            "user_data_path": app.user_data_path(),
            "language_path": app.language_path(),
            "visible": app.visible(),
            "unit": app.unit(),
            "documents": app.documents().and_then(|d| d.count()),
            "active_document": app.active_document().and_then(|d| d.name()),
        }))
    })
}

pub fn documents(_: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let docs = app
            .documents()
            .ok_or_else(|| ApiError::new("no_documents", "documents collection unavailable"))?;
        Ok(documents_list(&docs))
    })
}

pub fn create_document(_: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = app
            .create_document()
            .ok_or_else(|| ApiError::new("create_failed", "CreateDocument returned no document"))?;
        Ok(crate::info::doc_info(&doc))
    })
}

pub fn open_document(input: &serde_json::Value) -> ApiResult {
    let path = get_str(input, "path")?;
    if !std::path::Path::new(&path).is_file() {
        return Err(ApiError::new(
            "file_not_found",
            format!("file not found: {path}"),
        ));
    }
    with_app(|app| {
        let docs = app
            .documents()
            .ok_or_else(|| ApiError::new("no_documents", "documents collection unavailable"))?;
        let doc = docs
            .open(&path)
            .ok_or_else(|| ApiError::new("open_failed", format!("failed to open {path}")))?;
        Ok(crate::info::doc_info(&doc))
    })
}

pub fn convert_units(input: &serde_json::Value) -> ApiResult {
    let value = get_f64(input, "value")?;
    let from = get_i32(input, "from")?;
    let to = get_i32(input, "to")?;
    with_app(|app| {
        let converted = app
            .convert_units(value, from, to)
            .ok_or_else(|| ApiError::new("convert_failed", "unsupported unit code"))?;
        Ok(json!({
            "value": value,
            "from": from,
            "to": to,
            "result": converted,
        }))
    })
}

pub fn set_visible(input: &serde_json::Value) -> ApiResult {
    let visible = crate::json::opt_bool(input, "visible").unwrap_or(true);
    with_app(|app| {
        if !app.set_visible(visible) {
            return Err(ApiError::new(
                "set_visible_failed",
                "Application.Visible assignment failed",
            ));
        }
        Ok(json!({"visible": app.visible()}))
    })
}

pub fn create_color(input: &serde_json::Value) -> ApiResult {
    let r = opt_i32(input, "r").unwrap_or(0).clamp(0, 255);
    let g = opt_i32(input, "g").unwrap_or(0).clamp(0, 255);
    let b = opt_i32(input, "b").unwrap_or(0).clamp(0, 255);
    with_app(|app| {
        let color = app
            .create_rgb_color(r, g, b)
            .ok_or_else(|| ApiError::new("color_failed", "CreateRGBColor failed"))?;
        Ok(color_info(&color))
    })
}

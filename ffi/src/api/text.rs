//! Text 域接口。

use cdrsdk::prelude::*;
use serde_json::json;

use crate::com::with_app;
use crate::info::text_info;
use crate::json::{get_str, ApiError, ApiResult};
use crate::resolve::{resolve_doc, resolve_shape};

fn shape_text(shape: &IvgShape) -> Result<IvgText, ApiError> {
    shape
        .text()
        .ok_or_else(|| ApiError::new("not_text", "shape is not a text object"))
}

/// `text_info` 接口：信息。
///
/// 选择器约定：`doc` / `page` / `layer` / `name` 为 1-based 索引或名称，省略时取活动对象。
pub fn info(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let shape = resolve_shape(&doc, input)?;
        let text = shape_text(&shape)?;
        Ok(text_info(&text))
    })
}

/// `text_set_content` 接口：设置content。
///
/// 入参字段：`text`。
///
/// 选择器约定：`doc` / `page` / `layer` / `name` 为 1-based 索引或名称，省略时取活动对象。
pub fn set_content(input: &serde_json::Value) -> ApiResult {
    let content = get_str(input, "text")?;
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let shape = resolve_shape(&doc, input)?;
        let text = shape_text(&shape)?;
        let story = text
            .story()
            .ok_or_else(|| ApiError::new("no_story", "Text.Story unavailable"))?;
        if !story.set_text(&content) {
            return Err(ApiError::new(
                "set_text_failed",
                "TextRange.Text assignment failed",
            ));
        }
        Ok(json!({
            "shape": shape.name(),
            "text": text_info(&text),
        }))
    })
}

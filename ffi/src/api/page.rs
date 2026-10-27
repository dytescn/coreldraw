//! Page 域接口。


use crate::com::with_app;
use crate::json::{get_f64, ApiError, ApiResult};
use crate::resolve::{resolve_doc, resolve_page};

/// `page_info` 接口：信息。
///
/// 选择器约定：`doc` / `page` / `layer` / `name` 为 1-based 索引或名称，省略时取活动对象。
pub fn info(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let page = resolve_page(&doc, input)?;
        Ok(crate::info::page_info(&page))
    })
}

/// `page_set_size` 接口：设置尺寸。
///
/// 入参字段：`width`、`height`。
///
/// 选择器约定：`doc` / `page` / `layer` / `name` 为 1-based 索引或名称，省略时取活动对象。
pub fn set_size(input: &serde_json::Value) -> ApiResult {
    let width = get_f64(input, "width")?;
    let height = get_f64(input, "height")?;
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let page = resolve_page(&doc, input)?;
        if !page.set_size(width, height) {
            return Err(ApiError::new("set_size_failed", "Page.SetSize returned false"));
        }
        Ok(crate::info::page_info(&page))
    })
}

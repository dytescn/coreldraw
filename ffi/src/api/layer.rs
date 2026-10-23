//! Layer 域接口。

use serde_json::json;

use crate::com::with_app;
use crate::info::{layer_info, shapes_list};
use crate::json::{ApiError, ApiResult};
use crate::resolve::{resolve_doc, resolve_layer};

pub fn info(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let layer = resolve_layer(&doc, input)?;
        Ok(layer_info(&layer))
    })
}

pub fn shapes(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let layer = resolve_layer(&doc, input)?;
        let shapes = layer
            .shapes()
            .ok_or_else(|| ApiError::new("no_shapes", "layer shapes collection unavailable"))?;
        Ok(shapes_list(&shapes))
    })
}

pub fn delete(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let layer = resolve_layer(&doc, input)?;
        let name = layer.name();
        let index = layer.index();
        let is_special = layer.is_guides_layer() == Some(true)
            || layer.is_desktop_layer() == Some(true)
            || layer.is_grid_layer() == Some(true);
        if is_special {
            return Err(ApiError::new(
                "special_layer",
                "guides/desktop/grid layers cannot be deleted",
            ));
        }
        if !layer.delete() {
            return Err(ApiError::new("delete_failed", "Layer.Delete returned false"));
        }
        Ok(json!({
            "deleted": {"index": index, "name": name},
            "layers": doc.active_page().and_then(|p| p.layers()).and_then(|l| l.count()),
        }))
    })
}

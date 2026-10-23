//! Document 域接口。

use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::json;

use crate::com::with_app;
use crate::info::{doc_info, layer_info, layers_list, pages_list, shapes_list};
use crate::json::{get_str, opt_bool, opt_str, ApiError, ApiResult};
use crate::resolve::{resolve_doc, resolve_page};

static LAYER_SEQ: AtomicU64 = AtomicU64::new(1);

fn export_filter(input: &serde_json::Value) -> Result<i32, ApiError> {
    match opt_str(input, "filter").as_deref() {
        None | Some("png") => Ok(802),
        Some("jpg") | Some("jpeg") => Ok(774),
        Some(other) => other
            .parse::<i32>()
            .map_err(|_| ApiError::new("invalid_filter", format!("unsupported filter: {other}"))),
    }
}

pub fn info(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        Ok(doc_info(&doc))
    })
}

pub fn pages(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let pages = doc
            .pages()
            .ok_or_else(|| ApiError::new("no_pages", "pages collection unavailable"))?;
        Ok(pages_list(&pages))
    })
}

pub fn create_page(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let page = doc
            .create_page()
            .ok_or_else(|| ApiError::new("create_failed", "CreatePage failed"))?;
        Ok(crate::info::page_info(&page))
    })
}

pub fn delete_page(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let page = resolve_page(&doc, input)?;
        let count = doc.pages().and_then(|p| p.count()).unwrap_or(0);
        if count <= 1 {
            return Err(ApiError::new(
                "last_page",
                "cannot delete the only page of a document",
            ));
        }
        let index = page.index();
        let name = page.name();
        if !page.delete() {
            return Err(ApiError::new("delete_failed", "Page.Delete returned false"));
        }
        Ok(json!({
            "deleted": {"index": index, "name": name},
            "remaining": doc.pages().and_then(|p| p.count()),
        }))
    })
}

pub fn layers(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let page = resolve_page(&doc, input)?;
        let layers = page
            .layers()
            .ok_or_else(|| ApiError::new("no_layers", "layers collection unavailable"))?;
        Ok(layers_list(&layers))
    })
}

pub fn create_layer(input: &serde_json::Value) -> ApiResult {
    let name = opt_str(input, "layer")
        .or_else(|| opt_str(input, "name"))
        .unwrap_or_else(|| format!("ffi-layer-{}", LAYER_SEQ.fetch_add(1, Ordering::SeqCst)));
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let layer = doc
            .create_layer(&name)
            .ok_or_else(|| ApiError::new("create_failed", format!("CreateLayer({name:?}) failed")))?;
        Ok(layer_info(&layer))
    })
}

pub fn shapes(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        // CorelDRAW 的 `Document` 没有 `Shapes` 属性，图形挂在 Page 上。
        let page = resolve_page(&doc, input)?;
        let shapes = page
            .shapes()
            .ok_or_else(|| ApiError::new("no_shapes", "page shapes collection unavailable"))?;
        Ok(shapes_list(&shapes))
    })
}

pub fn selection(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let Some(range) = doc.selection_range() else {
            return Ok(json!({"count": 0, "items": []}));
        };
        let shapes = range
            .shapes()
            .ok_or_else(|| ApiError::new("no_shapes", "selection range has no shapes"))?;
        Ok(shapes_list(&shapes))
    })
}

pub fn clear_selection(input: &serde_json::Value) -> ApiResult {
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let cleared = doc.clear_selection();
        if !cleared {
            return Err(ApiError::new(
                "clear_failed",
                "Document.ClearSelection returned false",
            ));
        }
        Ok(json!({"cleared": true}))
    })
}

pub fn save_as(input: &serde_json::Value) -> ApiResult {
    let path = get_str(input, "path")?;
    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let saved = doc.save_as(&path);
        if !saved {
            return Err(ApiError::new("save_failed", format!("SaveAs({path:?}) returned false")));
        }
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        if size == 0 {
            return Err(ApiError::new("save_failed", format!("file not written: {path}")));
        }
        Ok(json!({
            "path": path,
            "size": size,
            "dirty": doc.dirty(),
            "saved": doc.saved(),
        }))
    })
}

pub fn export(input: &serde_json::Value) -> ApiResult {
    let path = get_str(input, "path")?;
    let filter = export_filter(input)?;
    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::remove_file(&path);

    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let exported = if let Some(options) = app.create_export_options() {
            let _ = options.set_overwrite(true);
            doc.export_ex(&path, filter, &options)
        } else {
            doc.export(&path, filter)
        };
        if !exported {
            return Err(ApiError::new("export_failed", format!("Export({path:?}) returned false")));
        }
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        if size == 0 {
            return Err(ApiError::new(
                "export_failed",
                format!("file not written: {path}"),
            ));
        }
        Ok(json!({
            "path": path,
            "filter": filter,
            "size": size,
        }))
    })
}

pub fn close(input: &serde_json::Value) -> ApiResult {
    let save = opt_bool(input, "save").unwrap_or(false);
    with_app(|app| {
        let doc = resolve_doc(app, input)?;
        let name = doc.name();
        let closed = if save {
            doc.close()
        } else {
            doc.close_without_saving()
        };
        if !closed {
            return Err(ApiError::new("close_failed", "Document.Close returned false"));
        }
        Ok(json!({
            "closed": true,
            "name": name,
            "documents": app.documents().and_then(|d| d.count()),
        }))
    })
}

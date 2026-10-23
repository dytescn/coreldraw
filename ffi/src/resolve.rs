//! 参数里的对象选择器：`doc` / `page` / `layer` / `name`。
//!
//! 约定：数字按 1-based 索引，字符串按名称；省略时取“活动对象”。

use cdrsdk::prelude::*;
use serde_json::Value;

use crate::json::{opt_i64, opt_str, ApiError};

pub fn resolve_doc(app: &IvgApplication, input: &Value) -> Result<IvgDocument, ApiError> {
    if let Some(index) = opt_i64(input, "doc") {
        let docs = app
            .documents()
            .ok_or_else(|| ApiError::new("no_documents", "documents collection unavailable"))?;
        return docs.item_by_index(index as i32).ok_or_else(|| {
            ApiError::new("doc_not_found", format!("document index {index} not found"))
        });
    }
    if let Some(name) = opt_str(input, "doc") {
        let docs = app
            .documents()
            .ok_or_else(|| ApiError::new("no_documents", "documents collection unavailable"))?;
        return docs
            .item_by_name(&name)
            .ok_or_else(|| ApiError::new("doc_not_found", format!("document {name:?} not found")));
    }
    app.active_document()
        .ok_or_else(|| ApiError::new("no_document", "no active document"))
}

pub fn resolve_page(doc: &IvgDocument, input: &Value) -> Result<IvgPage, ApiError> {
    if let Some(index) = opt_i64(input, "page") {
        let pages = doc
            .pages()
            .ok_or_else(|| ApiError::new("no_pages", "pages collection unavailable"))?;
        return pages.item(index as i32).ok_or_else(|| {
            ApiError::new("page_not_found", format!("page index {index} not found"))
        });
    }
    if let Some(name) = opt_str(input, "page") {
        let pages = doc
            .pages()
            .ok_or_else(|| ApiError::new("no_pages", "pages collection unavailable"))?;
        return pages
            .all()
            .into_iter()
            .find(|p| p.name().as_deref() == Some(name.as_str()))
            .ok_or_else(|| ApiError::new("page_not_found", format!("page {name:?} not found")));
    }
    doc.active_page()
        .ok_or_else(|| ApiError::new("no_page", "no active page"))
}

pub fn resolve_layer(doc: &IvgDocument, input: &Value) -> Result<IvgLayer, ApiError> {
    let page = resolve_page(doc, input)?;
    let layers = page
        .layers()
        .ok_or_else(|| ApiError::new("no_layers", "layers collection unavailable"))?;

    if let Some(index) = opt_i64(input, "layer") {
        let found = layers_bottom_up(&layers)
            .into_iter()
            .nth(index.saturating_sub(1) as usize);
        return found.ok_or_else(|| {
            ApiError::new("layer_not_found", format!("layer index {index} not found"))
        });
    }
    if let Some(name) = opt_str(input, "layer") {
        return layers
            .find(&name)
            .ok_or_else(|| ApiError::new("layer_not_found", format!("layer {name:?} not found")));
    }
    page.active_layer()
        .ok_or_else(|| ApiError::new("no_layer", "no active layer"))
}

fn layers_bottom_up(layers: &IvgLayers) -> Vec<IvgLayer> {
    let mut out = Vec::new();
    let mut current = layers.bottom();
    while let Some(layer) = current {
        current = layer.above(true);
        out.push(layer);
    }
    out
}

pub fn resolve_shape(doc: &IvgDocument, input: &Value) -> Result<IvgShape, ApiError> {
    if let Some(name) = opt_str(input, "name") {
        if let Some(shape) = doc.find_shape(&name, 0, 0, true) {
            return Ok(shape);
        }
        if let Some(shapes) = doc.active_page().and_then(|p| p.shapes()) {
            if let Some(shape) = shapes.all().into_iter().find(|s| s.name().as_deref() == Some(name.as_str())) {
                return Ok(shape);
            }
        }
        return Err(ApiError::new(
            "shape_not_found",
            format!("shape {name:?} not found"),
        ));
    }

    if let Some(index) = opt_i64(input, "index") {
        let shapes = doc
            .active_page()
            .and_then(|p| p.shapes())
            .ok_or_else(|| ApiError::new("no_shapes", "page shapes collection unavailable"))?;
        return shapes.item_by_index(index as i32).ok_or_else(|| {
            ApiError::new("shape_not_found", format!("shape index {index} not found"))
        });
    }

    doc.active_shape()
        .ok_or_else(|| ApiError::new("shape_not_found", "no active shape"))
}

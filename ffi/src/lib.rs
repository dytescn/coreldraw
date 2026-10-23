//! # coreldraw —— cyctron libary 插件
//!
//! 把 `cdrsdk` 的核心接口以 C ABI 动态库的形式暴露给 [cyctron]：
//!
//! ```text
//! POST /coreldraw
//! FFI-Symbol: app_info
//! Content-Type: application/json
//!
//! {"..."}  →  {"ok":true,"method":"POST","interface":"app_info","data":{...}}
//! ```
//!
//! ## 约定
//!
//! - 每个接口一个导出符号：`post_<interface>`，例如 `app_info` → `post_app_info`。
//! - 入参是 UTF-8 JSON（可为 `NULL`），出参写入 64 KB 缓冲区，以 `\0` 结尾。
//! - 业务错误返回 `0` + `{"ok":false,"error":"...","message":"..."}`；
//!   只有输出指针为空时返回 `-1`。
//! - 对象选择器：`doc` / `page` / `layer` / `name`，数字为 1-based 索引，
//!   字符串为名称，省略时取活动对象。
//!
//! [cyctron]: ../docs/libary.md

mod api;
mod com;
mod info;
mod json;
mod resolve;

use std::ffi::c_char;
use std::os::raw::c_int;

/// 全部接口名（与导出符号一一对应，`health` 也会返回这份清单）。
pub const INTERFACES: &[&str] = &[
    // meta
    "health",
    // application
    "app_info",
    "app_documents",
    "app_create_document",
    "app_open_document",
    "app_convert_units",
    "app_create_color",
    "app_set_visible",
    // document
    "doc_info",
    "doc_pages",
    "doc_create_page",
    "doc_delete_page",
    "doc_layers",
    "doc_create_layer",
    "doc_shapes",
    "doc_selection",
    "doc_clear_selection",
    "doc_save_as",
    "doc_export",
    "doc_close",
    // page
    "page_info",
    "page_set_size",
    // layer
    "layer_info",
    "layer_shapes",
    "layer_delete",
    // shape
    "shape_create_rectangle",
    "shape_create_ellipse",
    "shape_create_polygon",
    "shape_create_text",
    "shape_info",
    "shape_move",
    "shape_set_size",
    "shape_rotate",
    "shape_set_fill",
    "shape_set_outline",
    "shape_select",
    "shape_delete",
    // text
    "text_info",
    "text_set_content",
    // view
    "view_info",
    "view_set_zoom",
];

macro_rules! export_interface {
    ($symbol:ident, $name:expr, $handler:path) => {
        /// libary C ABI：`int post_xxx(const char *json_in, char *json_out)`
        #[no_mangle]
        pub extern "C" fn $symbol(json_in: *const c_char, json_out: *mut c_char) -> c_int {
            let input = json::read_input(json_in);
            json::write_result(json_out, $name, $handler(&input))
        }
    };
}

// meta
export_interface!(post_health, "health", api::app::health);
// application
export_interface!(post_app_info, "app_info", api::app::info);
export_interface!(post_app_documents, "app_documents", api::app::documents);
export_interface!(
    post_app_create_document,
    "app_create_document",
    api::app::create_document
);
export_interface!(
    post_app_open_document,
    "app_open_document",
    api::app::open_document
);
export_interface!(
    post_app_convert_units,
    "app_convert_units",
    api::app::convert_units
);
export_interface!(
    post_app_create_color,
    "app_create_color",
    api::app::create_color
);
export_interface!(
    post_app_set_visible,
    "app_set_visible",
    api::app::set_visible
);
// document
export_interface!(post_doc_info, "doc_info", api::document::info);
export_interface!(post_doc_pages, "doc_pages", api::document::pages);
export_interface!(
    post_doc_create_page,
    "doc_create_page",
    api::document::create_page
);
export_interface!(
    post_doc_delete_page,
    "doc_delete_page",
    api::document::delete_page
);
export_interface!(post_doc_layers, "doc_layers", api::document::layers);
export_interface!(
    post_doc_create_layer,
    "doc_create_layer",
    api::document::create_layer
);
export_interface!(post_doc_shapes, "doc_shapes", api::document::shapes);
export_interface!(post_doc_selection, "doc_selection", api::document::selection);
export_interface!(
    post_doc_clear_selection,
    "doc_clear_selection",
    api::document::clear_selection
);
export_interface!(post_doc_save_as, "doc_save_as", api::document::save_as);
export_interface!(post_doc_export, "doc_export", api::document::export);
export_interface!(post_doc_close, "doc_close", api::document::close);
// page
export_interface!(post_page_info, "page_info", api::page::info);
export_interface!(post_page_set_size, "page_set_size", api::page::set_size);
// layer
export_interface!(post_layer_info, "layer_info", api::layer::info);
export_interface!(post_layer_shapes, "layer_shapes", api::layer::shapes);
export_interface!(post_layer_delete, "layer_delete", api::layer::delete);
// shape
export_interface!(
    post_shape_create_rectangle,
    "shape_create_rectangle",
    api::shape::create_rectangle
);
export_interface!(
    post_shape_create_ellipse,
    "shape_create_ellipse",
    api::shape::create_ellipse
);
export_interface!(
    post_shape_create_polygon,
    "shape_create_polygon",
    api::shape::create_polygon
);
export_interface!(
    post_shape_create_text,
    "shape_create_text",
    api::shape::create_text
);
export_interface!(post_shape_info, "shape_info", api::shape::info);
export_interface!(post_shape_move, "shape_move", api::shape::move_by);
export_interface!(
    post_shape_set_size,
    "shape_set_size",
    api::shape::set_size
);
export_interface!(post_shape_rotate, "shape_rotate", api::shape::rotate);
export_interface!(
    post_shape_set_fill,
    "shape_set_fill",
    api::shape::set_fill
);
export_interface!(
    post_shape_set_outline,
    "shape_set_outline",
    api::shape::set_outline
);
export_interface!(post_shape_select, "shape_select", api::shape::select);
export_interface!(post_shape_delete, "shape_delete", api::shape::delete);
// text
export_interface!(post_text_info, "text_info", api::text::info);
export_interface!(
    post_text_set_content,
    "text_set_content",
    api::text::set_content
);
// view
export_interface!(post_view_info, "view_info", api::view::info);
export_interface!(
    post_view_set_zoom,
    "view_set_zoom",
    api::view::set_zoom
);

//! 文档层
//!
//! - [`IvgDocument`]        : 单个文档（`Document`）
//! - [`IvgDocuments`]       : 文档集合（`Documents`）
//! - [`IvgMetadata`]        : 文档元数据（`Metadata`）
//! - [`document_event`]     : 文档事件 sink（`DocumentEvents`）
//! - [`data_fields`]        : 数据字段 / 数据项

pub mod document;
pub mod documents;
pub mod document_event;
pub mod metadata;
pub mod data_fields;

pub use document::IvgDocument;
pub use documents::IvgDocuments;
pub use metadata::IvgMetadata;
pub use data_fields::{
    IvgDataField, IvgDataFields, IvgDataItem, IvgDataItems,
};
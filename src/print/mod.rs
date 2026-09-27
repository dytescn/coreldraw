//! 打印相关
//!
//! - [`IvgPrnPrinters`] / [`IvgPrnPrinter`]     : 打印机集合 / 打印机
//! - [`IvgPrnJob`]                              : 打印任务（`IPrnVBAPrintJob`）
//! - [`IvgPrnDocument`] / [`IvgPrnDocuments`]   : 打印文档
//! - [`IvgPrnPage`] / [`IvgPrnPages`]           : 打印页面
//! - [`IvgPrnSettings`]                         : 打印设置（`IPrnVBAPrintSettings`）
//! - [`IvgPrnSeparations`] 等                   : 分色
//! - [`IvgPrnPrepress`]                         : 印前
//! - [`IvgPrnPostScript`]                       : PostScript
//! - [`IvgPrnTrapping`] 等                      : 陷印
//! - [`IvgPrnOptions`]                          : 打印选项
//! - [`IvgPrnLayout`]                           : 打印排版
//! - [`IvgPdfVbaSettings`]                      : PDF 导出设置（`IPDFVBASettings`）

pub mod settings;
pub mod job;
pub mod document;
pub mod printer;
pub mod separations;
pub mod prepress;
pub mod postscript;
pub mod trapping;
pub mod options;
pub mod layout;
pub mod pdf_settings;

pub use settings::IvgPrnSettings;
pub use job::IvgPrnJob;
pub use document::{IvgPrnDocument, IvgPrnDocuments, IvgPrnPage, IvgPrnPages};
pub use printer::{IvgPrnPrinter, IvgPrnPrinters};
pub use separations::{
    IvgPrnSeparations, IvgPrnSeparationPlate, IvgPrnSeparationPlates,
};
pub use prepress::IvgPrnPrepress;
pub use postscript::IvgPrnPostScript;
pub use trapping::{IvgPrnTrapping, IvgPrnTrapLayer, IvgPrnTrapLayers};
pub use options::IvgPrnOptions;
pub use layout::IvgPrnLayout;
pub use pdf_settings::IvgPdfVbaSettings;
//! COM 结构体（`IVGStructXxx` 系列）
//!
//! 这些对象不直接"使用"，而是由 [`crate::app::IvgApplication`] 工厂
//! 创建，作为参数传给其它 API。
//!
//! ```no_run
//! use cdrsdk::prelude::*;
//!
//! let app = IvgApplication::new("26").expect("CorelDRAW 未启动");
//! let doc = app.create_document().expect("create_document");
//!
//! let opts = app.create_export_options().expect("create_export_options");
//! opts.set_overwrite(true);
//!
//! // cdrFilter::PNG 见 crate::enums::filter
//! doc.export_ex("out.png", 790, &opts);
//!
//! doc.close_without_saving();
//! ```

pub mod save_as_options;
pub mod export_options;
pub mod import_options;
pub mod open_options;
pub mod create_options;
pub mod paste_options;
pub mod palette_options;
pub mod font_properties;
pub mod align_properties;
pub mod space_properties;
pub mod hyphenation_settings;
pub mod color_conversion_options;

pub use save_as_options::IvgStructSaveAsOptions;
pub use export_options::IvgStructExportOptions;
pub use import_options::IvgStructImportOptions;
pub use open_options::IvgStructOpenOptions;
pub use create_options::IvgStructCreateOptions;
pub use paste_options::IvgStructPasteOptions;
pub use palette_options::IvgStructPaletteOptions;
pub use font_properties::IvgStructFontProperties;
pub use align_properties::IvgStructAlignProperties;
pub use space_properties::IvgStructSpaceProperties;
pub use hyphenation_settings::IvgStructHyphenationSettings;
pub use color_conversion_options::IvgStructColorConversionOptions;
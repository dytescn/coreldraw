//! COM 结构体（`IVGStructXxx` 系列）
//!
//! 这些对象不直接"使用"，而是由 [`crate::app::IvgApplication`] 工厂
//! 创建，作为参数传给其它 API。
//!
//! ```no_run
//! use coreldraw::prelude::*;
//!
//! let app = IvgApplication::new("24.0").unwrap();
//! let opts = app.create_export_options().unwrap();
//! opts.set_overwrite(true);
//! doc.export_ex("out.png", cdrFilter::PNG as i32, &opts);
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
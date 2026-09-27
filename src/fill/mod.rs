//! 填充相关
//!
//! - [`IvgFill`]              : 填充总入口（`IVGFill`）
//! - [`IvgFountainFill`]      : 渐变填充（`IVGFountainFill`）
//! - [`IvgPatternFill`]       : 图案填充（`IVGPatternFill`）
//! - [`IvgTextureFill`]       : 纹理填充（`IVGTextureFill`）
//! - [`IvgPostScriptFill`]    : PostScript 填充（`IVGPostScriptFill`）
//! - [`IvgHatchFill`]         : 剖面线填充（`IVGHatchFill`）
//! - [`IvgFillMetadata`]      : 填充元数据

pub mod fill;
pub mod fountain;
pub mod pattern;
pub mod texture;
pub mod postscript;
pub mod hatch;
pub mod metadata;

pub use fill::IvgFill;
pub use fountain::{IvgFountainFill, IvgFountainColor, IvgFountainColors};
pub use pattern::{IvgPatternFill, IvgPatternCanvas, IvgPatternCanvases};
pub use texture::{
    IvgTextureFill, IvgTextureFillProperty, IvgTextureFillProperties,
};
pub use postscript::{IvgPostScriptFill, IvgPSScreenOptions};
pub use hatch::{
    IvgHatchFill, IvgHatchPattern, IvgHatchPatterns,
    IvgHatchLibrary, IvgHatchLibraries, IvgHatchFills,
};
pub use metadata::{IvgFillMetadata, IvgLocalizableString};
//! 颜色相关
//!
//! - [`IvgColor`]              : 单个颜色（`IVGColor`）
//! - [`IvgColors`]             : 颜色集合（`IVGColors`）
//! - [`IvgColorContext`]       : 颜色上下文（`IVGColorContext`）
//! - [`IvgDuotone`]            : 双色套印（`IVGDuotone`）
//! - [`IvgColorManager`]       : 颜色管理器（`IVGColorManager`）

pub mod color;
pub mod colors;
pub mod context;
pub mod duotone;
pub mod manager;

pub use color::IvgColor;
pub use colors::IvgColors;
pub use context::IvgColorContext;
pub use duotone::{IvgDuotone, IvgDuotoneInk, IvgDuotoneOverprint};
pub use manager::{
    IvgColorManager, IvgColorManagementPolicy,
    IvgColorProfile, IvgColorProfiles,
};
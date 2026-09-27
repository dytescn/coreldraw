//! 轮廓相关
//!
//! - [`IvgOutline`]        : 单个图形的轮廓（`IVGOutline`）
//! - [`IvgOutlineStyle`]   : 轮廓线型（`IVGOutlineStyle`）
//! - [`IvgOutlineStyles`]  : 轮廓线型集合（`IVGOutlineStyles`）
//! - [`IvgArrowHead`]      : 箭头（`IVGArrowHead`）
//! - [`IvgArrowHeads`]     : 箭头集合（`IVGArrowHeads`）
//! - [`IvgArrowHeadOptions`] : 箭头几何调整（`IVGArrowHeadOptions`）

pub mod outline;
pub mod outline_style;
pub mod arrow_head;

pub use outline::IvgOutline;
pub use outline_style::{IvgOutlineStyle, IvgOutlineStyles};
pub use arrow_head::{IvgArrowHead, IvgArrowHeads, IvgArrowHeadOptions};
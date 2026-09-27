//! 样式（Style / Styles / StyleSheet）
//!
//! - [`IvgStyle`]            : 单个样式（`IVGStyle`）
//! - [`IvgStyles`]           : 样式集合（`IVGStyles`）
//! - [`IvgStyleSheet`]       : 样式表（`IVGStyleSheet`）
//! - 具体子属性：
//!   - [`IvgStyleOutline`]      : 轮廓
//!   - [`IvgStyleFill`]         : 填充
//!   - [`IvgStyleCharacter`]    : 字符
//!   - [`IvgStyleParagraph`]    : 段落
//!   - [`IvgStyleFrame`]        : 文本框
//!   - [`IvgStyleTransparency`] : 透明

pub mod style;
pub mod styles;
pub mod style_sheet;
pub mod style_outline;
pub mod style_fill;
pub mod style_character;
pub mod style_paragraph;
pub mod style_frame;
pub mod style_transparency;

pub use style::IvgStyle;
pub use styles::IvgStyles;
pub use style_sheet::IvgStyleSheet;
pub use style_outline::IvgStyleOutline;
pub use style_fill::IvgStyleFill;
pub use style_character::IvgStyleCharacter;
pub use style_paragraph::IvgStyleParagraph;
pub use style_frame::IvgStyleFrame;
pub use style_transparency::IvgStyleTransparency;
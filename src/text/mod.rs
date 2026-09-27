//! 文本相关
//!
//! - [`IvgText`]              : 文本对象（`IVGText`）
//! - [`IvgTextRange`]         : 文本范围（`IVGTextRange`）
//! - [`IvgTextRanges`]        : 文本范围集合（`IVGTextRanges`）
//! - [`IvgTextFrame`]         : 文本框（`IVGTextFrame`）
//! - [`IvgTextFrames`]        : 文本框集合（`IVGTextFrames`）
//! - [`IvgTextCharacters`]    : 字符视图
//! - [`IvgTextWords`]         : 词视图
//! - [`IvgTextLines`]         : 行视图
//! - [`IvgTextParagraphs`]    : 段落视图
//! - [`IvgTextColumns`]       : 分栏视图
//! - [`IvgTextTabPosition(s)`]: 制表位

pub mod text;
pub mod range;
pub mod frame;
pub mod characters;
pub mod words;
pub mod lines;
pub mod paragraphs;
pub mod columns;
pub mod tab_positions;

pub use text::IvgText;
pub use range::{IvgTextRange, IvgTextRanges};
pub use frame::{IvgTextFrame, IvgTextFrames};
pub use characters::IvgTextCharacters;
pub use words::IvgTextWords;
pub use lines::IvgTextLines;
pub use paragraphs::IvgTextParagraphs;
pub use columns::IvgTextColumns;
pub use tab_positions::{IvgTextTabPosition, IvgTextTabPositions};
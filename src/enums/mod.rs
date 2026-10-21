//! CorelDRAW 枚举类型
//!
//! 命名保持与 VGCore 类型库一致（`cdrXxx` / `clrXxx` / `PrnXxx` / `pdfXxx` / `cuiXxx`），
//! 每个枚举用 `#[repr(i32)]` 映射 COM 的实际整数值。
//!
//! ```no_run
//! use cdrsdk::enums::cdrShapeType;                 // ← 改
//! let t: i32 = 6;
//! assert_eq!(cdrShapeType::from_i32(t), Some(cdrShapeType::TextShape));
//! ```
pub mod app;
pub mod color;
pub mod curve;
pub mod effect;
pub mod fill;
pub mod filter;
pub mod image;
pub mod layer;
pub mod outline;
pub mod page;
pub mod print;
pub mod shape;
pub mod style;
pub mod text;
pub mod ui;
pub mod view;

pub use app::*;
pub use color::*;
pub use curve::*;
pub use effect::*;
pub use fill::*;
pub use filter::*;
pub use image::*;
pub use layer::*;
pub use outline::*;
pub use page::*;
pub use print::*;
pub use shape::*;
pub use style::*;
pub use text::*;
pub use ui::*;
pub use view::*;

//! CorelDRAW 枚举类型
//!
//! 命名保持与 VGCore 类型库一致（`cdrXxx` / `clrXxx` / `PrnXxx` / `pdfXxx` / `cuiXxx`），
//! 每个枚举用 `#[repr(i32)]` 映射 COM 的实际整数值。
//!
//! ```no_run
//! use coreldraw::enums::cdrShapeType;
//! let t: i32 = 6;
//! assert_eq!(cdrShapeType::from_i32(t), Some(cdrShapeType::TextShape));
//! ```

pub mod app;
pub mod shape;
pub mod curve;
pub mod color;
pub mod fill;
pub mod outline;
pub mod text;
pub mod effect;
pub mod style;
pub mod page;
pub mod layer;
pub mod view;
pub mod filter;
pub mod image;
pub mod print;
pub mod ui;

pub use app::*;
pub use shape::*;
pub use curve::*;
pub use color::*;
pub use fill::*;
pub use outline::*;
pub use text::*;
pub use effect::*;
pub use style::*;
pub use page::*;
pub use layer::*;
pub use view::*;
pub use filter::*;
pub use image::*;
pub use print::*;
pub use ui::*;
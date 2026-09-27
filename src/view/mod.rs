//! 视图 / 窗口
//!
//! - [`IvgActiveView`]       : 活动视图（`IVGActiveView`），从 `window.active_view()` 取得
//! - [`IvgView`]             : 视图（`IVGView`），保存在文档里的命名视图
//! - [`IvgViews`]            : 视图集合（`IVGViews`）
//! - [`IvgWindow`]           : 文档窗口（`IVGWindow`）
//! - [`IvgWindows`]          : 窗口集合（`IVGWindows`）
//! - [`IvgProofColorSettings`] : 校样颜色设置（`IVGProofColorSettings`）

pub mod active_view;
pub mod view;
pub mod views;
pub mod window;
pub mod windows;

pub use active_view::{IvgActiveView, IvgProofColorSettings};
pub use view::IvgView;
pub use views::IvgViews;
pub use window::IvgWindow;
pub use windows::IvgWindows;
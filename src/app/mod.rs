//! CorelDRAW 应用层对象
//!
//! - [`IvgApplication`]     : 主入口，`CorelDRAW.Application.{ver}`
//! - [`IvgAppStatus`]       : 进度条 / 状态（`IVGAppStatus`）
//! - [`IvgAppWindow`]       : 应用主窗口（`IVGAppWindow`）
//! - [`application_event`]  : 事件 sink（`IVGApplicationEvents`）

pub mod application;
pub mod application_event;
pub mod status;
pub mod window;

pub use application::IvgApplication;
pub use status::IvgAppStatus;
pub use window::IvgAppWindow;
//! CorelDRAW UI 框架（`ICUI*` 系列）
//!
//! 这一层跟主 `IVG*` API 相对独立，服务于插件 / 工具条 / 停靠面板开发。
//!
//! - [`ICuiApplication`]      : UI 应用入口
//! - [`ICuiFrameWork`]        : 框架
//! - [`ICuiCommandBar(s)`]    : 工具条
//! - [`ICuiControl(s)`]       : 工具条上的控件
//! - [`ICuiFrameWindow(s)`]   : 框架窗口
//! - [`ICuiViewHost(s)`]      : 视图宿主
//! - [`ICuiViewWindow(s)`]    : 视图窗口
//! - [`ICuiDockHost(s)`]      : 停靠宿主
//! - [`ICuiDockItem(s)`]      : 停靠项
//! - [`ICuiScreenRect`]       : 屏幕坐标矩形
//! - [`ICuiDataContext`] 等   : 数据上下文
//! - [`ICuiImageList`]        : 图像列表
//! - [`ICuiBitmapImage`]      : 位图图像
//! - [`ICuiStatusText`]       : 状态栏文本
//! - [`ICuiWarning`]          : 警告对话框
//! - [`ICuiTaskManager`] 等   : 任务管理
//! - [`ICuiAutomation`] 等    : UI 自动化

pub mod application;
pub mod framework;
pub mod command_bar;
pub mod control;
pub mod frame_window;
pub mod view_host;
pub mod view_window;
pub mod dock_host;
pub mod dock_item;
pub mod screen_rect;
pub mod data_context;
pub mod image_list;
pub mod status_text;
pub mod warning;
pub mod task_manager;
pub mod automation;

pub use application::ICuiApplication;
pub use framework::ICuiFrameWork;
pub use command_bar::{
    ICuiCommandBar, ICuiCommandBars, ICuiCommandBarMode, ICuiCommandBarModes,
};
pub use control::{ICuiControl, ICuiControls};
pub use frame_window::{ICuiFrameWindow, ICuiFrameWindows};
pub use view_host::{ICuiViewHost, ICuiViewHosts};
pub use view_window::{ICuiViewWindow, ICuiViewWindows};
pub use dock_host::{ICuiDockHost, ICuiDockHosts};
pub use dock_item::{ICuiDockItem, ICuiDockItems};
pub use screen_rect::ICuiScreenRect;
pub use data_context::{
    ICuiDataContext, ICuiDataSourceFactory, ICuiDataSourceProxy,
};
pub use image_list::{ICuiImageList, ICuiBitmapImage};
pub use status_text::ICuiStatusText;
pub use warning::ICuiWarning;
pub use task_manager::{
    ICuiTaskManager, ICuiTask, ICuiBackgroundTask,
    ICuiRunningTask, ICuiRunningBackgroundTask,
};
pub use automation::{ICuiAutomation, ICuiControlData};
//! 杂项（不属于其它模块的接口）
//!
//! - [`IvgProperties`]     : 属性包（`IVGProperties`）
//! - [`IvgTransparency`]   : 透明（`IVGTransparency`）
//! - [`IvgUrl`]            : URL（`IVGURL`）
//! - [`IvgComponent`]      : 组件（`IVGComponent(s)`）
//! - [`IvgCloneLink`]      : 克隆链（`IVGCloneLink`）
//! - [`IvgClipboard`]      : 剪贴板（`IVGClipboard`）
//! - [`IvgRulers`]         : 标尺（`IVGRulers`）
//! - [`IvgGrid`]           : 网格（`IVGGrid`）
//! - [`IvgRecentFile(s)`]  : 最近文件
//! - [`IvgFontList`]       : 字体列表
//! - [`IvgWorkspace(s)`]   : 工作区
//! - [`IvgPalette(s)`]     : 调色板 / 调色板管理器
//! - [`ICorelScriptTools`] : 脚本工具
//! - [`IvgGmsManager`] 等  : GMS 宏
//! - [`IvgOnScreen*`]      : 屏幕绘制对象
//! - [`IvgToolState`] 等   : 工具状态 / 工具图形
//! - [`IvgTraceSettings`]  : 描摹设置

pub mod properties;
pub mod transparency;
pub mod url;
pub mod component;
pub mod clone_link;
pub mod clipboard;
pub mod ruler;
pub mod grid;
pub mod recent_files;
pub mod font_list;
pub mod workspace;
pub mod palette;
pub mod script_tools;
pub mod gms;
pub mod on_screen;
pub mod tool_state;
pub mod tool_shape;
pub mod trace_settings;
pub mod filetypes;
pub mod opplist;

pub use properties::IvgProperties;
pub use transparency::IvgTransparency;
pub use url::IvgUrl;
pub use component::{IvgComponent, IvgComponents};
pub use clone_link::IvgCloneLink;
pub use clipboard::IvgClipboard;
pub use ruler::IvgRulers;
pub use grid::IvgGrid;
pub use recent_files::{IvgRecentFile, IvgRecentFiles};
pub use font_list::IvgFontList;
pub use workspace::{IvgWorkspace, IvgWorkspaces};
pub use palette::{IvgPalette, IvgPalettes, IvgPaletteManager};
pub use script_tools::ICorelScriptTools;
pub use gms::{
    IvgGmsManager, IvgGmsProject, IvgGmsProjects,
    IvgGmsMacro, IvgGmsMacros,
};
pub use on_screen::{IvgOnScreenCurve, IvgOnScreenHandle, IvgOnScreenText};
pub use tool_state::{IvgToolState, IvgToolStateAttributes};
pub use tool_shape::{IvgToolShape, IvgToolShapeAttributes};
pub use trace_settings::IvgTraceSettings;
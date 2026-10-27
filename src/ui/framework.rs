//! `ICUIFrameWork` —— UI 框架

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{
    ICuiApplication, ICuiCommandBar, ICuiCommandBars, ICuiFrameWindow,
    ICuiFrameWindows, ICuiTaskManager,
};

/// CorelDRAW 对象的后期绑定包装（文本框工作）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiFrameWork {
    disp: ComObject,
}

impl ICuiFrameWork {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 名称 / 应用
    // ---------------------------------------------------------

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    /// 读取应用（`Application`）。
    pub fn application(&self) -> Option<ICuiApplication> {
        self.prop_dispatch("Application").map(ICuiApplication::new)
    }

    // ---------------------------------------------------------
    // 工具条/ 菜单
    // ---------------------------------------------------------

    /// 读取 COM 属性 `CommandBars`。
    pub fn command_bars(&self) -> Option<ICuiCommandBars> {
        self.prop_dispatch("CommandBars").map(ICuiCommandBars::new)
    }

    /// 读取主菜单（`MainMenu`）。
    pub fn main_menu(&self) -> Option<ICuiCommandBar> {
        self.prop_dispatch("MainMenu").map(ICuiCommandBar::new)
    }

    /// 读取状态栏（`StatusBar`）。
    pub fn status_bar(&self) -> Option<ICuiCommandBar> {
        self.prop_dispatch("StatusBar").map(ICuiCommandBar::new)
    }

    // ---------------------------------------------------------
    // 窗口
    // ---------------------------------------------------------

    /// 读取文本框窗口集（`FrameWindows`）。
    pub fn frame_windows(&self) -> Option<ICuiFrameWindows> {
        self.prop_dispatch("FrameWindows").map(ICuiFrameWindows::new)
    }

    /// 读取主文本框窗口（`MainFrameWindow`）。
    pub fn main_frame_window(&self) -> Option<ICuiFrameWindow> {
        self.prop_dispatch("MainFrameWindow").map(ICuiFrameWindow::new)
    }

    // ---------------------------------------------------------
    // 任务
    // ---------------------------------------------------------

    /// 读取任务管理器（`TaskManager`）。
    pub fn task_manager(&self) -> Option<ICuiTaskManager> {
        self.prop_dispatch("TaskManager").map(ICuiTaskManager::new)
    }

    // ---------------------------------------------------------
    // 工作区
    // ---------------------------------------------------------

    /// 导入工作区（COM 方法 `ImportWorkspace`）。
    pub fn import_workspace(&self, file_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp.invoke_method("ImportWorkspace", args).is_ok()
    }

    // ---------------------------------------------------------
    // Docker（停靠面板）
    // ---------------------------------------------------------

    /// 显示停靠面板（COM 方法 `ShowDocker`）。
    pub fn show_docker(&self, guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(guid.into())];
        self.disp.invoke_method("ShowDocker", args).is_ok()
    }

    /// 隐藏停靠面板（COM 方法 `HideDocker`）。
    pub fn hide_docker(&self, guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(guid.into())];
        self.disp.invoke_method("HideDocker", args).is_ok()
    }

    /// 是否停靠面板可见性（COM 方法 `IsDockerVisible`）。
    pub fn is_docker_visible(&self, guid: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(guid.into())];
        self.disp
            .invoke_method("IsDockerVisible", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 添加停靠面板（COM 方法 `AddDocker`）。
    pub fn add_docker(
        &self,
        guid: impl Into<String>,
        class_name: impl Into<String>,
        assembly_path: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(guid.into()),
            Variant::from_str(class_name.into()),
            Variant::from_str(assembly_path.into()),
        ];
        self.disp.invoke_method("AddDocker", args).is_ok()
    }

    /// 移除停靠面板（COM 方法 `RemoveDocker`）。
    pub fn remove_docker(&self, guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(guid.into())];
        self.disp.invoke_method("RemoveDocker", args).is_ok()
    }

    // ---------------------------------------------------------
    // 对话框
    // ---------------------------------------------------------

    /// 显示对话框（COM 方法 `ShowDialog`）。
    pub fn show_dialog(&self, guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(guid.into())];
        self.disp.invoke_method("ShowDialog", args).is_ok()
    }

    /// 隐藏对话框（COM 方法 `HideDialog`）。
    pub fn hide_dialog(&self, guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(guid.into())];
        self.disp.invoke_method("HideDialog", args).is_ok()
    }

    // ---------------------------------------------------------
    // 消息框
    // ---------------------------------------------------------

    /// `cuiMessageBoxFlags` 见 `enums::ui`。
    #[allow(clippy::too_many_arguments)]
    pub fn show_message_box(
        &self,
        message: impl Into<String>,
        main_instruction: impl Into<String>,
        un_flags: i32,
        image: IDispatch,
        help_guid: impl Into<String>,
        warning_name: impl Into<String>,
        e_flags: i32,
        data_context: IDispatch,
    ) -> Option<i64> {
        let args = vec![
            Variant::from_str(message.into()),
            Variant::from_str(main_instruction.into()),
            Variant::from_i64(un_flags as i64),
            Variant::from_dispatch(&image),
            Variant::from_str(help_guid.into()),
            Variant::from_str(warning_name.into()),
            Variant::from_i64(e_flags as i64),
            Variant::from_dispatch(&data_context),
        ];
        self.disp
            .invoke_method("ShowMessageBox", args)
            .ok()?
            .to_i64()
            .ok()
    }

    // ---------------------------------------------------------
    // 创建 FrameWindow
    // ---------------------------------------------------------

    /// 创建文本框窗口视图宿主（COM 方法 `CreateFrameWindowForViewHost`）。
    pub fn create_frame_window_for_view_host(
        &self,
        host: IDispatch,
    ) -> Option<ICuiFrameWindow> {
        let args = vec![Variant::from_dispatch(&host)];
        self.disp
            .invoke_method("CreateFrameWindowForViewHost", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiFrameWindow::new)
    }

    /// 创建文本框窗口视图（COM 方法 `CreateFrameWindowForView`）。
    pub fn create_frame_window_for_view(
        &self,
        view: IDispatch,
    ) -> Option<ICuiFrameWindow> {
        let args = vec![Variant::from_dispatch(&view)];
        self.disp
            .invoke_method("CreateFrameWindowForView", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiFrameWindow::new)
    }
}
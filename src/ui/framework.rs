//! `ICUIFrameWork` 鈥斺€?UI 妗嗘灦

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{
    ICuiApplication, ICuiCommandBar, ICuiCommandBars, ICuiFrameWindow,
    ICuiFrameWindows, ICuiTaskManager,
};

pub struct ICuiFrameWork {
    disp: ComObject,
}

impl ICuiFrameWork {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

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
    // 鍚嶇О / 搴旂敤
    // ---------------------------------------------------------

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    pub fn application(&self) -> Option<ICuiApplication> {
        self.prop_dispatch("Application").map(ICuiApplication::new)
    }

    // ---------------------------------------------------------
    // 宸ュ叿鏉?/ 鑿滃崟
    // ---------------------------------------------------------

    pub fn command_bars(&self) -> Option<ICuiCommandBars> {
        self.prop_dispatch("CommandBars").map(ICuiCommandBars::new)
    }

    pub fn main_menu(&self) -> Option<ICuiCommandBar> {
        self.prop_dispatch("MainMenu").map(ICuiCommandBar::new)
    }

    pub fn status_bar(&self) -> Option<ICuiCommandBar> {
        self.prop_dispatch("StatusBar").map(ICuiCommandBar::new)
    }

    // ---------------------------------------------------------
    // 绐楀彛
    // ---------------------------------------------------------

    pub fn frame_windows(&self) -> Option<ICuiFrameWindows> {
        self.prop_dispatch("FrameWindows").map(ICuiFrameWindows::new)
    }

    pub fn main_frame_window(&self) -> Option<ICuiFrameWindow> {
        self.prop_dispatch("MainFrameWindow").map(ICuiFrameWindow::new)
    }

    // ---------------------------------------------------------
    // 浠诲姟
    // ---------------------------------------------------------

    pub fn task_manager(&self) -> Option<ICuiTaskManager> {
        self.prop_dispatch("TaskManager").map(ICuiTaskManager::new)
    }

    // ---------------------------------------------------------
    // 宸ヤ綔鍖?
    // ---------------------------------------------------------

    pub fn import_workspace(&self, file_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp.invoke_method("ImportWorkspace", args).is_ok()
    }

    // ---------------------------------------------------------
    // Docker锛堝仠闈犻潰鏉匡級
    // ---------------------------------------------------------

    pub fn show_docker(&self, guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(guid.into())];
        self.disp.invoke_method("ShowDocker", args).is_ok()
    }

    pub fn hide_docker(&self, guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(guid.into())];
        self.disp.invoke_method("HideDocker", args).is_ok()
    }

    pub fn is_docker_visible(&self, guid: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(guid.into())];
        self.disp
            .invoke_method("IsDockerVisible", args)
            .ok()?
            .to_bool()
            .ok()
    }

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

    pub fn remove_docker(&self, guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(guid.into())];
        self.disp.invoke_method("RemoveDocker", args).is_ok()
    }

    // ---------------------------------------------------------
    // 瀵硅瘽妗?
    // ---------------------------------------------------------

    pub fn show_dialog(&self, guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(guid.into())];
        self.disp.invoke_method("ShowDialog", args).is_ok()
    }

    pub fn hide_dialog(&self, guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(guid.into())];
        self.disp.invoke_method("HideDialog", args).is_ok()
    }

    // ---------------------------------------------------------
    // 娑堟伅妗?
    // ---------------------------------------------------------

    /// `cuiMessageBoxFlags` 瑙?`enums::ui`銆?
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
    // 鍒涘缓 FrameWindow
    // ---------------------------------------------------------

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
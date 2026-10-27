//! `ICUIFrameWindow(s)` —— 框架窗口

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{ICuiDockHost, ICuiDockHosts, ICuiScreenRect, ICuiViewHost, ICuiViewHosts};

// =============================================================
// ICuiFrameWindows
// =============================================================

/// CorelDRAW 对象的后期绑定包装（文本框窗口集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiFrameWindows {
    disp: ComObject,
}

impl ICuiFrameWindows {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<ICuiFrameWindow> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiFrameWindow::new)
    }

    /// 查找（COM 方法 `Find`）。
    pub fn find(&self, id: impl Into<String>) -> Option<ICuiFrameWindow> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiFrameWindow::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<ICuiFrameWindow> {
        self.prop_dispatch("First").map(ICuiFrameWindow::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<ICuiFrameWindow> {
        self.prop_dispatch("Last").map(ICuiFrameWindow::new)
    }
}

// =============================================================
// ICuiFrameWindow
// =============================================================

/// CorelDRAW 对象的后期绑定包装（文本框窗口）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiFrameWindow {
    disp: ComObject,
}

impl ICuiFrameWindow {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---- 标识 ----

    /// 读取标识（`ID`）。
    pub fn id(&self) -> Option<String> { self.prop_string("ID") }
    /// 读取标题（`Caption`）。
    pub fn caption(&self) -> Option<String> { self.prop_string("Caption") }

    // ---- 状态----

    /// `cuiWindowState`
    pub fn state(&self) -> Option<i64> { self.prop_i64("State") }
    /// 是否主文本框（`IsMainFrame`）。
    pub fn is_main_frame(&self) -> Option<bool> { self.prop_bool("IsMainFrame") }
    /// 是否活动（`IsActive`）。
    pub fn is_active(&self) -> Option<bool> { self.prop_bool("IsActive") }

    /// 内部窗口句柄（`HWND`）。
    pub fn handle(&self) -> Option<i64> { self.prop_i64("Handle") }

    /// 读取位置（`Position`）。
    pub fn position(&self) -> Option<ICuiScreenRect> {
        self.prop_dispatch("Position").map(ICuiScreenRect::new)
    }

    // ---- 宿主 ----

    /// 读取 COM 属性 `ViewHosts`。
    pub fn view_hosts(&self) -> Option<ICuiViewHosts> {
        self.prop_dispatch("ViewHosts").map(ICuiViewHosts::new)
    }

    /// 读取 COM 属性 `DockHosts`。
    pub fn dock_hosts(&self) -> Option<ICuiDockHosts> {
        self.prop_dispatch("DockHosts").map(ICuiDockHosts::new)
    }

    /// 读取 COM 属性 `RootDockHost`。
    pub fn root_dock_host(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("RootDockHost").map(ICuiDockHost::new)
    }

    /// 单个视图宿主（若有）。
    pub fn single_view_host(&self) -> Option<ICuiViewHost> {
        let hs = self.view_hosts()?;
        hs.item(1)
    }

    // ---- 操作 ----

    /// 调用 COM 方法 `Minimize`。
    pub fn minimize(&self) -> bool {
        self.disp.invoke_method("Minimize", vec![]).is_ok()
    }

    /// 调用 COM 方法 `Maximize`。
    pub fn maximize(&self) -> bool {
        self.disp.invoke_method("Maximize", vec![]).is_ok()
    }

    /// restore（COM 方法 `Restore`）。
    pub fn restore(&self) -> bool {
        self.disp.invoke_method("Restore", vec![]).is_ok()
    }

    /// 关闭（COM 方法 `Close`）。
    pub fn close(&self) -> bool {
        self.disp.invoke_method("Close", vec![]).is_ok()
    }

    /// activate（COM 方法 `Activate`）。
    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    /// 平铺视图集（COM 方法 `TileViews`）。
    pub fn tile_views(&self, horizontally: bool) -> bool {
        let args = vec![Variant::from_bool(horizontally)];
        self.disp.invoke_method("TileViews", args).is_ok()
    }

    /// 合并视图集（COM 方法 `CombineViews`）。
    pub fn combine_views(&self) -> bool {
        self.disp.invoke_method("CombineViews", vec![]).is_ok()
    }
}
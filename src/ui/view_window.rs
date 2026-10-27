//! `ICUIViewWindow(s)` —— 视图窗口

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{ICuiScreenRect, ICuiViewHost};

// =============================================================
// ICuiViewWindows
// =============================================================

/// CorelDRAW 对象的后期绑定包装（视图窗口集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiViewWindows {
    disp: ComObject,
}

impl ICuiViewWindows {
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
    pub fn item(&self, index: i32) -> Option<ICuiViewWindow> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiViewWindow::new)
    }

    /// 查找（COM 方法 `Find`）。
    pub fn find(&self, id: impl Into<String>) -> Option<ICuiViewWindow> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiViewWindow::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<ICuiViewWindow> {
        self.prop_dispatch("First").map(ICuiViewWindow::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<ICuiViewWindow> {
        self.prop_dispatch("Last").map(ICuiViewWindow::new)
    }
}

// =============================================================
// ICuiViewWindow
// =============================================================

/// CorelDRAW 对象的后期绑定包装（视图窗口）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiViewWindow {
    disp: ComObject,
}

impl ICuiViewWindow {
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---- 标识 ----

    /// 读取标识（`ID`）。
    pub fn id(&self) -> Option<String> { self.prop_string("ID") }
    /// 读取 COM 属性 `Kind`。
    pub fn kind(&self) -> Option<String> { self.prop_string("Kind") }
    /// 读取标题（`Title`）。
    pub fn title(&self) -> Option<String> { self.prop_string("Title") }
    /// 读取描述（`Description`）。
    pub fn description(&self) -> Option<String> { self.prop_string("Description") }
    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 读取视图宿主（`ViewHost`）。
    pub fn view_host(&self) -> Option<ICuiViewHost> {
        self.prop_dispatch("ViewHost").map(ICuiViewHost::new)
    }

    /// 读取位置（`Position`）。
    pub fn position(&self) -> Option<ICuiScreenRect> {
        self.prop_dispatch("Position").map(ICuiScreenRect::new)
    }

    /// 该视图关联的 CorelDRAW 文档视图（`IVGActiveView`）。
    pub fn app_view(&self) -> Option<IDispatch> {
        self.prop_dispatch("AppView")
    }

    // ---- 操作 ----

    /// activate（COM 方法 `Activate`）。
    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    /// 关闭（COM 方法 `Close`）。
    pub fn close(&self) -> bool {
        self.disp.invoke_method("Close", vec![]).is_ok()
    }
}
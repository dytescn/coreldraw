//! `ICUIViewHost(s)` —— 视图宿主

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{
    ICuiDockHost, ICuiDockItem, ICuiScreenRect, ICuiViewWindow, ICuiViewWindows,
};

// =============================================================
// ICuiViewHosts
// =============================================================

/// CorelDRAW 对象的后期绑定包装（视图宿主）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiViewHosts {
    disp: ComObject,
}

impl ICuiViewHosts {
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
    pub fn item(&self, index: i32) -> Option<ICuiViewHost> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiViewHost::new)
    }

    /// 查找（COM 方法 `Find`）。
    pub fn find(&self, id: impl Into<String>) -> Option<ICuiViewHost> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiViewHost::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<ICuiViewHost> {
        self.prop_dispatch("First").map(ICuiViewHost::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<ICuiViewHost> {
        self.prop_dispatch("Last").map(ICuiViewHost::new)
    }
}

// =============================================================
// ICuiViewHost
// =============================================================

/// CorelDRAW 对象的后期绑定包装（视图宿主）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiViewHost {
    disp: ComObject,
}

impl ICuiViewHost {
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取标识（`ID`）。
    pub fn id(&self) -> Option<String> { self.prop_string("ID") }
    /// 读取停靠宿主（`DockHost`）。
    pub fn dock_host(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("DockHost").map(ICuiDockHost::new)
    }
    /// 读取视图集（`Views`）。
    pub fn views(&self) -> Option<ICuiViewWindows> {
        self.prop_dispatch("Views").map(ICuiViewWindows::new)
    }
    /// 读取停靠条目（`DockItem`）。
    pub fn dock_item(&self) -> Option<ICuiDockItem> {
        self.prop_dispatch("DockItem").map(ICuiDockItem::new)
    }
    /// 读取位置（`Position`）。
    pub fn position(&self) -> Option<ICuiScreenRect> {
        self.prop_dispatch("Position").map(ICuiScreenRect::new)
    }

    /// 在 index 位置插入一个视图。
    pub fn insert_view(&self, view: &ICuiViewWindow, index: i32) -> bool {
        let args = vec![
            view.as_variant(),
            Variant::from_i64(index as i64),
        ];
        self.disp.invoke_method("InsertView", args).is_ok()
    }

    /// 在 index 位置插入一个视图宿主。
    pub fn insert_view_host(&self, host: &ICuiViewHost, index: i32) -> bool {
        let args = vec![
            host.as_variant(),
            Variant::from_i64(index as i64),
        ];
        self.disp.invoke_method("InsertViewHost", args).is_ok()
    }
}
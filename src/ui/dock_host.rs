//! `ICUIDockHost(s)` —— 停靠宿主

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{
    ICuiDockItem, ICuiDockItems, ICuiScreenRect,
    ICuiViewHost, ICuiViewWindow,
};

// =============================================================
// ICuiDockHosts
// =============================================================

/// CorelDRAW 对象的后期绑定包装（停靠宿主）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiDockHosts {
    disp: ComObject,
}

impl ICuiDockHosts {
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
    pub fn item(&self, index: i32) -> Option<ICuiDockHost> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiDockHost::new)
    }

    /// 查找（COM 方法 `Find`）。
    pub fn find(&self, id: impl Into<String>) -> Option<ICuiDockHost> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiDockHost::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("First").map(ICuiDockHost::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("Last").map(ICuiDockHost::new)
    }
}

// =============================================================
// ICuiDockHost
// =============================================================

/// CorelDRAW 对象的后期绑定包装（停靠宿主）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiDockHost {
    disp: ComObject,
}

impl ICuiDockHost {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    /// 读取标识（`ID`）。
    pub fn id(&self) -> Option<String> { self.prop_string("ID") }

    /// `cuiDockHostOrientation`
    pub fn orientation(&self) -> Option<i64> { self.prop_i64("Orientation") }

    /// 读取父对象停靠宿主（`ParentDockHost`）。
    pub fn parent_dock_host(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("ParentDockHost").map(ICuiDockHost::new)
    }

    /// 读取停靠条目（`DockItem`）。
    pub fn dock_item(&self) -> Option<ICuiDockItem> {
        self.prop_dispatch("DockItem").map(ICuiDockItem::new)
    }

    /// 读取子对象集（`Children`）。
    pub fn children(&self) -> Option<ICuiDockItems> {
        self.prop_dispatch("Children").map(ICuiDockItems::new)
    }

    /// 读取位置（`Position`）。
    pub fn position(&self) -> Option<ICuiScreenRect> {
        self.prop_dispatch("Position").map(ICuiScreenRect::new)
    }

    /// `cuiDockOperation` ——停靠操作。
    pub fn insert_view_host(
        &self,
        host: &ICuiViewHost,
        index: i32,
        operation: i32,
    ) -> bool {
        let args = vec![
            host.as_variant(),
            Variant::from_i64(index as i64),
            Variant::from_i64(operation as i64),
        ];
        self.disp.invoke_method("InsertViewHost", args).is_ok()
    }

    /// 插入视图（COM 方法 `InsertView`）。
    pub fn insert_view(
        &self,
        view: &ICuiViewWindow,
        index: i32,
        operation: i32,
    ) -> Option<ICuiViewHost> {
        let args = vec![
            view.as_variant(),
            Variant::from_i64(index as i64),
            Variant::from_i64(operation as i64),
        ];
        self.disp
            .invoke_method("InsertView", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiViewHost::new)
    }
}
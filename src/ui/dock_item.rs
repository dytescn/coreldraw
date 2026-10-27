//! `ICUIDockItem(s)` —— 停靠项

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{ICuiDockHost, ICuiScreenRect, ICuiViewHost};

// =============================================================
// ICuiDockItems
// =============================================================

/// CorelDRAW 对象的后期绑定包装（停靠条目集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiDockItems {
    disp: ComObject,
}

impl ICuiDockItems {
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
    pub fn item(&self, index: i32) -> Option<ICuiDockItem> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiDockItem::new)
    }

    /// 查找（COM 方法 `Find`）。
    pub fn find(&self, id: impl Into<String>) -> Option<ICuiDockItem> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiDockItem::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<ICuiDockItem> {
        self.prop_dispatch("First").map(ICuiDockItem::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<ICuiDockItem> {
        self.prop_dispatch("Last").map(ICuiDockItem::new)
    }
}

// =============================================================
// ICuiDockItem
// =============================================================

/// CorelDRAW 对象的后期绑定包装（停靠条目）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiDockItem {
    disp: ComObject,
}

impl ICuiDockItem {
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

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取标识（`ID`）。
    pub fn id(&self) -> Option<String> { self.prop_string("ID") }

    /// `cuiDockItemType`
    pub fn item_type(&self) -> Option<i64> { self.prop_i64("Type") }

    /// 读取视图宿主（`ViewHost`）。
    pub fn view_host(&self) -> Option<ICuiViewHost> {
        self.prop_dispatch("ViewHost").map(ICuiViewHost::new)
    }

    /// 读取停靠宿主（`DockHost`）。
    pub fn dock_host(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("DockHost").map(ICuiDockHost::new)
    }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 读取相对尺寸（`RelativeSize`）。
    pub fn relative_size(&self) -> Option<i64> { self.prop_i64("RelativeSize") }
    /// 设置相对尺寸（`RelativeSize`）。
    pub fn set_relative_size(&self, v: i32) -> bool { self.put_i64("RelativeSize", v as i64) }

    /// 读取位置（`Position`）。
    pub fn position(&self) -> Option<ICuiScreenRect> {
        self.prop_dispatch("Position").map(ICuiScreenRect::new)
    }
}
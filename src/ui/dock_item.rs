//! `ICUIDockItem(s)` 鈥斺€?鍋滈潬椤?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{ICuiDockHost, ICuiScreenRect, ICuiViewHost};

// =============================================================
// ICuiDockItems
// =============================================================

pub struct ICuiDockItems {
    disp: ComObject,
}

impl ICuiDockItems {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<ICuiDockItem> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiDockItem::new)
    }

    pub fn find(&self, id: impl Into<String>) -> Option<ICuiDockItem> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiDockItem::new)
    }

    pub fn first(&self) -> Option<ICuiDockItem> {
        self.prop_dispatch("First").map(ICuiDockItem::new)
    }

    pub fn last(&self) -> Option<ICuiDockItem> {
        self.prop_dispatch("Last").map(ICuiDockItem::new)
    }
}

// =============================================================
// ICuiDockItem
// =============================================================

pub struct ICuiDockItem {
    disp: ComObject,
}

impl ICuiDockItem {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

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

    pub fn id(&self) -> Option<String> { self.prop_string("ID") }

    /// `cuiDockItemType`
    pub fn item_type(&self) -> Option<i64> { self.prop_i64("Type") }

    pub fn view_host(&self) -> Option<ICuiViewHost> {
        self.prop_dispatch("ViewHost").map(ICuiViewHost::new)
    }

    pub fn dock_host(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("DockHost").map(ICuiDockHost::new)
    }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    pub fn relative_size(&self) -> Option<i64> { self.prop_i64("RelativeSize") }
    pub fn set_relative_size(&self, v: i32) -> bool { self.put_i64("RelativeSize", v as i64) }

    pub fn position(&self) -> Option<ICuiScreenRect> {
        self.prop_dispatch("Position").map(ICuiScreenRect::new)
    }
}
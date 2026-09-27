//! `ICUIDockHost(s)` 鈥斺€?鍋滈潬瀹夸富

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{
    ICuiDockItem, ICuiDockItems, ICuiScreenRect,
    ICuiViewHost, ICuiViewWindow,
};

// =============================================================
// ICuiDockHosts
// =============================================================

pub struct ICuiDockHosts {
    disp: ComObject,
}

impl ICuiDockHosts {
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

    pub fn item(&self, index: i32) -> Option<ICuiDockHost> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiDockHost::new)
    }

    pub fn find(&self, id: impl Into<String>) -> Option<ICuiDockHost> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiDockHost::new)
    }

    pub fn first(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("First").map(ICuiDockHost::new)
    }

    pub fn last(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("Last").map(ICuiDockHost::new)
    }
}

// =============================================================
// ICuiDockHost
// =============================================================

pub struct ICuiDockHost {
    disp: ComObject,
}

impl ICuiDockHost {
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

    pub fn id(&self) -> Option<String> { self.prop_string("ID") }

    /// `cuiDockHostOrientation`
    pub fn orientation(&self) -> Option<i64> { self.prop_i64("Orientation") }

    pub fn parent_dock_host(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("ParentDockHost").map(ICuiDockHost::new)
    }

    pub fn dock_item(&self) -> Option<ICuiDockItem> {
        self.prop_dispatch("DockItem").map(ICuiDockItem::new)
    }

    pub fn children(&self) -> Option<ICuiDockItems> {
        self.prop_dispatch("Children").map(ICuiDockItems::new)
    }

    pub fn position(&self) -> Option<ICuiScreenRect> {
        self.prop_dispatch("Position").map(ICuiScreenRect::new)
    }

    /// `cuiDockOperation` 鈥斺€?鍋滈潬鎿嶄綔銆?
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
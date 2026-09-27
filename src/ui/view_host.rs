//! `ICUIViewHost(s)` 鈥斺€?瑙嗗浘瀹夸富

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{
    ICuiDockHost, ICuiDockItem, ICuiScreenRect, ICuiViewWindow, ICuiViewWindows,
};

// =============================================================
// ICuiViewHosts
// =============================================================

pub struct ICuiViewHosts {
    disp: ComObject,
}

impl ICuiViewHosts {
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

    pub fn item(&self, index: i32) -> Option<ICuiViewHost> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiViewHost::new)
    }

    pub fn find(&self, id: impl Into<String>) -> Option<ICuiViewHost> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiViewHost::new)
    }

    pub fn first(&self) -> Option<ICuiViewHost> {
        self.prop_dispatch("First").map(ICuiViewHost::new)
    }

    pub fn last(&self) -> Option<ICuiViewHost> {
        self.prop_dispatch("Last").map(ICuiViewHost::new)
    }
}

// =============================================================
// ICuiViewHost
// =============================================================

pub struct ICuiViewHost {
    disp: ComObject,
}

impl ICuiViewHost {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn id(&self) -> Option<String> { self.prop_string("ID") }
    pub fn dock_host(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("DockHost").map(ICuiDockHost::new)
    }
    pub fn views(&self) -> Option<ICuiViewWindows> {
        self.prop_dispatch("Views").map(ICuiViewWindows::new)
    }
    pub fn dock_item(&self) -> Option<ICuiDockItem> {
        self.prop_dispatch("DockItem").map(ICuiDockItem::new)
    }
    pub fn position(&self) -> Option<ICuiScreenRect> {
        self.prop_dispatch("Position").map(ICuiScreenRect::new)
    }

    /// 鍦?index 浣嶇疆鎻掑叆涓€涓鍥俱€?
    pub fn insert_view(&self, view: &ICuiViewWindow, index: i32) -> bool {
        let args = vec![
            view.as_variant(),
            Variant::from_i64(index as i64),
        ];
        self.disp.invoke_method("InsertView", args).is_ok()
    }

    /// 鍦?index 浣嶇疆鎻掑叆涓€涓鍥惧涓汇€?
    pub fn insert_view_host(&self, host: &ICuiViewHost, index: i32) -> bool {
        let args = vec![
            host.as_variant(),
            Variant::from_i64(index as i64),
        ];
        self.disp.invoke_method("InsertViewHost", args).is_ok()
    }
}
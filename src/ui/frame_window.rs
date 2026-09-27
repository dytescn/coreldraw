//! `ICUIFrameWindow(s)` 鈥斺€?妗嗘灦绐楀彛

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{ICuiDockHost, ICuiDockHosts, ICuiScreenRect, ICuiViewHost, ICuiViewHosts};

// =============================================================
// ICuiFrameWindows
// =============================================================

pub struct ICuiFrameWindows {
    disp: ComObject,
}

impl ICuiFrameWindows {
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

    pub fn item(&self, index: i32) -> Option<ICuiFrameWindow> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiFrameWindow::new)
    }

    pub fn find(&self, id: impl Into<String>) -> Option<ICuiFrameWindow> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiFrameWindow::new)
    }

    pub fn first(&self) -> Option<ICuiFrameWindow> {
        self.prop_dispatch("First").map(ICuiFrameWindow::new)
    }

    pub fn last(&self) -> Option<ICuiFrameWindow> {
        self.prop_dispatch("Last").map(ICuiFrameWindow::new)
    }
}

// =============================================================
// ICuiFrameWindow
// =============================================================

pub struct ICuiFrameWindow {
    disp: ComObject,
}

impl ICuiFrameWindow {
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

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---- 鏍囪瘑 ----

    pub fn id(&self) -> Option<String> { self.prop_string("ID") }
    pub fn caption(&self) -> Option<String> { self.prop_string("Caption") }

    // ---- 鐘舵€?----

    /// `cuiWindowState`
    pub fn state(&self) -> Option<i64> { self.prop_i64("State") }
    pub fn is_main_frame(&self) -> Option<bool> { self.prop_bool("IsMainFrame") }
    pub fn is_active(&self) -> Option<bool> { self.prop_bool("IsActive") }

    /// 鍐呴儴绐楀彛鍙ユ焺锛坄HWND`锛夈€?
    pub fn handle(&self) -> Option<i64> { self.prop_i64("Handle") }

    pub fn position(&self) -> Option<ICuiScreenRect> {
        self.prop_dispatch("Position").map(ICuiScreenRect::new)
    }

    // ---- 瀹夸富 ----

    pub fn view_hosts(&self) -> Option<ICuiViewHosts> {
        self.prop_dispatch("ViewHosts").map(ICuiViewHosts::new)
    }

    pub fn dock_hosts(&self) -> Option<ICuiDockHosts> {
        self.prop_dispatch("DockHosts").map(ICuiDockHosts::new)
    }

    pub fn root_dock_host(&self) -> Option<ICuiDockHost> {
        self.prop_dispatch("RootDockHost").map(ICuiDockHost::new)
    }

    /// 鍗曚釜瑙嗗浘瀹夸富锛堣嫢鏈夛級銆?
    pub fn single_view_host(&self) -> Option<ICuiViewHost> {
        let hs = self.view_hosts()?;
        hs.item(1)
    }

    // ---- 鎿嶄綔 ----

    pub fn minimize(&self) -> bool {
        self.disp.invoke_method("Minimize", vec![]).is_ok()
    }

    pub fn maximize(&self) -> bool {
        self.disp.invoke_method("Maximize", vec![]).is_ok()
    }

    pub fn restore(&self) -> bool {
        self.disp.invoke_method("Restore", vec![]).is_ok()
    }

    pub fn close(&self) -> bool {
        self.disp.invoke_method("Close", vec![]).is_ok()
    }

    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    pub fn tile_views(&self, horizontally: bool) -> bool {
        let args = vec![Variant::from_bool(horizontally)];
        self.disp.invoke_method("TileViews", args).is_ok()
    }

    pub fn combine_views(&self) -> bool {
        self.disp.invoke_method("CombineViews", vec![]).is_ok()
    }
}
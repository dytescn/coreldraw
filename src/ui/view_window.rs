//! `ICUIViewWindow(s)` 鈥斺€?瑙嗗浘绐楀彛

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{ICuiScreenRect, ICuiViewHost};

// =============================================================
// ICuiViewWindows
// =============================================================

pub struct ICuiViewWindows {
    disp: ComObject,
}

impl ICuiViewWindows {
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

    pub fn item(&self, index: i32) -> Option<ICuiViewWindow> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiViewWindow::new)
    }

    pub fn find(&self, id: impl Into<String>) -> Option<ICuiViewWindow> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiViewWindow::new)
    }

    pub fn first(&self) -> Option<ICuiViewWindow> {
        self.prop_dispatch("First").map(ICuiViewWindow::new)
    }

    pub fn last(&self) -> Option<ICuiViewWindow> {
        self.prop_dispatch("Last").map(ICuiViewWindow::new)
    }
}

// =============================================================
// ICuiViewWindow
// =============================================================

pub struct ICuiViewWindow {
    disp: ComObject,
}

impl ICuiViewWindow {
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

    // ---- 鏍囪瘑 ----

    pub fn id(&self) -> Option<String> { self.prop_string("ID") }
    pub fn kind(&self) -> Option<String> { self.prop_string("Kind") }
    pub fn title(&self) -> Option<String> { self.prop_string("Title") }
    pub fn description(&self) -> Option<String> { self.prop_string("Description") }
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    pub fn view_host(&self) -> Option<ICuiViewHost> {
        self.prop_dispatch("ViewHost").map(ICuiViewHost::new)
    }

    pub fn position(&self) -> Option<ICuiScreenRect> {
        self.prop_dispatch("Position").map(ICuiScreenRect::new)
    }

    /// 璇ヨ鍥惧叧鑱旂殑 CorelDRAW 鏂囨。瑙嗗浘锛坄IVGActiveView`锛夈€?
    pub fn app_view(&self) -> Option<IDispatch> {
        self.prop_dispatch("AppView")
    }

    // ---- 鎿嶄綔 ----

    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    pub fn close(&self) -> bool {
        self.disp.invoke_method("Close", vec![]).is_ok()
    }
}
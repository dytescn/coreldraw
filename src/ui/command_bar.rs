//! `ICUICommandBar(s)` / `ICUICommandBarMode(s)` 鈥斺€?宸ュ叿鏉?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::ICuiControls;

// =============================================================
// ICuiCommandBars
// =============================================================

pub struct ICuiCommandBars {
    disp: ComObject,
}

impl ICuiCommandBars {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// `IndexOrName` 鏄?`VARIANT`锛堝彲涓虹储寮曟垨鍚嶇О锛夈€?
    pub fn item(&self, index_or_name: Variant) -> Option<ICuiCommandBar> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiCommandBar::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<ICuiCommandBar> {
        self.item(Variant::from_i64(i as i64))
    }

    pub fn item_by_name(&self, name: impl Into<String>) -> Option<ICuiCommandBar> {
        self.item(Variant::from_str(name.into()))
    }

    /// `cuiBarPosition` 瑙?`enums::ui`銆?
    pub fn add(
        &self,
        name: impl Into<String>,
        position: i32,
        temporary: bool,
    ) -> Option<ICuiCommandBar> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(position as i64),
            Variant::from_bool(temporary),
        ];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiCommandBar::new)
    }
}

// =============================================================
// ICuiCommandBar
// =============================================================

pub struct ICuiCommandBar {
    disp: ComObject,
}

impl ICuiCommandBar {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 绫诲瀷 / 鐘舵€?----

    /// `cuiBarType`
    pub fn bar_type(&self) -> Option<i64> { self.prop_i64("Type") }

    pub fn visible(&self) -> Option<bool> { self.prop_bool("Visible") }
    pub fn set_visible(&self, v: bool) -> bool { self.put_bool("Visible", v) }

    pub fn enabled(&self) -> Option<bool> { self.prop_bool("Enabled") }
    pub fn set_enabled(&self, v: bool) -> bool { self.put_bool("Enabled", v) }

    pub fn built_in(&self) -> Option<bool> { self.prop_bool("BuiltIn") }

    // ---- 浣嶇疆 / 灏哄 ----

    pub fn left(&self) -> Option<i64> { self.prop_i64("Left") }
    pub fn set_left(&self, v: i32) -> bool { self.put_i64("Left", v as i64) }

    pub fn top(&self) -> Option<i64> { self.prop_i64("Top") }
    pub fn set_top(&self, v: i32) -> bool { self.put_i64("Top", v as i64) }

    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }

    pub fn set_width(&self, v: i32) -> bool {
        let args = vec![Variant::from_i64(v as i64)];
        self.disp.invoke_method("SetWidth", args).is_ok()
    }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    // ---- 鍚嶇О ----

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    pub fn name_local(&self) -> Option<String> { self.prop_string("NameLocal") }
    pub fn set_name_local(&self, v: impl Into<String>) -> bool {
        self.put_string("NameLocal", v)
    }

    // ---- 浣嶇疆 / 淇濇姢 ----

    /// `cuiBarPosition`
    pub fn position(&self) -> Option<i64> { self.prop_i64("Position") }
    pub fn set_position(&self, v: i32) -> bool { self.put_i64("Position", v as i64) }

    /// `cuiBarProtection`
    pub fn protection(&self) -> Option<i64> { self.prop_i64("Protection") }
    pub fn set_protection(&self, v: i32) -> bool { self.put_i64("Protection", v as i64) }

    // ---- 鎺т欢 / 妯″紡 ----

    pub fn controls(&self) -> Option<ICuiControls> {
        self.prop_dispatch("Controls").map(ICuiControls::new)
    }

    pub fn modes(&self) -> Option<ICuiCommandBarModes> {
        self.prop_dispatch("Modes").map(ICuiCommandBarModes::new)
    }

    // ---- 鎿嶄綔 ----

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    pub fn show_popup(&self, x: Variant, y: Variant) -> bool {
        let args = vec![x, y];
        self.disp.invoke_method("ShowPopup", args).is_ok()
    }
}

// =============================================================
// ICuiCommandBarModes
// =============================================================

pub struct ICuiCommandBarModes {
    disp: ComObject,
}

impl ICuiCommandBarModes {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index_or_name: Variant) -> Option<ICuiCommandBarMode> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiCommandBarMode::new)
    }
}

// =============================================================
// ICuiCommandBarMode
// =============================================================

pub struct ICuiCommandBarMode {
    disp: ComObject,
}

impl ICuiCommandBarMode {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn name_local(&self) -> Option<String> { self.prop_string("NameLocal") }
    pub fn controls(&self) -> Option<ICuiControls> {
        self.prop_dispatch("Controls").map(ICuiControls::new)
    }
}
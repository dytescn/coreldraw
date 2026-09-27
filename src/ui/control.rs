//! `ICUIControl(s)` 鈥斺€?宸ュ叿鏉′笂鐨勬帶浠?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// ICuiControls
// =============================================================

pub struct ICuiControls {
    disp: ComObject,
}

impl ICuiControls {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<ICuiControl> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiControl::new)
    }

    /// 娣诲姞鎺т欢銆?
    pub fn add(
        &self,
        control_id: impl Into<String>,
        index: i32,
        temporary: bool,
    ) -> Option<ICuiControl> {
        let args = vec![
            Variant::from_str(control_id.into()),
            Variant::from_i64(index as i64),
            Variant::from_bool(temporary),
        ];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiControl::new)
    }

    /// 娣诲姞鑷畾涔夋寜閽€?
    pub fn add_custom_button(
        &self,
        category_id: impl Into<String>,
        command: impl Into<String>,
        index: i32,
        temporary: bool,
    ) -> Option<ICuiControl> {
        let args = vec![
            Variant::from_str(category_id.into()),
            Variant::from_str(command.into()),
            Variant::from_i64(index as i64),
            Variant::from_bool(temporary),
        ];
        self.disp
            .invoke_method("AddCustomButton", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiControl::new)
    }

    /// 娣诲姞鑷畾涔夋帶浠躲€?
    pub fn add_custom_control(
        &self,
        class_name: impl Into<String>,
        assembly_path: impl Into<String>,
        index: i32,
        temporary: bool,
    ) -> Option<ICuiControl> {
        let args = vec![
            Variant::from_str(class_name.into()),
            Variant::from_str(assembly_path.into()),
            Variant::from_i64(index as i64),
            Variant::from_bool(temporary),
        ];
        self.disp
            .invoke_method("AddCustomControl", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiControl::new)
    }

    /// 娣诲姞 toggle 鎸夐挳锛堝甫 GUID锛夈€?
    pub fn add_toggle_button(
        &self,
        guid: impl Into<String>,
        index: i32,
        temporary: bool,
    ) -> Option<ICuiControl> {
        let args = vec![
            Variant::from_str(guid.into()),
            Variant::from_i64(index as i64),
            Variant::from_bool(temporary),
        ];
        self.disp
            .invoke_method("AddToggleButton", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiControl::new)
    }

    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }
}

// =============================================================
// ICuiControl
// =============================================================

pub struct ICuiControl {
    disp: ComObject,
}

impl ICuiControl {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    fn put_variant(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---- 鍩烘湰灞炴€?----

    pub fn caption(&self) -> Option<String> { self.prop_string("Caption") }
    pub fn set_caption(&self, v: impl Into<String>) -> bool { self.put_string("Caption", v) }

    pub fn description_text(&self) -> Option<String> { self.prop_string("DescriptionText") }
    pub fn set_description_text(&self, v: impl Into<String>) -> bool {
        self.put_string("DescriptionText", v)
    }

    pub fn tooltip_text(&self) -> Option<String> { self.prop_string("ToolTipText") }
    pub fn set_tooltip_text(&self, v: impl Into<String>) -> bool {
        self.put_string("ToolTipText", v)
    }

    pub fn tag(&self) -> Option<String> { self.prop_string("Tag") }
    pub fn set_tag(&self, v: impl Into<String>) -> bool { self.put_string("Tag", v) }

    pub fn id(&self) -> Option<String> { self.prop_string("ID") }

    // ---- 灏哄 ----

    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    // ---- 鍙傛暟 / 鍙 ----

    /// 鑷畾涔夊弬鏁帮紙`VARIANT`锛夈€?
    pub fn parameter(&self) -> Option<Variant> {
        self.disp.get_property("Parameter").ok()
    }
    pub fn set_parameter(&self, v: Variant) -> bool {
        self.put_variant("Parameter", v)
    }

    pub fn visible(&self) -> Option<bool> { self.prop_bool("Visible") }
    pub fn set_visible(&self, v: bool) -> bool { self.put_bool("Visible", v) }

    // ---- 鍥炬爣 ----

    pub fn set_icon(&self, row_index: i32, column_index: i32) -> bool {
        let args = vec![
            Variant::from_i64(row_index as i64),
            Variant::from_i64(column_index as i64),
        ];
        self.disp.invoke_method("SetIcon", args).is_ok()
    }

    pub fn set_custom_icon(&self, image_file: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(image_file.into())];
        self.disp.invoke_method("SetCustomIcon", args).is_ok()
    }

    pub fn set_icon2(&self, icon: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(icon.into())];
        self.disp.invoke_method("SetIcon2", args).is_ok()
    }
}
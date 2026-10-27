//! `ICUIControl(s)` —— 工具条上的控件

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// ICuiControls
// =============================================================

/// CorelDRAW 对象的后期绑定包装（控件集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiControls {
    disp: ComObject,
}

impl ICuiControls {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<ICuiControl> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiControl::new)
    }

    /// 添加控件。
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

    /// 添加自定义按钮。
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

    /// 添加自定义控件。
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

    /// 添加 toggle 按钮（带 GUID）。
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

    /// 移除（COM 方法 `Remove`）。
    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }
}

// =============================================================
// ICuiControl
// =============================================================

/// CorelDRAW 对象的后期绑定包装（控件）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiControl {
    disp: ComObject,
}

impl ICuiControl {
    /// 由 COM `IDispatch` 构造包装对象。
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

    // ---- 基本属性----

    /// 读取标题（`Caption`）。
    pub fn caption(&self) -> Option<String> { self.prop_string("Caption") }
    /// 设置标题（`Caption`）。
    pub fn set_caption(&self, v: impl Into<String>) -> bool { self.put_string("Caption", v) }

    /// 读取描述文本（`DescriptionText`）。
    pub fn description_text(&self) -> Option<String> { self.prop_string("DescriptionText") }
    /// 设置描述文本（`DescriptionText`）。
    pub fn set_description_text(&self, v: impl Into<String>) -> bool {
        self.put_string("DescriptionText", v)
    }

    /// 读取 COM 属性 `ToolTipText`。
    pub fn tooltip_text(&self) -> Option<String> { self.prop_string("ToolTipText") }
    /// 设置 COM 属性 `ToolTipText`。
    pub fn set_tooltip_text(&self, v: impl Into<String>) -> bool {
        self.put_string("ToolTipText", v)
    }

    /// 读取 COM 属性 `Tag`。
    pub fn tag(&self) -> Option<String> { self.prop_string("Tag") }
    /// 设置 COM 属性 `Tag`。
    pub fn set_tag(&self, v: impl Into<String>) -> bool { self.put_string("Tag", v) }

    /// 读取标识（`ID`）。
    pub fn id(&self) -> Option<String> { self.prop_string("ID") }

    // ---- 尺寸 ----

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    /// 设置高度（`Height`）。
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    // ---- 参数 / 可见 ----

    /// 自定义参数（`VARIANT`）。
    pub fn parameter(&self) -> Option<Variant> {
        self.disp.get_property("Parameter").ok()
    }
    /// 设置 COM 属性 `Parameter`。
    pub fn set_parameter(&self, v: Variant) -> bool {
        self.put_variant("Parameter", v)
    }

    /// 读取可见性（`Visible`）。
    pub fn visible(&self) -> Option<bool> { self.prop_bool("Visible") }
    /// 设置可见性（`Visible`）。
    pub fn set_visible(&self, v: bool) -> bool { self.put_bool("Visible", v) }

    // ---- 图标 ----

    /// 设置图标（`SetIcon`）。
    pub fn set_icon(&self, row_index: i32, column_index: i32) -> bool {
        let args = vec![
            Variant::from_i64(row_index as i64),
            Variant::from_i64(column_index as i64),
        ];
        self.disp.invoke_method("SetIcon", args).is_ok()
    }

    /// 设置自定义图标（`SetCustomIcon`）。
    pub fn set_custom_icon(&self, image_file: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(image_file.into())];
        self.disp.invoke_method("SetCustomIcon", args).is_ok()
    }

    /// 设置 COM 属性 `SetIcon2`。
    pub fn set_icon2(&self, icon: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(icon.into())];
        self.disp.invoke_method("SetIcon2", args).is_ok()
    }
}
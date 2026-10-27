//! `ICUICommandBar(s)` / `ICUICommandBarMode(s)` —— 工具条

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::ICuiControls;

// =============================================================
// ICuiCommandBars
// =============================================================

/// CorelDRAW 对象的后期绑定包装（命令栏）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiCommandBars {
    disp: ComObject,
}

impl ICuiCommandBars {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// `IndexOrName` 是 `VARIANT`（可为索引或名称）。
    pub fn item(&self, index_or_name: Variant) -> Option<ICuiCommandBar> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiCommandBar::new)
    }

    /// 条目索引。
    pub fn item_by_index(&self, i: i32) -> Option<ICuiCommandBar> {
        self.item(Variant::from_i64(i as i64))
    }

    /// 条目名称。
    pub fn item_by_name(&self, name: impl Into<String>) -> Option<ICuiCommandBar> {
        self.item(Variant::from_str(name.into()))
    }

    /// `cuiBarPosition` 见 `enums::ui`。
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

/// CorelDRAW 对象的后期绑定包装（命令栏）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiCommandBar {
    disp: ComObject,
}

impl ICuiCommandBar {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
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

    // ---- 类型 / 状态----

    /// `cuiBarType`
    pub fn bar_type(&self) -> Option<i64> { self.prop_i64("Type") }

    /// 读取可见性（`Visible`）。
    pub fn visible(&self) -> Option<bool> { self.prop_bool("Visible") }
    /// 设置可见性（`Visible`）。
    pub fn set_visible(&self, v: bool) -> bool { self.put_bool("Visible", v) }

    /// 读取启用状态（`Enabled`）。
    pub fn enabled(&self) -> Option<bool> { self.prop_bool("Enabled") }
    /// 设置启用状态（`Enabled`）。
    pub fn set_enabled(&self, v: bool) -> bool { self.put_bool("Enabled", v) }

    /// 读取 COM 属性 `BuiltIn`。
    pub fn built_in(&self) -> Option<bool> { self.prop_bool("BuiltIn") }

    // ---- 位置 / 尺寸 ----

    /// 读取左（`Left`）。
    pub fn left(&self) -> Option<i64> { self.prop_i64("Left") }
    /// 设置左（`Left`）。
    pub fn set_left(&self, v: i32) -> bool { self.put_i64("Left", v as i64) }

    /// 读取顶（`Top`）。
    pub fn top(&self) -> Option<i64> { self.prop_i64("Top") }
    /// 设置顶（`Top`）。
    pub fn set_top(&self, v: i32) -> bool { self.put_i64("Top", v as i64) }

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }

    /// 设置宽度（`SetWidth`）。
    pub fn set_width(&self, v: i32) -> bool {
        let args = vec![Variant::from_i64(v as i64)];
        self.disp.invoke_method("SetWidth", args).is_ok()
    }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    // ---- 名称 ----

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    /// 读取名称本地（`NameLocal`）。
    pub fn name_local(&self) -> Option<String> { self.prop_string("NameLocal") }
    /// 设置名称本地（`NameLocal`）。
    pub fn set_name_local(&self, v: impl Into<String>) -> bool {
        self.put_string("NameLocal", v)
    }

    // ---- 位置 / 保护 ----

    /// `cuiBarPosition`
    pub fn position(&self) -> Option<i64> { self.prop_i64("Position") }
    /// 设置位置（`Position`）。
    pub fn set_position(&self, v: i32) -> bool { self.put_i64("Position", v as i64) }

    /// `cuiBarProtection`
    pub fn protection(&self) -> Option<i64> { self.prop_i64("Protection") }
    /// 设置 COM 属性 `Protection`。
    pub fn set_protection(&self, v: i32) -> bool { self.put_i64("Protection", v as i64) }

    // ---- 控件 / 模式 ----

    /// 读取控件集（`Controls`）。
    pub fn controls(&self) -> Option<ICuiControls> {
        self.prop_dispatch("Controls").map(ICuiControls::new)
    }

    /// 读取 COM 属性 `Modes`。
    pub fn modes(&self) -> Option<ICuiCommandBarModes> {
        self.prop_dispatch("Modes").map(ICuiCommandBarModes::new)
    }

    // ---- 操作 ----

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    /// 重置（COM 方法 `Reset`）。
    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    /// 调用 COM 方法 `ShowPopup`。
    pub fn show_popup(&self, x: Variant, y: Variant) -> bool {
        let args = vec![x, y];
        self.disp.invoke_method("ShowPopup", args).is_ok()
    }
}

// =============================================================
// ICuiCommandBarModes
// =============================================================

/// CorelDRAW 对象的后期绑定包装（命令栏模式）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiCommandBarModes {
    disp: ComObject,
}

impl ICuiCommandBarModes {
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

/// CorelDRAW 对象的后期绑定包装（命令栏模式）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiCommandBarMode {
    disp: ComObject,
}

impl ICuiCommandBarMode {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 读取名称本地（`NameLocal`）。
    pub fn name_local(&self) -> Option<String> { self.prop_string("NameLocal") }
    /// 读取控件集（`Controls`）。
    pub fn controls(&self) -> Option<ICuiControls> {
        self.prop_dispatch("Controls").map(ICuiControls::new)
    }
}
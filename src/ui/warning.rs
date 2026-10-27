//! `ICUIWarning` —— 警告对话框

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW 对象的后期绑定包装（警告）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiWarning {
    disp: ComObject,
}

impl ICuiWarning {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取启用状态（`Enabled`）。
    pub fn enabled(&self) -> Option<bool> { self.prop_bool("Enabled") }
    /// 设置启用状态（`Enabled`）。
    pub fn set_enabled(&self, v: bool) -> bool { self.put_bool("Enabled", v) }

    /// 读取标识（`ID`）。
    pub fn id(&self) -> Option<String> { self.prop_string("ID") }
    /// 读取描述（`Description`）。
    pub fn description(&self) -> Option<String> { self.prop_string("Description") }
    /// 读取文本（`Text`）。
    pub fn text(&self) -> Option<String> { self.prop_string("Text") }
    /// 读取标题（`Title`）。
    pub fn title(&self) -> Option<String> { self.prop_string("Title") }

    /// `unFlags` 是 `cuiMessageBoxFlags` 组合。
    pub fn do_warning_dialog(&self, flags: i32, text: impl Into<String>) -> Option<i64> {
        let args = vec![
            Variant::from_i64(flags as i64),
            Variant::from_str(text.into()),
        ];
        self.disp
            .invoke_method("DoWarningDialog", args)
            .ok()?
            .to_i64()
            .ok()
    }
}
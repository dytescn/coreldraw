//! `IVGClipboard` —— 剪贴板

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGClipboard` 对象的后期绑定包装（剪贴板）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgClipboard {
    disp: ComObject,
}

impl IvgClipboard {
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

    /// 读取 COM 属性 `Valid`。
    pub fn valid(&self) -> Option<bool> { self.prop_bool("Valid") }
    /// 是否空（`Empty`）。
    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("Empty") }

    /// 清空全部内容（COM 方法 `Clear`）。
    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    /// 数据存在（COM 方法 `DataPresent`）。
    pub fn data_present(&self, format_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(format_name.into())];
        self.disp
            .invoke_method("DataPresent", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
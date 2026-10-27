//! `IVGFontList` —— 已安装字体列表
use wincom::{ComObject, Variant};

/// CorelDRAW `IVGFontList` 对象的后期绑定包装（字体列表）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgFontList {
    disp: ComObject,
}

impl IvgFontList {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: windows::Win32::System::Com::IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 按 1-based 索引取字体名。
    pub fn item(&self, index: i32) -> Option<String> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_string().ok()
    }

    /// 返回所有字体名。
    pub fn all(&self) -> Vec<String> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }
}
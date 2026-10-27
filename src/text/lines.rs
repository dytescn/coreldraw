//! `IVGTextLines` —— 行视图

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::text::IvgTextRange;

/// CorelDRAW `IVGTextLines` 对象的后期绑定包装（文本线）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTextLines {
    disp: ComObject,
}

impl IvgTextLines {
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32, count: i32) -> Option<IvgTextRange> {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(count as i64),
        ];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    /// 读取返回全部条目（`All`）。
    pub fn all(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("All").map(IvgTextRange::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("First").map(IvgTextRange::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("Last").map(IvgTextRange::new)
    }
}
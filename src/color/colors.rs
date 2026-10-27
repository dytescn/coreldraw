//! `IVGColors` —— 颜色集合
//!
//! ⚠️ RIDL 里 `IVGColors` 是*空接口*（无方法/属性））
//! 只作为返回类型出现。保留骨架以备将来扩展。

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGColors` 对象的后期绑定包装（颜色集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgColors {
    disp: ComObject,
}

impl IvgColors {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }
}
//! `IVGStyleCharacter` —— 样式里的字符部分
//!
//! RIDL 里只有 `Style` 属性；实际字符属性通过 `IVGStyle` 的
//! `GetProperty` / `SetProperty` 访问。此处仅保留骨架。

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGStyleCharacter` 对象的后期绑定包装（样式字符）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStyleCharacter {
    disp: ComObject,
}

impl IvgStyleCharacter {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    /// 关联的父样式。
    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.disp
            .get_property("Style")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::style::IvgStyle::new)
    }
}
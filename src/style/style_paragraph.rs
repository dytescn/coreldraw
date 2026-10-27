//! `IVGStyleParagraph` —— 样式里的段落部分

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGStyleParagraph` 对象的后期绑定包装（样式段落）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStyleParagraph {
    disp: ComObject,
}

impl IvgStyleParagraph {
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

    /// 读取样式（`Style`）。
    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.disp
            .get_property("Style")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::style::IvgStyle::new)
    }
}
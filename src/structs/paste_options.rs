//! `IVGStructPasteOptions` —— 粘贴选项

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::structs::IvgStructColorConversionOptions;

/// CorelDRAW `IVGStructPasteOptions` 对象的后期绑定包装（粘贴选项集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructPasteOptions {
    disp: ComObject,
}

impl IvgStructPasteOptions {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取颜色转换选项集（`ColorConversionOptions`）。
    pub fn color_conversion_options(&self) -> Option<IvgStructColorConversionOptions> {
        self.prop_dispatch("ColorConversionOptions")
            .map(IvgStructColorConversionOptions::new)
    }
}
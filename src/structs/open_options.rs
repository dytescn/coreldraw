//! `IVGStructOpenOptions` —— 打开文档选项

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::structs::IvgStructColorConversionOptions;

/// CorelDRAW `IVGStructOpenOptions` 对象的后期绑定包装（打开选项集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructOpenOptions {
    disp: ComObject,
}

impl IvgStructOpenOptions {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 代码须----

    /// 读取 COM 属性 `CodePage`。
    pub fn code_page(&self) -> Option<i64> { self.prop_i64("CodePage") }
    /// 设置 COM 属性 `CodePage`。
    pub fn set_code_page(&self, v: i32) -> bool { self.put_i64("CodePage", v as i64) }

    // ---- 颜色转换 ----

    /// 读取颜色转换选项集（`ColorConversionOptions`）。
    pub fn color_conversion_options(&self) -> Option<IvgStructColorConversionOptions> {
        self.prop_dispatch("ColorConversionOptions")
            .map(IvgStructColorConversionOptions::new)
    }
}
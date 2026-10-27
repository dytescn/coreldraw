//! `IVGStructCreateOptions` —— 新建文档选项

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColorContext;

/// CorelDRAW `IVGStructCreateOptions` 对象的后期绑定包装（创建选项集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructCreateOptions {
    disp: ComObject,
}

impl IvgStructCreateOptions {
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

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---- 颜色上下文----

    /// 读取颜色上下文（`ColorContext`）。
    pub fn color_context(&self) -> Option<IvgColorContext> {
        self.prop_dispatch("ColorContext").map(IvgColorContext::new)
    }
    /// 设置颜色上下文（`ColorContext`）。
    pub fn set_color_context(&self, c: &IvgColorContext) -> bool {
        self.put_dispatch("ColorContext", c.as_variant())
    }

    // ---- 文档名----

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool {
        self.put_string("Name", v)
    }

    // ---- 页面尺寸 ----

    /// 读取页面宽度（`PageWidth`）。
    pub fn page_width(&self) -> Option<f64> { self.prop_f64("PageWidth") }
    /// 设置页面宽度（`PageWidth`）。
    pub fn set_page_width(&self, v: f64) -> bool { self.put_f64("PageWidth", v) }

    /// 读取页面高度（`PageHeight`）。
    pub fn page_height(&self) -> Option<f64> { self.prop_f64("PageHeight") }
    /// 设置页面高度（`PageHeight`）。
    pub fn set_page_height(&self, v: f64) -> bool { self.put_f64("PageHeight", v) }

    /// `cdrUnit`
    pub fn units(&self) -> Option<i64> { self.prop_i64("Units") }
    /// 设置单位（`Units`）。
    pub fn set_units(&self, v: i32) -> bool { self.put_i64("Units", v as i64) }

    /// 读取分辨率（`Resolution`）。
    pub fn resolution(&self) -> Option<f64> { self.prop_f64("Resolution") }
    /// 设置分辨率（`Resolution`）。
    pub fn set_resolution(&self, v: f64) -> bool { self.put_f64("Resolution", v) }
}
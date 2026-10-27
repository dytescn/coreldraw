//! `IVGStyleTransparency` —— 样式里的透明部分

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGStyleTransparency` 对象的后期绑定包装（样式透明度）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStyleTransparency {
    disp: ComObject,
}

impl IvgStyleTransparency {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取样式（`Style`）。
    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.prop_dispatch("Style").map(crate::style::IvgStyle::new)
    }

    /// 读取填充（`Fill`）。
    pub fn fill(&self) -> Option<crate::style::IvgStyleFill> {
        self.prop_dispatch("Fill").map(crate::style::IvgStyleFill::new)
    }

    /// `cdrMergeMode`
    pub fn mode(&self) -> Option<i64> { self.prop_i64("Mode") }
    /// 设置模式（`Mode`）。
    pub fn set_mode(&self, v: i32) -> bool { self.put_i64("Mode", v as i64) }

    // ---- 不透明度----

    /// 读取均匀透明度（`UniformTransparency`）。
    pub fn uniform_transparency(&self) -> Option<f64> { self.prop_f64("UniformTransparency") }
    /// 设置均匀透明度（`UniformTransparency`）。
    pub fn set_uniform_transparency(&self, v: f64) -> bool {
        self.put_f64("UniformTransparency", v)
    }

    /// 读取白色透明度（`WhiteTransparency`）。
    pub fn white_transparency(&self) -> Option<f64> { self.prop_f64("WhiteTransparency") }
    /// 设置白色透明度（`WhiteTransparency`）。
    pub fn set_white_transparency(&self, v: f64) -> bool {
        self.put_f64("WhiteTransparency", v)
    }

    /// 读取黑色透明度（`BlackTransparency`）。
    pub fn black_transparency(&self) -> Option<f64> { self.prop_f64("BlackTransparency") }
    /// 设置黑色透明度（`BlackTransparency`）。
    pub fn set_black_transparency(&self, v: f64) -> bool {
        self.put_f64("BlackTransparency", v)
    }

    // ---- 应用对象 ----

    /// `cdrTransparencyAppliedTo`
    pub fn applies_to(&self) -> Option<i64> { self.prop_i64("AppliesTo") }
    /// 设置 COM 属性 `AppliesTo`。
    pub fn set_applies_to(&self, v: i32) -> bool { self.put_i64("AppliesTo", v as i64) }
}
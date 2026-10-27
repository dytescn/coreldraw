//! `IVGStructPaletteOptions` —— 调色板/ 位图转调色板选项
//!
//! 用于 `IVGBitmap::ConvertToPaletted2` 等。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGStructPaletteOptions` 对象的后期绑定包装（调色板选项集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructPaletteOptions {
    disp: ComObject,
}

impl IvgStructPaletteOptions {
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

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 调色板类型 / 颜色数 ----

    /// `cdrImagePaletteType`
    pub fn palette_type(&self) -> Option<i64> { self.prop_i64("PaletteType") }
    /// 设置调色板类型（`PaletteType`）。
    pub fn set_palette_type(&self, v: i32) -> bool {
        self.put_i64("PaletteType", v as i64)
    }

    /// 读取 COM 属性 `NumColors`。
    pub fn num_colors(&self) -> Option<i64> { self.prop_i64("NumColors") }
    /// 设置 COM 属性 `NumColors`。
    pub fn set_num_colors(&self, v: i32) -> bool { self.put_i64("NumColors", v as i64) }

    // ---- 抖动 ----

    /// `cdrDitherType`
    pub fn dither_type(&self) -> Option<i64> { self.prop_i64("DitherType") }
    /// 设置抖动类型（`DitherType`）。
    pub fn set_dither_type(&self, v: i32) -> bool { self.put_i64("DitherType", v as i64) }

    /// 读取抖动强度（`DitherIntensity`）。
    pub fn dither_intensity(&self) -> Option<i64> { self.prop_i64("DitherIntensity") }
    /// 设置抖动强度（`DitherIntensity`）。
    pub fn set_dither_intensity(&self, v: i32) -> bool {
        self.put_i64("DitherIntensity", v as i64)
    }

    // ---- 平滑 / 颜色敏感 ----

    /// 读取平滑（`Smoothing`）。
    pub fn smoothing(&self) -> Option<i64> { self.prop_i64("Smoothing") }
    /// 设置平滑（`Smoothing`）。
    pub fn set_smoothing(&self, v: i32) -> bool { self.put_i64("Smoothing", v as i64) }

    /// 读取 COM 属性 `ColorSensitive`。
    pub fn color_sensitive(&self) -> Option<bool> { self.prop_bool("ColorSensitive") }
    /// 设置 COM 属性 `ColorSensitive`。
    pub fn set_color_sensitive(&self, v: bool) -> bool {
        self.put_bool("ColorSensitive", v)
    }

    // ---- 目标色/ 重要性----

    /// 读取目标颜色（`TargetColor`）。
    pub fn target_color(&self) -> Option<i64> { self.prop_i64("TargetColor") }
    /// 设置目标颜色（`TargetColor`）。
    pub fn set_target_color(&self, v: i32) -> bool { self.put_i64("TargetColor", v as i64) }

    /// 读取 COM 属性 `Importance`。
    pub fn importance(&self) -> Option<i64> { self.prop_i64("Importance") }
    /// 设置 COM 属性 `Importance`。
    pub fn set_importance(&self, v: i32) -> bool { self.put_i64("Importance", v as i64) }

    // ---- 明度 / 容差 ----

    /// 读取亮度（`Lightness`）。
    pub fn lightness(&self) -> Option<i64> { self.prop_i64("Lightness") }
    /// 设置亮度（`Lightness`）。
    pub fn set_lightness(&self, v: i32) -> bool { self.put_i64("Lightness", v as i64) }

    /// 读取 COM 属性 `ToleranceA`。
    pub fn tolerance_a(&self) -> Option<i64> { self.prop_i64("ToleranceA") }
    /// 设置 COM 属性 `ToleranceA`。
    pub fn set_tolerance_a(&self, v: i32) -> bool { self.put_i64("ToleranceA", v as i64) }

    /// 读取 COM 属性 `ToleranceB`。
    pub fn tolerance_b(&self) -> Option<i64> { self.prop_i64("ToleranceB") }
    /// 设置 COM 属性 `ToleranceB`。
    pub fn set_tolerance_b(&self, v: i32) -> bool { self.put_i64("ToleranceB", v as i64) }

    // ---- 调色板数据----

    /// 调色板数组（`SAFEARRAY`）。
    pub fn palette(&self) -> Option<Variant> {
        self.disp.get_property("Palette").ok()
    }
    /// 设置调色板（`Palette`）。
    pub fn set_palette(&self, v: Variant) -> bool {
        self.disp.set_property("Palette", vec![v]).is_ok()
    }
}
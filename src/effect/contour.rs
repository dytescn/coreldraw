//! `IVGEffectContour` —— 轮廓效果

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::shape::IvgShape;

/// CorelDRAW `IVGEffectContour` 对象的后期绑定包装（效果轮廓）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectContour {
    disp: ComObject,
}

impl IvgEffectContour {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
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

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_color(&self, name: &str, c: &IvgColor) -> bool {
        let arg = c.as_variant();
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// `cdrContourDirection`
    pub fn direction(&self) -> Option<i64> { self.prop_i64("Direction") }
    /// 设置方向（`Direction`）。
    pub fn set_direction(&self, v: i32) -> bool { self.put_i64("Direction", v as i64) }

    /// 读取偏移（`Offset`）。
    pub fn offset(&self) -> Option<f64> { self.prop_f64("Offset") }
    /// 设置偏移（`Offset`）。
    pub fn set_offset(&self, v: f64) -> bool { self.put_f64("Offset", v) }

    /// 读取步数（`Steps`）。
    pub fn steps(&self) -> Option<i64> { self.prop_i64("Steps") }
    /// 设置步数（`Steps`）。
    pub fn set_steps(&self, v: i32) -> bool { self.put_i64("Steps", v as i64) }

    /// `cdrFountainFillBlendType`
    pub fn color_blend_type(&self) -> Option<i64> {
        self.prop_i64("ColorBlendType")
    }
    /// 设置颜色调和类型（`ColorBlendType`）。
    pub fn set_color_blend_type(&self, v: i32) -> bool {
        self.put_i64("ColorBlendType", v as i64)
    }

    /// 读取轮廓颜色（`OutlineColor`）。
    pub fn outline_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("OutlineColor").map(IvgColor::new)
    }
    /// 设置轮廓颜色（`OutlineColor`）。
    pub fn set_outline_color(&self, c: &IvgColor) -> bool {
        self.put_color("OutlineColor", c)
    }

    /// 读取填充颜色（`FillColor`）。
    pub fn fill_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("FillColor").map(IvgColor::new)
    }
    /// 设置填充颜色（`FillColor`）。
    pub fn set_fill_color(&self, c: &IvgColor) -> bool {
        self.put_color("FillColor", c)
    }

    /// 读取填充颜色（`FillColorTo`）。
    pub fn fill_color_to(&self) -> Option<IvgColor> {
        self.prop_dispatch("FillColorTo").map(IvgColor::new)
    }
    /// 设置填充颜色（`FillColorTo`）。
    pub fn set_fill_color_to(&self, c: &IvgColor) -> bool {
        self.put_color("FillColorTo", c)
    }

    /// 读取链接加速度（`LinkAcceleration`）。
    pub fn link_acceleration(&self) -> Option<bool> {
        self.prop_bool("LinkAcceleration")
    }
    /// 设置链接加速度（`LinkAcceleration`）。
    pub fn set_link_acceleration(&self, v: bool) -> bool {
        self.put_bool("LinkAcceleration", v)
    }

    /// 读取颜色加速度（`ColorAcceleration`）。
    pub fn color_acceleration(&self) -> Option<i64> {
        self.prop_i64("ColorAcceleration")
    }
    /// 设置颜色加速度（`ColorAcceleration`）。
    pub fn set_color_acceleration(&self, v: i32) -> bool {
        self.put_i64("ColorAcceleration", v as i64)
    }

    /// 读取间距加速度（`SpacingAcceleration`）。
    pub fn spacing_acceleration(&self) -> Option<i64> {
        self.prop_i64("SpacingAcceleration")
    }
    /// 设置间距加速度（`SpacingAcceleration`）。
    pub fn set_spacing_acceleration(&self, v: i32) -> bool {
        self.put_i64("SpacingAcceleration", v as i64)
    }

    /// 轮廓组合图形。
    pub fn contour_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ContourGroup").map(IvgShape::new)
    }

    /// `cdrContourEndCapType`
    pub fn end_cap_type(&self) -> Option<i64> {
        self.prop_i64("EndCapType")
    }
    /// 设置结束端点类型（`EndCapType`）。
    pub fn set_end_cap_type(&self, v: i32) -> bool {
        self.put_i64("EndCapType", v as i64)
    }

    /// `cdrContourCornerType`
    pub fn corner_type(&self) -> Option<i64> {
        self.prop_i64("CornerType")
    }
    /// 设置圆角类型（`CornerType`）。
    pub fn set_corner_type(&self, v: i32) -> bool {
        self.put_i64("CornerType", v as i64)
    }

    /// 读取斜接限制（`MiterLimit`）。
    pub fn miter_limit(&self) -> Option<f64> {
        self.prop_f64("MiterLimit")
    }
    /// 设置斜接限制（`MiterLimit`）。
    pub fn set_miter_limit(&self, v: f64) -> bool {
        self.put_f64("MiterLimit", v)
    }
}
//! `IVGEffectContour` 鈥斺€?杞粨鏁堟灉

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::shape::IvgShape;

pub struct IvgEffectContour {
    disp: ComObject,
}

impl IvgEffectContour {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

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
    pub fn set_direction(&self, v: i32) -> bool { self.put_i64("Direction", v as i64) }

    pub fn offset(&self) -> Option<f64> { self.prop_f64("Offset") }
    pub fn set_offset(&self, v: f64) -> bool { self.put_f64("Offset", v) }

    pub fn steps(&self) -> Option<i64> { self.prop_i64("Steps") }
    pub fn set_steps(&self, v: i32) -> bool { self.put_i64("Steps", v as i64) }

    /// `cdrFountainFillBlendType`
    pub fn color_blend_type(&self) -> Option<i64> {
        self.prop_i64("ColorBlendType")
    }
    pub fn set_color_blend_type(&self, v: i32) -> bool {
        self.put_i64("ColorBlendType", v as i64)
    }

    pub fn outline_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("OutlineColor").map(IvgColor::new)
    }
    pub fn set_outline_color(&self, c: &IvgColor) -> bool {
        self.put_color("OutlineColor", c)
    }

    pub fn fill_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("FillColor").map(IvgColor::new)
    }
    pub fn set_fill_color(&self, c: &IvgColor) -> bool {
        self.put_color("FillColor", c)
    }

    pub fn fill_color_to(&self) -> Option<IvgColor> {
        self.prop_dispatch("FillColorTo").map(IvgColor::new)
    }
    pub fn set_fill_color_to(&self, c: &IvgColor) -> bool {
        self.put_color("FillColorTo", c)
    }

    pub fn link_acceleration(&self) -> Option<bool> {
        self.prop_bool("LinkAcceleration")
    }
    pub fn set_link_acceleration(&self, v: bool) -> bool {
        self.put_bool("LinkAcceleration", v)
    }

    pub fn color_acceleration(&self) -> Option<i64> {
        self.prop_i64("ColorAcceleration")
    }
    pub fn set_color_acceleration(&self, v: i32) -> bool {
        self.put_i64("ColorAcceleration", v as i64)
    }

    pub fn spacing_acceleration(&self) -> Option<i64> {
        self.prop_i64("SpacingAcceleration")
    }
    pub fn set_spacing_acceleration(&self, v: i32) -> bool {
        self.put_i64("SpacingAcceleration", v as i64)
    }

    /// 杞粨缁勫悎鍥惧舰銆?
    pub fn contour_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ContourGroup").map(IvgShape::new)
    }

    /// `cdrContourEndCapType`
    pub fn end_cap_type(&self) -> Option<i64> {
        self.prop_i64("EndCapType")
    }
    pub fn set_end_cap_type(&self, v: i32) -> bool {
        self.put_i64("EndCapType", v as i64)
    }

    /// `cdrContourCornerType`
    pub fn corner_type(&self) -> Option<i64> {
        self.prop_i64("CornerType")
    }
    pub fn set_corner_type(&self, v: i32) -> bool {
        self.put_i64("CornerType", v as i64)
    }

    pub fn miter_limit(&self) -> Option<f64> {
        self.prop_f64("MiterLimit")
    }
    pub fn set_miter_limit(&self, v: f64) -> bool {
        self.put_f64("MiterLimit", v)
    }
}
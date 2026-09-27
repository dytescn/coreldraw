//! `IVGStyleOutline` 鈥斺€?鏍峰紡閲岀殑杞粨閮ㄥ垎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgStyleOutline {
    disp: ComObject,
}

impl IvgStyleOutline {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
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

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---- 涓诲睘鎬?----

    /// `cdrOutlineType`
    pub fn outline_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_outline_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    pub fn color(&self) -> Option<crate::color::IvgColor> {
        self.prop_dispatch("Color").map(crate::color::IvgColor::new)
    }
    pub fn set_color(&self, c: &crate::color::IvgColor) -> bool {
        self.put_dispatch("Color", c.as_variant())
    }

    pub fn overprint(&self) -> Option<bool> { self.prop_bool("Overprint") }
    pub fn set_overprint(&self, v: bool) -> bool { self.put_bool("Overprint", v) }

    pub fn behind_fill(&self) -> Option<bool> { self.prop_bool("BehindFill") }
    pub fn set_behind_fill(&self, v: bool) -> bool { self.put_bool("BehindFill", v) }

    pub fn scale_with_shape(&self) -> Option<bool> { self.prop_bool("ScaleWithShape") }
    pub fn set_scale_with_shape(&self, v: bool) -> bool {
        self.put_bool("ScaleWithShape", v)
    }

    // ---- 绠ご ----

    pub fn overlap_arrow(&self) -> Option<bool> { self.prop_bool("OverlapArrow") }
    pub fn set_overlap_arrow(&self, v: bool) -> bool { self.put_bool("OverlapArrow", v) }

    pub fn share_arrow(&self) -> Option<bool> { self.prop_bool("ShareArrow") }
    pub fn set_share_arrow(&self, v: bool) -> bool { self.put_bool("ShareArrow", v) }

    // ---- 鏂滄帴 / 绗斿皷 ----

    pub fn miter_limit(&self) -> Option<f64> { self.prop_f64("MiterLimit") }
    pub fn set_miter_limit(&self, v: f64) -> bool { self.put_f64("MiterLimit", v) }

    pub fn nib_stretch(&self) -> Option<i64> { self.prop_i64("NibStretch") }
    pub fn set_nib_stretch(&self, v: i32) -> bool { self.put_i64("NibStretch", v as i64) }

    pub fn nib_angle(&self) -> Option<f64> { self.prop_f64("NibAngle") }
    pub fn set_nib_angle(&self, v: f64) -> bool { self.put_f64("NibAngle", v) }

    pub fn wideline_width(&self) -> Option<f64> { self.prop_f64("WidelineWidth") }
    pub fn set_wideline_width(&self, v: f64) -> bool { self.put_f64("WidelineWidth", v) }

    // ---- 绔偣 / 杩炴帴 / 瀵归綈 ----

    /// `cdrOutlineLineCaps`
    pub fn line_caps(&self) -> Option<i64> { self.prop_i64("LineCaps") }
    pub fn set_line_caps(&self, v: i32) -> bool { self.put_i64("LineCaps", v as i64) }

    /// `cdrOutlineLineJoin`
    pub fn line_join(&self) -> Option<i64> { self.prop_i64("LineJoin") }
    pub fn set_line_join(&self, v: i32) -> bool { self.put_i64("LineJoin", v as i64) }

    /// `cdrOutlineJustification`
    pub fn justification(&self) -> Option<i64> { self.prop_i64("Justification") }
    pub fn set_justification(&self, v: i32) -> bool {
        self.put_i64("Justification", v as i64)
    }

    /// `cdrOutlineDashAdjust`
    pub fn adjust_dashes(&self) -> Option<i64> { self.prop_i64("AdjustDashes") }
    pub fn set_adjust_dashes(&self, v: i32) -> bool {
        self.put_i64("AdjustDashes", v as i64)
    }

    // ---- 鍏宠仈鐨勬牱寮?----

    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.prop_dispatch("Style").map(crate::style::IvgStyle::new)
    }
}
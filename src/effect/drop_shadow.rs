//! `IVGEffectDropShadow` 鈥斺€?闃村奖鏁堟灉

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::shape::IvgShape;

pub struct IvgEffectDropShadow {
    disp: ComObject,
}

impl IvgEffectDropShadow {
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

    fn put_color(&self, name: &str, c: &IvgColor) -> bool {
        let arg = c.as_variant();
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 浣嶇疆 ----

    pub fn offset_x(&self) -> Option<f64> { self.prop_f64("OffsetX") }
    pub fn set_offset_x(&self, v: f64) -> bool { self.put_f64("OffsetX", v) }

    pub fn offset_y(&self) -> Option<f64> { self.prop_f64("OffsetY") }
    pub fn set_offset_y(&self, v: f64) -> bool { self.put_f64("OffsetY", v) }

    pub fn set_offset(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetOffset", args).is_ok()
    }

    // ---- 閫忔槑 / 缇藉寲 ----

    pub fn opacity(&self) -> Option<i64> { self.prop_i64("Opacity") }
    pub fn set_opacity(&self, v: i32) -> bool { self.put_i64("Opacity", v as i64) }

    pub fn feather(&self) -> Option<i64> { self.prop_i64("Feather") }
    pub fn set_feather(&self, v: i32) -> bool { self.put_i64("Feather", v as i64) }

    /// `cdrFeatherType`
    pub fn feather_type(&self) -> Option<i64> { self.prop_i64("FeatherType") }
    pub fn set_feather_type(&self, v: i32) -> bool { self.put_i64("FeatherType", v as i64) }

    /// `cdrEdgeType`
    pub fn feather_edge(&self) -> Option<i64> { self.prop_i64("FeatherEdge") }
    pub fn set_feather_edge(&self, v: i32) -> bool { self.put_i64("FeatherEdge", v as i64) }

    // ---- 绫诲瀷 ----

    /// `cdrDropShadowType`
    pub fn shadow_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_shadow_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn perspective_angle(&self) -> Option<f64> {
        self.prop_f64("PerspectiveAngle")
    }
    pub fn set_perspective_angle(&self, v: f64) -> bool {
        self.put_f64("PerspectiveAngle", v)
    }

    pub fn perspective_stretch(&self) -> Option<f64> {
        self.prop_f64("PerspectiveStretch")
    }
    pub fn set_perspective_stretch(&self, v: f64) -> bool {
        self.put_f64("PerspectiveStretch", v)
    }

    pub fn fade(&self) -> Option<i64> { self.prop_i64("Fade") }
    pub fn set_fade(&self, v: i32) -> bool { self.put_i64("Fade", v as i64) }

    // ---- 棰滆壊 / 娣峰悎 ----

    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    pub fn set_color(&self, c: &IvgColor) -> bool {
        self.put_color("Color", c)
    }

    /// `cdrMergeMode`
    pub fn merge_mode(&self) -> Option<i64> { self.prop_i64("MergeMode") }
    pub fn set_merge_mode(&self, v: i32) -> bool { self.put_i64("MergeMode", v as i64) }

    /// 闃村奖缁勶紙缁勫悎鍥惧舰锛夈€?
    pub fn shadow_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ShadowGroup").map(IvgShape::new)
    }
}
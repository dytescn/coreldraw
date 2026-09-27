//! `IVGEffectExtrude` 鈥斺€?绔嬩綋鍖栨晥鏋?
//! 浠ュ強 `IVGExtrudeVanishingPoint`锛堟秷澶辩偣锛?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::effect::IvgEffects;
use crate::shape::IvgShape;

// =============================================================
// IvgExtrudeVanishingPoint
// =============================================================

pub struct IvgExtrudeVanishingPoint {
    disp: ComObject,
}

impl IvgExtrudeVanishingPoint {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    /// `cdrExtrudeVPType`
    pub fn vp_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_vp_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    pub fn set_position_x(&self, v: f64) -> bool { self.put_f64("PositionX", v) }

    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    pub fn set_position_y(&self, v: f64) -> bool { self.put_f64("PositionY", v) }

    pub fn effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("Effects").map(IvgEffects::new)
    }

    /// 涓庡彟涓€涓珛浣撳寲鏁堟灉鍏变韩娑堝け鐐广€?
    pub fn share(&self, src: &IvgEffectExtrude) -> Option<bool> {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("Share", args).ok()?.to_bool().ok()
    }
}

// =============================================================
// IvgEffectExtrude
// =============================================================

pub struct IvgEffectExtrude {
    disp: ComObject,
}

impl IvgEffectExtrude {
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

    // ---- 绫诲瀷 ----

    /// `cdrExtrudeType`
    pub fn extrude_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_extrude_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn vanishing_point(&self) -> Option<IvgExtrudeVanishingPoint> {
        self.prop_dispatch("VanishingPoint").map(IvgExtrudeVanishingPoint::new)
    }

    // ---- 娣卞害 / 瑙掑害 ----

    pub fn depth(&self) -> Option<i64> { self.prop_i64("Depth") }
    pub fn set_depth(&self, v: i32) -> bool { self.put_i64("Depth", v as i64) }

    pub fn angle_x(&self) -> Option<f64> { self.prop_f64("AngleX") }
    pub fn set_angle_x(&self, v: f64) -> bool { self.put_f64("AngleX", v) }

    pub fn angle_y(&self) -> Option<f64> { self.prop_f64("AngleY") }
    pub fn set_angle_y(&self, v: f64) -> bool { self.put_f64("AngleY", v) }

    pub fn angle_z(&self) -> Option<f64> { self.prop_f64("AngleZ") }
    pub fn set_angle_z(&self, v: f64) -> bool { self.put_f64("AngleZ", v) }

    // ---- 鐫€鑹?----

    /// `cdrExtrudeShading`
    pub fn shading(&self) -> Option<i64> { self.prop_i64("Shading") }
    pub fn set_shading(&self, v: i32) -> bool { self.put_i64("Shading", v as i64) }

    pub fn base_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("BaseColor").map(IvgColor::new)
    }
    pub fn set_base_color(&self, c: &IvgColor) -> bool {
        self.put_color("BaseColor", c)
    }

    pub fn shading_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("ShadingColor").map(IvgColor::new)
    }
    pub fn set_shading_color(&self, c: &IvgColor) -> bool {
        self.put_color("ShadingColor", c)
    }

    pub fn use_full_color_range(&self) -> Option<bool> {
        self.prop_bool("UseFullColorRange")
    }
    pub fn set_use_full_color_range(&self, v: bool) -> bool {
        self.put_bool("UseFullColorRange", v)
    }

    // ---- 鏂滈潰 ----

    pub fn use_bevel(&self) -> Option<bool> { self.prop_bool("UseBevel") }
    pub fn set_use_bevel(&self, v: bool) -> bool { self.put_bool("UseBevel", v) }

    pub fn show_bevel_only(&self) -> Option<bool> { self.prop_bool("ShowBevelOnly") }
    pub fn set_show_bevel_only(&self, v: bool) -> bool { self.put_bool("ShowBevelOnly", v) }

    pub fn bevel_depth(&self) -> Option<f64> { self.prop_f64("BevelDepth") }
    pub fn set_bevel_depth(&self, v: f64) -> bool { self.put_f64("BevelDepth", v) }

    pub fn bevel_angle(&self) -> Option<f64> { self.prop_f64("BevelAngle") }
    pub fn set_bevel_angle(&self, v: f64) -> bool { self.put_f64("BevelAngle", v) }

    pub fn use_extrude_color_for_bevel(&self) -> Option<bool> {
        self.prop_bool("UseExtrudeColorForBevel")
    }
    pub fn set_use_extrude_color_for_bevel(&self, v: bool) -> bool {
        self.put_bool("UseExtrudeColorForBevel", v)
    }

    pub fn bevel_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("BevelColor").map(IvgColor::new)
    }
    pub fn set_bevel_color(&self, c: &IvgColor) -> bool {
        self.put_color("BevelColor", c)
    }

    pub fn set_bevel(&self, depth: f64, angle: f64, show_only: bool) -> bool {
        let args = vec![
            Variant::from_f64(depth),
            Variant::from_f64(angle),
            Variant::from_bool(show_only),
        ];
        self.disp.invoke_method("SetBevel", args).is_ok()
    }

    // ---- 鍏夋簮锛? 涓級 ----

    /// 绱㈠紩 1..3
    pub fn light_present(&self, index: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("LightPresent", args).ok()?.to_bool().ok()
    }

    pub fn set_light_present(&self, index: i32, v: bool) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_bool(v),
        ];
        self.disp.invoke_method("put_LightPresent", args).is_ok()
    }

    /// `cdrExtrudeLightPosition`
    pub fn light_position(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("LightPosition", args).ok()?.to_i64().ok()
    }

    pub fn set_light_position(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_LightPosition", args).is_ok()
    }

    pub fn light_intensity(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("LightIntensity", args).ok()?.to_i64().ok()
    }

    pub fn set_light_intensity(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_LightIntensity", args).is_ok()
    }

    pub fn set_light(
        &self,
        index: i32,
        position: i32,
        intensity: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(position as i64),
            Variant::from_i64(intensity as i64),
        ];
        self.disp.invoke_method("SetLight", args).is_ok()
    }

    // ---- 闈?/ 缁?----

    pub fn face_visible(&self) -> Option<bool> { self.prop_bool("FaceVisible") }

    pub fn face_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("FaceShape").map(IvgShape::new)
    }

    pub fn bevel_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("BevelGroup").map(IvgShape::new)
    }

    pub fn extrude_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ExtrudeGroup").map(IvgShape::new)
    }

    // ---- 鏃嬭浆 / 澶嶅埗 ----

    pub fn rotate(&self, ax: f64, ay: f64, az: f64) -> bool {
        let args = vec![
            Variant::from_f64(ax),
            Variant::from_f64(ay),
            Variant::from_f64(az),
        ];
        self.disp.invoke_method("Rotate", args).is_ok()
    }

    pub fn copy_from(&self, src: &IvgEffectExtrude) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyFrom", args).is_ok()
    }
}
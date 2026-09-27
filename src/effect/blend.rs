//! `IVGEffectBlend` 鈥斺€?璋冨拰鏁堟灉

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgSnapPoint;
use crate::shape::IvgShape;

pub struct IvgEffectBlend {
    disp: ComObject,
}

impl IvgEffectBlend {
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

    fn put_dispatch(&self, name: &str, d: &IvgShape) -> bool {
        let arg = d.as_variant();
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 绔偣鍥惧舰 ----

    pub fn start_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("StartShape").map(IvgShape::new)
    }
    pub fn set_start_shape(&self, s: &IvgShape) -> bool {
        self.put_dispatch("StartShape", s)
    }

    pub fn end_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("EndShape").map(IvgShape::new)
    }
    pub fn set_end_shape(&self, s: &IvgShape) -> bool {
        self.put_dispatch("EndShape", s)
    }

    /// 璋冨拰鐢熸垚鐨勭粍鍚堝浘褰€?
    pub fn blend_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("BlendGroup").map(IvgShape::new)
    }

    // ---- 璺緞 ----

    pub fn path(&self) -> Option<IvgShape> {
        self.prop_dispatch("Path").map(IvgShape::new)
    }
    pub fn set_path(&self, s: &IvgShape) -> bool {
        self.put_dispatch("Path", s)
    }

    pub fn start_shape_offset(&self) -> Option<f64> {
        self.prop_f64("StartShapeOffset")
    }
    pub fn set_start_shape_offset(&self, v: f64) -> bool {
        self.put_f64("StartShapeOffset", v)
    }

    pub fn end_shape_offset(&self) -> Option<f64> {
        self.prop_f64("EndShapeOffset")
    }
    pub fn set_end_shape_offset(&self, v: f64) -> bool {
        self.put_f64("EndShapeOffset", v)
    }

    // ---- 姝ヨ繘 ----

    /// `cdrBlendMode` 鈥斺€?瑙?`enums/effect`
    pub fn mode(&self) -> Option<i64> { self.prop_i64("Mode") }
    pub fn set_mode(&self, v: i32) -> bool { self.put_i64("Mode", v as i64) }

    pub fn steps(&self) -> Option<i64> { self.prop_i64("Steps") }
    pub fn set_steps(&self, v: i32) -> bool { self.put_i64("Steps", v as i64) }

    pub fn spacing(&self) -> Option<f64> { self.prop_f64("Spacing") }
    pub fn set_spacing(&self, v: f64) -> bool { self.put_f64("Spacing", v) }

    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }

    // ---- 璺緞寰幆 ----

    pub fn looping(&self) -> Option<bool> { self.prop_bool("Loop") }
    pub fn set_looping(&self, v: bool) -> bool { self.put_bool("Loop", v) }

    pub fn full_path(&self) -> Option<bool> { self.prop_bool("FullPath") }
    pub fn set_full_path(&self, v: bool) -> bool { self.put_bool("FullPath", v) }

    pub fn rotate_shapes(&self) -> Option<bool> { self.prop_bool("RotateShapes") }
    pub fn set_rotate_shapes(&self, v: bool) -> bool { self.put_bool("RotateShapes", v) }

    // ---- 棰滆壊娣峰悎 ----

    /// `cdrFountainFillBlendType`
    pub fn color_blend_type(&self) -> Option<i64> {
        self.prop_i64("ColorBlendType")
    }
    pub fn set_color_blend_type(&self, v: i32) -> bool {
        self.put_i64("ColorBlendType", v as i64)
    }

    pub fn spacing_acceleration(&self) -> Option<i64> {
        self.prop_i64("SpacingAcceleration")
    }
    pub fn set_spacing_acceleration(&self, v: i32) -> bool {
        self.put_i64("SpacingAcceleration", v as i64)
    }

    pub fn color_acceleration(&self) -> Option<i64> {
        self.prop_i64("ColorAcceleration")
    }
    pub fn set_color_acceleration(&self, v: i32) -> bool {
        self.put_i64("ColorAcceleration", v as i64)
    }

    pub fn link_acceleration(&self) -> Option<bool> {
        self.prop_bool("LinkAcceleration")
    }
    pub fn set_link_acceleration(&self, v: bool) -> bool {
        self.put_bool("LinkAcceleration", v)
    }

    pub fn accelerate_size(&self) -> Option<bool> {
        self.prop_bool("AccelerateSize")
    }
    pub fn set_accelerate_size(&self, v: bool) -> bool {
        self.put_bool("AccelerateSize", v)
    }

    pub fn map_nodes(&self) -> Option<bool> { self.prop_bool("MapNodes") }
    pub fn set_map_nodes(&self, v: bool) -> bool { self.put_bool("MapNodes", v) }

    // ---- 鎶撳彇鐐?----

    pub fn start_point(&self) -> Option<IvgSnapPoint> {
        self.prop_dispatch("StartPoint").map(IvgSnapPoint::new)
    }

    pub fn end_point(&self) -> Option<IvgSnapPoint> {
        self.prop_dispatch("EndPoint").map(IvgSnapPoint::new)
    }

    // ---- 鎿嶄綔 ----

    /// 鍦ㄧ `step_no` 姝ュ鎷嗗垎銆?
    pub fn split(&self, step_no: i32) -> Option<IvgShape> {
        let args = vec![Variant::from_i64(step_no as i64)];
        self.disp
            .invoke_method("Split", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn fuse_start(&self) -> Option<bool> {
        self.disp.invoke_method("FuseStart", vec![]).ok()?.to_bool().ok()
    }

    pub fn fuse_end(&self) -> Option<bool> {
        self.disp.invoke_method("FuseEnd", vec![]).ok()?.to_bool().ok()
    }

    /// 浠庡叾瀹?blend 澶嶅埗灞炴€с€?
    pub fn copy_from(&self, src: &IvgEffectBlend) -> Option<bool> {
        let args = vec![src.as_variant()];
        self.disp
            .invoke_method("CopyFrom", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
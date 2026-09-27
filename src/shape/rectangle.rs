//! `IVGRectangle` 鈥斺€?鐭╁舰

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgRectangle {
    disp: ComObject,
}

impl IvgRectangle {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
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

    // ---- 鍦嗚鐧惧垎姣?----

    pub fn corner_upper_left(&self) -> Option<i64> { self.prop_i64("CornerUpperLeft") }
    pub fn set_corner_upper_left(&self, v: i32) -> bool { self.put_i64("CornerUpperLeft", v as i64) }

    pub fn corner_upper_right(&self) -> Option<i64> { self.prop_i64("CornerUpperRight") }
    pub fn set_corner_upper_right(&self, v: i32) -> bool { self.put_i64("CornerUpperRight", v as i64) }

    pub fn corner_lower_left(&self) -> Option<i64> { self.prop_i64("CornerLowerLeft") }
    pub fn set_corner_lower_left(&self, v: i32) -> bool { self.put_i64("CornerLowerLeft", v as i64) }

    pub fn corner_lower_right(&self) -> Option<i64> { self.prop_i64("CornerLowerRight") }
    pub fn set_corner_lower_right(&self, v: i32) -> bool { self.put_i64("CornerLowerRight", v as i64) }

    pub fn equal_corners(&self) -> Option<bool> { self.prop_bool("EqualCorners") }
    pub fn max_radius(&self) -> Option<f64> { self.prop_f64("MaxRadius") }

    // ---- 鍦嗚鍗婂緞 ----

    pub fn radius_upper_left(&self) -> Option<f64> { self.prop_f64("RadiusUpperLeft") }
    pub fn set_radius_upper_left(&self, v: f64) -> bool { self.put_f64("RadiusUpperLeft", v) }

    pub fn radius_upper_right(&self) -> Option<f64> { self.prop_f64("RadiusUpperRight") }
    pub fn set_radius_upper_right(&self, v: f64) -> bool { self.put_f64("RadiusUpperRight", v) }

    pub fn radius_lower_left(&self) -> Option<f64> { self.prop_f64("RadiusLowerLeft") }
    pub fn set_radius_lower_left(&self, v: f64) -> bool { self.put_f64("RadiusLowerLeft", v) }

    pub fn radius_lower_right(&self) -> Option<f64> { self.prop_f64("RadiusLowerRight") }
    pub fn set_radius_lower_right(&self, v: f64) -> bool { self.put_f64("RadiusLowerRight", v) }

    // ---- 鎿嶄綔 ----

    pub fn set_roundness(&self, roundness: i32) -> bool {
        let args = vec![Variant::from_i64(roundness as i64)];
        self.disp.invoke_method("SetRoundness", args).is_ok()
    }

    pub fn set_radius(&self, radius: f64) -> bool {
        let args = vec![Variant::from_f64(radius)];
        self.disp.invoke_method("SetRadius", args).is_ok()
    }

    /// `cdrCornerType`
    pub fn corner_type(&self) -> Option<i64> { self.prop_i64("CornerType") }
    pub fn set_corner_type(&self, v: i32) -> bool { self.put_i64("CornerType", v as i64) }

    pub fn relative_corner_scaling(&self) -> Option<bool> {
        self.prop_bool("RelativeCornerScaling")
    }
    pub fn set_relative_corner_scaling(&self, v: bool) -> bool {
        self.put_bool("RelativeCornerScaling", v)
    }
}
//! `IVGEllipse` 鈥斺€?妞渾

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgEllipse {
    disp: ComObject,
}

impl IvgEllipse {
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

    /// `cdrEllipseType`
    pub fn ellipse_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_ellipse_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn start_angle(&self) -> Option<f64> { self.prop_f64("StartAngle") }
    pub fn set_start_angle(&self, v: f64) -> bool { self.put_f64("StartAngle", v) }

    pub fn end_angle(&self) -> Option<f64> { self.prop_f64("EndAngle") }
    pub fn set_end_angle(&self, v: f64) -> bool { self.put_f64("EndAngle", v) }

    pub fn clockwise(&self) -> Option<bool> { self.prop_bool("Clockwise") }
    pub fn set_clockwise(&self, v: bool) -> bool { self.put_bool("Clockwise", v) }

    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    pub fn set_center_x(&self, v: f64) -> bool { self.put_f64("CenterX", v) }

    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }
    pub fn set_center_y(&self, v: f64) -> bool { self.put_f64("CenterY", v) }

    pub fn h_radius(&self) -> Option<f64> { self.prop_f64("HRadius") }
    pub fn set_h_radius(&self, v: f64) -> bool { self.put_f64("HRadius", v) }

    pub fn v_radius(&self) -> Option<f64> { self.prop_f64("VRadius") }
    pub fn set_v_radius(&self, v: f64) -> bool { self.put_f64("VRadius", v) }

    pub fn set_center_position(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetCenterPosition", args).is_ok()
    }

    pub fn get_center_position(&self) -> Option<(f64, f64)> {
        Some((self.center_x()?, self.center_y()?))
    }

    pub fn set_radius(&self, h: f64, v: f64) -> bool {
        let args = vec![
            Variant::from_f64(h),
            Variant::from_f64(v),
        ];
        self.disp.invoke_method("SetRadius", args).is_ok()
    }

    pub fn get_radius(&self) -> Option<(f64, f64)> {
        Some((self.h_radius()?, self.v_radius()?))
    }
}
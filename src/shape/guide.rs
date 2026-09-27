//! `IVGGuide` 鈥斺€?鍙傝€冪嚎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgGuide {
    disp: ComObject,
}

impl IvgGuide {
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

    // ---- 绫诲瀷 / 棰勮 ----

    /// `cdrGuideType`
    pub fn guide_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn preset(&self) -> Option<bool> { self.prop_bool("Preset") }
    pub fn make_editable(&self) -> bool {
        self.disp.invoke_method("MakeEditable", vec![]).is_ok()
    }

    // ---- 鐐?/ 瑙?----

    pub fn point1_x(&self) -> Option<f64> { self.prop_f64("Point1X") }
    pub fn point1_y(&self) -> Option<f64> { self.prop_f64("Point1Y") }
    pub fn point2_x(&self) -> Option<f64> { self.prop_f64("Point2X") }
    pub fn point2_y(&self) -> Option<f64> { self.prop_f64("Point2Y") }
    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }

    pub fn get_points(&self) -> Option<(f64, f64, f64, f64)> {
        Some((self.point1_x()?, self.point1_y()?, self.point2_x()?, self.point2_y()?))
    }

    pub fn get_point_and_angle(&self) -> Option<(f64, f64, f64)> {
        Some((self.point1_x()?, self.point1_y()?, self.angle()?))
    }

    pub fn set_points(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
        ];
        self.disp.invoke_method("SetPoints", args).is_ok()
    }

    pub fn set_point_and_angle(&self, x: f64, y: f64, angle: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(angle),
        ];
        self.disp.invoke_method("SetPointAndAngle", args).is_ok()
    }

    // ---- 鎴窛 / 涓績 ----

    pub fn intercept_x(&self) -> Option<f64> { self.prop_f64("InterceptX") }
    pub fn intercept_y(&self) -> Option<f64> { self.prop_f64("InterceptY") }
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }
}
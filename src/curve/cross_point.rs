//! `IVGCrossPoint` / `IVGCrossPoints` 鈥斺€?浜ょ偣

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgCrossPoints
// =============================================================

pub struct IvgCrossPoints {
    disp: ComObject,
}

impl IvgCrossPoints {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgCrossPoint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgCrossPoint::new)
    }
}

// =============================================================
// IvgCrossPoint
// =============================================================

pub struct IvgCrossPoint {
    disp: ComObject,
}

impl IvgCrossPoint {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    pub fn offset(&self) -> Option<f64> { self.prop_f64("Offset") }
    pub fn offset2(&self) -> Option<f64> { self.prop_f64("Offset2") }
}
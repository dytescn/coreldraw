//! `IVGPolygon` 鈥斺€?澶氳竟褰?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgPolygon {
    disp: ComObject,
}

impl IvgPolygon {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// `cdrPolygonType`
    pub fn polygon_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_polygon_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn sides(&self) -> Option<i64> { self.prop_i64("Sides") }
    pub fn set_sides(&self, v: i32) -> bool { self.put_i64("Sides", v as i64) }

    pub fn sharpness(&self) -> Option<i64> { self.prop_i64("Sharpness") }
    pub fn set_sharpness(&self, v: i32) -> bool { self.put_i64("Sharpness", v as i64) }
}
//! `IVGGrid` 鈥斺€?鏂囨。缃戞牸

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgGrid {
    disp: ComObject,
}

impl IvgGrid {
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

    pub fn visible(&self) -> Option<bool> { self.prop_bool("Visible") }
    pub fn set_visible(&self, v: bool) -> bool { self.put_bool("Visible", v) }

    /// `cdrGridType`
    pub fn grid_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_grid_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn snap(&self) -> Option<bool> { self.prop_bool("Snap") }
    pub fn set_snap(&self, v: bool) -> bool { self.put_bool("Snap", v) }

    pub fn set_frequency(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetFrequency", args).is_ok()
    }

    pub fn spacing_x(&self) -> Option<f64> { self.prop_f64("SpacingX") }
    pub fn set_spacing_x(&self, v: f64) -> bool { self.put_f64("SpacingX", v) }

    pub fn spacing_y(&self) -> Option<f64> { self.prop_f64("SpacingY") }
    pub fn set_spacing_y(&self, v: f64) -> bool { self.put_f64("SpacingY", v) }
}
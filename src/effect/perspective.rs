//! `IVGEffectPerspective` 鈥斺€?閫忚鏁堟灉
use wincom::{ComObject, Variant};


pub struct IvgEffectPerspective {
    disp: ComObject,
}

impl IvgEffectPerspective {
    pub fn new(disp: windows::Win32::System::Com::IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 姘村钩娑堝け鐐?----

    pub fn use_horiz_vanishing_point(&self) -> Option<bool> {
        self.prop_bool("UseHorizVanishingPoint")
    }
    pub fn set_use_horiz_vanishing_point(&self, v: bool) -> bool {
        self.put_bool("UseHorizVanishingPoint", v)
    }

    pub fn horiz_vanishing_point_x(&self) -> Option<f64> {
        self.prop_f64("HorizVanishingPointX")
    }
    pub fn set_horiz_vanishing_point_x(&self, v: f64) -> bool {
        self.put_f64("HorizVanishingPointX", v)
    }

    pub fn horiz_vanishing_point_y(&self) -> Option<f64> {
        self.prop_f64("HorizVanishingPointY")
    }
    pub fn set_horiz_vanishing_point_y(&self, v: f64) -> bool {
        self.put_f64("HorizVanishingPointY", v)
    }

    // ---- 鍨傜洿娑堝け鐐?----

    pub fn use_vert_vanishing_point(&self) -> Option<bool> {
        self.prop_bool("UseVertVanishingPoint")
    }
    pub fn set_use_vert_vanishing_point(&self, v: bool) -> bool {
        self.put_bool("UseVertVanishingPoint", v)
    }

    pub fn vert_vanishing_point_x(&self) -> Option<f64> {
        self.prop_f64("VertVanishingPointX")
    }
    pub fn set_vert_vanishing_point_x(&self, v: f64) -> bool {
        self.put_f64("VertVanishingPointX", v)
    }

    pub fn vert_vanishing_point_y(&self) -> Option<f64> {
        self.prop_f64("VertVanishingPointY")
    }
    pub fn set_vert_vanishing_point_y(&self, v: f64) -> bool {
        self.put_f64("VertVanishingPointY", v)
    }
}
//! `IPrnVBAPrintSeparations` / `IPrnVBASeparationPlate(s)` 鈥斺€?鍒嗚壊

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPrnSeparations
// =============================================================

pub struct IvgPrnSeparations {
    disp: ComObject,
}

impl IvgPrnSeparations {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
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

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 涓诲紑鍏?----

    pub fn enabled(&self) -> Option<bool> { self.prop_bool("Enabled") }
    pub fn set_enabled(&self, v: bool) -> bool { self.put_bool("Enabled", v) }

    pub fn in_color(&self) -> Option<bool> { self.prop_bool("InColor") }
    pub fn set_in_color(&self, v: bool) -> bool { self.put_bool("InColor", v) }

    pub fn hexachrome(&self) -> Option<bool> { self.prop_bool("Hexachrome") }
    pub fn set_hexachrome(&self, v: bool) -> bool { self.put_bool("Hexachrome", v) }

    pub fn spot_to_cmyk(&self) -> Option<bool> { self.prop_bool("SpotToCMYK") }
    pub fn set_spot_to_cmyk(&self, v: bool) -> bool { self.put_bool("SpotToCMYK", v) }

    pub fn empty_plates(&self) -> Option<bool> { self.prop_bool("EmptyPlates") }
    pub fn set_empty_plates(&self, v: bool) -> bool { self.put_bool("EmptyPlates", v) }

    // ---- 鍙犲嵃 ----

    pub fn preserve_overprints(&self) -> Option<bool> { self.prop_bool("PreserveOverprints") }
    pub fn set_preserve_overprints(&self, v: bool) -> bool {
        self.put_bool("PreserveOverprints", v)
    }

    pub fn always_overprint_black(&self) -> Option<bool> {
        self.prop_bool("AlwaysOverprintBlack")
    }
    pub fn set_always_overprint_black(&self, v: bool) -> bool {
        self.put_bool("AlwaysOverprintBlack", v)
    }

    // ---- 鑷姩琛ユ紡鐧?----

    pub fn auto_spreading(&self) -> Option<bool> { self.prop_bool("AutoSpreading") }
    pub fn set_auto_spreading(&self, v: bool) -> bool {
        self.put_bool("AutoSpreading", v)
    }

    pub fn auto_spread_amount(&self) -> Option<f64> { self.prop_f64("AutoSpreadAmount") }
    pub fn set_auto_spread_amount(&self, v: f64) -> bool {
        self.put_f64("AutoSpreadAmount", v)
    }

    pub fn auto_spread_fixed(&self) -> Option<bool> { self.prop_bool("AutoSpreadFixed") }
    pub fn set_auto_spread_fixed(&self, v: bool) -> bool {
        self.put_bool("AutoSpreadFixed", v)
    }

    pub fn auto_spread_text_above(&self) -> Option<f64> {
        self.prop_f64("AutoSpreadTextAbove")
    }
    pub fn set_auto_spread_text_above(&self, v: f64) -> bool {
        self.put_f64("AutoSpreadTextAbove", v)
    }

    // ---- 楂樼骇璁剧疆 ----

    pub fn advanced_settings(&self) -> Option<bool> { self.prop_bool("AdvancedSettings") }
    pub fn set_advanced_settings(&self, v: bool) -> bool {
        self.put_bool("AdvancedSettings", v)
    }

    // ---- 鍒嗚壊鏉?/ 鍒嗚鲸鐜?----

    pub fn plates(&self) -> Option<IvgPrnSeparationPlates> {
        self.prop_dispatch("Plates").map(IvgPrnSeparationPlates::new)
    }

    pub fn resolution(&self) -> Option<i64> { self.prop_i64("Resolution") }
    pub fn set_resolution(&self, v: i32) -> bool { self.put_i64("Resolution", v as i64) }

    pub fn basic_screen(&self) -> Option<String> { self.prop_string("BasicScreen") }
    pub fn set_basic_screen(&self, v: impl Into<String>) -> bool {
        self.put_string("BasicScreen", v)
    }

    pub fn halftone_type(&self) -> Option<String> { self.prop_string("HalftoneType") }
    pub fn set_halftone_type(&self, v: impl Into<String>) -> bool {
        self.put_string("HalftoneType", v)
    }

    pub fn screen_technology(&self) -> Option<String> { self.prop_string("ScreenTechnology") }
    pub fn set_screen_technology(&self, v: impl Into<String>) -> bool {
        self.put_string("ScreenTechnology", v)
    }
}

// =============================================================
// IvgPrnSeparationPlates
// =============================================================

pub struct IvgPrnSeparationPlates {
    disp: ComObject,
}

impl IvgPrnSeparationPlates {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    pub fn item(&self, index: i32) -> Option<IvgPrnSeparationPlate> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPrnSeparationPlate::new)
    }
}

// =============================================================
// IvgPrnSeparationPlate
// =============================================================

pub struct IvgPrnSeparationPlate {
    disp: ComObject,
}

impl IvgPrnSeparationPlate {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn enabled(&self) -> Option<bool> { self.prop_bool("Enabled") }
    pub fn set_enabled(&self, v: bool) -> bool { self.put_bool("Enabled", v) }

    /// `PrnPlateType`
    pub fn plate_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn color(&self) -> Option<String> { self.prop_string("Color") }

    pub fn frequency(&self) -> Option<f64> { self.prop_f64("Frequency") }
    pub fn set_frequency(&self, v: f64) -> bool { self.put_f64("Frequency", v) }

    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }

    pub fn overprint_text(&self) -> Option<bool> { self.prop_bool("OverprintText") }
    pub fn set_overprint_text(&self, v: bool) -> bool { self.put_bool("OverprintText", v) }

    pub fn overprint_graphic(&self) -> Option<bool> { self.prop_bool("OverprintGraphic") }
    pub fn set_overprint_graphic(&self, v: bool) -> bool {
        self.put_bool("OverprintGraphic", v)
    }
}
//! `IPrnVBAPrintLayout` 鈥斺€?鎵撳嵃鎺掔増锛堝嚭琛€ / 鎷肩増 / 璐存爣锛?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgPrnLayout {
    disp: ComObject,
}

impl IvgPrnLayout {
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

    // ---- 鍑鸿 ----

    pub fn use_bleed_limit(&self) -> Option<bool> { self.prop_bool("UseBleedLimit") }
    pub fn set_use_bleed_limit(&self, v: bool) -> bool { self.put_bool("UseBleedLimit", v) }

    pub fn bleed_limit(&self) -> Option<f64> { self.prop_f64("BleedLimit") }
    pub fn set_bleed_limit(&self, v: f64) -> bool { self.put_f64("BleedLimit", v) }

    // ---- 骞抽摵 ----

    pub fn print_tiled_pages(&self) -> Option<bool> { self.prop_bool("PrintTiledPages") }
    pub fn set_print_tiled_pages(&self, v: bool) -> bool {
        self.put_bool("PrintTiledPages", v)
    }

    pub fn print_tiling_marks(&self) -> Option<bool> { self.prop_bool("PrintTilingMarks") }
    pub fn set_print_tiling_marks(&self, v: bool) -> bool {
        self.put_bool("PrintTilingMarks", v)
    }

    pub fn tile_overlap(&self) -> Option<f64> { self.prop_f64("TileOverlap") }
    pub fn set_tile_overlap(&self, v: f64) -> bool { self.put_f64("TileOverlap", v) }

    // ---- 浣嶇疆 ----

    /// `PrnPlaceType`
    pub fn placement(&self) -> Option<i64> { self.prop_i64("Placement") }
    pub fn set_placement(&self, v: i32) -> bool { self.put_i64("Placement", v as i64) }
}
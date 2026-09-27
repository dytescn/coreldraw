//! `IVGStructHyphenationSettings` 鈥斺€?鏂瓧璁剧疆

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgStructHyphenationSettings {
    disp: ComObject,
}

impl IvgStructHyphenationSettings {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
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

    // ---- 鑷姩鏂瓧 ----

    pub fn use_automatic_hyphenation(&self) -> Option<bool> {
        self.prop_bool("UseAutomaticHyphenation")
    }
    pub fn set_use_automatic_hyphenation(&self, v: bool) -> bool {
        self.put_bool("UseAutomaticHyphenation", v)
    }

    // ---- 澶у啓鍗曡瘝 ----

    pub fn break_capitalized(&self) -> Option<bool> {
        self.prop_bool("BreakCapitalized")
    }
    pub fn set_break_capitalized(&self, v: bool) -> bool {
        self.put_bool("BreakCapitalized", v)
    }

    pub fn break_all_cap_words(&self) -> Option<bool> {
        self.prop_bool("BreakAllCapWords")
    }
    pub fn set_break_all_cap_words(&self, v: bool) -> bool {
        self.put_bool("BreakAllCapWords", v)
    }

    // ---- 鐑尯 / 闀垮害 ----

    pub fn hot_zone(&self) -> Option<f64> { self.prop_f64("HotZone") }
    pub fn set_hot_zone(&self, v: f64) -> bool { self.put_f64("HotZone", v) }

    pub fn min_word_length(&self) -> Option<i64> { self.prop_i64("MinWordLength") }
    pub fn set_min_word_length(&self, v: i32) -> bool {
        self.put_i64("MinWordLength", v as i64)
    }

    pub fn min_characters_before(&self) -> Option<i64> {
        self.prop_i64("MinCharactersBefore")
    }
    pub fn set_min_characters_before(&self, v: i32) -> bool {
        self.put_i64("MinCharactersBefore", v as i64)
    }

    pub fn min_characters_after(&self) -> Option<i64> {
        self.prop_i64("MinCharactersAfter")
    }
    pub fn set_min_characters_after(&self, v: i32) -> bool {
        self.put_i64("MinCharactersAfter", v as i64)
    }
}
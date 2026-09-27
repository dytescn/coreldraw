//! `IVGStructAlignProperties` 鈥斺€?娈佃惤瀵归綈灞炴€?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgStructAlignProperties {
    disp: ComObject,
}

impl IvgStructAlignProperties {
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

    fn prop_f32(&self, name: &str) -> Option<f32> {
        self.prop_f64(name).map(|v| v as f32)
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f32(&self, name: &str, v: f32) -> bool {
        let arg = Variant::from_f64(v as f64);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 瀵归綈 ----

    /// `cdrAlignment`
    pub fn alignment(&self) -> Option<i64> { self.prop_i64("Alignment") }
    pub fn set_alignment(&self, v: i32) -> bool { self.put_i64("Alignment", v as i64) }

    // ---- 缂╄繘 ----

    pub fn first_line_indent(&self) -> Option<f64> { self.prop_f64("FirstLineIndent") }
    pub fn set_first_line_indent(&self, v: f64) -> bool {
        self.put_f64("FirstLineIndent", v)
    }

    pub fn left_indent(&self) -> Option<f64> { self.prop_f64("LeftIndent") }
    pub fn set_left_indent(&self, v: f64) -> bool { self.put_f64("LeftIndent", v) }

    pub fn right_indent(&self) -> Option<f64> { self.prop_f64("RightIndent") }
    pub fn set_right_indent(&self, v: f64) -> bool { self.put_f64("RightIndent", v) }

    // ---- 璇嶉棿璺?----

    pub fn max_word_spacing(&self) -> Option<f32> { self.prop_f32("MaxWordSpacing") }
    pub fn set_max_word_spacing(&self, v: f32) -> bool {
        self.put_f32("MaxWordSpacing", v)
    }

    pub fn min_word_spacing(&self) -> Option<f32> { self.prop_f32("MinWordSpacing") }
    pub fn set_min_word_spacing(&self, v: f32) -> bool {
        self.put_f32("MinWordSpacing", v)
    }

    // ---- 瀛楃闂磋窛 ----

    pub fn max_character_spacing(&self) -> Option<f32> {
        self.prop_f32("MaxCharacterSpacing")
    }
    pub fn set_max_character_spacing(&self, v: f32) -> bool {
        self.put_f32("MaxCharacterSpacing", v)
    }

    // ---- 浣嶇Щ / 鏃嬭浆 ----

    pub fn horizontal_character_shift(&self) -> Option<i64> {
        self.prop_i64("HorizontalCharacterShift")
    }
    pub fn set_horizontal_character_shift(&self, v: i32) -> bool {
        self.put_i64("HorizontalCharacterShift", v as i64)
    }

    pub fn vertical_character_shift(&self) -> Option<i64> {
        self.prop_i64("VerticalCharacterShift")
    }
    pub fn set_vertical_character_shift(&self, v: i32) -> bool {
        self.put_i64("VerticalCharacterShift", v as i64)
    }

    pub fn character_rotation(&self) -> Option<f32> { self.prop_f32("CharacterRotation") }
    pub fn set_character_rotation(&self, v: f32) -> bool {
        self.put_f32("CharacterRotation", v)
    }
}
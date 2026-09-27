//! `IVGStructSpaceProperties` 鈥斺€?鏂囨湰闂磋窛灞炴€?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgStructSpaceProperties {
    disp: ComObject,
}

impl IvgStructSpaceProperties {
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

    fn prop_f32(&self, name: &str) -> Option<f32> {
        self.disp.get_property(name).ok()?.to_f64().ok().map(|v| v as f32)
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f32(&self, name: &str, v: f32) -> bool {
        let arg = Variant::from_f64(v as f64);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 瀛楃 / 璇嶉棿璺?----

    pub fn character_spacing(&self) -> Option<f32> { self.prop_f32("CharacterSpacing") }
    pub fn set_character_spacing(&self, v: f32) -> bool {
        self.put_f32("CharacterSpacing", v)
    }

    pub fn word_spacing(&self) -> Option<f32> { self.prop_f32("WordSpacing") }
    pub fn set_word_spacing(&self, v: f32) -> bool { self.put_f32("WordSpacing", v) }

    // ---- 琛岃窛 ----

    pub fn line_spacing(&self) -> Option<f32> { self.prop_f32("LineSpacing") }
    pub fn set_line_spacing(&self, v: f32) -> bool {
        self.put_f32("LineSpacing", v)
    }

    /// `cdrLineSpacingType`
    pub fn line_spacing_type(&self) -> Option<i64> { self.prop_i64("LineSpacingType") }
    pub fn set_line_spacing_type(&self, v: i32) -> bool {
        self.put_i64("LineSpacingType", v as i64)
    }

    // ---- 娈靛墠 / 娈靛悗 ----

    pub fn before_paragraph_spacing(&self) -> Option<f32> {
        self.prop_f32("BeforeParagraphSpacing")
    }
    pub fn set_before_paragraph_spacing(&self, v: f32) -> bool {
        self.put_f32("BeforeParagraphSpacing", v)
    }

    pub fn after_paragraph_spacing(&self) -> Option<f32> {
        self.prop_f32("AfterParagraphSpacing")
    }
    pub fn set_after_paragraph_spacing(&self, v: f32) -> bool {
        self.put_f32("AfterParagraphSpacing", v)
    }
}
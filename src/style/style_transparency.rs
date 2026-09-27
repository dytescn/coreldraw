//! `IVGStyleTransparency` 鈥斺€?鏍峰紡閲岀殑閫忔槑閮ㄥ垎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgStyleTransparency {
    disp: ComObject,
}

impl IvgStyleTransparency {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
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

    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.prop_dispatch("Style").map(crate::style::IvgStyle::new)
    }

    pub fn fill(&self) -> Option<crate::style::IvgStyleFill> {
        self.prop_dispatch("Fill").map(crate::style::IvgStyleFill::new)
    }

    /// `cdrMergeMode`
    pub fn mode(&self) -> Option<i64> { self.prop_i64("Mode") }
    pub fn set_mode(&self, v: i32) -> bool { self.put_i64("Mode", v as i64) }

    // ---- 涓嶉€忔槑搴?----

    pub fn uniform_transparency(&self) -> Option<f64> { self.prop_f64("UniformTransparency") }
    pub fn set_uniform_transparency(&self, v: f64) -> bool {
        self.put_f64("UniformTransparency", v)
    }

    pub fn white_transparency(&self) -> Option<f64> { self.prop_f64("WhiteTransparency") }
    pub fn set_white_transparency(&self, v: f64) -> bool {
        self.put_f64("WhiteTransparency", v)
    }

    pub fn black_transparency(&self) -> Option<f64> { self.prop_f64("BlackTransparency") }
    pub fn set_black_transparency(&self, v: f64) -> bool {
        self.put_f64("BlackTransparency", v)
    }

    // ---- 搴旂敤瀵硅薄 ----

    /// `cdrTransparencyAppliedTo`
    pub fn applies_to(&self) -> Option<i64> { self.prop_i64("AppliesTo") }
    pub fn set_applies_to(&self, v: i32) -> bool { self.put_i64("AppliesTo", v as i64) }
}
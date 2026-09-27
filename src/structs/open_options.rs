//! `IVGStructOpenOptions` 鈥斺€?鎵撳紑鏂囨。閫夐」

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::structs::IvgStructColorConversionOptions;

pub struct IvgStructOpenOptions {
    disp: ComObject,
}

impl IvgStructOpenOptions {
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 浠ｇ爜椤?----

    pub fn code_page(&self) -> Option<i64> { self.prop_i64("CodePage") }
    pub fn set_code_page(&self, v: i32) -> bool { self.put_i64("CodePage", v as i64) }

    // ---- 棰滆壊杞崲 ----

    pub fn color_conversion_options(&self) -> Option<IvgStructColorConversionOptions> {
        self.prop_dispatch("ColorConversionOptions")
            .map(IvgStructColorConversionOptions::new)
    }
}
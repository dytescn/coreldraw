//! `IVGStructPasteOptions` 鈥斺€?绮樿创閫夐」

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::structs::IvgStructColorConversionOptions;

pub struct IvgStructPasteOptions {
    disp: ComObject,
}

impl IvgStructPasteOptions {
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn color_conversion_options(&self) -> Option<IvgStructColorConversionOptions> {
        self.prop_dispatch("ColorConversionOptions")
            .map(IvgStructColorConversionOptions::new)
    }
}
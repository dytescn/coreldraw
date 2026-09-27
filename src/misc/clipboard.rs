//! `IVGClipboard` 鈥斺€?鍓创鏉?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgClipboard {
    disp: ComObject,
}

impl IvgClipboard {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    pub fn valid(&self) -> Option<bool> { self.prop_bool("Valid") }
    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("Empty") }

    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    pub fn data_present(&self, format_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(format_name.into())];
        self.disp
            .invoke_method("DataPresent", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
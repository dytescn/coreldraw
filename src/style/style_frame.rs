//! `IVGStyleFrame` 鈥斺€?鏍峰紡閲岀殑鏂囨湰妗嗛儴鍒?

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

pub struct IvgStyleFrame {
    disp: ComObject,
}

impl IvgStyleFrame {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.disp
            .get_property("Style")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::style::IvgStyle::new)
    }
}
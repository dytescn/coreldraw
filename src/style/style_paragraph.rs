//! `IVGStyleParagraph` 鈥斺€?鏍峰紡閲岀殑娈佃惤閮ㄥ垎

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

pub struct IvgStyleParagraph {
    disp: ComObject,
}

impl IvgStyleParagraph {
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
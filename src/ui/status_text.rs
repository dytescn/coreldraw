//! `ICUIStatusText` 鈥斺€?鐘舵€佹爮鏂囨湰

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::ICuiBitmapImage;

pub struct ICuiStatusText {
    disp: ComObject,
}

impl ICuiStatusText {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn set_caption_text(&self, text: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(text.into())];
        self.disp.invoke_method("SetCaptionText", args).is_ok()
    }

    pub fn set_bitmap(&self, bitmap: &ICuiBitmapImage) -> bool {
        let args = vec![bitmap.as_variant()];
        self.disp.invoke_method("SetBitmap", args).is_ok()
    }
}
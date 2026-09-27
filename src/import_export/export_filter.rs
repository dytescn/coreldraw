//! `ICorelExportFilter` 鈥斺€?瀵煎嚭杩囨护鍣?
//!
//! 鐢?`IVGBitmap::SaveAs` 绛?API 杩斿洖锛涜皟鐢ㄦ柟閫氳繃瀹冩帶鍒跺鍑鸿繃绋嬨€?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct ICorelExportFilter {
    disp: ComObject,
}

impl ICorelExportFilter {
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

    // ---------------------------------------------------------
    // 灞炴€?
    // ---------------------------------------------------------

    pub fn has_dialog(&self) -> Option<bool> {
        self.disp.get_property("HasDialog").ok()?.to_bool().ok()
    }

    // ---------------------------------------------------------
    // 鏂规硶
    // ---------------------------------------------------------

    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    pub fn finish(&self) -> bool {
        self.disp.invoke_method("Finish", vec![]).is_ok()
    }

    pub fn show_dialog(&self, h_wnd: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(h_wnd as i64)];
        self.disp
            .invoke_method("ShowDialog", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
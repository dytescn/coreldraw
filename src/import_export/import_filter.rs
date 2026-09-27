//! `ICorelImportFilter` 鈥斺€?瀵煎叆杩囨护鍣?
//!
//! 鐢?`IVGLayer::ImportEx` 杩斿洖锛涜皟鐢ㄦ柟閫氳繃瀹冩帶鍒跺鍏ヨ繃绋嬨€?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct ICorelImportFilter {
    disp: ComObject,
}

impl ICorelImportFilter {
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

    /// 閲嶇疆杩囨护鍣紙閲嶆柊寮€濮嬪鍏ユ祦绋嬶級銆?
    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    /// 瀹屾垚瀵煎叆锛堥噴鏀捐祫婧愩€佹彁浜ょ粨鏋滐級銆?
    pub fn finish(&self) -> bool {
        self.disp.invoke_method("Finish", vec![]).is_ok()
    }

    /// 鏄剧ず瀵煎叆璁剧疆瀵硅瘽妗嗐€?
    ///
    /// `hWnd` 鏄埗绐楀彛鍙ユ焺锛? 琛ㄧず鏃犵埗绐楀彛锛夈€?
    /// 杩斿洖 `Some(true)` 琛ㄧず鐢ㄦ埛鐐逛簡"纭畾"銆?
    pub fn show_dialog(&self, h_wnd: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(h_wnd as i64)];
        self.disp
            .invoke_method("ShowDialog", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
//! `IVGFontList` 鈥斺€?宸插畨瑁呭瓧浣撳垪琛?
use wincom::{ComObject, Variant};

pub struct IvgFontList {
    disp: ComObject,
}

impl IvgFontList {
    pub fn new(disp: windows::Win32::System::Com::IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 鎸?1-based 绱㈠紩鍙栧瓧浣撳悕銆?
    pub fn item(&self, index: i32) -> Option<String> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_string().ok()
    }

    /// 杩斿洖鎵€鏈夊瓧浣撳悕銆?
    pub fn all(&self) -> Vec<String> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }
}
//! `IVGURL` 鈥斺€?鍥惧舰涓婃寕鐨?URL / 涔︾

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgUrl {
    disp: ComObject,
}

impl IvgUrl {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn address(&self) -> Option<String> { self.prop_string("Address") }
    pub fn set_address(&self, v: impl Into<String>) -> bool {
        self.put_string("Address", v)
    }

    pub fn target_frame(&self) -> Option<String> { self.prop_string("TargetFrame") }
    pub fn set_target_frame(&self, v: impl Into<String>) -> bool {
        self.put_string("TargetFrame", v)
    }

    pub fn alt_comment(&self) -> Option<String> { self.prop_string("AltComment") }
    pub fn set_alt_comment(&self, v: impl Into<String>) -> bool {
        self.put_string("AltComment", v)
    }

    pub fn bookmark(&self) -> Option<String> { self.prop_string("BookMark") }
    pub fn set_bookmark(&self, v: impl Into<String>) -> bool {
        self.put_string("BookMark", v)
    }

    /// `cdrURLRegion`
    pub fn region(&self) -> Option<i64> { self.prop_i64("Region") }
    pub fn set_region(&self, v: i32) -> bool { self.put_i64("Region", v as i64) }
}
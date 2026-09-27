//! `IVGRulers` 鈥斺€?鏂囨。鏍囧昂

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgRulers {
    disp: ComObject,
}

impl IvgRulers {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 鍨傜洿鏍囧昂鍗曚綅锛坄cdrUnit`锛夈€?
    pub fn v_units(&self) -> Option<i64> { self.prop_i64("VUnits") }
    pub fn set_v_units(&self, v: i32) -> bool { self.put_i64("VUnits", v as i64) }

    /// 姘村钩鏍囧昂鍗曚綅銆?
    pub fn h_units(&self) -> Option<i64> { self.prop_i64("HUnits") }
    pub fn set_h_units(&self, v: i32) -> bool { self.put_i64("HUnits", v as i64) }
}
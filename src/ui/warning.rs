//! `ICUIWarning` 鈥斺€?璀﹀憡瀵硅瘽妗?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct ICuiWarning {
    disp: ComObject,
}

impl ICuiWarning {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn enabled(&self) -> Option<bool> { self.prop_bool("Enabled") }
    pub fn set_enabled(&self, v: bool) -> bool { self.put_bool("Enabled", v) }

    pub fn id(&self) -> Option<String> { self.prop_string("ID") }
    pub fn description(&self) -> Option<String> { self.prop_string("Description") }
    pub fn text(&self) -> Option<String> { self.prop_string("Text") }
    pub fn title(&self) -> Option<String> { self.prop_string("Title") }

    /// `unFlags` 鏄?`cuiMessageBoxFlags` 缁勫悎銆?
    pub fn do_warning_dialog(&self, flags: i32, text: impl Into<String>) -> Option<i64> {
        let args = vec![
            Variant::from_i64(flags as i64),
            Variant::from_str(text.into()),
        ];
        self.disp
            .invoke_method("DoWarningDialog", args)
            .ok()?
            .to_i64()
            .ok()
    }
}
//! `IPrnVBAPrinter` / `IPrnVBAPrinters` 鈥斺€?鎵撳嵃鏈?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPrnPrinters
// =============================================================

pub struct IvgPrnPrinters {
    disp: ComObject,
}

impl IvgPrnPrinters {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgPrnPrinter> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPrnPrinter::new)
    }

    pub fn default(&self) -> Option<IvgPrnPrinter> {
        self.prop_dispatch("Default").map(IvgPrnPrinter::new)
    }

    pub fn all(&self) -> Vec<IvgPrnPrinter> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }
}

// =============================================================
// IvgPrnPrinter
// =============================================================

pub struct IvgPrnPrinter {
    disp: ComObject,
}

impl IvgPrnPrinter {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 灞炴€?----

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn printer_type(&self) -> Option<String> { self.prop_string("Type") }
    pub fn is_default(&self) -> Option<bool> { self.prop_bool("Default") }
    pub fn ready(&self) -> Option<bool> { self.prop_bool("Ready") }
    pub fn port(&self) -> Option<String> { self.prop_string("Port") }
    pub fn description(&self) -> Option<String> { self.prop_string("Description") }
    pub fn postscript_enabled(&self) -> Option<bool> { self.prop_bool("PostScriptEnabled") }
    pub fn color_enabled(&self) -> Option<bool> { self.prop_bool("ColorEnabled") }

    pub fn page_size_matching_supported(&self) -> Option<bool> {
        self.prop_bool("PageSizeMatchingSupported")
    }
    pub fn set_page_size_matching_supported(&self, v: bool) -> bool {
        self.put_bool("PageSizeMatchingSupported", v)
    }

    /// 寮瑰嚭鎵撳嵃鏈哄睘鎬у璇濇銆?
    pub fn show_dialog(&self) -> bool {
        self.disp.invoke_method("ShowDialog", vec![]).is_ok()
    }
}
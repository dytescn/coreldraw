//! `IVGWorkspace(s)` 鈥斺€?宸ヤ綔鍖?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgWorkspaces
// =============================================================

pub struct IvgWorkspaces {
    disp: ComObject,
}

impl IvgWorkspaces {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgWorkspace> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgWorkspace::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgWorkspace> {
        self.item(Variant::from_i64(i as i64))
    }

    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgWorkspace> {
        self.item(Variant::from_str(name.into()))
    }
}

// =============================================================
// IvgWorkspace
// =============================================================

pub struct IvgWorkspace {
    disp: ComObject,
}

impl IvgWorkspace {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn description(&self) -> Option<String> { self.prop_string("Description") }
    pub fn active(&self) -> Option<bool> { self.prop_bool("Active") }

    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }
}
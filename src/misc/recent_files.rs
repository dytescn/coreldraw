//! `IVGRecentFile(s)` 鈥斺€?鏈€杩戞墦寮€鐨勬枃浠?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgRecentFiles
// =============================================================

pub struct IvgRecentFiles {
    disp: ComObject,
}

impl IvgRecentFiles {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }
    pub fn maximum(&self) -> Option<i64> { self.prop_i64("Maximum") }

    pub fn item(&self, index: i32) -> Option<IvgRecentFile> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRecentFile::new)
    }

    pub fn add(&self, name: impl Into<String>, path: impl Into<String>) -> Option<IvgRecentFile> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_str(path.into()),
        ];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRecentFile::new)
    }
}

// =============================================================
// IvgRecentFile
// =============================================================

pub struct IvgRecentFile {
    disp: ComObject,
}

impl IvgRecentFile {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    pub fn path(&self) -> Option<String> { self.prop_string("Path") }
    pub fn set_path(&self, v: impl Into<String>) -> bool { self.put_string("Path", v) }

    pub fn full_name(&self) -> Option<String> { self.prop_string("FullName") }
    pub fn set_full_name(&self, v: impl Into<String>) -> bool {
        self.put_string("FullName", v)
    }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    pub fn open(&self) -> Option<crate::document::IvgDocument> {
        self.disp
            .invoke_method("Open", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::document::IvgDocument::new)
    }
}
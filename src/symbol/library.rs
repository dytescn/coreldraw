//! `IVGSymbolLibrary` / `IVGSymbolLibraries` 鈥斺€?绗﹀彿搴?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::symbol::{IvgSymbolDefinition, IvgSymbolDefinitions};

// =============================================================
// IvgSymbolLibraries
// =============================================================

pub struct IvgSymbolLibraries {
    disp: ComObject,
}

impl IvgSymbolLibraries {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgSymbolLibrary> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSymbolLibrary::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgSymbolLibrary> {
        self.item(Variant::from_i64(i as i64))
    }

    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgSymbolLibrary> {
        self.item(Variant::from_str(name.into()))
    }

    /// 娣诲姞搴撴枃浠躲€?
    pub fn add(
        &self,
        file_name: impl Into<String>,
        copy_locally: bool,
    ) -> Option<IvgSymbolLibrary> {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_bool(copy_locally),
        ];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSymbolLibrary::new)
    }

    /// 浠庢枃浠跺す鎵归噺娣诲姞銆?
    pub fn add_from_folder(
        &self,
        folder: impl Into<String>,
        recursive: bool,
        copy_locally: bool,
    ) -> Option<i64> {
        let args = vec![
            Variant::from_str(folder.into()),
            Variant::from_bool(recursive),
            Variant::from_bool(copy_locally),
        ];
        self.disp
            .invoke_method("AddFromFolder", args)
            .ok()?
            .to_i64()
            .ok()
    }

    pub fn all(&self) -> Vec<IvgSymbolLibrary> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item_by_index(i)).collect()
    }
}

// =============================================================
// IvgSymbolLibrary
// =============================================================

pub struct IvgSymbolLibrary {
    disp: ComObject,
}

impl IvgSymbolLibrary {
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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---- 鍩烘湰淇℃伅 ----

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn file_path(&self) -> Option<String> { self.prop_string("FilePath") }
    pub fn read_only(&self) -> Option<bool> { self.prop_bool("ReadOnly") }

    /// 搴撳唴鎵€鏈夌鍙峰畾涔夈€?
    pub fn symbols(&self) -> Option<IvgSymbolDefinitions> {
        self.prop_dispatch("Symbols").map(IvgSymbolDefinitions::new)
    }

    // ---- 鎿嶄綔 ----

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    /// 娓呮帀鏈寮曠敤鐨勭鍙枫€?
    pub fn purge_unused_symbols(&self) -> bool {
        self.disp.invoke_method("PurgeUnusedSymbols", vec![]).is_ok()
    }

    /// 鎶婄鍙风矘璐村埌鏈簱銆?
    pub fn paste(&self, name: impl Into<String>) -> Option<IvgSymbolDefinition> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("Paste", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSymbolDefinition::new)
    }
}
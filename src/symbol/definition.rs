//! `IVGSymbolDefinition(s)` 鈥斺€?绗﹀彿瀹氫箟

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::IvgShapeRange;

// =============================================================
// IvgSymbolDefinitions
// =============================================================

pub struct IvgSymbolDefinitions {
    disp: ComObject,
}

impl IvgSymbolDefinitions {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgSymbolDefinition> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSymbolDefinition::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgSymbolDefinition> {
        self.item(Variant::from_i64(i as i64))
    }

    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgSymbolDefinition> {
        self.item(Variant::from_str(name.into()))
    }

    pub fn all(&self) -> Vec<IvgSymbolDefinition> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item_by_index(i)).collect()
    }
}

// =============================================================
// IvgSymbolDefinition
// =============================================================

pub struct IvgSymbolDefinition {
    disp: ComObject,
}

impl IvgSymbolDefinition {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 鍩烘湰淇℃伅
    // ---------------------------------------------------------

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    pub fn description(&self) -> Option<String> { self.prop_string("Description") }
    pub fn set_description(&self, v: impl Into<String>) -> bool {
        self.put_string("Description", v)
    }

    /// `cdrSymbolType`
    pub fn symbol_type(&self) -> Option<i64> { self.prop_i64("Type") }

    // ---------------------------------------------------------
    // 閾炬帴
    // ---------------------------------------------------------

    pub fn linked(&self) -> Option<bool> { self.prop_bool("Linked") }
    pub fn link_library_path(&self) -> Option<String> {
        self.prop_string("LinkLibraryPath")
    }
    pub fn is_link_broken(&self) -> Option<bool> { self.prop_bool("IsLinkBroken") }
    pub fn is_link_updated(&self) -> Option<bool> { self.prop_bool("IsLinkUpdated") }
    pub fn has_broken_links(&self) -> Option<bool> { self.prop_bool("HasBrokenLinks") }
    pub fn has_updated_links(&self) -> Option<bool> { self.prop_bool("HasUpdatedLinks") }

    // ---------------------------------------------------------
    // 宓屽
    // ---------------------------------------------------------

    pub fn nested(&self) -> Option<bool> { self.prop_bool("Nested") }
    pub fn has_links(&self) -> Option<bool> { self.prop_bool("HasLinks") }

    /// 宓屽鐨勫瓙绗﹀彿瀹氫箟銆?
    pub fn nested_symbols(&self) -> Option<IvgSymbolDefinitions> {
        self.prop_dispatch("NestedSymbols").map(IvgSymbolDefinitions::new)
    }

    // ---------------------------------------------------------
    // 鍙紪杈?/ 瀹炰緥
    // ---------------------------------------------------------

    pub fn editable(&self) -> Option<bool> { self.prop_bool("Editable") }

    pub fn instance_count(&self) -> Option<i64> { self.prop_i64("InstanceCount") }

    /// 鎵€鏈夊疄渚嬶紙鍥惧舰鑼冨洿锛夈€?
    pub fn instances(&self) -> Option<IvgShapeRange> {
        self.prop_dispatch("Instances").map(IvgShapeRange::new)
    }

    // ---------------------------------------------------------
    // 缂栬緫 / 澶嶅埗 / 鍒犻櫎
    // ---------------------------------------------------------

    pub fn enter_edit_mode(&self) -> bool {
        self.disp.invoke_method("EnterEditMode", vec![]).is_ok()
    }

    pub fn leave_edit_mode(&self) -> bool {
        self.disp.invoke_method("LeaveEditMode", vec![]).is_ok()
    }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    pub fn copy(&self) -> bool {
        self.disp.invoke_method("Copy", vec![]).is_ok()
    }

    pub fn duplicate(&self, name: impl Into<String>) -> Option<IvgSymbolDefinition> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("Duplicate", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSymbolDefinition::new)
    }

    // ---------------------------------------------------------
    // 閾炬帴缁存姢
    // ---------------------------------------------------------

    pub fn break_link(&self) -> bool {
        self.disp.invoke_method("BreakLink", vec![]).is_ok()
    }

    pub fn update_links(&self) -> bool {
        self.disp.invoke_method("UpdateLinks", vec![]).is_ok()
    }

    pub fn fix_link(&self, library_path: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(library_path.into())];
        self.disp.invoke_method("FixLink", args).is_ok()
    }
}
//! `IVGPalette(s)` / `IVGPaletteManager` 鈥斺€?璋冭壊鏉?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;

// =============================================================
// IvgPalettes
// =============================================================

pub struct IvgPalettes {
    disp: ComObject,
}

impl IvgPalettes {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgPalette> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }

    pub fn open(&self, file_name: impl Into<String>) -> Option<IvgPalette> {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp
            .invoke_method("Open", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }

    pub fn open_fixed(&self, palette_id: i32) -> Option<IvgPalette> {
        let args = vec![Variant::from_i64(palette_id as i64)];
        self.disp
            .invoke_method("OpenFixed", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }
}

// =============================================================
// IvgPalette
// =============================================================

pub struct IvgPalette {
    disp: ComObject,
}

impl IvgPalette {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property("Name", vec![arg]).is_ok()
    }

    pub fn palette_id(&self) -> Option<i64> { self.prop_i64("PaletteID") }
    pub fn file_name(&self) -> Option<String> { self.prop_string("FileName") }
    pub fn identifier(&self) -> Option<String> { self.prop_string("Identifier") }
    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("IsEmpty") }
    pub fn locked(&self) -> Option<bool> { self.prop_bool("Locked") }
    pub fn is_default(&self) -> Option<bool> { self.prop_bool("Default") }
    pub fn is_open(&self) -> Option<bool> { self.prop_bool("IsOpen") }

    pub fn close(&self) -> bool {
        self.disp.invoke_method("Close", vec![]).is_ok()
    }

    pub fn open(&self) -> bool {
        self.disp.invoke_method("Open", vec![]).is_ok()
    }

    pub fn save(&self) -> bool {
        self.disp.invoke_method("Save", vec![]).is_ok()
    }

    pub fn make_default(&self) -> bool {
        self.disp.invoke_method("MakeDefault", vec![]).is_ok()
    }

    pub fn delete(&self) -> Option<bool> {
        self.disp.invoke_method("Delete", vec![]).ok()?.to_bool().ok()
    }

    /// `Index` 涓?1-based銆?
    pub fn color(&self, index: i32) -> Option<IvgColor> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Color", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColor::new)
    }

    pub fn set_color(&self, index: i32, color: &IvgColor) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            color.as_variant(),
        ];
        self.disp.invoke_method("put_Color", args).is_ok()
    }

    pub fn add_color(&self, color: &IvgColor) -> bool {
        let args = vec![color.as_variant()];
        self.disp.invoke_method("AddColor", args).is_ok()
    }

    pub fn insert_color(&self, index: i32, color: &IvgColor) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            color.as_variant(),
        ];
        self.disp.invoke_method("InsertColor", args).is_ok()
    }

    pub fn remove_color(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("RemoveColor", args).is_ok()
    }

    pub fn color_count(&self) -> Option<i64> {
        self.disp.get_property("ColorCount").ok()?.to_i64().ok()
    }

    pub fn match_color(&self, color: &IvgColor) -> Option<i64> {
        let args = vec![color.as_variant()];
        self.disp.invoke_method("MatchColor", args).ok()?.to_i64().ok()
    }

    pub fn find_color(&self, name: impl Into<String>) -> Option<i64> {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("FindColor", args).ok()?.to_i64().ok()
    }
}

// =============================================================
// IvgPaletteManager
// =============================================================

pub struct IvgPaletteManager {
    disp: ComObject,
}

impl IvgPaletteManager {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn palette_count(&self) -> Option<i64> {
        self.disp.get_property("PaletteCount").ok()?.to_i64().ok()
    }

    pub fn get_palette(&self, index_or_name: Variant) -> Option<IvgPalette> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("GetPalette", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }

    pub fn default_palette(&self) -> Option<IvgPalette> {
        self.disp
            .get_property("DefaultPalette")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }

    pub fn open_palettes(&self) -> Option<IvgPalettes> {
        self.disp
            .get_property("OpenPalettes")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalettes::new)
    }

    pub fn load_all_palettes(&self) -> bool {
        self.disp.invoke_method("LoadAllPalettes", vec![]).is_ok()
    }

    pub fn load_palette(&self, file_name: impl Into<String>) -> Option<IvgPalette> {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp
            .invoke_method("LoadPalette", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }
}
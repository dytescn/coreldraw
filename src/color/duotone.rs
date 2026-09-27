//! `IVGDuotone` / `IVGDuotoneInk` / `IVGDuotoneOverprint` 鈥斺€?鍙岃壊濂楀嵃

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::color::IvgColor;

// =============================================================
// IvgDuotoneOverprint
// =============================================================

pub struct IvgDuotoneOverprint {
    disp: ComObject,
}

impl IvgDuotoneOverprint {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn cyan(&self) -> Option<i64> { self.prop_i64("Cyan") }
    pub fn set_cyan(&self, v: i32) -> bool { self.put_i64("Cyan", v as i64) }

    pub fn magenta(&self) -> Option<i64> { self.prop_i64("Magenta") }
    pub fn set_magenta(&self, v: i32) -> bool { self.put_i64("Magenta", v as i64) }

    pub fn yellow(&self) -> Option<i64> { self.prop_i64("Yellow") }
    pub fn set_yellow(&self, v: i32) -> bool { self.put_i64("Yellow", v as i64) }

    pub fn black(&self) -> Option<i64> { self.prop_i64("Black") }
    pub fn set_black(&self, v: i32) -> bool { self.put_i64("Black", v as i64) }

    pub fn color(&self) -> Option<IvgColor> {
        self.disp.get_property("Color").ok()?.to_idispatch().ok().map(IvgColor::new)
    }

    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    pub fn set_values(&self, c: i32, m: i32, y: i32, k: i32) -> bool {
        let args = vec![
            Variant::from_i64(c as i64),
            Variant::from_i64(m as i64),
            Variant::from_i64(y as i64),
            Variant::from_i64(k as i64),
        ];
        self.disp.invoke_method("SetValues", args).is_ok()
    }
}

// =============================================================
// IvgDuotoneInk
// =============================================================

pub struct IvgDuotoneInk {
    disp: ComObject,
}

impl IvgDuotoneInk {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn color(&self) -> Option<IvgColor> {
        self.disp.get_property("Color").ok()?.to_idispatch().ok().map(IvgColor::new)
    }

    pub fn handle_count(&self) -> Option<i64> { self.prop_i64("HandleCount") }

    pub fn handle_x(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("HandleX", args).ok()?.to_i64().ok()
    }

    pub fn set_handle_x(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_HandleX", args).is_ok()
    }

    pub fn handle_y(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("HandleY", args).ok()?.to_i64().ok()
    }

    pub fn set_handle_y(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_HandleY", args).is_ok()
    }

    pub fn add_handle(&self, x: i32, y: i32) -> Option<i64> {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
        ];
        self.disp.invoke_method("AddHandle", args).ok()?.to_i64().ok()
    }

    pub fn remove_handle(&self, index: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("RemoveHandle", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn find_handle(&self, x: i32, y: i32) -> Option<i64> {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
        ];
        self.disp.invoke_method("FindHandle", args).ok()?.to_i64().ok()
    }

    pub fn curve_level(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("CurveLevel", args).ok()?.to_i64().ok()
    }

    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    pub fn load(&self, file: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file.into())];
        self.disp.invoke_method("Load", args).ok()?.to_bool().ok()
    }

    pub fn save(&self, file: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file.into())];
        self.disp.invoke_method("Save", args).ok()?.to_bool().ok()
    }
}

// =============================================================
// IvgDuotone
// =============================================================

pub struct IvgDuotone {
    disp: ComObject,
}

impl IvgDuotone {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn duotone_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_duotone_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn use_overprints(&self) -> Option<bool> { self.prop_bool("UseOverprints") }
    pub fn set_use_overprints(&self, v: bool) -> bool { self.put_bool("UseOverprints", v) }

    pub fn overprint_count(&self) -> Option<i64> { self.prop_i64("OverprintCount") }

    pub fn overprints(&self, index: i32) -> Option<IvgDuotoneOverprint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Overprints", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDuotoneOverprint::new)
    }

    pub fn ink_count(&self) -> Option<i64> { self.prop_i64("InkCount") }

    pub fn inks(&self, index: i32) -> Option<IvgDuotoneInk> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Inks", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDuotoneInk::new)
    }

    pub fn get_copy(&self) -> Option<IvgDuotone> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDuotone::new)
    }

    pub fn copy_assign(&self, other: &IvgDuotone) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    pub fn reset_overprints(&self) -> bool {
        self.disp.invoke_method("ResetOverprints", vec![]).is_ok()
    }

    pub fn load(&self, file: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file.into())];
        self.disp.invoke_method("Load", args).ok()?.to_bool().ok()
    }

    pub fn save(&self, file: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file.into())];
        self.disp.invoke_method("Save", args).ok()?.to_bool().ok()
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }
}
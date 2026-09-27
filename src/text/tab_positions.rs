//! `IVGTextTabPosition(s)` 鈥斺€?鍒惰〃浣?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgTextTabPositions
// =============================================================

pub struct IvgTextTabPositions {
    disp: ComObject,
}

impl IvgTextTabPositions {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgTextTabPosition> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextTabPosition::new)
    }

    /// 鍓嶅绗︼紙leadered锛夐棿闅斻€?
    pub fn leader_spacing(&self) -> Option<i64> { self.prop_i64("LeaderSpacing") }
    pub fn set_leader_spacing(&self, v: i32) -> bool {
        self.put_i64("LeaderSpacing", v as i64)
    }

    pub fn leader_character(&self) -> Option<String> { self.prop_string("LeaderCharacter") }
    pub fn set_leader_character(&self, v: impl Into<String>) -> bool {
        self.put_string("LeaderCharacter", v)
    }

    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    /// 娣诲姞鍒惰〃浣嶃€?
    pub fn add(
        &self,
        position: f64,
        alignment: i32,
        leadered: bool,
    ) -> Option<IvgTextTabPosition> {
        let args = vec![
            Variant::from_f64(position),
            Variant::from_i64(alignment as i64),
            Variant::from_bool(leadered),
        ];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextTabPosition::new)
    }

    /// 绛夐棿闅旀坊鍔犮€?
    pub fn add_every(
        &self,
        position: f64,
        alignment: i32,
        leadered: bool,
    ) -> bool {
        let args = vec![
            Variant::from_f64(position),
            Variant::from_i64(alignment as i64),
            Variant::from_bool(leadered),
        ];
        self.disp.invoke_method("AddEvery", args).is_ok()
    }
}

// =============================================================
// IvgTextTabPosition
// =============================================================

pub struct IvgTextTabPosition {
    disp: ComObject,
}

impl IvgTextTabPosition {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn position(&self) -> Option<f64> { self.prop_f64("Position") }
    pub fn set_position(&self, v: f64) -> bool { self.put_f64("Position", v) }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// `cdrTextTabAlignment`
    pub fn alignment(&self) -> Option<i64> { self.prop_i64("Alignment") }
    pub fn set_alignment(&self, v: i32) -> bool { self.put_i64("Alignment", v as i64) }

    pub fn leadered(&self) -> Option<bool> { self.prop_bool("Leadered") }
    pub fn set_leadered(&self, v: bool) -> bool { self.put_bool("Leadered", v) }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}
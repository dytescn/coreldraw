//! `IVGStyles` 鈥斺€?鏍峰紡闆嗗悎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::style::IvgStyle;

pub struct IvgStyles {
    disp: ComObject,
}

impl IvgStyles {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgStyle> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStyle::new)
    }

    pub fn find(&self, name: impl Into<String>) -> Option<IvgStyle> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStyle::new)
    }

    pub fn first(&self) -> Option<IvgStyle> {
        self.prop_dispatch("First").map(IvgStyle::new)
    }

    pub fn last(&self) -> Option<IvgStyle> {
        self.prop_dispatch("Last").map(IvgStyle::new)
    }

    pub fn all(&self) -> Vec<IvgStyle> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }
}
//! `IVGTextCharacters` 鈥斺€?瀛楃瑙嗗浘

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::text::IvgTextRange;

pub struct IvgTextCharacters {
    disp: ComObject,
}

impl IvgTextCharacters {
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

    /// 鍙栦粠 index 寮€濮嬬殑 count 涓瓧绗︺€?
    pub fn item(&self, index: i32, count: i32) -> Option<IvgTextRange> {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(count as i64),
        ];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    pub fn all(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("All").map(IvgTextRange::new)
    }

    pub fn first(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("First").map(IvgTextRange::new)
    }

    pub fn last(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("Last").map(IvgTextRange::new)
    }
}
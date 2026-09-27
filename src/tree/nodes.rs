//! `IVGTreeNodes` 鈥斺€?瀛愯妭鐐归泦鍚?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::tree::IvgTreeNode;

pub struct IvgTreeNodes {
    disp: ComObject,
}

impl IvgTreeNodes {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 1-based 绱㈠紩銆?
    pub fn item(&self, index: i32) -> Option<IvgTreeNode> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTreeNode::new)
    }

    /// 閬嶅巻鎵€鏈夎妭鐐广€?
    pub fn all(&self) -> Vec<IvgTreeNode> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }
}
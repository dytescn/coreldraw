//! `IVGViews` 鈥斺€?鍛藉悕瑙嗗浘闆嗗悎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::view::IvgView;

pub struct IvgViews {
    disp: ComObject,
}

impl IvgViews {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgView> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgView::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgView> {
        self.item(Variant::from_i64(i as i64))
    }

    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgView> {
        self.item(Variant::from_str(name.into()))
    }

    /// 浠庡綋鍓嶆椿鍔ㄨ鍥惧垱寤轰竴涓懡鍚嶈鍥俱€?
    pub fn add_active_view(&self, name: impl Into<String>) -> Option<IvgView> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("AddActiveView", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgView::new)
    }

    pub fn all(&self) -> Vec<IvgView> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item_by_index(i)).collect()
    }
}
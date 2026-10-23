//! `IVGPages` 鈥斺€?椤甸潰闆嗗悎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::page::IvgPage;

pub struct IvgPages {
    disp: ComObject,
}

impl IvgPages {
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

    // ---------------------------------------------------------
    // 基本信息
    // ---------------------------------------------------------

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    pub fn application(&self) -> Option<crate::app::IvgApplication> {
        self.disp
            .get_property("Application")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::app::IvgApplication::from_disp)
    }

    // ---------------------------------------------------------
    // 索引访问
    // ---------------------------------------------------------

    pub fn item(&self, index: i32) -> Option<IvgPage> {
        // `Item` 在类型库里可能注册为带参数的属性，也可能是方法。
        let v = self
            .disp
            .get_property_with("Item", vec![Variant::from_i64(index as i64)])
            .or_else(|_| self.disp.invoke_method("Item", vec![Variant::from_i64(index as i64)]))
            .ok()?;
        v.to_idispatch().ok().map(IvgPage::new)
    }

    pub fn first(&self) -> Option<IvgPage> {
        self.disp
            .get_property("First")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPage::new)
    }

    pub fn last(&self) -> Option<IvgPage> {
        self.disp
            .get_property("Last")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPage::new)
    }

    /// 閬嶅巻鎵€鏈夐〉闈€?
    pub fn all(&self) -> Vec<IvgPage> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }
}
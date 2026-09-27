//! `IVGPageSize` / `IVGPageSizes` 鈥斺€?鍛藉悕椤甸潰灏哄

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPageSizes
// =============================================================

pub struct IvgPageSizes {
    disp: ComObject,
}

impl IvgPageSizes {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgPageSize> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPageSize::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgPageSize> {
        self.item(Variant::from_i64(i as i64))
    }

    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgPageSize> {
        self.item(Variant::from_str(name.into()))
    }

    /// 娣诲姞鑷畾涔夐〉闈㈠昂瀵搞€?
    pub fn add(
        &self,
        name: impl Into<String>,
        width: f64,
        height: f64,
    ) -> Option<IvgPageSize> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_f64(width),
            Variant::from_f64(height),
        ];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPageSize::new)
    }

    /// 閬嶅巻鎵€鏈夈€?
    pub fn all(&self) -> Vec<IvgPageSize> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item_by_index(i)).collect()
    }
}

// =============================================================
// IvgPageSize
// =============================================================

pub struct IvgPageSize {
    disp: ComObject,
}

impl IvgPageSize {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
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

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    pub fn height(&self) -> Option<f64> { self.prop_f64("Height") }
    pub fn set_height(&self, v: f64) -> bool { self.put_f64("Height", v) }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    pub fn built_in(&self) -> Option<bool> { self.prop_bool("BuiltIn") }
    pub fn fixed_orientation(&self) -> Option<bool> { self.prop_bool("FixedOrientation") }

    /// `cdrUnit` 鈥斺€?璇ュ昂瀵告墍鐢ㄥ崟浣嶃€?
    pub fn default_unit(&self) -> Option<i64> { self.prop_i64("DefaultUnit") }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}
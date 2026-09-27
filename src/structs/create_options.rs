//! `IVGStructCreateOptions` 鈥斺€?鏂板缓鏂囨。閫夐」

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColorContext;

pub struct IvgStructCreateOptions {
    disp: ComObject,
}

impl IvgStructCreateOptions {
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

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---- 棰滆壊涓婁笅鏂?----

    pub fn color_context(&self) -> Option<IvgColorContext> {
        self.prop_dispatch("ColorContext").map(IvgColorContext::new)
    }
    pub fn set_color_context(&self, c: &IvgColorContext) -> bool {
        self.put_dispatch("ColorContext", c.as_variant())
    }

    // ---- 鏂囨。鍚?----

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool {
        self.put_string("Name", v)
    }

    // ---- 椤甸潰灏哄 ----

    pub fn page_width(&self) -> Option<f64> { self.prop_f64("PageWidth") }
    pub fn set_page_width(&self, v: f64) -> bool { self.put_f64("PageWidth", v) }

    pub fn page_height(&self) -> Option<f64> { self.prop_f64("PageHeight") }
    pub fn set_page_height(&self, v: f64) -> bool { self.put_f64("PageHeight", v) }

    /// `cdrUnit`
    pub fn units(&self) -> Option<i64> { self.prop_i64("Units") }
    pub fn set_units(&self, v: i32) -> bool { self.put_i64("Units", v as i64) }

    pub fn resolution(&self) -> Option<f64> { self.prop_f64("Resolution") }
    pub fn set_resolution(&self, v: f64) -> bool { self.put_f64("Resolution", v) }
}
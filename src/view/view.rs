//! `IVGView` 鈥斺€?鍛藉悕瑙嗗浘
//!
//! 淇濆瓨鍦ㄦ枃妗ｉ噷锛岄€氳繃 `document.views()` 璁块棶銆?
//! 涓?[`IvgActiveView`]锛堝綋鍓嶆椿鍔ㄨ鍥撅級涓嶅悓锛屽懡鍚嶈鍥炬槸鎸佷箙鍖栫殑銆?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::page::IvgPage;

pub struct IvgView {
    disp: ComObject,
}

impl IvgView {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
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

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---------------------------------------------------------
    // 鍚嶇О / 鍘熺偣 / 缂╂斁
    // ---------------------------------------------------------

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    /// 鏄惁浣跨敤椤甸潰瑙嗗浘锛坄UsePage = true` 鏃朵笌椤甸潰缁戝畾锛夈€?
    pub fn use_page(&self) -> Option<bool> { self.prop_bool("UsePage") }
    pub fn set_use_page(&self, v: bool) -> bool { self.put_bool("UsePage", v) }

    pub fn page(&self) -> Option<IvgPage> {
        self.prop_dispatch("Page").map(IvgPage::new)
    }
    pub fn set_page(&self, p: &IvgPage) -> bool {
        self.put_dispatch("Page", p.as_variant())
    }

    /// 鏄惁浣跨敤鑷畾涔夌缉鏀撅紙`false` 鏃惰嚜鍔ㄩ€傞厤锛夈€?
    pub fn use_zoom(&self) -> Option<bool> { self.prop_bool("UseZoom") }
    pub fn set_use_zoom(&self, v: bool) -> bool { self.put_bool("UseZoom", v) }

    pub fn zoom(&self) -> Option<f64> { self.prop_f64("Zoom") }
    pub fn set_zoom(&self, v: f64) -> bool { self.put_f64("Zoom", v) }

    // ---------------------------------------------------------
    // 鎿嶄綔
    // ---------------------------------------------------------

    /// 婵€娲昏瑙嗗浘锛堝垏鎹㈡椿鍔ㄧ獥鍙ｅ埌鏈鍥撅級銆?
    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}
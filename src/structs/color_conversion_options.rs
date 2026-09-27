//! `IVGStructColorConversionOptions` 鈥斺€?棰滆壊杞崲閫夐」

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColorManagementPolicy;

pub struct IvgStructColorConversionOptions {
    disp: ComObject,
}

impl IvgStructColorConversionOptions {
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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 棰滆壊绛栫暐 ----

    pub fn color_policy(&self) -> Option<IvgColorManagementPolicy> {
        self.prop_dispatch("ColorPolicy").map(IvgColorManagementPolicy::new)
    }

    // ---- 婧?/ 鐩爣 profile ----

    pub fn source_color_profile_list(&self) -> Option<String> {
        self.prop_string("SourceColorProfileList")
    }
    pub fn set_source_color_profile_list(&self, v: impl Into<String>) -> bool {
        self.put_string("SourceColorProfileList", v)
    }

    pub fn target_color_profile_list(&self) -> Option<String> {
        self.prop_string("TargetColorProfileList")
    }
    pub fn set_target_color_profile_list(&self, v: impl Into<String>) -> bool {
        self.put_string("TargetColorProfileList", v)
    }

    // ---- 杞崲鍥炶皟 ----

    /// `IColorConversionHandler` 鐨?dispatch锛堢敤鎴峰疄鐜帮級銆?
    pub fn color_conversion_handler(&self) -> Option<IDispatch> {
        self.prop_dispatch("ColorConversionHandler")
    }

    pub fn set_color_conversion_handler(&self, h: IDispatch) -> bool {
        let arg = Variant::from_dispatch(&h);
        self.disp.set_property("ColorConversionHandler", vec![arg]).is_ok()
    }
}
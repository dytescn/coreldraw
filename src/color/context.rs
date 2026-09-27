//! `IVGColorContext` 鈥斺€?棰滆壊涓婁笅鏂囷紙RGB / CMYK / 鐏板害 profile + 娓叉煋鎰忓浘锛?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::manager::{IvgColorProfile, IvgColorProfiles};

pub struct IvgColorContext {
    disp: ComObject,
}

impl IvgColorContext {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_dispatch(&self, name: &str, disp: &IDispatch) -> bool {
        let arg = Variant::from_dispatch(disp);

        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 鍚勬ā寮忕殑 Profile
    // ---------------------------------------------------------

    pub fn rgb_color_profile(&self) -> Option<IvgColorProfile> {
        self.prop_dispatch("RGBColorProfile").map(IvgColorProfile::new)
    }

    pub fn set_rgb_color_profile(&self, p: &IvgColorProfile) -> bool {
        self.put_dispatch("RGBColorProfile", p.dispatch())
    }

    pub fn cmyk_color_profile(&self) -> Option<IvgColorProfile> {
        self.prop_dispatch("CMYKColorProfile").map(IvgColorProfile::new)
    }

    pub fn set_cmyk_color_profile(&self, p: &IvgColorProfile) -> bool {
        self.put_dispatch("CMYKColorProfile", p.dispatch())
    }

    pub fn grayscale_color_profile(&self) -> Option<IvgColorProfile> {
        self.prop_dispatch("GrayscaleColorProfile").map(IvgColorProfile::new)
    }

    pub fn set_grayscale_color_profile(&self, p: &IvgColorProfile) -> bool {
        self.put_dispatch("GrayscaleColorProfile", p.dispatch())
    }

    /// `clrColorModel` 鏋氫妇鍊?
    pub fn color_profile(&self, model: i32) -> Option<IvgColorProfile> {
        let args = vec![Variant::from_i64(model as i64)];
        self.disp
            .invoke_method("ColorProfile", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfile::new)
    }

    // ---------------------------------------------------------
    // 娓叉煋鎰忓浘 / 娣峰悎妯″紡
    // ---------------------------------------------------------

    /// `clrRenderingIntent`
    pub fn rendering_intent(&self) -> Option<i64> {
        self.prop_i64("RenderingIntent")
    }
    pub fn set_rendering_intent(&self, v: i32) -> bool {
        self.put_i64("RenderingIntent", v as i64)
    }

    /// `clrColorModel`
    pub fn blending_color_model(&self) -> Option<i64> {
        self.prop_i64("BlendingColorModel")
    }
    pub fn set_blending_color_model(&self, v: i32) -> bool {
        self.put_i64("BlendingColorModel", v as i64)
    }

    // ---------------------------------------------------------
    // 澶嶅埗 / 姣旇緝
    // ---------------------------------------------------------

    pub fn get_copy(&self) -> Option<IvgColorContext> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorContext::new)
    }

    pub fn copy_assign(&self, other: &IvgColorContext) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    pub fn is_same(&self, other: &IvgColorContext) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("IsSame", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn merge(&self, other: &IvgColorContext) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("Merge", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 鍏跺畠
    // ---------------------------------------------------------

    pub fn color_profiles(&self) -> Option<IvgColorProfiles> {
        self.prop_dispatch("ColorProfiles").map(IvgColorProfiles::new)
    }

    pub fn read_only(&self) -> Option<bool> {
        self.prop_bool("ReadOnly")
    }

    pub fn color_profile_name_list(&self) -> Option<String> {
        self.prop_string("ColorProfileNameList")
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn dispatch(&self) -> &IDispatch {
        // 鑻?ComObject 涓嶇洿鎺ユ毚闇?IDispatch锛岄渶瑕佽皟鏁?
        unimplemented!("鎸?wincom 鐨勫疄闄?API 璋冩暣锛氳繑鍥炲唴閮?IDispatch")
    }
}
//! 棰滆壊绠＄悊鍣?& Profile & 绠＄悊绛栫暐

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::color::IvgColor;
use crate::color::context::IvgColorContext;

// =============================================================
// IvgColorProfile
// =============================================================

pub struct IvgColorProfile {
    disp: ComObject,
}

impl IvgColorProfile {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn file_name(&self) -> Option<String> { self.prop_string("FileName") }
    pub fn manufacturer(&self) -> Option<String> { self.prop_string("Manufacturer") }
    pub fn device_model(&self) -> Option<String> { self.prop_string("DeviceModel") }
    pub fn device_type(&self) -> Option<i64> { self.prop_i64("DeviceType") }
    pub fn selected(&self) -> Option<bool> { self.prop_bool("Selected") }
    pub fn generic(&self) -> Option<bool> { self.prop_bool("Generic") }
    pub fn installed(&self) -> Option<bool> { self.prop_bool("Installed") }
    pub fn id(&self) -> Option<String> { self.prop_string("ID") }
    pub fn color_model(&self) -> Option<i64> { self.prop_i64("ColorModel") }

    pub fn select(&self) -> bool {
        self.disp.invoke_method("Select", vec![]).is_ok()
    }

    /// 鐢ㄥ綋鍓?profile + 娓叉煋鎰忓浘鍒涘缓 ColorContext
    pub fn create_color_context(&self, rendering_intent: i32) -> Option<IvgColorContext> {
        let args = vec![Variant::from_i64(rendering_intent as i64)];
        self.disp
            .invoke_method("CreateColorContext", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorContext::new)
    }

    pub fn is_same(&self, other: &IvgColorProfile) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("IsSame", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn dispatch(&self) -> &IDispatch {
        unimplemented!("鎸?wincom 瀹為檯 API 璋冩暣")
    }
}

// =============================================================
// IvgColorProfiles
// =============================================================

pub struct IvgColorProfiles {
    disp: ComObject,
}

impl IvgColorProfiles {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgColorProfile> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfile::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgColorProfile> {
        self.item(Variant::from_i64(i as i64))
    }

    pub fn item_by_name(&self, s: impl Into<String>) -> Option<IvgColorProfile> {
        self.item(Variant::from_str(s.into()))
    }

    pub fn device_type(&self) -> Option<i64> {
        self.disp.get_property("DeviceType").ok()?.to_i64().ok()
    }

    pub fn generic_profile(&self) -> Option<IvgColorProfile> {
        self.disp
            .get_property("GenericProfile")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfile::new)
    }

    pub fn select_by_name(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("SelectByName", args)
            .ok()
            .and_then(|v| v.to_bool().ok())
            .unwrap_or(false)
    }

    pub fn install(&self, file_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp
            .invoke_method("Install", args)
            .ok()
            .and_then(|v| v.to_bool().ok())
            .unwrap_or(false)
    }
}

// =============================================================
// IvgColorManagementPolicy
// =============================================================

pub struct IvgColorManagementPolicy {
    disp: ComObject,
}

impl IvgColorManagementPolicy {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn action_for_rgb(&self) -> Option<i64> { self.prop_i64("ActionForRGB") }
    pub fn set_action_for_rgb(&self, v: i32) -> bool {
        self.put_i64("ActionForRGB", v as i64)
    }

    pub fn action_for_cmyk(&self) -> Option<i64> { self.prop_i64("ActionForCMYK") }
    pub fn set_action_for_cmyk(&self, v: i32) -> bool {
        self.put_i64("ActionForCMYK", v as i64)
    }

    pub fn action_for_grayscale(&self) -> Option<i64> { self.prop_i64("ActionForGrayscale") }
    pub fn set_action_for_grayscale(&self, v: i32) -> bool {
        self.put_i64("ActionForGrayscale", v as i64)
    }

    pub fn warn_on_mismatched_profiles(&self) -> Option<bool> {
        self.prop_bool("WarnOnMismatchedProfiles")
    }
    pub fn set_warn_on_mismatched_profiles(&self, v: bool) -> bool {
        self.put_bool("WarnOnMismatchedProfiles", v)
    }

    pub fn warn_on_missing_profiles(&self) -> Option<bool> {
        self.prop_bool("WarnOnMissingProfiles")
    }
    pub fn set_warn_on_missing_profiles(&self, v: bool) -> bool {
        self.put_bool("WarnOnMissingProfiles", v)
    }
}

// =============================================================
// IvgColorManager
// =============================================================

pub struct IvgColorManager {
    disp: ComObject,
}

impl IvgColorManager {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 鏍″噯
    // ---------------------------------------------------------

    pub fn scanner_calibrated(&self) -> Option<bool> { self.prop_bool("ScannerCalibrated") }
    pub fn set_scanner_calibrated(&self, v: bool) -> bool { self.put_bool("ScannerCalibrated", v) }

    pub fn separation_printer_calibrated(&self) -> Option<bool> {
        self.prop_bool("SeparationPrinterCalibrated")
    }
    pub fn set_separation_printer_calibrated(&self, v: bool) -> bool {
        self.put_bool("SeparationPrinterCalibrated", v)
    }

    pub fn composite_printer_calibrated(&self) -> Option<bool> {
        self.prop_bool("CompositePrinterCalibrated")
    }

    pub fn composite_printer_calibration(&self) -> Option<i64> {
        self.prop_i64("CompositePrinterCalibration")
    }
    pub fn set_composite_printer_calibration(&self, v: i32) -> bool {
        self.put_i64("CompositePrinterCalibration", v as i64)
    }

    pub fn monitor_calibration(&self) -> Option<i64> { self.prop_i64("MonitorCalibration") }
    pub fn set_monitor_calibration(&self, v: i32) -> bool {
        self.put_i64("MonitorCalibration", v as i64)
    }

    // ---------------------------------------------------------
    // 鍑虹晫璀﹀憡
    // ---------------------------------------------------------

    pub fn show_out_of_gamut(&self) -> Option<bool> { self.prop_bool("ShowOutOfGamut") }
    pub fn set_show_out_of_gamut(&self, v: bool) -> bool { self.put_bool("ShowOutOfGamut", v) }

    pub fn out_of_gamut_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("OutOfGamutColor").map(IvgColor::new)
    }

    pub fn out_of_gamut_transparency(&self) -> Option<i64> {
        self.prop_i64("OutOfGamutTransparency")
    }
    pub fn set_out_of_gamut_transparency(&self, v: i32) -> bool {
        self.put_i64("OutOfGamutTransparency", v as i64)
    }

    // ---------------------------------------------------------
    // CMYK 琛ㄧ幇
    // ---------------------------------------------------------

    pub fn cmyk_in_percents(&self) -> Option<bool> { self.prop_bool("CMYKInPercents") }
    pub fn set_cmyk_in_percents(&self, v: bool) -> bool { self.put_bool("CMYKInPercents", v) }

    pub fn cmyk_gamut_for_spot_colors(&self) -> Option<bool> {
        self.prop_bool("CMYKGamutForSpotColors")
    }
    pub fn set_cmyk_gamut_for_spot_colors(&self, v: bool) -> bool {
        self.put_bool("CMYKGamutForSpotColors", v)
    }

    pub fn rendering_intent(&self) -> Option<i64> { self.prop_i64("RenderingIntent") }
    pub fn set_rendering_intent(&self, v: i32) -> bool {
        self.put_i64("RenderingIntent", v as i64)
    }

    pub fn color_engine(&self) -> Option<i64> { self.prop_i64("ColorEngine") }
    pub fn set_color_engine(&self, v: i32) -> bool {
        self.put_i64("ColorEngine", v as i64)
    }

    // ---------------------------------------------------------
    // 鏍峰紡
    // ---------------------------------------------------------

    pub fn style_count(&self) -> Option<i64> { self.prop_i64("StyleCount") }

    pub fn style_by_index(&self, index: i32) -> Option<String> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("StyleByIndex", args)
            .ok()?
            .to_string()
            .ok()
    }

    pub fn load_style(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("LoadStyle", args)
            .ok()
            .and_then(|v| v.to_bool().ok())
            .unwrap_or(false)
    }

    pub fn delete_style(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("DeleteStyle", args)
            .ok()
            .and_then(|v| v.to_bool().ok())
            .unwrap_or(false)
    }

    pub fn save_style(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("SaveStyle", args)
            .ok()
            .and_then(|v| v.to_bool().ok())
            .unwrap_or(false)
    }

    pub fn can_delete_style(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("CanDeleteStyle", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn unsaved_style_name(&self) -> Option<String> {
        self.prop_string("UnsavedStyleName")
    }

    // ---------------------------------------------------------
    // Profile 鏌ヨ
    // ---------------------------------------------------------

    pub fn current_profile(&self, device_type: i32) -> Option<IvgColorProfile> {
        let args = vec![Variant::from_i64(device_type as i64)];
        self.disp
            .invoke_method("CurrentProfile", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfile::new)
    }

    pub fn installed_profiles(&self, device_type: i32) -> Option<IvgColorProfiles> {
        let args = vec![Variant::from_i64(device_type as i64)];
        self.disp
            .invoke_method("InstalledProfiles", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfiles::new)
    }

    pub fn profiles_by_color_model(&self, color_model: i32) -> Option<IvgColorProfiles> {
        let args = vec![Variant::from_i64(color_model as i64)];
        self.disp
            .invoke_method("GetProfilesByColorModel", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfiles::new)
    }

    pub fn profiles_for_device(
        &self,
        device_type: i32,
        device_name: impl Into<String>,
    ) -> Option<IvgColorProfiles> {
        let args = vec![
            Variant::from_i64(device_type as i64),
            Variant::from_str(device_name.into()),
        ];
        self.disp
            .invoke_method("GetProfilesForDevice", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfiles::new)
    }

    pub fn default_color_context(&self) -> Option<IvgColorContext> {
        self.prop_dispatch("DefaultColorContext").map(IvgColorContext::new)
    }

    pub fn color_profiles(&self) -> Option<IvgColorProfiles> {
        self.prop_dispatch("ColorProfiles").map(IvgColorProfiles::new)
    }

    pub fn monitor_color_profiles(&self) -> Option<IvgColorProfiles> {
        self.prop_dispatch("MonitorColorProfiles").map(IvgColorProfiles::new)
    }

    // ---------------------------------------------------------
    // 绛栫暐
    // ---------------------------------------------------------

    pub fn policy_for_open(&self) -> Option<IvgColorManagementPolicy> {
        self.prop_dispatch("PolicyForOpen").map(IvgColorManagementPolicy::new)
    }

    pub fn policy_for_import(&self) -> Option<IvgColorManagementPolicy> {
        self.prop_dispatch("PolicyForImport").map(IvgColorManagementPolicy::new)
    }

    // ---------------------------------------------------------
    // 瀵煎叆 / 瀵煎嚭鏍℃
    // ---------------------------------------------------------

    pub fn color_correction_on_import(&self) -> Option<i64> {
        self.prop_i64("ColorCorrectionOnImport")
    }
    pub fn set_color_correction_on_import(&self, v: i32) -> bool {
        self.put_i64("ColorCorrectionOnImport", v as i64)
    }

    pub fn color_correction_on_export(&self) -> Option<i64> {
        self.prop_i64("ColorCorrectionOnExport")
    }
    pub fn set_color_correction_on_export(&self, v: i32) -> bool {
        self.put_i64("ColorCorrectionOnExport", v as i64)
    }

    // ---------------------------------------------------------
    // 鍏跺畠
    // ---------------------------------------------------------

    pub fn is_icm2_available(&self) -> Option<bool> { self.prop_bool("IsICM2Available") }
    pub fn is_composite_printer_cmyk(&self) -> Option<bool> {
        self.prop_bool("IsCompositePrinterCMYK")
    }

    pub fn map_gray_to_cmyk_black(&self) -> Option<bool> { self.prop_bool("MapGrayToCMYKBlack") }
    pub fn set_map_gray_to_cmyk_black(&self, v: bool) -> bool {
        self.put_bool("MapGrayToCMYKBlack", v)
    }

    pub fn preserve_pure_black(&self) -> Option<bool> { self.prop_bool("PreservePureBlack") }
    pub fn set_preserve_pure_black(&self, v: bool) -> bool {
        self.put_bool("PreservePureBlack", v)
    }

    pub fn spot_color_definition(&self) -> Option<i64> { self.prop_i64("SpotColorDefinition") }
    pub fn set_spot_color_definition(&self, v: i32) -> bool {
        self.put_i64("SpotColorDefinition", v as i64)
    }

    pub fn color_engine_present(&self, color_engine: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(color_engine as i64)];
        self.disp
            .invoke_method("ColorEnginePresent", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
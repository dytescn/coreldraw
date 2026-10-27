//! 颜色管理器& Profile & 管理策略

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::color::IvgColor;
use crate::color::context::IvgColorContext;

// =============================================================
// IvgColorProfile
// =============================================================

/// CorelDRAW 对象的后期绑定包装（颜色配置文件）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgColorProfile {
    disp: ComObject,
}

impl IvgColorProfile {
    /// 由 COM `IDispatch` 构造包装对象。
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

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 读取文件名称（`FileName`）。
    pub fn file_name(&self) -> Option<String> { self.prop_string("FileName") }
    /// 读取 COM 属性 `Manufacturer`。
    pub fn manufacturer(&self) -> Option<String> { self.prop_string("Manufacturer") }
    /// 读取设备模型（`DeviceModel`）。
    pub fn device_model(&self) -> Option<String> { self.prop_string("DeviceModel") }
    /// 读取设备类型（`DeviceType`）。
    pub fn device_type(&self) -> Option<i64> { self.prop_i64("DeviceType") }
    /// 读取已选中（`Selected`）。
    pub fn selected(&self) -> Option<bool> { self.prop_bool("Selected") }
    /// 读取 COM 属性 `Generic`。
    pub fn generic(&self) -> Option<bool> { self.prop_bool("Generic") }
    /// 读取已安装（`Installed`）。
    pub fn installed(&self) -> Option<bool> { self.prop_bool("Installed") }
    /// 读取标识（`ID`）。
    pub fn id(&self) -> Option<String> { self.prop_string("ID") }
    /// 读取颜色模型（`ColorModel`）。
    pub fn color_model(&self) -> Option<i64> { self.prop_i64("ColorModel") }

    /// 选择（COM 方法 `Select`）。
    pub fn select(&self) -> bool {
        self.disp.invoke_method("Select", vec![]).is_ok()
    }

    /// 用当前 profile + 渲染意图创建 ColorContext
    pub fn create_color_context(&self, rendering_intent: i32) -> Option<IvgColorContext> {
        let args = vec![Variant::from_i64(rendering_intent as i64)];
        self.disp
            .invoke_method("CreateColorContext", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorContext::new)
    }

    /// 是否相同（COM 方法 `IsSame`）。
    pub fn is_same(&self, other: &IvgColorProfile) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("IsSame", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    /// 分发。
    pub fn dispatch(&self) -> &IDispatch {
        self.disp.as_dispatch()
    }
}

// =============================================================
// IvgColorProfiles
// =============================================================

/// CorelDRAW 对象的后期绑定包装（颜色配置文件集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgColorProfiles {
    disp: ComObject,
}

impl IvgColorProfiles {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgColorProfile> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfile::new)
    }

    /// 条目索引。
    pub fn item_by_index(&self, i: i32) -> Option<IvgColorProfile> {
        self.item(Variant::from_i64(i as i64))
    }

    /// 条目名称。
    pub fn item_by_name(&self, s: impl Into<String>) -> Option<IvgColorProfile> {
        self.item(Variant::from_str(s.into()))
    }

    /// 读取设备类型（`DeviceType`）。
    pub fn device_type(&self) -> Option<i64> {
        self.disp.get_property("DeviceType").ok()?.to_i64().ok()
    }

    /// 读取 COM 属性 `GenericProfile`。
    pub fn generic_profile(&self) -> Option<IvgColorProfile> {
        self.disp
            .get_property("GenericProfile")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfile::new)
    }

    /// 选择名称（COM 方法 `SelectByName`）。
    pub fn select_by_name(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("SelectByName", args)
            .ok()
            .and_then(|v| v.to_bool().ok())
            .unwrap_or(false)
    }

    /// 调用 COM 方法 `Install`。
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

/// CorelDRAW 对象的后期绑定包装（颜色管理策略）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgColorManagementPolicy {
    disp: ComObject,
}

impl IvgColorManagementPolicy {
    /// 由 COM `IDispatch` 构造包装对象。
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

    /// 读取动作 RGB（`ActionForRGB`）。
    pub fn action_for_rgb(&self) -> Option<i64> { self.prop_i64("ActionForRGB") }
    /// 设置动作 RGB（`ActionForRGB`）。
    pub fn set_action_for_rgb(&self, v: i32) -> bool {
        self.put_i64("ActionForRGB", v as i64)
    }

    /// 读取动作 CMYK（`ActionForCMYK`）。
    pub fn action_for_cmyk(&self) -> Option<i64> { self.prop_i64("ActionForCMYK") }
    /// 设置动作 CMYK（`ActionForCMYK`）。
    pub fn set_action_for_cmyk(&self, v: i32) -> bool {
        self.put_i64("ActionForCMYK", v as i64)
    }

    /// 读取动作灰度（`ActionForGrayscale`）。
    pub fn action_for_grayscale(&self) -> Option<i64> { self.prop_i64("ActionForGrayscale") }
    /// 设置动作灰度（`ActionForGrayscale`）。
    pub fn set_action_for_grayscale(&self, v: i32) -> bool {
        self.put_i64("ActionForGrayscale", v as i64)
    }

    /// 读取 COM 属性 `WarnOnMismatchedProfiles`。
    pub fn warn_on_mismatched_profiles(&self) -> Option<bool> {
        self.prop_bool("WarnOnMismatchedProfiles")
    }
    /// 设置 COM 属性 `WarnOnMismatchedProfiles`。
    pub fn set_warn_on_mismatched_profiles(&self, v: bool) -> bool {
        self.put_bool("WarnOnMismatchedProfiles", v)
    }

    /// 读取 COM 属性 `WarnOnMissingProfiles`。
    pub fn warn_on_missing_profiles(&self) -> Option<bool> {
        self.prop_bool("WarnOnMissingProfiles")
    }
    /// 设置 COM 属性 `WarnOnMissingProfiles`。
    pub fn set_warn_on_missing_profiles(&self, v: bool) -> bool {
        self.put_bool("WarnOnMissingProfiles", v)
    }
}

// =============================================================
// IvgColorManager
// =============================================================

/// CorelDRAW 对象的后期绑定包装（颜色管理器）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgColorManager {
    disp: ComObject,
}

impl IvgColorManager {
    /// 由 COM `IDispatch` 构造包装对象。
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
    // 校准
    // ---------------------------------------------------------

    /// 读取 COM 属性 `ScannerCalibrated`。
    pub fn scanner_calibrated(&self) -> Option<bool> { self.prop_bool("ScannerCalibrated") }
    /// 设置 COM 属性 `ScannerCalibrated`。
    pub fn set_scanner_calibrated(&self, v: bool) -> bool { self.put_bool("ScannerCalibrated", v) }

    /// 读取分色打印机已校准（`SeparationPrinterCalibrated`）。
    pub fn separation_printer_calibrated(&self) -> Option<bool> {
        self.prop_bool("SeparationPrinterCalibrated")
    }
    /// 设置分色打印机已校准（`SeparationPrinterCalibrated`）。
    pub fn set_separation_printer_calibrated(&self, v: bool) -> bool {
        self.put_bool("SeparationPrinterCalibrated", v)
    }

    /// 读取合成打印机已校准（`CompositePrinterCalibrated`）。
    pub fn composite_printer_calibrated(&self) -> Option<bool> {
        self.prop_bool("CompositePrinterCalibrated")
    }

    /// 读取合成打印机校准（`CompositePrinterCalibration`）。
    pub fn composite_printer_calibration(&self) -> Option<i64> {
        self.prop_i64("CompositePrinterCalibration")
    }
    /// 设置合成打印机校准（`CompositePrinterCalibration`）。
    pub fn set_composite_printer_calibration(&self, v: i32) -> bool {
        self.put_i64("CompositePrinterCalibration", v as i64)
    }

    /// 读取监视器校准（`MonitorCalibration`）。
    pub fn monitor_calibration(&self) -> Option<i64> { self.prop_i64("MonitorCalibration") }
    /// 设置监视器校准（`MonitorCalibration`）。
    pub fn set_monitor_calibration(&self, v: i32) -> bool {
        self.put_i64("MonitorCalibration", v as i64)
    }

    // ---------------------------------------------------------
    // 出界警告
    // ---------------------------------------------------------

    /// 读取显示输出色域（`ShowOutOfGamut`）。
    pub fn show_out_of_gamut(&self) -> Option<bool> { self.prop_bool("ShowOutOfGamut") }
    /// 设置显示输出色域（`ShowOutOfGamut`）。
    pub fn set_show_out_of_gamut(&self, v: bool) -> bool { self.put_bool("ShowOutOfGamut", v) }

    /// 读取输出色域颜色（`OutOfGamutColor`）。
    pub fn out_of_gamut_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("OutOfGamutColor").map(IvgColor::new)
    }

    /// 读取输出色域透明度（`OutOfGamutTransparency`）。
    pub fn out_of_gamut_transparency(&self) -> Option<i64> {
        self.prop_i64("OutOfGamutTransparency")
    }
    /// 设置输出色域透明度（`OutOfGamutTransparency`）。
    pub fn set_out_of_gamut_transparency(&self, v: i32) -> bool {
        self.put_i64("OutOfGamutTransparency", v as i64)
    }

    // ---------------------------------------------------------
    // CMYK 表现
    // ---------------------------------------------------------

    /// 读取 COM 属性 `CMYKInPercents`。
    pub fn cmyk_in_percents(&self) -> Option<bool> { self.prop_bool("CMYKInPercents") }
    /// 设置 COM 属性 `CMYKInPercents`。
    pub fn set_cmyk_in_percents(&self, v: bool) -> bool { self.put_bool("CMYKInPercents", v) }

    /// 读取 CMYK 色域专色颜色集（`CMYKGamutForSpotColors`）。
    pub fn cmyk_gamut_for_spot_colors(&self) -> Option<bool> {
        self.prop_bool("CMYKGamutForSpotColors")
    }
    /// 设置 CMYK 色域专色颜色集（`CMYKGamutForSpotColors`）。
    pub fn set_cmyk_gamut_for_spot_colors(&self, v: bool) -> bool {
        self.put_bool("CMYKGamutForSpotColors", v)
    }

    /// 读取渲染意图（`RenderingIntent`）。
    pub fn rendering_intent(&self) -> Option<i64> { self.prop_i64("RenderingIntent") }
    /// 设置渲染意图（`RenderingIntent`）。
    pub fn set_rendering_intent(&self, v: i32) -> bool {
        self.put_i64("RenderingIntent", v as i64)
    }

    /// 读取颜色引擎（`ColorEngine`）。
    pub fn color_engine(&self) -> Option<i64> { self.prop_i64("ColorEngine") }
    /// 设置颜色引擎（`ColorEngine`）。
    pub fn set_color_engine(&self, v: i32) -> bool {
        self.put_i64("ColorEngine", v as i64)
    }

    // ---------------------------------------------------------
    // 样式
    // ---------------------------------------------------------

    /// 读取样式数量（`StyleCount`）。
    pub fn style_count(&self) -> Option<i64> { self.prop_i64("StyleCount") }

    /// 样式索引（COM 方法 `StyleByIndex`）。
    pub fn style_by_index(&self, index: i32) -> Option<String> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("StyleByIndex", args)
            .ok()?
            .to_string()
            .ok()
    }

    /// 加载样式（COM 方法 `LoadStyle`）。
    pub fn load_style(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("LoadStyle", args)
            .ok()
            .and_then(|v| v.to_bool().ok())
            .unwrap_or(false)
    }

    /// 删除样式（COM 方法 `DeleteStyle`）。
    pub fn delete_style(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("DeleteStyle", args)
            .ok()
            .and_then(|v| v.to_bool().ok())
            .unwrap_or(false)
    }

    /// 保存样式（COM 方法 `SaveStyle`）。
    pub fn save_style(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("SaveStyle", args)
            .ok()
            .and_then(|v| v.to_bool().ok())
            .unwrap_or(false)
    }

    /// 是否可以删除样式（COM 方法 `CanDeleteStyle`）。
    pub fn can_delete_style(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("CanDeleteStyle", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 读取 COM 属性 `UnsavedStyleName`。
    pub fn unsaved_style_name(&self) -> Option<String> {
        self.prop_string("UnsavedStyleName")
    }

    // ---------------------------------------------------------
    // Profile 查询
    // ---------------------------------------------------------

    /// 调用 COM 方法 `CurrentProfile`。
    pub fn current_profile(&self, device_type: i32) -> Option<IvgColorProfile> {
        let args = vec![Variant::from_i64(device_type as i64)];
        self.disp
            .invoke_method("CurrentProfile", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfile::new)
    }

    /// 已安装配置文件集（COM 方法 `InstalledProfiles`）。
    pub fn installed_profiles(&self, device_type: i32) -> Option<IvgColorProfiles> {
        let args = vec![Variant::from_i64(device_type as i64)];
        self.disp
            .invoke_method("InstalledProfiles", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfiles::new)
    }

    /// 配置文件集颜色模型（COM 方法 `GetProfilesByColorModel`）。
    pub fn profiles_by_color_model(&self, color_model: i32) -> Option<IvgColorProfiles> {
        let args = vec![Variant::from_i64(color_model as i64)];
        self.disp
            .invoke_method("GetProfilesByColorModel", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfiles::new)
    }

    /// 配置文件集设备（COM 方法 `GetProfilesForDevice`）。
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

    /// 读取默认值颜色上下文（`DefaultColorContext`）。
    pub fn default_color_context(&self) -> Option<IvgColorContext> {
        self.prop_dispatch("DefaultColorContext").map(IvgColorContext::new)
    }

    /// 读取颜色配置文件集（`ColorProfiles`）。
    pub fn color_profiles(&self) -> Option<IvgColorProfiles> {
        self.prop_dispatch("ColorProfiles").map(IvgColorProfiles::new)
    }

    /// 读取监视器颜色配置文件集（`MonitorColorProfiles`）。
    pub fn monitor_color_profiles(&self) -> Option<IvgColorProfiles> {
        self.prop_dispatch("MonitorColorProfiles").map(IvgColorProfiles::new)
    }

    // ---------------------------------------------------------
    // 策略
    // ---------------------------------------------------------

    /// 读取策略打开（`PolicyForOpen`）。
    pub fn policy_for_open(&self) -> Option<IvgColorManagementPolicy> {
        self.prop_dispatch("PolicyForOpen").map(IvgColorManagementPolicy::new)
    }

    /// 读取策略导入（`PolicyForImport`）。
    pub fn policy_for_import(&self) -> Option<IvgColorManagementPolicy> {
        self.prop_dispatch("PolicyForImport").map(IvgColorManagementPolicy::new)
    }

    // ---------------------------------------------------------
    // 导入 / 导出校正
    // ---------------------------------------------------------

    /// 读取导入上的颜色校正（`ColorCorrectionOnImport`）。
    pub fn color_correction_on_import(&self) -> Option<i64> {
        self.prop_i64("ColorCorrectionOnImport")
    }
    /// 设置导入上的颜色校正（`ColorCorrectionOnImport`）。
    pub fn set_color_correction_on_import(&self, v: i32) -> bool {
        self.put_i64("ColorCorrectionOnImport", v as i64)
    }

    /// 读取导出上的颜色校正（`ColorCorrectionOnExport`）。
    pub fn color_correction_on_export(&self) -> Option<i64> {
        self.prop_i64("ColorCorrectionOnExport")
    }
    /// 设置导出上的颜色校正（`ColorCorrectionOnExport`）。
    pub fn set_color_correction_on_export(&self, v: i32) -> bool {
        self.put_i64("ColorCorrectionOnExport", v as i64)
    }

    // ---------------------------------------------------------
    // 其它
    // ---------------------------------------------------------

    /// 读取 COM 属性 `IsICM2Available`。
    pub fn is_icm2_available(&self) -> Option<bool> { self.prop_bool("IsICM2Available") }
    /// 是否合成打印机 CMYK（`IsCompositePrinterCMYK`）。
    pub fn is_composite_printer_cmyk(&self) -> Option<bool> {
        self.prop_bool("IsCompositePrinterCMYK")
    }

    /// 读取映射灰度 CMYK 黑色（`MapGrayToCMYKBlack`）。
    pub fn map_gray_to_cmyk_black(&self) -> Option<bool> { self.prop_bool("MapGrayToCMYKBlack") }
    /// 设置映射灰度 CMYK 黑色（`MapGrayToCMYKBlack`）。
    pub fn set_map_gray_to_cmyk_black(&self, v: bool) -> bool {
        self.put_bool("MapGrayToCMYKBlack", v)
    }

    /// 读取保留纯黑色（`PreservePureBlack`）。
    pub fn preserve_pure_black(&self) -> Option<bool> { self.prop_bool("PreservePureBlack") }
    /// 设置保留纯黑色（`PreservePureBlack`）。
    pub fn set_preserve_pure_black(&self, v: bool) -> bool {
        self.put_bool("PreservePureBlack", v)
    }

    /// 读取专色颜色定义（`SpotColorDefinition`）。
    pub fn spot_color_definition(&self) -> Option<i64> { self.prop_i64("SpotColorDefinition") }
    /// 设置专色颜色定义（`SpotColorDefinition`）。
    pub fn set_spot_color_definition(&self, v: i32) -> bool {
        self.put_i64("SpotColorDefinition", v as i64)
    }

    /// 颜色引擎存在（COM 方法 `ColorEnginePresent`）。
    pub fn color_engine_present(&self, color_engine: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(color_engine as i64)];
        self.disp
            .invoke_method("ColorEnginePresent", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
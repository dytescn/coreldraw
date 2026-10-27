//! `IPrnVBAPrintSeparations` / `IPrnVBASeparationPlate(s)` —— 分色

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPrnSeparations
// =============================================================

/// CorelDRAW `IPrnVBAPrintSeparations` 对象的后期绑定包装（分色）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnSeparations {
    disp: ComObject,
}

impl IvgPrnSeparations {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 主开关----

    /// 读取启用状态（`Enabled`）。
    pub fn enabled(&self) -> Option<bool> { self.prop_bool("Enabled") }
    /// 设置启用状态（`Enabled`）。
    pub fn set_enabled(&self, v: bool) -> bool { self.put_bool("Enabled", v) }

    /// 读取在颜色（`InColor`）。
    pub fn in_color(&self) -> Option<bool> { self.prop_bool("InColor") }
    /// 设置在颜色（`InColor`）。
    pub fn set_in_color(&self, v: bool) -> bool { self.put_bool("InColor", v) }

    /// 读取 COM 属性 `Hexachrome`。
    pub fn hexachrome(&self) -> Option<bool> { self.prop_bool("Hexachrome") }
    /// 设置 COM 属性 `Hexachrome`。
    pub fn set_hexachrome(&self, v: bool) -> bool { self.put_bool("Hexachrome", v) }

    /// 读取专色 CMYK（`SpotToCMYK`）。
    pub fn spot_to_cmyk(&self) -> Option<bool> { self.prop_bool("SpotToCMYK") }
    /// 设置专色 CMYK（`SpotToCMYK`）。
    pub fn set_spot_to_cmyk(&self, v: bool) -> bool { self.put_bool("SpotToCMYK", v) }

    /// 读取空印版集（`EmptyPlates`）。
    pub fn empty_plates(&self) -> Option<bool> { self.prop_bool("EmptyPlates") }
    /// 设置空印版集（`EmptyPlates`）。
    pub fn set_empty_plates(&self, v: bool) -> bool { self.put_bool("EmptyPlates", v) }

    // ---- 叠印 ----

    /// 读取保留叠印（`PreserveOverprints`）。
    pub fn preserve_overprints(&self) -> Option<bool> { self.prop_bool("PreserveOverprints") }
    /// 设置保留叠印（`PreserveOverprints`）。
    pub fn set_preserve_overprints(&self, v: bool) -> bool {
        self.put_bool("PreserveOverprints", v)
    }

    /// 读取总是叠印黑色（`AlwaysOverprintBlack`）。
    pub fn always_overprint_black(&self) -> Option<bool> {
        self.prop_bool("AlwaysOverprintBlack")
    }
    /// 设置总是叠印黑色（`AlwaysOverprintBlack`）。
    pub fn set_always_overprint_black(&self, v: bool) -> bool {
        self.put_bool("AlwaysOverprintBlack", v)
    }

    // ---- 自动补漏百----

    /// 读取 COM 属性 `AutoSpreading`。
    pub fn auto_spreading(&self) -> Option<bool> { self.prop_bool("AutoSpreading") }
    /// 设置 COM 属性 `AutoSpreading`。
    pub fn set_auto_spreading(&self, v: bool) -> bool {
        self.put_bool("AutoSpreading", v)
    }

    /// 读取 COM 属性 `AutoSpreadAmount`。
    pub fn auto_spread_amount(&self) -> Option<f64> { self.prop_f64("AutoSpreadAmount") }
    /// 设置 COM 属性 `AutoSpreadAmount`。
    pub fn set_auto_spread_amount(&self, v: f64) -> bool {
        self.put_f64("AutoSpreadAmount", v)
    }

    /// 读取自动跨页固定（`AutoSpreadFixed`）。
    pub fn auto_spread_fixed(&self) -> Option<bool> { self.prop_bool("AutoSpreadFixed") }
    /// 设置自动跨页固定（`AutoSpreadFixed`）。
    pub fn set_auto_spread_fixed(&self, v: bool) -> bool {
        self.put_bool("AutoSpreadFixed", v)
    }

    /// 读取自动跨页文本上方（`AutoSpreadTextAbove`）。
    pub fn auto_spread_text_above(&self) -> Option<f64> {
        self.prop_f64("AutoSpreadTextAbove")
    }
    /// 设置自动跨页文本上方（`AutoSpreadTextAbove`）。
    pub fn set_auto_spread_text_above(&self, v: f64) -> bool {
        self.put_f64("AutoSpreadTextAbove", v)
    }

    // ---- 高级设置 ----

    /// 读取 COM 属性 `AdvancedSettings`。
    pub fn advanced_settings(&self) -> Option<bool> { self.prop_bool("AdvancedSettings") }
    /// 设置 COM 属性 `AdvancedSettings`。
    pub fn set_advanced_settings(&self, v: bool) -> bool {
        self.put_bool("AdvancedSettings", v)
    }

    // ---- 分色板/ 分辨率----

    /// 读取印版集（`Plates`）。
    pub fn plates(&self) -> Option<IvgPrnSeparationPlates> {
        self.prop_dispatch("Plates").map(IvgPrnSeparationPlates::new)
    }

    /// 读取分辨率（`Resolution`）。
    pub fn resolution(&self) -> Option<i64> { self.prop_i64("Resolution") }
    /// 设置分辨率（`Resolution`）。
    pub fn set_resolution(&self, v: i32) -> bool { self.put_i64("Resolution", v as i64) }

    /// 读取 COM 属性 `BasicScreen`。
    pub fn basic_screen(&self) -> Option<String> { self.prop_string("BasicScreen") }
    /// 设置 COM 属性 `BasicScreen`。
    pub fn set_basic_screen(&self, v: impl Into<String>) -> bool {
        self.put_string("BasicScreen", v)
    }

    /// 读取 COM 属性 `HalftoneType`。
    pub fn halftone_type(&self) -> Option<String> { self.prop_string("HalftoneType") }
    /// 设置 COM 属性 `HalftoneType`。
    pub fn set_halftone_type(&self, v: impl Into<String>) -> bool {
        self.put_string("HalftoneType", v)
    }

    /// 读取 COM 属性 `ScreenTechnology`。
    pub fn screen_technology(&self) -> Option<String> { self.prop_string("ScreenTechnology") }
    /// 设置 COM 属性 `ScreenTechnology`。
    pub fn set_screen_technology(&self, v: impl Into<String>) -> bool {
        self.put_string("ScreenTechnology", v)
    }
}

// =============================================================
// IvgPrnSeparationPlates
// =============================================================

/// CorelDRAW `IPrnVBAPrintSeparations` 对象的后期绑定包装（分色印版集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnSeparationPlates {
    disp: ComObject,
}

impl IvgPrnSeparationPlates {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgPrnSeparationPlate> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPrnSeparationPlate::new)
    }
}

// =============================================================
// IvgPrnSeparationPlate
// =============================================================

/// CorelDRAW `IPrnVBAPrintSeparations` 对象的后期绑定包装（分色印版）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnSeparationPlate {
    disp: ComObject,
}

impl IvgPrnSeparationPlate {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取启用状态（`Enabled`）。
    pub fn enabled(&self) -> Option<bool> { self.prop_bool("Enabled") }
    /// 设置启用状态（`Enabled`）。
    pub fn set_enabled(&self, v: bool) -> bool { self.put_bool("Enabled", v) }

    /// `PrnPlateType`
    pub fn plate_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 读取颜色（`Color`）。
    pub fn color(&self) -> Option<String> { self.prop_string("Color") }

    /// 读取频率（`Frequency`）。
    pub fn frequency(&self) -> Option<f64> { self.prop_f64("Frequency") }
    /// 设置频率（`Frequency`）。
    pub fn set_frequency(&self, v: f64) -> bool { self.put_f64("Frequency", v) }

    /// 读取角度（`Angle`）。
    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    /// 设置角度（`Angle`）。
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }

    /// 读取叠印文本（`OverprintText`）。
    pub fn overprint_text(&self) -> Option<bool> { self.prop_bool("OverprintText") }
    /// 设置叠印文本（`OverprintText`）。
    pub fn set_overprint_text(&self, v: bool) -> bool { self.put_bool("OverprintText", v) }

    /// 读取 COM 属性 `OverprintGraphic`。
    pub fn overprint_graphic(&self) -> Option<bool> { self.prop_bool("OverprintGraphic") }
    /// 设置 COM 属性 `OverprintGraphic`。
    pub fn set_overprint_graphic(&self, v: bool) -> bool {
        self.put_bool("OverprintGraphic", v)
    }
}
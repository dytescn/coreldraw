//! `IVGColor` —— 单个颜色对象
//!
//! 由 `IvgApplication::create_rgb_color` / `create_cmyk_color` / ... 创建：
//! 或从 `IvgShape::fill()` 等属性获取。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::context::IvgColorContext;

/// CorelDRAW `IVGColor` 对象的后期绑定包装（颜色）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgColor {
    disp: ComObject,
}

impl IvgColor {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 通用属性读取
    // ---------------------------------------------------------

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

    fn put_string(&self, name: &str, s: impl Into<String>) -> bool {
        let arg = Variant::from_str(s.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 类型
    // ---------------------------------------------------------

    /// `cdrColorType` ——见[`crate::enums::color`]
    pub fn color_type(&self) -> Option<i64> {
        self.prop_i64("Type")
    }

    // ---------------------------------------------------------
    // RGB
    // ---------------------------------------------------------

    /// 赋值 RGB（COM 方法 `RGBAssign`）。
    pub fn assign_rgb(&self, r: i32, g: i32, b: i32) -> bool {
        let args = vec![
            Variant::from_i64(r as i64),
            Variant::from_i64(g as i64),
            Variant::from_i64(b as i64),
        ];
        self.disp.invoke_method("RGBAssign", args).is_ok()
    }

    /// 读取 RGB 红（`RGBRed`）。
    pub fn rgb_red(&self) -> Option<i64> {
        self.prop_i64("RGBRed")
    }
    /// 设置 RGB 红（`RGBRed`）。
    pub fn set_rgb_red(&self, v: i32) -> bool {
        self.put_i64("RGBRed", v as i64)
    }

    /// 读取 RGB 绿（`RGBGreen`）。
    pub fn rgb_green(&self) -> Option<i64> {
        self.prop_i64("RGBGreen")
    }
    /// 设置 RGB 绿（`RGBGreen`）。
    pub fn set_rgb_green(&self, v: i32) -> bool {
        self.put_i64("RGBGreen", v as i64)
    }

    /// 读取 RGB 蓝（`RGBBlue`）。
    pub fn rgb_blue(&self) -> Option<i64> {
        self.prop_i64("RGBBlue")
    }
    /// 设置 RGB 蓝（`RGBBlue`）。
    pub fn set_rgb_blue(&self, v: i32) -> bool {
        self.put_i64("RGBBlue", v as i64)
    }

    /// 转换 RGB（COM 方法 `ConvertToRGB`）。
    pub fn convert_to_rgb(&self) -> bool {
        self.disp.invoke_method("ConvertToRGB", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // CMYK
    // ---------------------------------------------------------

    /// 赋值 CMYK（COM 方法 `CMYKAssign`）。
    pub fn assign_cmyk(&self, c: i32, m: i32, y: i32, k: i32) -> bool {
        let args = vec![
            Variant::from_i64(c as i64),
            Variant::from_i64(m as i64),
            Variant::from_i64(y as i64),
            Variant::from_i64(k as i64),
        ];
        self.disp.invoke_method("CMYKAssign", args).is_ok()
    }

    /// 读取 CMYK 青（`CMYKCyan`）。
    pub fn cmyk_cyan(&self) -> Option<i64> {
        self.prop_i64("CMYKCyan")
    }
    /// 设置 CMYK 青（`CMYKCyan`）。
    pub fn set_cmyk_cyan(&self, v: i32) -> bool {
        self.put_i64("CMYKCyan", v as i64)
    }

    /// 读取 CMYK 品红（`CMYKMagenta`）。
    pub fn cmyk_magenta(&self) -> Option<i64> {
        self.prop_i64("CMYKMagenta")
    }
    /// 设置 CMYK 品红（`CMYKMagenta`）。
    pub fn set_cmyk_magenta(&self, v: i32) -> bool {
        self.put_i64("CMYKMagenta", v as i64)
    }

    /// 读取 CMYK 黄（`CMYKYellow`）。
    pub fn cmyk_yellow(&self) -> Option<i64> {
        self.prop_i64("CMYKYellow")
    }
    /// 设置 CMYK 黄（`CMYKYellow`）。
    pub fn set_cmyk_yellow(&self, v: i32) -> bool {
        self.put_i64("CMYKYellow", v as i64)
    }

    /// 读取 CMYK 黑色（`CMYKBlack`）。
    pub fn cmyk_black(&self) -> Option<i64> {
        self.prop_i64("CMYKBlack")
    }
    /// 设置 CMYK 黑色（`CMYKBlack`）。
    pub fn set_cmyk_black(&self, v: i32) -> bool {
        self.put_i64("CMYKBlack", v as i64)
    }

    /// 转换 CMYK（COM 方法 `ConvertToCMYK`）。
    pub fn convert_to_cmyk(&self) -> bool {
        self.disp.invoke_method("ConvertToCMYK", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // HSB
    // ---------------------------------------------------------

    /// 赋值 HSB（COM 方法 `HSBAssign`）。
    pub fn assign_hsb(&self, h: i32, s: i32, b: i32) -> bool {
        let args = vec![
            Variant::from_i64(h as i64),
            Variant::from_i64(s as i64),
            Variant::from_i64(b as i64),
        ];
        self.disp.invoke_method("HSBAssign", args).is_ok()
    }

    /// 读取 HSB 色相（`HSBHue`）。
    pub fn hsb_hue(&self) -> Option<i64> { self.prop_i64("HSBHue") }
    /// 设置 HSB 色相（`HSBHue`）。
    pub fn set_hsb_hue(&self, v: i32) -> bool { self.put_i64("HSBHue", v as i64) }

    /// 读取 HSB 饱和度（`HSBSaturation`）。
    pub fn hsb_saturation(&self) -> Option<i64> { self.prop_i64("HSBSaturation") }
    /// 设置 HSB 饱和度（`HSBSaturation`）。
    pub fn set_hsb_saturation(&self, v: i32) -> bool { self.put_i64("HSBSaturation", v as i64) }

    /// 读取 COM 属性 `HSBBrightness`。
    pub fn hsb_brightness(&self) -> Option<i64> { self.prop_i64("HSBBrightness") }
    /// 设置 COM 属性 `HSBBrightness`。
    pub fn set_hsb_brightness(&self, v: i32) -> bool { self.put_i64("HSBBrightness", v as i64) }

    /// 转换 HSB（COM 方法 `ConvertToHSB`）。
    pub fn convert_to_hsb(&self) -> bool {
        self.disp.invoke_method("ConvertToHSB", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // HLS
    // ---------------------------------------------------------

    /// 赋值 HLS（COM 方法 `HLSAssign`）。
    pub fn assign_hls(&self, h: i32, l: i32, s: i32) -> bool {
        let args = vec![
            Variant::from_i64(h as i64),
            Variant::from_i64(l as i64),
            Variant::from_i64(s as i64),
        ];
        self.disp.invoke_method("HLSAssign", args).is_ok()
    }

    /// 读取 HLS 色相（`HLSHue`）。
    pub fn hls_hue(&self) -> Option<i64> { self.prop_i64("HLSHue") }
    /// 设置 HLS 色相（`HLSHue`）。
    pub fn set_hls_hue(&self, v: i32) -> bool { self.put_i64("HLSHue", v as i64) }

    /// 读取 HLS 亮度（`HLSLightness`）。
    pub fn hls_lightness(&self) -> Option<i64> { self.prop_i64("HLSLightness") }
    /// 设置 HLS 亮度（`HLSLightness`）。
    pub fn set_hls_lightness(&self, v: i32) -> bool { self.put_i64("HLSLightness", v as i64) }

    /// 读取 HLS 饱和度（`HLSSaturation`）。
    pub fn hls_saturation(&self) -> Option<i64> { self.prop_i64("HLSSaturation") }
    /// 设置 HLS 饱和度（`HLSSaturation`）。
    pub fn set_hls_saturation(&self, v: i32) -> bool { self.put_i64("HLSSaturation", v as i64) }

    /// 转换 HLS（COM 方法 `ConvertToHLS`）。
    pub fn convert_to_hls(&self) -> bool {
        self.disp.invoke_method("ConvertToHLS", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 灰度 / 黑白
    // ---------------------------------------------------------

    /// 赋值灰度（COM 方法 `GrayAssign`）。
    pub fn assign_gray(&self, gray: i32) -> bool {
        let args = vec![Variant::from_i64(gray as i64)];
        self.disp.invoke_method("GrayAssign", args).is_ok()
    }

    /// 读取灰度（`Gray`）。
    pub fn gray(&self) -> Option<i64> { self.prop_i64("Gray") }
    /// 设置灰度（`Gray`）。
    pub fn set_gray(&self, v: i32) -> bool { self.put_i64("Gray", v as i64) }

    /// 转换灰度（COM 方法 `ConvertToGray`）。
    pub fn convert_to_gray(&self) -> bool {
        self.disp.invoke_method("ConvertToGray", vec![]).is_ok()
    }

    /// 赋值黑白（COM 方法 `BWAssign`）。
    pub fn assign_bw(&self, white: bool) -> bool {
        let args = vec![Variant::from_bool(white)];
        self.disp.invoke_method("BWAssign", args).is_ok()
    }

    /// 读取黑白（`BW`）。
    pub fn bw(&self) -> Option<bool> { self.prop_bool("BW") }
    /// 设置黑白（`BW`）。
    pub fn set_bw(&self, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property("BW", vec![arg]).is_ok()
    }

    /// 转换黑白（COM 方法 `ConvertToBW`）。
    pub fn convert_to_bw(&self) -> bool {
        self.disp.invoke_method("ConvertToBW", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 名称 / 字符串
    // ---------------------------------------------------------

    /// `GetName(Components)` ——`Components=true` 时带分量描述。
    pub fn name(&self, components: bool) -> Option<String> {
        let args = vec![Variant::from_bool(components)];
        self.disp.invoke_method("Name", args).ok()?.to_string().ok()
    }

    /// 设置名称（`SetName`）。
    pub fn set_name(&self, s: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(s.into())];
        self.disp.invoke_method("SetName", args).is_ok()
    }

    /// 转换为颜色字符串（COM 方法 `ToString`）。
    pub fn to_color_string(&self) -> Option<String> {
        self.disp.invoke_method("ToString", vec![]).ok()?.to_string().ok()
    }

    /// 字符串 assign（COM 方法 `StringAssign`）。
    pub fn string_assign(&self, s: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(s.into())];
        self.disp
            .invoke_method("StringAssign", args)
            .ok()
            .and_then(|v| v.to_bool().ok())
            .unwrap_or(false)
    }

    /// 读取 COM 属性 `HexValue`。
    pub fn hex_value(&self) -> Option<String> {
        self.prop_string("HexValue")
    }

    /// 设置 COM 属性 `HexValue`。
    pub fn set_hex_value(&self, s: impl Into<String>) -> bool {
        self.put_string("HexValue", s)
    }

    // ---------------------------------------------------------
    // 复制 / 比较
    // ---------------------------------------------------------

    /// 复制 assign（COM 方法 `CopyAssign`）。
    pub fn copy_assign(&self, other: &IvgColor) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgColor> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColor::new)
    }

    /// 是否相同（COM 方法 `IsSame`）。
    pub fn is_same(&self, other: &IvgColor) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("IsSame", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 调用 COM 方法 `IsSimilar`。
    pub fn is_similar(&self, other: &IvgColor) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("IsSimilar", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 两色之间的距离"
    pub fn distance_from(&self, other: &IvgColor) -> Option<i64> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("GetColorDistanceFrom", args)
            .ok()?
            .to_i64()
            .ok()
    }

    // ---------------------------------------------------------
    // 判定
    // ---------------------------------------------------------

    /// 是否在色域（`IsInGamut`）。
    pub fn is_in_gamut(&self) -> Option<bool> { self.prop_bool("IsInGamut") }
    /// 是否 CMYK（`IsCMYK`）。
    pub fn is_cmyk(&self) -> Option<bool> { self.prop_bool("IsCMYK") }
    /// 是否灰度（`IsGray`）。
    pub fn is_gray(&self) -> Option<bool> { self.prop_bool("IsGray") }
    /// 是否白色（`IsWhite`）。
    pub fn is_white(&self) -> Option<bool> { self.prop_bool("IsWhite") }
    /// 是否专色（`IsSpot`）。
    pub fn is_spot(&self) -> Option<bool> { self.prop_bool("IsSpot") }
    /// 读取 COM 属性 `IsTintable`。
    pub fn is_tintable(&self) -> Option<bool> { self.prop_bool("IsTintable") }

    // ---------------------------------------------------------
    // 调色板/ 专色
    // ---------------------------------------------------------

    /// 读取调色板标识（`PaletteID`）。
    pub fn palette_id(&self) -> Option<i64> { self.prop_i64("PaletteID") }

    /// 读取调色板索引（`PaletteIndex`）。
    pub fn palette_index(&self) -> Option<i64> { self.prop_i64("PaletteIndex") }
    /// 设置调色板索引（`PaletteIndex`）。
    pub fn set_palette_index(&self, v: i32) -> bool {
        self.put_i64("PaletteIndex", v as i64)
    }

    /// 读取淡色（`Tint`）。
    pub fn tint(&self) -> Option<i64> { self.prop_i64("Tint") }
    /// 设置淡色（`Tint`）。
    pub fn set_tint(&self, v: i32) -> bool { self.put_i64("Tint", v as i64) }

    /// 读取专色颜色标识（`SpotColorID`）。
    pub fn spot_color_id(&self) -> Option<i64> { self.prop_i64("SpotColorID") }
    /// 设置专色颜色标识（`SpotColorID`）。
    pub fn set_spot_color_id(&self, v: i32) -> bool {
        self.put_i64("SpotColorID", v as i64)
    }

    /// 读取专色颜色名称（`SpotColorName`）。
    pub fn spot_color_name(&self) -> Option<String> { self.prop_string("SpotColorName") }

    /// 读取 COM 属性 `PaletteIdentifier`。
    pub fn palette_identifier(&self) -> Option<String> {
        self.prop_string("PaletteIdentifier")
    }

    // ---------------------------------------------------------
    // 上下文
    // ---------------------------------------------------------

    /// 读取颜色上下文（`ColorContext`）。
    pub fn color_context(&self) -> Option<IvgColorContext> {
        self.prop_dispatch("ColorContext").map(IvgColorContext::new)
    }

    // ---------------------------------------------------------
    // 辅助
    // ---------------------------------------------------------
    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }
}
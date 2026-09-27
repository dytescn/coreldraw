//! `IVGColor` 鈥斺€?鍗曚釜棰滆壊瀵硅薄
//!
//! 鐢?`IvgApplication::create_rgb_color` / `create_cmyk_color` / ... 鍒涘缓锛?
//! 鎴栦粠 `IvgShape::fill()` 绛夊睘鎬ц幏鍙栥€?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::context::IvgColorContext;

pub struct IvgColor {
    disp: ComObject,
}

impl IvgColor {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 閫氱敤灞炴€ц鍙?
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
    // 绫诲瀷
    // ---------------------------------------------------------

    /// `cdrColorType` 鈥斺€?瑙?[`crate::enums::color`]
    pub fn color_type(&self) -> Option<i64> {
        self.prop_i64("Type")
    }

    // ---------------------------------------------------------
    // RGB
    // ---------------------------------------------------------

    pub fn assign_rgb(&self, r: i32, g: i32, b: i32) -> bool {
        let args = vec![
            Variant::from_i64(r as i64),
            Variant::from_i64(g as i64),
            Variant::from_i64(b as i64),
        ];
        self.disp.invoke_method("RGBAssign", args).is_ok()
    }

    pub fn rgb_red(&self) -> Option<i64> {
        self.prop_i64("RGBRed")
    }
    pub fn set_rgb_red(&self, v: i32) -> bool {
        self.put_i64("RGBRed", v as i64)
    }

    pub fn rgb_green(&self) -> Option<i64> {
        self.prop_i64("RGBGreen")
    }
    pub fn set_rgb_green(&self, v: i32) -> bool {
        self.put_i64("RGBGreen", v as i64)
    }

    pub fn rgb_blue(&self) -> Option<i64> {
        self.prop_i64("RGBBlue")
    }
    pub fn set_rgb_blue(&self, v: i32) -> bool {
        self.put_i64("RGBBlue", v as i64)
    }

    pub fn convert_to_rgb(&self) -> bool {
        self.disp.invoke_method("ConvertToRGB", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // CMYK
    // ---------------------------------------------------------

    pub fn assign_cmyk(&self, c: i32, m: i32, y: i32, k: i32) -> bool {
        let args = vec![
            Variant::from_i64(c as i64),
            Variant::from_i64(m as i64),
            Variant::from_i64(y as i64),
            Variant::from_i64(k as i64),
        ];
        self.disp.invoke_method("CMYKAssign", args).is_ok()
    }

    pub fn cmyk_cyan(&self) -> Option<i64> {
        self.prop_i64("CMYKCyan")
    }
    pub fn set_cmyk_cyan(&self, v: i32) -> bool {
        self.put_i64("CMYKCyan", v as i64)
    }

    pub fn cmyk_magenta(&self) -> Option<i64> {
        self.prop_i64("CMYKMagenta")
    }
    pub fn set_cmyk_magenta(&self, v: i32) -> bool {
        self.put_i64("CMYKMagenta", v as i64)
    }

    pub fn cmyk_yellow(&self) -> Option<i64> {
        self.prop_i64("CMYKYellow")
    }
    pub fn set_cmyk_yellow(&self, v: i32) -> bool {
        self.put_i64("CMYKYellow", v as i64)
    }

    pub fn cmyk_black(&self) -> Option<i64> {
        self.prop_i64("CMYKBlack")
    }
    pub fn set_cmyk_black(&self, v: i32) -> bool {
        self.put_i64("CMYKBlack", v as i64)
    }

    pub fn convert_to_cmyk(&self) -> bool {
        self.disp.invoke_method("ConvertToCMYK", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // HSB
    // ---------------------------------------------------------

    pub fn assign_hsb(&self, h: i32, s: i32, b: i32) -> bool {
        let args = vec![
            Variant::from_i64(h as i64),
            Variant::from_i64(s as i64),
            Variant::from_i64(b as i64),
        ];
        self.disp.invoke_method("HSBAssign", args).is_ok()
    }

    pub fn hsb_hue(&self) -> Option<i64> { self.prop_i64("HSBHue") }
    pub fn set_hsb_hue(&self, v: i32) -> bool { self.put_i64("HSBHue", v as i64) }

    pub fn hsb_saturation(&self) -> Option<i64> { self.prop_i64("HSBSaturation") }
    pub fn set_hsb_saturation(&self, v: i32) -> bool { self.put_i64("HSBSaturation", v as i64) }

    pub fn hsb_brightness(&self) -> Option<i64> { self.prop_i64("HSBBrightness") }
    pub fn set_hsb_brightness(&self, v: i32) -> bool { self.put_i64("HSBBrightness", v as i64) }

    pub fn convert_to_hsb(&self) -> bool {
        self.disp.invoke_method("ConvertToHSB", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // HLS
    // ---------------------------------------------------------

    pub fn assign_hls(&self, h: i32, l: i32, s: i32) -> bool {
        let args = vec![
            Variant::from_i64(h as i64),
            Variant::from_i64(l as i64),
            Variant::from_i64(s as i64),
        ];
        self.disp.invoke_method("HLSAssign", args).is_ok()
    }

    pub fn hls_hue(&self) -> Option<i64> { self.prop_i64("HLSHue") }
    pub fn set_hls_hue(&self, v: i32) -> bool { self.put_i64("HLSHue", v as i64) }

    pub fn hls_lightness(&self) -> Option<i64> { self.prop_i64("HLSLightness") }
    pub fn set_hls_lightness(&self, v: i32) -> bool { self.put_i64("HLSLightness", v as i64) }

    pub fn hls_saturation(&self) -> Option<i64> { self.prop_i64("HLSSaturation") }
    pub fn set_hls_saturation(&self, v: i32) -> bool { self.put_i64("HLSSaturation", v as i64) }

    pub fn convert_to_hls(&self) -> bool {
        self.disp.invoke_method("ConvertToHLS", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 鐏板害 / 榛戠櫧
    // ---------------------------------------------------------

    pub fn assign_gray(&self, gray: i32) -> bool {
        let args = vec![Variant::from_i64(gray as i64)];
        self.disp.invoke_method("GrayAssign", args).is_ok()
    }

    pub fn gray(&self) -> Option<i64> { self.prop_i64("Gray") }
    pub fn set_gray(&self, v: i32) -> bool { self.put_i64("Gray", v as i64) }

    pub fn convert_to_gray(&self) -> bool {
        self.disp.invoke_method("ConvertToGray", vec![]).is_ok()
    }

    pub fn assign_bw(&self, white: bool) -> bool {
        let args = vec![Variant::from_bool(white)];
        self.disp.invoke_method("BWAssign", args).is_ok()
    }

    pub fn bw(&self) -> Option<bool> { self.prop_bool("BW") }
    pub fn set_bw(&self, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property("BW", vec![arg]).is_ok()
    }

    pub fn convert_to_bw(&self) -> bool {
        self.disp.invoke_method("ConvertToBW", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 鍚嶇О / 瀛楃涓?
    // ---------------------------------------------------------

    /// `GetName(Components)` 鈥斺€?`Components=true` 鏃跺甫鍒嗛噺鎻忚堪銆?
    pub fn name(&self, components: bool) -> Option<String> {
        let args = vec![Variant::from_bool(components)];
        self.disp.invoke_method("Name", args).ok()?.to_string().ok()
    }

    pub fn set_name(&self, s: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(s.into())];
        self.disp.invoke_method("SetName", args).is_ok()
    }

    pub fn to_color_string(&self) -> Option<String> {
        self.disp.invoke_method("ToString", vec![]).ok()?.to_string().ok()
    }

    pub fn string_assign(&self, s: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(s.into())];
        self.disp
            .invoke_method("StringAssign", args)
            .ok()
            .and_then(|v| v.to_bool().ok())
            .unwrap_or(false)
    }

    pub fn hex_value(&self) -> Option<String> {
        self.prop_string("HexValue")
    }

    pub fn set_hex_value(&self, s: impl Into<String>) -> bool {
        self.put_string("HexValue", s)
    }

    // ---------------------------------------------------------
    // 澶嶅埗 / 姣旇緝
    // ---------------------------------------------------------

    pub fn copy_assign(&self, other: &IvgColor) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    pub fn get_copy(&self) -> Option<IvgColor> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColor::new)
    }

    pub fn is_same(&self, other: &IvgColor) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("IsSame", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn is_similar(&self, other: &IvgColor) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("IsSimilar", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 涓よ壊涔嬮棿鐨?璺濈"
    pub fn distance_from(&self, other: &IvgColor) -> Option<i64> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("GetColorDistanceFrom", args)
            .ok()?
            .to_i64()
            .ok()
    }

    // ---------------------------------------------------------
    // 鍒ゅ畾
    // ---------------------------------------------------------

    pub fn is_in_gamut(&self) -> Option<bool> { self.prop_bool("IsInGamut") }
    pub fn is_cmyk(&self) -> Option<bool> { self.prop_bool("IsCMYK") }
    pub fn is_gray(&self) -> Option<bool> { self.prop_bool("IsGray") }
    pub fn is_white(&self) -> Option<bool> { self.prop_bool("IsWhite") }
    pub fn is_spot(&self) -> Option<bool> { self.prop_bool("IsSpot") }
    pub fn is_tintable(&self) -> Option<bool> { self.prop_bool("IsTintable") }

    // ---------------------------------------------------------
    // 璋冭壊鏉?/ 涓撹壊
    // ---------------------------------------------------------

    pub fn palette_id(&self) -> Option<i64> { self.prop_i64("PaletteID") }

    pub fn palette_index(&self) -> Option<i64> { self.prop_i64("PaletteIndex") }
    pub fn set_palette_index(&self, v: i32) -> bool {
        self.put_i64("PaletteIndex", v as i64)
    }

    pub fn tint(&self) -> Option<i64> { self.prop_i64("Tint") }
    pub fn set_tint(&self, v: i32) -> bool { self.put_i64("Tint", v as i64) }

    pub fn spot_color_id(&self) -> Option<i64> { self.prop_i64("SpotColorID") }
    pub fn set_spot_color_id(&self, v: i32) -> bool {
        self.put_i64("SpotColorID", v as i64)
    }

    pub fn spot_color_name(&self) -> Option<String> { self.prop_string("SpotColorName") }

    pub fn palette_identifier(&self) -> Option<String> {
        self.prop_string("PaletteIdentifier")
    }

    // ---------------------------------------------------------
    // 涓婁笅鏂?
    // ---------------------------------------------------------

    pub fn color_context(&self) -> Option<IvgColorContext> {
        self.prop_dispatch("ColorContext").map(IvgColorContext::new)
    }

    // ---------------------------------------------------------
    // 杈呭姪
    // ---------------------------------------------------------
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }
}
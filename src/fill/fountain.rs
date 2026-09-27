//! `IVGFountainFill` 鈥斺€?娓愬彉濉厖
//! `IVGFountainColor(s)` 鈥斺€?娓愬彉涓婄殑棰滆壊鍋滈潬鐐?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;

// =============================================================
// IvgFountainFill
// =============================================================

pub struct IvgFountainFill {
    disp: ComObject,
}

impl IvgFountainFill {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
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

    fn put_color(&self, name: &str, c: &IvgColor) -> bool {
        self.disp.set_property(name, vec![c.as_variant()]).is_ok()
    }

    // ---- 绫诲瀷 ----

    /// `cdrFountainFillType`
    pub fn fountain_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_fountain_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    // ---- 璧锋鍧愭爣 ----

    pub fn start_x(&self) -> Option<f64> { self.prop_f64("StartX") }
    pub fn set_start_x(&self, v: f64) -> bool { self.put_f64("StartX", v) }

    pub fn start_y(&self) -> Option<f64> { self.prop_f64("StartY") }
    pub fn set_start_y(&self, v: f64) -> bool { self.put_f64("StartY", v) }

    pub fn end_x(&self) -> Option<f64> { self.prop_f64("EndX") }
    pub fn set_end_x(&self, v: f64) -> bool { self.put_f64("EndX", v) }

    pub fn end_y(&self) -> Option<f64> { self.prop_f64("EndY") }
    pub fn set_end_y(&self, v: f64) -> bool { self.put_f64("EndY", v) }

    pub fn end2_x(&self) -> Option<f64> { self.prop_f64("End2X") }
    pub fn set_end2_x(&self, v: f64) -> bool { self.put_f64("End2X", v) }

    pub fn end2_y(&self) -> Option<f64> { self.prop_f64("End2Y") }
    pub fn set_end2_y(&self, v: f64) -> bool { self.put_f64("End2Y", v) }

    // ---- 瑙掑害 ----

    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }

    pub fn set_angle(&self, v: f64) -> bool {
        let args = vec![Variant::from_f64(v)];
        self.disp.invoke_method("SetAngle", args).is_ok()
    }

    // ---- 姝ヨ繘 / 娣峰悎 ----

    pub fn steps(&self) -> Option<i64> { self.prop_i64("Steps") }
    pub fn set_steps(&self, v: i32) -> bool { self.put_i64("Steps", v as i64) }

    pub fn edge_pad(&self) -> Option<i64> { self.prop_i64("EdgePad") }
    pub fn set_edge_pad(&self, v: i32) -> bool {
        let args = vec![Variant::from_i64(v as i64)];
        self.disp.invoke_method("SetEdgePad", args).is_ok()
    }

    /// `cdrFountainFillBlendType`
    pub fn blend_type(&self) -> Option<i64> { self.prop_i64("BlendType") }
    pub fn set_blend_type(&self, v: i32) -> bool { self.put_i64("BlendType", v as i64) }

    pub fn mid_point(&self) -> Option<i64> { self.prop_i64("MidPoint") }
    pub fn set_mid_point(&self, v: i32) -> bool { self.put_i64("MidPoint", v as i64) }

    pub fn blend_acceleration(&self) -> Option<f64> {
        self.prop_f64("BlendAcceleration")
    }
    pub fn set_blend_acceleration(&self, v: f64) -> bool {
        self.put_f64("BlendAcceleration", v)
    }

    // ---- 棰滆壊 ----

    pub fn colors(&self) -> Option<IvgFountainColors> {
        self.prop_dispatch("Colors").map(IvgFountainColors::new)
    }

    pub fn set_colors(&self, colors: &IvgFountainColors) -> bool {
        self.disp
            .set_property("Colors", vec![colors.as_variant()])
            .is_ok()
    }

    pub fn start_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("StartColor").map(IvgColor::new)
    }
    pub fn set_start_color(&self, c: &IvgColor) -> bool {
        self.put_color("StartColor", c)
    }

    pub fn end_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("EndColor").map(IvgColor::new)
    }
    pub fn set_end_color(&self, c: &IvgColor) -> bool {
        self.put_color("EndColor", c)
    }

    // ---- 涓績鍋忕Щ ----

    pub fn center_offset_x(&self) -> Option<f64> {
        self.prop_f64("CenterOffsetX")
    }
    pub fn set_center_offset_x(&self, v: f64) -> bool {
        self.put_f64("CenterOffsetX", v)
    }

    pub fn center_offset_y(&self) -> Option<f64> {
        self.prop_f64("CenterOffsetY")
    }
    pub fn set_center_offset_y(&self, v: f64) -> bool {
        self.put_f64("CenterOffsetY", v)
    }

    // ---- 骞虫粦 / 鎵╂暎 ----

    pub fn smooth_blend(&self) -> Option<bool> { self.prop_bool("SmoothBlend") }
    pub fn set_smooth_blend(&self, v: bool) -> bool { self.put_bool("SmoothBlend", v) }

    /// `cdrFountainFillSpreadMethod`
    pub fn spread_method(&self) -> Option<i64> {
        self.prop_i64("SpreadMethod")
    }
    pub fn set_spread_method(&self, v: i32) -> bool {
        self.put_i64("SpreadMethod", v as i64)
    }

    pub fn anisotropic(&self) -> Option<bool> { self.prop_bool("Anisotropic") }
    pub fn set_anisotropic(&self, v: bool) -> bool { self.put_bool("Anisotropic", v) }

    // ---- 閫忔槑 / 娣峰悎妯″紡 ----

    pub fn is_transparent(&self) -> Option<bool> { self.prop_bool("IsTransparent") }

    /// `cdrMergeMode`
    pub fn merge_mode(&self) -> Option<i64> { self.prop_i64("MergeMode") }
    pub fn set_merge_mode(&self, v: i32) -> bool { self.put_i64("MergeMode", v as i64) }

    // ---- 缂╂斁 / 鍊炬枩 ----

    pub fn scale_x(&self) -> Option<f64> { self.prop_f64("ScaleX") }
    pub fn set_scale_x(&self, v: f64) -> bool { self.put_f64("ScaleX", v) }

    pub fn scale_y(&self) -> Option<f64> { self.prop_f64("ScaleY") }
    pub fn set_scale_y(&self, v: f64) -> bool { self.put_f64("ScaleY", v) }

    pub fn skew(&self) -> Option<f64> { self.prop_f64("Skew") }
    pub fn set_skew(&self, v: f64) -> bool { self.put_f64("Skew", v) }

    // ---- 鍙樻崲鐭╅樀 ----

    /// 杩斿洖 (d11, d12, d21, d22)
    pub fn get_transformations(&self) -> Option<(f64, f64, f64, f64)> {
        // 4 涓?out 鍙傛暟鍦?IDispatch 閲屾嬁涓嶅埌锛岃繑鍥?None 鍗犱綅锛?
        // 鐢?ScaleX / ScaleY / Skew 鏇夸唬銆?
        None
    }

    pub fn set_transformations(&self, d11: f64, d12: f64, d21: f64, d22: f64) -> bool {
        let args = vec![
            Variant::from_f64(d11),
            Variant::from_f64(d12),
            Variant::from_f64(d21),
            Variant::from_f64(d22),
        ];
        self.disp.invoke_method("SetTransformations", args).is_ok()
    }

    // ---- 娣峰悎妫€娴?----

    pub fn has_hsb_blends(&self) -> Option<bool> { self.prop_bool("HasHSBBlends") }
    pub fn has_non_linear_blends(&self) -> Option<bool> {
        self.prop_bool("HasNonLinearBlends")
    }

    // ---- 鎿嶄綔 ----

    pub fn translate(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("Translate", args).is_ok()
    }

    pub fn make_opaque(&self) -> bool {
        self.disp.invoke_method("MakeOpaque", vec![]).is_ok()
    }
}

// =============================================================
// IvgFountainColors
// =============================================================

pub struct IvgFountainColors {
    disp: ComObject,
}

impl IvgFountainColors {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgFountainColor> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgFountainColor::new)
    }

    pub fn first(&self) -> Option<IvgFountainColor> {
        self.prop_dispatch("First").map(IvgFountainColor::new)
    }

    pub fn last(&self) -> Option<IvgFountainColor> {
        self.prop_dispatch("Last").map(IvgFountainColor::new)
    }

    /// 娣诲姞棰滆壊鍋滈潬鐐广€?
    pub fn add(&self, color: &IvgColor, position: i32) -> bool {
        let args = vec![
            color.as_variant(),
            Variant::from_i64(position as i64),
        ];
        self.disp.invoke_method("Add", args).is_ok()
    }

    /// 娣诲姞鐏板害鍋滈潬鐐广€?
    pub fn add_gray_level(&self, gray_level: i32, position: i32) -> bool {
        let args = vec![
            Variant::from_i64(gray_level as i64),
            Variant::from_i64(position as i64),
        ];
        self.disp.invoke_method("AddGrayLevel", args).is_ok()
    }

    /// 鑾峰彇/璁剧疆鐏板害绾у埆銆?
    pub fn gray_level(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("GrayLevel", args).ok()?.to_i64().ok()
    }

    pub fn set_gray_level(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_GrayLevel", args).is_ok()
    }

    /// 鍒犻櫎鍏ㄩ儴锛堥€氳繃 `set_colors` 瑕嗙洊锛夈€?
    pub fn clear(&self) -> bool {
        // 娌℃湁鐩存帴 Clear 鏂规硶锛屾敼鐢?`IvgFountainFill::set_colors` 浼犳柊闆嗗悎銆?
        false
    }
}

// =============================================================
// IvgFountainColor
// =============================================================

pub struct IvgFountainColor {
    disp: ComObject,
}

impl IvgFountainColor {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_u8(&self, name: &str) -> Option<u8> {
        self.disp.get_property(name).ok()?.to_u8().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn position(&self) -> Option<i64> { self.prop_i64("Position") }

    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    pub fn set_color(&self, c: &IvgColor) -> bool {
        self.disp.set_property("Color", vec![c.as_variant()]).is_ok()
    }

    pub fn mid_point(&self) -> Option<i64> { self.prop_i64("MidPoint") }
    pub fn set_mid_point(&self, v: i32) -> bool { self.put_i64("MidPoint", v as i64) }

    /// `cdrFountainFillBlendType`
    pub fn blend_type(&self) -> Option<i64> { self.prop_i64("BlendType") }
    pub fn set_blend_type(&self, v: i32) -> bool {
        self.put_i64("BlendType", v as i64)
    }

    /// 0..255
    pub fn opacity(&self) -> Option<u8> { self.prop_u8("Opacity") }
    pub fn set_opacity(&self, v: u8) -> bool {
        let arg = Variant::from_i64(v as i64);
        self.disp.set_property("Opacity", vec![arg]).is_ok()
    }

    pub fn move_to(&self, new_position: i32) -> bool {
        let args = vec![Variant::from_i64(new_position as i64)];
        self.disp.invoke_method("Move", args).is_ok()
    }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}
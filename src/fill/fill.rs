//! `IVGFill` 鈥斺€?濉厖瀵硅薄鎬诲叆鍙?
//!
//! 閫氳繃 `shape.fill()` 鎴?`layer.create_...()` 寰楀埌銆?
//! `Type` 灞炴€у尯鍒嗗～鍏呯被鍨嬶紙`cdrFillType`锛夛紝鍐嶇敤瀵瑰簲 getter 鍙栧己绫诲瀷锛?
//! - `.uniform_color()`    鈫?`IvgColor`
//! - `.fountain()`         鈫?`IvgFountainFill`
//! - `.pattern()`          鈫?`IvgPatternFill`
//! - `.texture()`          鈫?`IvgTextureFill`
//! - `.postscript()`       鈫?`IvgPostScriptFill`
//! - `.hatch()`            鈫?`IvgHatchFill`

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::fill::{
    IvgFountainFill, IvgHatchFill, IvgPatternFill, IvgPostScriptFill,
    IvgTextureFill, IvgPSScreenOptions
};

pub struct IvgFill {
    disp: ComObject,
}

impl IvgFill {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    // ---------------------------------------------------------
    // 宸ュ叿
    // ---------------------------------------------------------

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_dispatch(&self, name: &str, d: Variant) -> bool {
        self.disp.set_property(name, vec![d]).is_ok()
    }

    // ---------------------------------------------------------
    // 绫诲瀷
    // ---------------------------------------------------------

    /// `cdrFillType` 鈥斺€?瑙?`enums/fill`
    pub fn fill_type(&self) -> Option<i64> {
        self.prop_i64("Type")
    }

    // ---------------------------------------------------------
    // 鍗曡壊
    // ---------------------------------------------------------

    pub fn uniform_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("UniformColor").map(IvgColor::new)
    }

    pub fn set_uniform_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("UniformColor", c.as_variant())
    }

    // ---------------------------------------------------------
    // 鍚勭被鍨嬪瓙瀵硅薄
    // ---------------------------------------------------------

    pub fn fountain(&self) -> Option<IvgFountainFill> {
        self.prop_dispatch("Fountain").map(IvgFountainFill::new)
    }

    pub fn pattern(&self) -> Option<IvgPatternFill> {
        self.prop_dispatch("Pattern").map(IvgPatternFill::new)
    }

    pub fn texture(&self) -> Option<IvgTextureFill> {
        self.prop_dispatch("Texture").map(IvgTextureFill::new)
    }

    pub fn postscript(&self) -> Option<IvgPostScriptFill> {
        self.prop_dispatch("PostScript").map(IvgPostScriptFill::new)
    }

    pub fn hatch(&self) -> Option<IvgHatchFill> {
        self.prop_dispatch("Hatch").map(IvgHatchFill::new)
    }

    // ---------------------------------------------------------
    // 蹇嵎搴旂敤
    // ---------------------------------------------------------

    pub fn apply_no_fill(&self) -> bool {
        self.disp.invoke_method("ApplyNoFill", vec![]).is_ok()
    }

    pub fn apply_uniform_fill(&self, color: &IvgColor) -> bool {
        let args = vec![color.as_variant()];
        self.disp.invoke_method("ApplyUniformFill", args).is_ok()
    }

    /// 搴旂敤娓愬彉濉厖锛岃繑鍥炴柊寤虹殑 `IvgFountainFill`銆?
    #[allow(clippy::too_many_arguments)]
    pub fn apply_fountain_fill(
        &self,
        start_color: &IvgColor,
        end_color: &IvgColor,
        fill_type: i32,
        angle: f64,
        steps: i32,
        edge_pad: i32,
        mid_point: i32,
        blend_type: i32,
        center_offset_x: f64,
        center_offset_y: f64,
    ) -> Option<IvgFountainFill> {
        let args = vec![
            start_color.as_variant(),
            end_color.as_variant(),
            Variant::from_i64(fill_type as i64),
            Variant::from_f64(angle),
            Variant::from_i64(steps as i64),
            Variant::from_i64(edge_pad as i64),
            Variant::from_i64(mid_point as i64),
            Variant::from_i64(blend_type as i64),
            Variant::from_f64(center_offset_x),
            Variant::from_f64(center_offset_y),
        ];
        self.disp
            .invoke_method("ApplyFountainFill", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgFountainFill::new)
    }

    /// 搴旂敤鍥炬濉厖銆?
    pub fn apply_pattern_fill(
        &self,
        pattern_type: i32,
        file_name: impl Into<String>,
        canvas_index: i32,
        front_color: &IvgColor,
        back_color: &IvgColor,
        transform_with_shape: bool,
    ) -> Option<IvgPatternFill> {
        let args = vec![
            Variant::from_i64(pattern_type as i64),
            Variant::from_str(file_name.into()),
            Variant::from_i64(canvas_index as i64),
            front_color.as_variant(),
            back_color.as_variant(),
            Variant::from_bool(transform_with_shape),
        ];
        self.disp
            .invoke_method("ApplyPatternFill", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPatternFill::new)
    }

    /// 搴旂敤绾圭悊濉厖銆?
    pub fn apply_texture_fill(
        &self,
        texture_name: impl Into<String>,
        library_name: impl Into<String>,
    ) -> Option<IvgTextureFill> {
        let args = vec![
            Variant::from_str(texture_name.into()),
            Variant::from_str(library_name.into()),
        ];
        self.disp
            .invoke_method("ApplyTextureFill", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextureFill::new)
    }

    /// 搴旂敤 PostScript 濉厖銆?
    pub fn apply_postscript_fill(&self, index_or_name: Variant) -> Option<IvgPostScriptFill> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("ApplyPostscriptFill", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPostScriptFill::new)
    }

    /// 搴旂敤鑷畾涔夊墫闈㈢嚎濉厖銆?
    #[allow(clippy::too_many_arguments)]
    pub fn apply_custom_hatch_fill(
        &self,
        angle: f64,
        spacing: f64,
        shift: f64,
        origin_x: f64,
        origin_y: f64,
        width: f64,
        color: &IvgColor,
        style: &crate::outline::IvgOutlineStyle,
        dash_dot_length: f64,
        pen_width: f64,
        back_color: &IvgColor,
        transform_with_shape: bool,
        scale_lines_with_shape: bool,
        use_world_coordinates: bool,
        fill_scale: f64,
        line_scale: f64,
        fill_angle: f64,
        fill_skew: f64,
    ) -> Option<IvgHatchFill> {
        let args = vec![
            Variant::from_f64(angle),
            Variant::from_f64(spacing),
            Variant::from_f64(shift),
            Variant::from_f64(origin_x),
            Variant::from_f64(origin_y),
            Variant::from_f64(width),
            color.as_variant(),
            style.as_variant(),
            Variant::from_f64(dash_dot_length),
            Variant::from_f64(pen_width),
            back_color.as_variant(),
            Variant::from_bool(transform_with_shape),
            Variant::from_bool(scale_lines_with_shape),
            Variant::from_bool(use_world_coordinates),
            Variant::from_f64(fill_scale),
            Variant::from_f64(line_scale),
            Variant::from_f64(fill_angle),
            Variant::from_f64(fill_skew),
        ];
        self.disp
            .invoke_method("ApplyCustomHatchFill", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgHatchFill::new)
    }

    /// 搴旂敤鍓栭潰绾垮簱濉厖銆?
    #[allow(clippy::too_many_arguments)]
    pub fn apply_hatch_fill(
        &self,
        library_name: impl Into<String>,
        hatch_name_or_index: Variant,
        back_color: &IvgColor,
        transform_with_shape: bool,
        scale_lines_with_shape: bool,
        use_world_coordinates: bool,
        fill_scale: f64,
        line_scale: f64,
        fill_angle: f64,
        fill_skew: f64,
    ) -> Option<IvgHatchFill> {
        let args = vec![
            Variant::from_str(library_name.into()),
            hatch_name_or_index,
            back_color.as_variant(),
            Variant::from_bool(transform_with_shape),
            Variant::from_bool(scale_lines_with_shape),
            Variant::from_bool(use_world_coordinates),
            Variant::from_f64(fill_scale),
            Variant::from_f64(line_scale),
            Variant::from_f64(fill_angle),
            Variant::from_f64(fill_skew),
        ];
        self.disp
            .invoke_method("ApplyHatchFill", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgHatchFill::new)
    }

    // ---------------------------------------------------------
    // 澶嶅埗 / 姣旇緝
    // ---------------------------------------------------------

    pub fn get_copy(&self) -> Option<IvgFill> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgFill::new)
    }

    pub fn copy_assign(&self, other: &IvgFill) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    pub fn compare_with(&self, other: &IvgFill) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("CompareWith", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 瀛楃涓茶〃绀?
    // ---------------------------------------------------------

    pub fn to_fill_string(&self) -> Option<String> {
        self.disp.invoke_method("ToString", vec![]).ok()?.to_string().ok()
    }

    pub fn string_assign(&self, s: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(s.into())];
        self.disp
            .invoke_method("StringAssign", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 灞忓箷閫夐」锛堢敤浜?PostScript / PS锛?
    // ---------------------------------------------------------

    pub fn ps_screen(&self) -> Option<IvgPSScreenOptions> {
        self.prop_dispatch("PSScreen").map(IvgPSScreenOptions::new)
    }
}
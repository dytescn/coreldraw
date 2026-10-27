//! `IVGFill` —— 填充对象总入口
//!
//! 通过 `shape.fill()` 或 `layer.create_...()` 得到。
//! `Type` 属性区分填充类型（`cdrFillType`），再用对应 getter 取强类型）
//! - `.uniform_color()`    → `IvgColor`
//! - `.fountain()`         → `IvgFountainFill`
//! - `.pattern()`          → `IvgPatternFill`
//! - `.texture()`          → `IvgTextureFill`
//! - `.postscript()`       → `IvgPostScriptFill`
//! - `.hatch()`            → `IvgHatchFill`

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::fill::{
    IvgFountainFill, IvgHatchFill, IvgPatternFill, IvgPostScriptFill,
    IvgTextureFill, IvgPSScreenOptions
};

/// CorelDRAW `IVGFill` 对象的后期绑定包装（填充）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgFill {
    disp: ComObject,
}

impl IvgFill {
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

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    // ---------------------------------------------------------
    // 工具
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
    // 类型
    // ---------------------------------------------------------

    /// `cdrFillType` ——见 `enums/fill`
    pub fn fill_type(&self) -> Option<i64> {
        self.prop_i64("Type")
    }

    // ---------------------------------------------------------
    // 单色
    // ---------------------------------------------------------

    /// 读取均匀颜色（`UniformColor`）。
    pub fn uniform_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("UniformColor").map(IvgColor::new)
    }

    /// 设置均匀颜色（`UniformColor`）。
    pub fn set_uniform_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("UniformColor", c.as_variant())
    }

    // ---------------------------------------------------------
    // 各类型子对象
    // ---------------------------------------------------------

    /// 读取渐变（`Fountain`）。
    pub fn fountain(&self) -> Option<IvgFountainFill> {
        self.prop_dispatch("Fountain").map(IvgFountainFill::new)
    }

    /// 读取图案（`Pattern`）。
    pub fn pattern(&self) -> Option<IvgPatternFill> {
        self.prop_dispatch("Pattern").map(IvgPatternFill::new)
    }

    /// 读取纹理（`Texture`）。
    pub fn texture(&self) -> Option<IvgTextureFill> {
        self.prop_dispatch("Texture").map(IvgTextureFill::new)
    }

    /// 读取 PostScript（`PostScript`）。
    pub fn postscript(&self) -> Option<IvgPostScriptFill> {
        self.prop_dispatch("PostScript").map(IvgPostScriptFill::new)
    }

    /// 读取剖面线（`Hatch`）。
    pub fn hatch(&self) -> Option<IvgHatchFill> {
        self.prop_dispatch("Hatch").map(IvgHatchFill::new)
    }

    // ---------------------------------------------------------
    // 快捷应用
    // ---------------------------------------------------------

    /// 取消填充（COM 方法 `ApplyNoFill`）。
    pub fn apply_no_fill(&self) -> bool {
        self.disp.invoke_method("ApplyNoFill", vec![]).is_ok()
    }

    /// 应用均匀填充（COM 方法 `ApplyUniformFill`）。
    pub fn apply_uniform_fill(&self, color: &IvgColor) -> bool {
        let args = vec![color.as_variant()];
        self.disp.invoke_method("ApplyUniformFill", args).is_ok()
    }

    /// 应用渐变填充，返回新建的 `IvgFountainFill`。
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

    /// 应用图案填充。
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

    /// 应用纹理填充。
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

    /// 应用 PostScript 填充。
    pub fn apply_postscript_fill(&self, index_or_name: Variant) -> Option<IvgPostScriptFill> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("ApplyPostscriptFill", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPostScriptFill::new)
    }

    /// 应用自定义剖面线填充。
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

    /// 应用剖面线库填充。
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
    // 复制 / 比较
    // ---------------------------------------------------------

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgFill> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgFill::new)
    }

    /// 复制 assign（COM 方法 `CopyAssign`）。
    pub fn copy_assign(&self, other: &IvgFill) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    /// 比较（COM 方法 `CompareWith`）。
    pub fn compare_with(&self, other: &IvgFill) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("CompareWith", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 字符串表社
    // ---------------------------------------------------------

    /// 转换为填充字符串（COM 方法 `ToString`）。
    pub fn to_fill_string(&self) -> Option<String> {
        self.disp.invoke_method("ToString", vec![]).ok()?.to_string().ok()
    }

    /// 字符串 assign（COM 方法 `StringAssign`）。
    pub fn string_assign(&self, s: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(s.into())];
        self.disp
            .invoke_method("StringAssign", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 屏幕选项（用于 PostScript / PS）
    // ---------------------------------------------------------

    /// 读取 COM 属性 `PSScreen`。
    pub fn ps_screen(&self) -> Option<IvgPSScreenOptions> {
        self.prop_dispatch("PSScreen").map(IvgPSScreenOptions::new)
    }
}
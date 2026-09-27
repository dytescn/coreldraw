//! `IVGTransparency` 鈥斺€?鍥惧舰閫忔槑

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::fill::{IvgFountainFill, IvgPatternFill, IvgTextureFill};

pub struct IvgTransparency {
    disp: ComObject,
}

impl IvgTransparency {
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

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---------------------------------------------------------
    // 绫诲瀷
    // ---------------------------------------------------------

    /// `cdrTransparencyType`
    pub fn transparency_type(&self) -> Option<i64> { self.prop_i64("Type") }

    // ---------------------------------------------------------
    // 鍗曡壊
    // ---------------------------------------------------------

    pub fn uniform(&self) -> Option<i64> { self.prop_i64("Uniform") }
    pub fn set_uniform(&self, v: i32) -> bool { self.put_i64("Uniform", v as i64) }

    // ---------------------------------------------------------
    // 娓愬彉
    // ---------------------------------------------------------

    pub fn fountain(&self) -> Option<IvgFountainFill> {
        self.prop_dispatch("Fountain").map(IvgFountainFill::new)
    }
    pub fn set_fountain(&self, f: &IvgFountainFill) -> bool {
        self.put_dispatch("Fountain", f.as_variant())
    }

    // ---------------------------------------------------------
    // 鍥炬 / 绾圭悊
    // ---------------------------------------------------------

    pub fn pattern(&self) -> Option<IvgPatternFill> {
        self.prop_dispatch("Pattern").map(IvgPatternFill::new)
    }
    pub fn set_pattern(&self, f: &IvgPatternFill) -> bool {
        self.put_dispatch("Pattern", f.as_variant())
    }

    pub fn texture(&self) -> Option<IvgTextureFill> {
        self.prop_dispatch("Texture").map(IvgTextureFill::new)
    }
    pub fn set_texture(&self, f: &IvgTextureFill) -> bool {
        self.put_dispatch("Texture", f.as_variant())
    }

    // ---------------------------------------------------------
    // 璧锋
    // ---------------------------------------------------------

    pub fn start(&self) -> Option<i64> { self.prop_i64("Start") }
    pub fn set_start(&self, v: i32) -> bool { self.put_i64("Start", v as i64) }

    pub fn end(&self) -> Option<i64> { self.prop_i64("End") }
    pub fn set_end(&self, v: i32) -> bool { self.put_i64("End", v as i64) }

    // ---------------------------------------------------------
    // 鐘舵€?
    // ---------------------------------------------------------

    pub fn frozen(&self) -> Option<bool> { self.prop_bool("Frozen") }

    // ---------------------------------------------------------
    // 搴旂敤
    // ---------------------------------------------------------

    pub fn apply_no_transparency(&self) -> bool {
        self.disp.invoke_method("ApplyNoTransparency", vec![]).is_ok()
    }

    pub fn apply_uniform_transparency(&self, value: i32) -> bool {
        let args = vec![Variant::from_i64(value as i64)];
        self.disp.invoke_method("ApplyUniformTransparency", args).is_ok()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn apply_fountain_transparency(
        &self,
        start: i32, end: i32,
        fill_type: i32, angle: f64, steps: i32,
        edge_pad: i32, mid_point: i32,
        center_offset_x: f64, center_offset_y: f64,
    ) -> Option<IvgFountainFill> {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(end as i64),
            Variant::from_i64(fill_type as i64),
            Variant::from_f64(angle),
            Variant::from_i64(steps as i64),
            Variant::from_i64(edge_pad as i64),
            Variant::from_i64(mid_point as i64),
            Variant::from_f64(center_offset_x),
            Variant::from_f64(center_offset_y),
        ];
        self.disp
            .invoke_method("ApplyFountainTransparency", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgFountainFill::new)
    }

    pub fn apply_pattern_transparency(
        &self,
        pattern_type: i32,
        file_name: impl Into<String>,
        canvas_index: i32,
        front: i32,
        back: i32,
        transform_with_shape: bool,
    ) -> Option<IvgPatternFill> {
        let args = vec![
            Variant::from_i64(pattern_type as i64),
            Variant::from_str(file_name.into()),
            Variant::from_i64(canvas_index as i64),
            Variant::from_i64(front as i64),
            Variant::from_i64(back as i64),
            Variant::from_bool(transform_with_shape),
        ];
        self.disp
            .invoke_method("ApplyPatternTransparency", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPatternFill::new)
    }

    pub fn apply_texture_transparency(
        &self,
        texture_name: impl Into<String>,
        library_name: impl Into<String>,
        front: i32,
        back: i32,
    ) -> Option<IvgTextureFill> {
        let args = vec![
            Variant::from_str(texture_name.into()),
            Variant::from_str(library_name.into()),
            Variant::from_i64(front as i64),
            Variant::from_i64(back as i64),
        ];
        self.disp
            .invoke_method("ApplyTextureTransparency", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextureFill::new)
    }

    // ---------------------------------------------------------
    // 鍐荤粨 / 瑙ｅ喕
    // ---------------------------------------------------------

    pub fn freeze(&self) -> bool {
        self.disp.invoke_method("Freeze", vec![]).is_ok()
    }
    pub fn unfreeze(&self) -> bool {
        self.disp.invoke_method("Unfreeze", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 搴旂敤瀵硅薄 / 娣峰悎妯″紡
    // ---------------------------------------------------------

    /// `cdrTransparencyAppliedTo`
    pub fn applied_to(&self) -> Option<i64> { self.prop_i64("AppliedTo") }
    pub fn set_applied_to(&self, v: i32) -> bool {
        self.put_i64("AppliedTo", v as i64)
    }

    /// `cdrMergeMode`
    pub fn merge_mode(&self) -> Option<i64> { self.prop_i64("MergeMode") }
    pub fn set_merge_mode(&self, v: i32) -> bool {
        self.put_i64("MergeMode", v as i64)
    }

    // ---------------------------------------------------------
    // 浜や簰寮忚缃?
    // ---------------------------------------------------------

    pub fn user_assign(
        &self,
        transparency_type: i32,
        pattern_type: i32,
        parent_window_handle: i32,
    ) -> Option<bool> {
        let args = vec![
            Variant::from_i64(transparency_type as i64),
            Variant::from_i64(pattern_type as i64),
            Variant::from_i64(parent_window_handle as i64),
        ];
        self.disp
            .invoke_method("UserAssign", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
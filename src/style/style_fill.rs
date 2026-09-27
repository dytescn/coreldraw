//! `IVGStyleFill` 鈥斺€?鏍峰紡閲岀殑濉厖閮ㄥ垎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgStyleFill {
    disp: ComObject,
}

impl IvgStyleFill {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
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

    fn put_u8(&self, name: &str, v: u8) -> bool {
        let arg = Variant::from_i64(v as i64);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 绫诲瀷 ----

    /// `cdrFillStyleType`
    pub fn fill_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_fill_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    // ---- 鍙犲嵃 / 缂犵粫 ----

    pub fn overprint(&self) -> Option<bool> { self.prop_bool("Overprint") }
    pub fn set_overprint(&self, v: bool) -> bool { self.put_bool("Overprint", v) }

    pub fn winding_fill(&self) -> Option<bool> { self.prop_bool("WindingFill") }
    pub fn set_winding_fill(&self, v: bool) -> bool { self.put_bool("WindingFill", v) }

    // ---- 娓愬彉 ----

    /// `cdrFountainFillType`
    pub fn fountain_fill_type(&self) -> Option<i64> { self.prop_i64("FountainFillType") }
    pub fn set_fountain_fill_type(&self, v: i32) -> bool {
        self.put_i64("FountainFillType", v as i64)
    }

    pub fn edge_pad(&self) -> Option<i64> { self.prop_i64("EdgePad") }
    pub fn set_edge_pad(&self, v: i32) -> bool { self.put_i64("EdgePad", v as i64) }

    pub fn fountain_center_offset_x(&self) -> Option<i64> {
        self.prop_i64("FountainCenterOffsetX")
    }
    pub fn set_fountain_center_offset_x(&self, v: i32) -> bool {
        self.put_i64("FountainCenterOffsetX", v as i64)
    }

    pub fn fountain_center_offset_y(&self) -> Option<i64> {
        self.prop_i64("FountainCenterOffsetY")
    }
    pub fn set_fountain_center_offset_y(&self, v: i32) -> bool {
        self.put_i64("FountainCenterOffsetY", v as i64)
    }

    pub fn fountain_steps(&self) -> Option<i64> { self.prop_i64("FountainSteps") }
    pub fn set_fountain_steps(&self, v: i32) -> bool {
        self.put_i64("FountainSteps", v as i64)
    }

    /// `cdrFountainFillBlendType`
    pub fn fountain_blend_type(&self) -> Option<i64> {
        self.prop_i64("FountainBlendType")
    }
    pub fn set_fountain_blend_type(&self, v: i32) -> bool {
        self.put_i64("FountainBlendType", v as i64)
    }

    pub fn mid_point(&self) -> Option<i64> { self.prop_i64("MidPoint") }
    pub fn set_mid_point(&self, v: i32) -> bool { self.put_i64("MidPoint", v as i64) }

    pub fn flip_colors(&self) -> Option<bool> { self.prop_bool("FlipColors") }
    pub fn set_flip_colors(&self, v: bool) -> bool { self.put_bool("FlipColors", v) }

    // ---- PostScript ----

    pub fn postscript_name(&self) -> Option<String> { self.prop_string("PostScriptName") }
    pub fn set_postscript_name(&self, v: impl Into<String>) -> bool {
        self.put_string("PostScriptName", v)
    }

    // ---- 骞抽摵 ----

    pub fn tile_width(&self) -> Option<f64> { self.prop_f64("TileWidth") }
    pub fn set_tile_width(&self, v: f64) -> bool { self.put_f64("TileWidth", v) }

    pub fn tile_height(&self) -> Option<f64> { self.prop_f64("TileHeight") }
    pub fn set_tile_height(&self, v: f64) -> bool { self.put_f64("TileHeight", v) }

    pub fn tile_origin_x(&self) -> Option<f64> { self.prop_f64("TileOriginX") }
    pub fn set_tile_origin_x(&self, v: f64) -> bool { self.put_f64("TileOriginX", v) }

    pub fn tile_origin_y(&self) -> Option<f64> { self.prop_f64("TileOriginY") }
    pub fn set_tile_origin_y(&self, v: f64) -> bool { self.put_f64("TileOriginY", v) }

    /// `cdrTileOffsetType`
    pub fn tile_offset_type(&self) -> Option<i64> { self.prop_i64("TileOffsetType") }
    pub fn set_tile_offset_type(&self, v: i32) -> bool {
        self.put_i64("TileOffsetType", v as i64)
    }

    pub fn tile_offset(&self) -> Option<i64> { self.prop_i64("TileOffset") }
    pub fn set_tile_offset(&self, v: i32) -> bool { self.put_i64("TileOffset", v as i64) }

    // ---- 鏃嬭浆 / 鍊炬枩 / 闀滃儚 / 鍙樻崲 ----

    pub fn rotation_angle(&self) -> Option<f64> { self.prop_f64("RotationAngle") }
    pub fn set_rotation_angle(&self, v: f64) -> bool { self.put_f64("RotationAngle", v) }

    pub fn skew_angle(&self) -> Option<f64> { self.prop_f64("SkewAngle") }
    pub fn set_skew_angle(&self, v: f64) -> bool { self.put_f64("SkewAngle", v) }

    pub fn mirror_fill(&self) -> Option<bool> { self.prop_bool("MirrorFill") }
    pub fn set_mirror_fill(&self, v: bool) -> bool { self.put_bool("MirrorFill", v) }

    pub fn mirror_fill_x(&self) -> Option<bool> { self.prop_bool("MirrorFillX") }
    pub fn set_mirror_fill_x(&self, v: bool) -> bool { self.put_bool("MirrorFillX", v) }

    pub fn mirror_fill_y(&self) -> Option<bool> { self.prop_bool("MirrorFillY") }
    pub fn set_mirror_fill_y(&self, v: bool) -> bool { self.put_bool("MirrorFillY", v) }

    pub fn transform_with_shape(&self) -> Option<bool> { self.prop_bool("TransformWithShape") }
    pub fn set_transform_with_shape(&self, v: bool) -> bool {
        self.put_bool("TransformWithShape", v)
    }

    // ---- 棰滆壊 ----

    pub fn primary_color(&self) -> Option<crate::color::IvgColor> {
        self.prop_dispatch("PrimaryColor").map(crate::color::IvgColor::new)
    }

    pub fn secondary_color(&self) -> Option<crate::color::IvgColor> {
        self.prop_dispatch("SecondaryColor").map(crate::color::IvgColor::new)
    }

    pub fn primary_opacity(&self) -> Option<i64> {
        self.disp.get_property("PrimaryOpacity").ok()?.to_i64().ok()
    }
    pub fn set_primary_opacity(&self, v: u8) -> bool { self.put_u8("PrimaryOpacity", v) }

    pub fn secondary_opacity(&self) -> Option<i64> {
        self.disp.get_property("SecondaryOpacity").ok()?.to_i64().ok()
    }
    pub fn set_secondary_opacity(&self, v: u8) -> bool { self.put_u8("SecondaryOpacity", v) }

    // ---- 娓愬彉涓績鍋忕Щ锛坒64 鐗堬級 ----

    pub fn fountain_center_x_offset(&self) -> Option<f64> {
        self.prop_f64("FountainCenterXOffset")
    }
    pub fn set_fountain_center_x_offset(&self, v: f64) -> bool {
        self.put_f64("FountainCenterXOffset", v)
    }

    pub fn fountain_center_y_offset(&self) -> Option<f64> {
        self.prop_f64("FountainCenterYOffset")
    }
    pub fn set_fountain_center_y_offset(&self, v: f64) -> bool {
        self.put_f64("FountainCenterYOffset", v)
    }

    pub fn fountain_blend_acceleration(&self) -> Option<f64> {
        self.prop_f64("FountainBlendAcceleration")
    }
    pub fn set_fountain_blend_acceleration(&self, v: f64) -> bool {
        self.put_f64("FountainBlendAcceleration", v)
    }

    pub fn fountain_scale_x(&self) -> Option<f64> { self.prop_f64("FountainScaleX") }
    pub fn set_fountain_scale_x(&self, v: f64) -> bool {
        self.put_f64("FountainScaleX", v)
    }

    pub fn fountain_scale_y(&self) -> Option<f64> { self.prop_f64("FountainScaleY") }
    pub fn set_fountain_scale_y(&self, v: f64) -> bool {
        self.put_f64("FountainScaleY", v)
    }

    pub fn fountain_anisotropic(&self) -> Option<bool> {
        self.prop_bool("FountainAnisotropic")
    }
    pub fn set_fountain_anisotropic(&self, v: bool) -> bool {
        self.put_bool("FountainAnisotropic", v)
    }

    /// `cdrFountainFillSpreadMethod`
    pub fn fountain_spread_method(&self) -> Option<i64> {
        self.prop_i64("FountainSpreadMethod")
    }
    pub fn set_fountain_spread_method(&self, v: i32) -> bool {
        self.put_i64("FountainSpreadMethod", v as i64)
    }

    // ---- 娣峰悎妯″紡 ----

    /// `cdrMergeMode`
    pub fn merge_mode(&self) -> Option<i64> { self.prop_i64("MergeMode") }
    pub fn set_merge_mode(&self, v: i32) -> bool { self.put_i64("MergeMode", v as i64) }

    // ---- 淇濆瓨 / 鍔犺浇 ----

    pub fn save_fill(
        &self,
        file_name: impl Into<String>,
        metadata: &crate::fill::IvgFillMetadata,
    ) -> Option<bool> {
        let args = vec![
            Variant::from_str(file_name.into()),
            metadata.as_variant(),
        ];
        self.disp
            .invoke_method("SaveFill", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn load_fill(
        &self,
        file_name: impl Into<String>,
    ) -> Option<(crate::fill::IvgFillMetadata, bool)> {
        let args = vec![Variant::from_str(file_name.into())];
        let meta = self
            .disp
            .invoke_method("LoadFill", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgFillMetadata::new)?;
        Some((meta, true))
    }

    // ---- 鍏宠仈鏍峰紡 ----

    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.prop_dispatch("Style").map(crate::style::IvgStyle::new)
    }
}
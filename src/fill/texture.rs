//! `IVGTextureFill` 鈥斺€?绾圭悊濉厖
//! `IVGTextureFillProperty(ies)` 鈥斺€?绾圭悊灞炴€?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgTextureFill
// =============================================================

pub struct IvgTextureFill {
    disp: ComObject,
}

impl IvgTextureFill {
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

    // ---- 鍘熺偣 ----

    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    // ---- 骞抽摵 ----

    pub fn tile_width(&self) -> Option<f64> { self.prop_f64("TileWidth") }
    pub fn set_tile_width(&self, v: f64) -> bool { self.put_f64("TileWidth", v) }

    pub fn tile_height(&self) -> Option<f64> { self.prop_f64("TileHeight") }
    pub fn set_tile_height(&self, v: f64) -> bool { self.put_f64("TileHeight", v) }

    /// `cdrTileOffsetType`
    pub fn tile_offset_type(&self) -> Option<i64> {
        self.prop_i64("TileOffsetType")
    }
    pub fn set_tile_offset_type(&self, v: i32) -> bool {
        self.put_i64("TileOffsetType", v as i64)
    }

    pub fn tile_offset(&self) -> Option<i64> { self.prop_i64("TileOffset") }
    pub fn set_tile_offset(&self, v: i32) -> bool { self.put_i64("TileOffset", v as i64) }

    // ---- 鏃嬭浆 / 鍊炬枩 ----

    pub fn skew_angle(&self) -> Option<f64> { self.prop_f64("SkewAngle") }
    pub fn set_skew_angle(&self, v: f64) -> bool { self.put_f64("SkewAngle", v) }

    pub fn rotation_angle(&self) -> Option<f64> {
        self.prop_f64("RotationAngle")
    }
    pub fn set_rotation_angle(&self, v: f64) -> bool {
        self.put_f64("RotationAngle", v)
    }

    // ---- 鍙樻崲 ----

    pub fn transform_with_shape(&self) -> Option<bool> {
        self.prop_bool("TransformWithShape")
    }
    pub fn set_transform_with_shape(&self, v: bool) -> bool {
        self.put_bool("TransformWithShape", v)
    }

    // ---- 鍒嗚鲸鐜?----

    pub fn resolution(&self) -> Option<i64> { self.prop_i64("Resolution") }
    pub fn set_resolution(&self, v: i32) -> bool { self.put_i64("Resolution", v as i64) }

    pub fn maximum_tile_width(&self) -> Option<i64> {
        self.prop_i64("MaximumTileWidth")
    }
    pub fn set_maximum_tile_width(&self, v: i32) -> bool {
        self.put_i64("MaximumTileWidth", v as i64)
    }

    // ---- 搴?/ 鍚嶇О ----

    pub fn library_name(&self) -> Option<String> {
        self.prop_string("LibraryName")
    }

    pub fn texture_name(&self) -> Option<String> {
        self.prop_string("TextureName")
    }

    pub fn style_name(&self) -> Option<String> {
        self.prop_string("StyleName")
    }

    // ---- 閫夋嫨 ----

    pub fn select(
        &self,
        texture: impl Into<String>,
        library: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(texture.into()),
            Variant::from_str(library.into()),
        ];
        self.disp.invoke_method("Select", args).is_ok()
    }

    /// 璁剧疆绾圭悊灞炴€э紙`SAFEARRAY`锛夈€?
    pub fn set_properties(&self, setting_array: Variant) -> bool {
        let args = vec![setting_array];
        self.disp.invoke_method("SetProperties", args).is_ok()
    }

    // ---- 闀滃儚 ----

    pub fn mirror_fill(&self) -> Option<bool> { self.prop_bool("MirrorFill") }
    pub fn set_mirror_fill(&self, v: bool) -> bool { self.put_bool("MirrorFill", v) }

    pub fn mirror_fill_x(&self) -> Option<bool> { self.prop_bool("MirrorFillX") }
    pub fn set_mirror_fill_x(&self, v: bool) -> bool { self.put_bool("MirrorFillX", v) }

    pub fn mirror_fill_y(&self) -> Option<bool> { self.prop_bool("MirrorFillY") }
    pub fn set_mirror_fill_y(&self, v: bool) -> bool { self.put_bool("MirrorFillY", v) }

    // ---- 灞炴€ч泦鍚?----

    pub fn properties(&self) -> Option<IvgTextureFillProperties> {
        self.prop_dispatch("Properties").map(IvgTextureFillProperties::new)
    }
}

// =============================================================
// IvgTextureFillProperties
// =============================================================

pub struct IvgTextureFillProperties {
    disp: ComObject,
}

impl IvgTextureFillProperties {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgTextureFillProperty> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextureFillProperty::new)
    }
}

// =============================================================
// IvgTextureFillProperty
// =============================================================

pub struct IvgTextureFillProperty {
    disp: ComObject,
}

impl IvgTextureFillProperty {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    /// `cdrTexturePropertyType`
    pub fn prop_type(&self) -> Option<i64> { self.prop_i64("Type") }

    pub fn value(&self) -> Option<Variant> {
        self.disp.get_property("Value").ok()
    }
    pub fn set_value(&self, v: Variant) -> bool {
        self.disp.set_property("Value", vec![v]).is_ok()
    }
}
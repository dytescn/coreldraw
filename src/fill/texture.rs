//! `IVGTextureFill` —— 纹理填充
//! `IVGTextureFillProperty(ies)` —— 纹理属性

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgTextureFill
// =============================================================

/// CorelDRAW `IVGTextureFill` 对象的后期绑定包装（纹理填充）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTextureFill {
    disp: ComObject,
}

impl IvgTextureFill {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
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

    // ---- 原点 ----

    /// 读取原点 X 坐标（`OriginX`）。
    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    /// 设置原点 X 坐标（`OriginX`）。
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    /// 读取原点 Y 坐标（`OriginY`）。
    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    /// 设置原点 Y 坐标（`OriginY`）。
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    // ---- 平铺 ----

    /// 读取平铺宽度（`TileWidth`）。
    pub fn tile_width(&self) -> Option<f64> { self.prop_f64("TileWidth") }
    /// 设置平铺宽度（`TileWidth`）。
    pub fn set_tile_width(&self, v: f64) -> bool { self.put_f64("TileWidth", v) }

    /// 读取平铺高度（`TileHeight`）。
    pub fn tile_height(&self) -> Option<f64> { self.prop_f64("TileHeight") }
    /// 设置平铺高度（`TileHeight`）。
    pub fn set_tile_height(&self, v: f64) -> bool { self.put_f64("TileHeight", v) }

    /// `cdrTileOffsetType`
    pub fn tile_offset_type(&self) -> Option<i64> {
        self.prop_i64("TileOffsetType")
    }
    /// 设置平铺偏移类型（`TileOffsetType`）。
    pub fn set_tile_offset_type(&self, v: i32) -> bool {
        self.put_i64("TileOffsetType", v as i64)
    }

    /// 读取平铺偏移（`TileOffset`）。
    pub fn tile_offset(&self) -> Option<i64> { self.prop_i64("TileOffset") }
    /// 设置平铺偏移（`TileOffset`）。
    pub fn set_tile_offset(&self, v: i32) -> bool { self.put_i64("TileOffset", v as i64) }

    // ---- 旋转 / 倾斜 ----

    /// 读取倾斜角度（`SkewAngle`）。
    pub fn skew_angle(&self) -> Option<f64> { self.prop_f64("SkewAngle") }
    /// 设置倾斜角度（`SkewAngle`）。
    pub fn set_skew_angle(&self, v: f64) -> bool { self.put_f64("SkewAngle", v) }

    /// 读取旋转角度（`RotationAngle`）。
    pub fn rotation_angle(&self) -> Option<f64> {
        self.prop_f64("RotationAngle")
    }
    /// 设置旋转角度（`RotationAngle`）。
    pub fn set_rotation_angle(&self, v: f64) -> bool {
        self.put_f64("RotationAngle", v)
    }

    // ---- 变换 ----

    /// 读取变换图形（`TransformWithShape`）。
    pub fn transform_with_shape(&self) -> Option<bool> {
        self.prop_bool("TransformWithShape")
    }
    /// 设置变换图形（`TransformWithShape`）。
    pub fn set_transform_with_shape(&self, v: bool) -> bool {
        self.put_bool("TransformWithShape", v)
    }

    // ---- 分辨率----

    /// 读取分辨率（`Resolution`）。
    pub fn resolution(&self) -> Option<i64> { self.prop_i64("Resolution") }
    /// 设置分辨率（`Resolution`）。
    pub fn set_resolution(&self, v: i32) -> bool { self.put_i64("Resolution", v as i64) }

    /// 读取最大平铺宽度（`MaximumTileWidth`）。
    pub fn maximum_tile_width(&self) -> Option<i64> {
        self.prop_i64("MaximumTileWidth")
    }
    /// 设置最大平铺宽度（`MaximumTileWidth`）。
    pub fn set_maximum_tile_width(&self, v: i32) -> bool {
        self.put_i64("MaximumTileWidth", v as i64)
    }

    // ---- 应/ 名称 ----

    /// 读取库名称（`LibraryName`）。
    pub fn library_name(&self) -> Option<String> {
        self.prop_string("LibraryName")
    }

    /// 读取纹理名称（`TextureName`）。
    pub fn texture_name(&self) -> Option<String> {
        self.prop_string("TextureName")
    }

    /// 读取样式名称（`StyleName`）。
    pub fn style_name(&self) -> Option<String> {
        self.prop_string("StyleName")
    }

    // ---- 选择 ----

    /// 选择（COM 方法 `Select`）。
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

    /// 设置纹理属性（`SAFEARRAY`）。
    pub fn set_properties(&self, setting_array: Variant) -> bool {
        let args = vec![setting_array];
        self.disp.invoke_method("SetProperties", args).is_ok()
    }

    // ---- 镜像 ----

    /// 读取镜像填充（`MirrorFill`）。
    pub fn mirror_fill(&self) -> Option<bool> { self.prop_bool("MirrorFill") }
    /// 设置镜像填充（`MirrorFill`）。
    pub fn set_mirror_fill(&self, v: bool) -> bool { self.put_bool("MirrorFill", v) }

    /// 读取镜像填充 X 坐标（`MirrorFillX`）。
    pub fn mirror_fill_x(&self) -> Option<bool> { self.prop_bool("MirrorFillX") }
    /// 设置镜像填充 X 坐标（`MirrorFillX`）。
    pub fn set_mirror_fill_x(&self, v: bool) -> bool { self.put_bool("MirrorFillX", v) }

    /// 读取镜像填充 Y 坐标（`MirrorFillY`）。
    pub fn mirror_fill_y(&self) -> Option<bool> { self.prop_bool("MirrorFillY") }
    /// 设置镜像填充 Y 坐标（`MirrorFillY`）。
    pub fn set_mirror_fill_y(&self, v: bool) -> bool { self.put_bool("MirrorFillY", v) }

    // ---- 属性集合----

    /// 读取属性集（`Properties`）。
    pub fn properties(&self) -> Option<IvgTextureFillProperties> {
        self.prop_dispatch("Properties").map(IvgTextureFillProperties::new)
    }
}

// =============================================================
// IvgTextureFillProperties
// =============================================================

/// CorelDRAW `IVGTextureFill` 对象的后期绑定包装（纹理填充属性集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTextureFillProperties {
    disp: ComObject,
}

impl IvgTextureFillProperties {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
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

/// CorelDRAW `IVGTextureFill` 对象的后期绑定包装（纹理填充属性）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTextureFillProperty {
    disp: ComObject,
}

impl IvgTextureFillProperty {
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

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    /// `cdrTexturePropertyType`
    pub fn prop_type(&self) -> Option<i64> { self.prop_i64("Type") }

    /// 读取值（`Value`）。
    pub fn value(&self) -> Option<Variant> {
        self.disp.get_property("Value").ok()
    }
    /// 设置值（`Value`）。
    pub fn set_value(&self, v: Variant) -> bool {
        self.disp.set_property("Value", vec![v]).is_ok()
    }
}
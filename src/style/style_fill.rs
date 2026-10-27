//! `IVGStyleFill` —— 样式里的填充部分

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGStyleFill` 对象的后期绑定包装（样式填充）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStyleFill {
    disp: ComObject,
}

impl IvgStyleFill {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
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

    // ---- 类型 ----

    /// `cdrFillStyleType`
    pub fn fill_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置填充类型（`Type`）。
    pub fn set_fill_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    // ---- 叠印 / 缠绕 ----

    /// 读取叠印（`Overprint`）。
    pub fn overprint(&self) -> Option<bool> { self.prop_bool("Overprint") }
    /// 设置叠印（`Overprint`）。
    pub fn set_overprint(&self, v: bool) -> bool { self.put_bool("Overprint", v) }

    /// 读取 COM 属性 `WindingFill`。
    pub fn winding_fill(&self) -> Option<bool> { self.prop_bool("WindingFill") }
    /// 设置 COM 属性 `WindingFill`。
    pub fn set_winding_fill(&self, v: bool) -> bool { self.put_bool("WindingFill", v) }

    // ---- 渐变 ----

    /// `cdrFountainFillType`
    pub fn fountain_fill_type(&self) -> Option<i64> { self.prop_i64("FountainFillType") }
    /// 设置渐变填充类型（`FountainFillType`）。
    pub fn set_fountain_fill_type(&self, v: i32) -> bool {
        self.put_i64("FountainFillType", v as i64)
    }

    /// 读取边缘边距（`EdgePad`）。
    pub fn edge_pad(&self) -> Option<i64> { self.prop_i64("EdgePad") }
    /// 设置边缘边距（`EdgePad`）。
    pub fn set_edge_pad(&self, v: i32) -> bool { self.put_i64("EdgePad", v as i64) }

    /// 读取渐变中心偏移 X 坐标（`FountainCenterOffsetX`）。
    pub fn fountain_center_offset_x(&self) -> Option<i64> {
        self.prop_i64("FountainCenterOffsetX")
    }
    /// 设置渐变中心偏移 X 坐标（`FountainCenterOffsetX`）。
    pub fn set_fountain_center_offset_x(&self, v: i32) -> bool {
        self.put_i64("FountainCenterOffsetX", v as i64)
    }

    /// 读取渐变中心偏移 Y 坐标（`FountainCenterOffsetY`）。
    pub fn fountain_center_offset_y(&self) -> Option<i64> {
        self.prop_i64("FountainCenterOffsetY")
    }
    /// 设置渐变中心偏移 Y 坐标（`FountainCenterOffsetY`）。
    pub fn set_fountain_center_offset_y(&self, v: i32) -> bool {
        self.put_i64("FountainCenterOffsetY", v as i64)
    }

    /// 读取渐变步数（`FountainSteps`）。
    pub fn fountain_steps(&self) -> Option<i64> { self.prop_i64("FountainSteps") }
    /// 设置渐变步数（`FountainSteps`）。
    pub fn set_fountain_steps(&self, v: i32) -> bool {
        self.put_i64("FountainSteps", v as i64)
    }

    /// `cdrFountainFillBlendType`
    pub fn fountain_blend_type(&self) -> Option<i64> {
        self.prop_i64("FountainBlendType")
    }
    /// 设置渐变调和类型（`FountainBlendType`）。
    pub fn set_fountain_blend_type(&self, v: i32) -> bool {
        self.put_i64("FountainBlendType", v as i64)
    }

    /// 读取中间点（`MidPoint`）。
    pub fn mid_point(&self) -> Option<i64> { self.prop_i64("MidPoint") }
    /// 设置中间点（`MidPoint`）。
    pub fn set_mid_point(&self, v: i32) -> bool { self.put_i64("MidPoint", v as i64) }

    /// 读取翻转颜色集（`FlipColors`）。
    pub fn flip_colors(&self) -> Option<bool> { self.prop_bool("FlipColors") }
    /// 设置翻转颜色集（`FlipColors`）。
    pub fn set_flip_colors(&self, v: bool) -> bool { self.put_bool("FlipColors", v) }

    // ---- PostScript ----

    /// 读取 PostScript 名称（`PostScriptName`）。
    pub fn postscript_name(&self) -> Option<String> { self.prop_string("PostScriptName") }
    /// 设置 PostScript 名称（`PostScriptName`）。
    pub fn set_postscript_name(&self, v: impl Into<String>) -> bool {
        self.put_string("PostScriptName", v)
    }

    // ---- 平铺 ----

    /// 读取平铺宽度（`TileWidth`）。
    pub fn tile_width(&self) -> Option<f64> { self.prop_f64("TileWidth") }
    /// 设置平铺宽度（`TileWidth`）。
    pub fn set_tile_width(&self, v: f64) -> bool { self.put_f64("TileWidth", v) }

    /// 读取平铺高度（`TileHeight`）。
    pub fn tile_height(&self) -> Option<f64> { self.prop_f64("TileHeight") }
    /// 设置平铺高度（`TileHeight`）。
    pub fn set_tile_height(&self, v: f64) -> bool { self.put_f64("TileHeight", v) }

    /// 读取平铺原点 X 坐标（`TileOriginX`）。
    pub fn tile_origin_x(&self) -> Option<f64> { self.prop_f64("TileOriginX") }
    /// 设置平铺原点 X 坐标（`TileOriginX`）。
    pub fn set_tile_origin_x(&self, v: f64) -> bool { self.put_f64("TileOriginX", v) }

    /// 读取平铺原点 Y 坐标（`TileOriginY`）。
    pub fn tile_origin_y(&self) -> Option<f64> { self.prop_f64("TileOriginY") }
    /// 设置平铺原点 Y 坐标（`TileOriginY`）。
    pub fn set_tile_origin_y(&self, v: f64) -> bool { self.put_f64("TileOriginY", v) }

    /// `cdrTileOffsetType`
    pub fn tile_offset_type(&self) -> Option<i64> { self.prop_i64("TileOffsetType") }
    /// 设置平铺偏移类型（`TileOffsetType`）。
    pub fn set_tile_offset_type(&self, v: i32) -> bool {
        self.put_i64("TileOffsetType", v as i64)
    }

    /// 读取平铺偏移（`TileOffset`）。
    pub fn tile_offset(&self) -> Option<i64> { self.prop_i64("TileOffset") }
    /// 设置平铺偏移（`TileOffset`）。
    pub fn set_tile_offset(&self, v: i32) -> bool { self.put_i64("TileOffset", v as i64) }

    // ---- 旋转 / 倾斜 / 镜像 / 变换 ----

    /// 读取旋转角度（`RotationAngle`）。
    pub fn rotation_angle(&self) -> Option<f64> { self.prop_f64("RotationAngle") }
    /// 设置旋转角度（`RotationAngle`）。
    pub fn set_rotation_angle(&self, v: f64) -> bool { self.put_f64("RotationAngle", v) }

    /// 读取倾斜角度（`SkewAngle`）。
    pub fn skew_angle(&self) -> Option<f64> { self.prop_f64("SkewAngle") }
    /// 设置倾斜角度（`SkewAngle`）。
    pub fn set_skew_angle(&self, v: f64) -> bool { self.put_f64("SkewAngle", v) }

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

    /// 读取变换图形（`TransformWithShape`）。
    pub fn transform_with_shape(&self) -> Option<bool> { self.prop_bool("TransformWithShape") }
    /// 设置变换图形（`TransformWithShape`）。
    pub fn set_transform_with_shape(&self, v: bool) -> bool {
        self.put_bool("TransformWithShape", v)
    }

    // ---- 颜色 ----

    /// 读取 COM 属性 `PrimaryColor`。
    pub fn primary_color(&self) -> Option<crate::color::IvgColor> {
        self.prop_dispatch("PrimaryColor").map(crate::color::IvgColor::new)
    }

    /// 读取 COM 属性 `SecondaryColor`。
    pub fn secondary_color(&self) -> Option<crate::color::IvgColor> {
        self.prop_dispatch("SecondaryColor").map(crate::color::IvgColor::new)
    }

    /// 读取 COM 属性 `PrimaryOpacity`。
    pub fn primary_opacity(&self) -> Option<i64> {
        self.disp.get_property("PrimaryOpacity").ok()?.to_i64().ok()
    }
    /// 设置 COM 属性 `PrimaryOpacity`。
    pub fn set_primary_opacity(&self, v: u8) -> bool { self.put_u8("PrimaryOpacity", v) }

    /// 读取 COM 属性 `SecondaryOpacity`。
    pub fn secondary_opacity(&self) -> Option<i64> {
        self.disp.get_property("SecondaryOpacity").ok()?.to_i64().ok()
    }
    /// 设置 COM 属性 `SecondaryOpacity`。
    pub fn set_secondary_opacity(&self, v: u8) -> bool { self.put_u8("SecondaryOpacity", v) }

    // ---- 渐变中心偏移（f64 版） ----

    /// 读取渐变中心 X 坐标偏移（`FountainCenterXOffset`）。
    pub fn fountain_center_x_offset(&self) -> Option<f64> {
        self.prop_f64("FountainCenterXOffset")
    }
    /// 设置渐变中心 X 坐标偏移（`FountainCenterXOffset`）。
    pub fn set_fountain_center_x_offset(&self, v: f64) -> bool {
        self.put_f64("FountainCenterXOffset", v)
    }

    /// 读取渐变中心 Y 坐标偏移（`FountainCenterYOffset`）。
    pub fn fountain_center_y_offset(&self) -> Option<f64> {
        self.prop_f64("FountainCenterYOffset")
    }
    /// 设置渐变中心 Y 坐标偏移（`FountainCenterYOffset`）。
    pub fn set_fountain_center_y_offset(&self, v: f64) -> bool {
        self.put_f64("FountainCenterYOffset", v)
    }

    /// 读取渐变调和加速度（`FountainBlendAcceleration`）。
    pub fn fountain_blend_acceleration(&self) -> Option<f64> {
        self.prop_f64("FountainBlendAcceleration")
    }
    /// 设置渐变调和加速度（`FountainBlendAcceleration`）。
    pub fn set_fountain_blend_acceleration(&self, v: f64) -> bool {
        self.put_f64("FountainBlendAcceleration", v)
    }

    /// 读取渐变缩放 X 坐标（`FountainScaleX`）。
    pub fn fountain_scale_x(&self) -> Option<f64> { self.prop_f64("FountainScaleX") }
    /// 设置渐变缩放 X 坐标（`FountainScaleX`）。
    pub fn set_fountain_scale_x(&self, v: f64) -> bool {
        self.put_f64("FountainScaleX", v)
    }

    /// 读取渐变缩放 Y 坐标（`FountainScaleY`）。
    pub fn fountain_scale_y(&self) -> Option<f64> { self.prop_f64("FountainScaleY") }
    /// 设置渐变缩放 Y 坐标（`FountainScaleY`）。
    pub fn set_fountain_scale_y(&self, v: f64) -> bool {
        self.put_f64("FountainScaleY", v)
    }

    /// 读取渐变各向异性（`FountainAnisotropic`）。
    pub fn fountain_anisotropic(&self) -> Option<bool> {
        self.prop_bool("FountainAnisotropic")
    }
    /// 设置渐变各向异性（`FountainAnisotropic`）。
    pub fn set_fountain_anisotropic(&self, v: bool) -> bool {
        self.put_bool("FountainAnisotropic", v)
    }

    /// `cdrFountainFillSpreadMethod`
    pub fn fountain_spread_method(&self) -> Option<i64> {
        self.prop_i64("FountainSpreadMethod")
    }
    /// 设置渐变跨页方法（`FountainSpreadMethod`）。
    pub fn set_fountain_spread_method(&self, v: i32) -> bool {
        self.put_i64("FountainSpreadMethod", v as i64)
    }

    // ---- 混合模式 ----

    /// `cdrMergeMode`
    pub fn merge_mode(&self) -> Option<i64> { self.prop_i64("MergeMode") }
    /// 设置合并模式（`MergeMode`）。
    pub fn set_merge_mode(&self, v: i32) -> bool { self.put_i64("MergeMode", v as i64) }

    // ---- 保存 / 加载 ----

    /// 保存填充（COM 方法 `SaveFill`）。
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

    /// 加载填充（COM 方法 `LoadFill`）。
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

    // ---- 关联样式 ----

    /// 读取样式（`Style`）。
    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.prop_dispatch("Style").map(crate::style::IvgStyle::new)
    }
}
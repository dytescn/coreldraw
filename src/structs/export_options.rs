//! `IVGStructExportOptions` —— 导出选项

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::geometry::IvgRect;

/// CorelDRAW `IVGStructExportOptions` 对象的后期绑定包装（导出选项集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructExportOptions {
    disp: ComObject,
}

impl IvgStructExportOptions {
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

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
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

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---- 尺寸 / 分辨率----

    /// 读取尺寸 X 坐标（`SizeX`）。
    pub fn size_x(&self) -> Option<i64> { self.prop_i64("SizeX") }
    /// 设置尺寸 X 坐标（`SizeX`）。
    pub fn set_size_x(&self, v: i32) -> bool { self.put_i64("SizeX", v as i64) }

    /// 读取尺寸 Y 坐标（`SizeY`）。
    pub fn size_y(&self) -> Option<i64> { self.prop_i64("SizeY") }
    /// 设置尺寸 Y 坐标（`SizeY`）。
    pub fn set_size_y(&self, v: i32) -> bool { self.put_i64("SizeY", v as i64) }

    /// 读取分辨率 X 坐标（`ResolutionX`）。
    pub fn resolution_x(&self) -> Option<i64> { self.prop_i64("ResolutionX") }
    /// 设置分辨率 X 坐标（`ResolutionX`）。
    pub fn set_resolution_x(&self, v: i32) -> bool { self.put_i64("ResolutionX", v as i64) }

    /// 读取分辨率 Y 坐标（`ResolutionY`）。
    pub fn resolution_y(&self) -> Option<i64> { self.prop_i64("ResolutionY") }
    /// 设置分辨率 Y 坐标（`ResolutionY`）。
    pub fn set_resolution_y(&self, v: i32) -> bool { self.put_i64("ResolutionY", v as i64) }

    // ---- 抗锯齿/ 图像类型 ----

    /// `cdrAntiAliasingType`
    pub fn anti_aliasing_type(&self) -> Option<i64> { self.prop_i64("AntiAliasingType") }
    /// 设置 COM 属性 `AntiAliasingType`。
    pub fn set_anti_aliasing_type(&self, v: i32) -> bool {
        self.put_i64("AntiAliasingType", v as i64)
    }

    /// `cdrImageType`
    pub fn image_type(&self) -> Option<i64> { self.prop_i64("ImageType") }
    /// 设置图像类型（`ImageType`）。
    pub fn set_image_type(&self, v: i32) -> bool {
        self.put_i64("ImageType", v as i64)
    }

    /// `cdrCompressionType`
    pub fn compression(&self) -> Option<i64> { self.prop_i64("Compression") }
    /// 设置压缩（`Compression`）。
    pub fn set_compression(&self, v: i32) -> bool {
        self.put_i64("Compression", v as i64)
    }

    // ---- 标志你----

    /// 读取覆盖（`Overwrite`）。
    pub fn overwrite(&self) -> Option<bool> { self.prop_bool("Overwrite") }
    /// 设置覆盖（`Overwrite`）。
    pub fn set_overwrite(&self, v: bool) -> bool { self.put_bool("Overwrite", v) }

    /// 读取 COM 属性 `Dithered`。
    pub fn dithered(&self) -> Option<bool> { self.prop_bool("Dithered") }
    /// 设置 COM 属性 `Dithered`。
    pub fn set_dithered(&self, v: bool) -> bool { self.put_bool("Dithered", v) }

    /// 读取透明（`Transparent`）。
    pub fn transparent(&self) -> Option<bool> { self.prop_bool("Transparent") }
    /// 设置透明（`Transparent`）。
    pub fn set_transparent(&self, v: bool) -> bool { self.put_bool("Transparent", v) }

    /// 读取使用颜色配置文件（`UseColorProfile`）。
    pub fn use_color_profile(&self) -> Option<bool> { self.prop_bool("UseColorProfile") }
    /// 设置使用颜色配置文件（`UseColorProfile`）。
    pub fn set_use_color_profile(&self, v: bool) -> bool {
        self.put_bool("UseColorProfile", v)
    }

    /// 读取保持图层集（`MaintainLayers`）。
    pub fn maintain_layers(&self) -> Option<bool> { self.prop_bool("MaintainLayers") }
    /// 设置保持图层集（`MaintainLayers`）。
    pub fn set_maintain_layers(&self, v: bool) -> bool {
        self.put_bool("MaintainLayers", v)
    }

    /// 读取 COM 属性 `MaintainAspect`。
    pub fn maintain_aspect(&self) -> Option<bool> { self.prop_bool("MaintainAspect") }
    /// 设置 COM 属性 `MaintainAspect`。
    pub fn set_maintain_aspect(&self, v: bool) -> bool {
        self.put_bool("MaintainAspect", v)
    }

    /// 读取 COM 属性 `Matted`。
    pub fn matted(&self) -> Option<bool> { self.prop_bool("Matted") }
    /// 设置 COM 属性 `Matted`。
    pub fn set_matted(&self, v: bool) -> bool { self.put_bool("Matted", v) }

    /// 读取 COM 属性 `MatteMaskedOnly`。
    pub fn matte_masked_only(&self) -> Option<bool> { self.prop_bool("MatteMaskedOnly") }
    /// 设置 COM 属性 `MatteMaskedOnly`。
    pub fn set_matte_masked_only(&self, v: bool) -> bool {
        self.put_bool("MatteMaskedOnly", v)
    }

    /// 读取总是叠印黑色（`AlwaysOverprintBlack`）。
    pub fn always_overprint_black(&self) -> Option<bool> {
        self.prop_bool("AlwaysOverprintBlack")
    }
    /// 设置总是叠印黑色（`AlwaysOverprintBlack`）。
    pub fn set_always_overprint_black(&self, v: bool) -> bool {
        self.put_bool("AlwaysOverprintBlack", v)
    }

    // ---- 导出区域 ----

    /// 读取导出区域（`ExportArea`）。
    pub fn export_area(&self) -> Option<IvgRect> {
        self.prop_dispatch("ExportArea").map(IvgRect::new)
    }

    /// 设置导出区域（`ExportArea`）。
    pub fn set_export_area(&self, r: &IvgRect) -> bool {
        self.put_dispatch("ExportArea", r.as_variant())
    }

    // ---- 遮罩色----

    /// 读取蒙版颜色（`MatteColor`）。
    pub fn matte_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("MatteColor").map(IvgColor::new)
    }
    /// 设置蒙版颜色（`MatteColor`）。
    pub fn set_matte_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("MatteColor", c.as_variant())
    }

    // ---- 校样 ----

    /// 读取校样颜色设置集（`ProofColorSettings`）。
    pub fn proof_color_settings(&self) -> Option<crate::misc::IvgProperties> {
        // 实际是 IVGProofColorSettings，暂用 properties 占位
        self.prop_dispatch("ProofColorSettings").map(crate::misc::IvgProperties::new)
    }
}
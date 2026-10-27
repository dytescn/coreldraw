//! `IVGBitmap` —— 位图

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;
use crate::import_export::ICorelExportFilter;
use crate::shape::IvgImage;
use crate::structs::IvgStructPaletteOptions;

/// CorelDRAW `IVGBitmap` 对象的后期绑定包装（位图）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgBitmap {
    disp: ComObject,
}

impl IvgBitmap {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject { &self.disp }

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

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 基本信息 ----

    /// 读取尺寸宽度（`SizeWidth`）。
    pub fn size_width(&self) -> Option<i64> { self.prop_i64("SizeWidth") }
    /// 读取尺寸高度（`SizeHeight`）。
    pub fn size_height(&self) -> Option<i64> { self.prop_i64("SizeHeight") }
    /// 读取分辨率 X 坐标（`ResolutionX`）。
    pub fn resolution_x(&self) -> Option<i64> { self.prop_i64("ResolutionX") }
    /// 读取分辨率 Y 坐标（`ResolutionY`）。
    pub fn resolution_y(&self) -> Option<i64> { self.prop_i64("ResolutionY") }

    /// 读取 COM 属性 `ExternallyLinked`。
    pub fn externally_linked(&self) -> Option<bool> { self.prop_bool("ExternallyLinked") }
    /// 读取链接文件名称（`LinkFileName`）。
    pub fn link_file_name(&self) -> Option<String> { self.prop_string("LinkFileName") }
    /// 设置链接文件名称（`LinkFileName`）。
    pub fn set_link_file_name(&self, v: impl Into<String>) -> bool {
        self.put_string("LinkFileName", v)
    }

    /// `cdrImageType`
    pub fn mode(&self) -> Option<i64> { self.prop_i64("Mode") }

    /// 读取透明（`Transparent`）。
    pub fn transparent(&self) -> Option<bool> { self.prop_bool("Transparent") }
    /// 读取 COM 属性 `Watermarked`。
    pub fn watermarked(&self) -> Option<bool> { self.prop_bool("Watermarked") }
    /// 读取 OPI 已链接（`OPILinked`）。
    pub fn opi_linked(&self) -> Option<bool> { self.prop_bool("OPILinked") }
    /// 是否 EPS（`IsEPS`）。
    pub fn is_eps(&self) -> Option<bool> { self.prop_bool("IsEPS") }
    /// 读取 COM 属性 `Embedded`。
    pub fn embedded(&self) -> Option<bool> { self.prop_bool("Embedded") }
    /// 读取 COM 属性 `Cropped`。
    pub fn cropped(&self) -> Option<bool> { self.prop_bool("Cropped") }
    /// 读取 COM 属性 `CropEnvelopeModified`。
    pub fn crop_envelope_modified(&self) -> Option<bool> { self.prop_bool("CropEnvelopeModified") }

    /// 读取裁剪封套（`CropEnvelope`）。
    pub fn crop_envelope(&self) -> Option<IvgCurve> {
        self.prop_dispatch("CropEnvelope").map(IvgCurve::new)
    }

    /// 读取边界框路径（`BoundingBoxPath`）。
    pub fn bounding_box_path(&self) -> Option<IvgCurve> {
        self.prop_dispatch("BoundingBoxPath").map(IvgCurve::new)
    }

    // ---- 操作 ----

    /// 调用 COM 方法 `ResolveLink`。
    pub fn resolve_link(&self) -> bool { self.disp.invoke_method("ResolveLink", vec![]).is_ok() }
    /// 更新链接（COM 方法 `UpdateLink`）。
    pub fn update_link(&self) -> bool { self.disp.invoke_method("UpdateLink", vec![]).is_ok() }

    /// 膨胀（COM 方法 `Inflate`）。
    pub fn inflate(&self, w: i32, h: i32) -> bool {
        let args = vec![
            Variant::from_i64(w as i64),
            Variant::from_i64(h as i64),
        ];
        self.disp.invoke_method("Inflate", args).is_ok()
    }

    /// 重采样（COM 方法 `Resample`）。
    pub fn resample(
        &self,
        width: i32, height: i32,
        anti_alias: bool,
        resolution_x: f64, resolution_y: f64,
    ) -> bool {
        let args = vec![
            Variant::from_i64(width as i64),
            Variant::from_i64(height as i64),
            Variant::from_bool(anti_alias),
            Variant::from_f64(resolution_x),
            Variant::from_f64(resolution_y),
        ];
        self.disp.invoke_method("Resample", args).is_ok()
    }

    /// 转换（COM 方法 `ConvertTo`）。
    pub fn convert_to(&self, mode: i32) -> bool {
        let args = vec![Variant::from_i64(mode as i64)];
        self.disp.invoke_method("ConvertTo", args).is_ok()
    }

    /// 应用位图效果（COM 方法 `ApplyBitmapEffect`）。
    pub fn apply_bitmap_effect(
        &self,
        undo_string: impl Into<String>,
        command: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(undo_string.into()),
            Variant::from_str(command.into()),
        ];
        self.disp.invoke_method("ApplyBitmapEffect", args).is_ok()
    }

    /// 裁剪（COM 方法 `Crop`）。
    pub fn crop(&self) -> bool { self.disp.invoke_method("Crop", vec![]).is_ok() }
    /// 重置裁剪封套（COM 方法 `ResetCropEnvelope`）。
    pub fn reset_crop_envelope(&self) -> bool {
        self.disp.invoke_method("ResetCropEnvelope", vec![]).is_ok()
    }

    // ---- 保存 ----

    /// 保存 as（COM 方法 `SaveAs`）。
    pub fn save_as(
        &self,
        file_name: impl Into<String>,
        filter: i32,
        compression: i32,
    ) -> Option<ICorelExportFilter> {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_i64(filter as i64),
            Variant::from_i64(compression as i64),
        ];
        self.disp
            .invoke_method("SaveAs", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICorelExportFilter::new)
    }

    // ---- 调色板----

    /// 调用 COM 方法 `ConvertToPaletted`。
    pub fn convert_to_paletted(
        &self,
        palette_type: i32,
        dither_type: i32,
        dither_intensity: i32,
        smoothing: i32,
        num_colors: i32,
        color_sensitive: bool,
        target_color: i32,
        importance: i32,
        lightness: i32,
        tolerance_a: i32,
        tolerance_b: i32,
        palette: Variant,
    ) -> bool {
        let args = vec![
            Variant::from_i64(palette_type as i64),
            Variant::from_i64(dither_type as i64),
            Variant::from_i64(dither_intensity as i64),
            Variant::from_i64(smoothing as i64),
            Variant::from_i64(num_colors as i64),
            Variant::from_bool(color_sensitive),
            Variant::from_i64(target_color as i64),
            Variant::from_i64(importance as i64),
            Variant::from_i64(lightness as i64),
            Variant::from_i64(tolerance_a as i64),
            Variant::from_i64(tolerance_b as i64),
            palette,
        ];
        self.disp.invoke_method("ConvertToPaletted", args).is_ok()
    }

    /// 调用 COM 方法 `ConvertToPaletted2`。
    pub fn convert_to_paletted2(&self, options: &IvgStructPaletteOptions) -> bool {
        let args = vec![options.as_variant()];
        self.disp.invoke_method("ConvertToPaletted2", args).is_ok()
    }

    /// 转换黑白（COM 方法 `ConvertToBW`）。
    pub fn convert_to_bw(
        &self,
        render_type: i32,
        intensity: i32,
        threshold: i32,
        halftone: i32,
        halftone_angle: i32,
        halftone_size: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(render_type as i64),
            Variant::from_i64(intensity as i64),
            Variant::from_i64(threshold as i64),
            Variant::from_i64(halftone as i64),
            Variant::from_i64(halftone_angle as i64),
            Variant::from_i64(halftone_size as i64),
        ];
        self.disp.invoke_method("ConvertToBW", args).is_ok()
    }

    // ---- 图像数据 ----

    /// 读取图像（`Image`）。
    pub fn image(&self) -> Option<IvgImage> {
        self.prop_dispatch("Image").map(IvgImage::new)
    }

    /// 读取图像透明度（`ImageAlpha`）。
    pub fn image_alpha(&self) -> Option<IvgImage> {
        self.prop_dispatch("ImageAlpha").map(IvgImage::new)
    }
    /// 设置图像数据（`SetImageData`）。
    #[allow(unused)]
    pub fn set_image_data(
        &self,
        image: &IvgImage,
        alpha: &IvgImage,
        offset_x: i32,
        offset_y: i32,
    ) -> bool {
        let args = vec![
            image.as_variant(),
            image.as_variant(),
            Variant::from_i64(offset_x as i64),
            Variant::from_i64(offset_y as i64),
        ];
        self.disp.invoke_method("SetImageData", args).is_ok()
    }

    // ---- 描摹 ----

    /// 描摹（COM 方法 `Trace`）。
    #[allow(clippy::too_many_arguments)]
    pub fn trace(
        &self,
        trace_type: i32,
        smoothing: i16,
        detail_level_percent: i16,
        color_mode: i32,
        palette_id: i32,
        color_count: i32,
        delete_original: bool,
        remove_background: bool,
        remove_entire_back: bool,
    ) -> Option<crate::misc::IvgTraceSettings> {
        let args = vec![
            Variant::from_i64(trace_type as i64),
            Variant::from_i64(smoothing as i64),
            Variant::from_i64(detail_level_percent as i64),
            Variant::from_i64(color_mode as i64),
            Variant::from_i64(palette_id as i64),
            Variant::from_i64(color_count as i64),
            Variant::from_bool(delete_original),
            Variant::from_bool(remove_background),
            Variant::from_bool(remove_entire_back),
        ];
        self.disp
            .invoke_method("Trace", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::misc::IvgTraceSettings::new)
    }
}
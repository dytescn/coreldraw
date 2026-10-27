//! `IPDFVBASettings` —— PDF 导出设置
//!
//! 由 `document.pdf_settings()` 取得。
//! **注意**：该接口的方法命名不是 `get_/put_` 而是 `get_/put_`，属性也很多。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IPDFVBASettings` 对象的后期绑定包装（PDFVBA 设置集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPdfVbaSettings {
    disp: ComObject,
}

impl IvgPdfVbaSettings {
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

    // fn prop_f64(&self, name: &str) -> Option<f64> {
    //     self.disp.get_property(name).ok()?.to_f64().ok()
    // }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // fn put_f64(&self, name: &str, v: f64) -> bool {
    //     let arg = Variant::from_f64(v);
    //     self.disp.set_property(name, vec![arg]).is_ok()
    // }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 主操作
    // ---------------------------------------------------------

    /// 重置（COM 方法 `Reset`）。
    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    /// load（COM 方法 `Load`）。
    pub fn load(&self, setting_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(setting_name.into())];
        self.disp.invoke_method("Load", args).ok()?.to_bool().ok()
    }

    /// 保存（COM 方法 `Save`）。
    pub fn save(&self, setting_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(setting_name.into())];
        self.disp.invoke_method("Save", args).ok()?.to_bool().ok()
    }

    /// 显示对话框（COM 方法 `ShowDialog`）。
    pub fn show_dialog(&self) -> Option<bool> {
        self.disp.invoke_method("ShowDialog", vec![]).ok()?.to_bool().ok()
    }

    /// 导出世 PDF。
    pub fn publish_to_pdf(&self, file_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp.invoke_method("PublishToPDF", args).is_ok()
    }

    // ---------------------------------------------------------
    // 发布范围
    // ---------------------------------------------------------

    /// `pdfExportRange`
    pub fn publish_range(&self) -> Option<i64> { self.prop_i64("PublishRange") }
    /// 设置 COM 属性 `PublishRange`。
    pub fn set_publish_range(&self, v: i32) -> bool { self.put_i64("PublishRange", v as i64) }

    /// 读取页面范围（`PageRange`）。
    pub fn page_range(&self) -> Option<String> { self.prop_string("PageRange") }
    /// 设置页面范围（`PageRange`）。
    pub fn set_page_range(&self, v: impl Into<String>) -> bool {
        self.put_string("PageRange", v)
    }

    // ---------------------------------------------------------
    // 文档信息
    // ---------------------------------------------------------

    /// 读取作者（`Author`）。
    pub fn author(&self) -> Option<String> { self.prop_string("Author") }
    /// 设置作者（`Author`）。
    pub fn set_author(&self, v: impl Into<String>) -> bool {
        self.put_string("Author", v)
    }

    /// 读取主题（`Subject`）。
    pub fn subject(&self) -> Option<String> { self.prop_string("Subject") }
    /// 设置主题（`Subject`）。
    pub fn set_subject(&self, v: impl Into<String>) -> bool {
        self.put_string("Subject", v)
    }

    /// 读取关键词（`Keywords`）。
    pub fn keywords(&self) -> Option<String> { self.prop_string("Keywords") }
    /// 设置关键词（`Keywords`）。
    pub fn set_keywords(&self, v: impl Into<String>) -> bool {
        self.put_string("Keywords", v)
    }

    // ---------------------------------------------------------
    // 位图压缩
    // ---------------------------------------------------------

    /// `pdfBitmapCompressionType`
    pub fn bitmap_compression(&self) -> Option<i64> { self.prop_i64("BitmapCompression") }
    /// 设置位图压缩（`BitmapCompression`）。
    pub fn set_bitmap_compression(&self, v: i32) -> bool {
        self.put_i64("BitmapCompression", v as i64)
    }

    /// 读取 JPEG 质量系数（`JPEGQualityFactor`）。
    pub fn jpeg_quality_factor(&self) -> Option<i64> { self.prop_i64("JPEGQualityFactor") }
    /// 设置 JPEG 质量系数（`JPEGQualityFactor`）。
    pub fn set_jpeg_quality_factor(&self, v: i32) -> bool {
        self.put_i64("JPEGQualityFactor", v as i64)
    }

    /// 读取 COM 属性 `JP2QualityFactor`。
    pub fn jp2_quality_factor(&self) -> Option<i64> { self.prop_i64("JP2QualityFactor") }
    /// 设置 COM 属性 `JP2QualityFactor`。
    pub fn set_jp2_quality_factor(&self, v: i32) -> bool {
        self.put_i64("JP2QualityFactor", v as i64)
    }

    // ---------------------------------------------------------
    // 文本 / 字体
    // ---------------------------------------------------------

    /// 读取文本 as 曲线集（`TextAsCurves`）。
    pub fn text_as_curves(&self) -> Option<bool> { self.prop_bool("TextAsCurves") }
    /// 设置文本 as 曲线集（`TextAsCurves`）。
    pub fn set_text_as_curves(&self, v: bool) -> bool {
        self.put_bool("TextAsCurves", v)
    }

    /// 读取嵌入字体集（`EmbedFonts`）。
    pub fn embed_fonts(&self) -> Option<bool> { self.prop_bool("EmbedFonts") }
    /// 设置嵌入字体集（`EmbedFonts`）。
    pub fn set_embed_fonts(&self, v: bool) -> bool { self.put_bool("EmbedFonts", v) }

    /// 读取嵌入基础字体集（`EmbedBaseFonts`）。
    pub fn embed_base_fonts(&self) -> Option<bool> { self.prop_bool("EmbedBaseFonts") }
    /// 设置嵌入基础字体集（`EmbedBaseFonts`）。
    pub fn set_embed_base_fonts(&self, v: bool) -> bool {
        self.put_bool("EmbedBaseFonts", v)
    }

    /// 读取 TrueTypeType1（`TrueTypeToType1`）。
    pub fn truetype_to_type1(&self) -> Option<bool> { self.prop_bool("TrueTypeToType1") }
    /// 设置 TrueTypeType1（`TrueTypeToType1`）。
    pub fn set_truetype_to_type1(&self, v: bool) -> bool {
        self.put_bool("TrueTypeToType1", v)
    }

    /// 读取子集字体集（`SubsetFonts`）。
    pub fn subset_fonts(&self) -> Option<bool> { self.prop_bool("SubsetFonts") }
    /// 设置子集字体集（`SubsetFonts`）。
    pub fn set_subset_fonts(&self, v: bool) -> bool { self.put_bool("SubsetFonts", v) }

    /// 读取 COM 属性 `SubsetPct`。
    pub fn subset_pct(&self) -> Option<i64> { self.prop_i64("SubsetPct") }
    /// 设置 COM 属性 `SubsetPct`。
    pub fn set_subset_pct(&self, v: i32) -> bool { self.put_i64("SubsetPct", v as i64) }

    /// 读取压缩文本（`CompressText`）。
    pub fn compress_text(&self) -> Option<bool> { self.prop_bool("CompressText") }
    /// 设置压缩文本（`CompressText`）。
    pub fn set_compress_text(&self, v: bool) -> bool { self.put_bool("CompressText", v) }

    /// `pdfEncodingType`
    pub fn encoding(&self) -> Option<i64> { self.prop_i64("Encoding") }
    /// 设置 COM 属性 `Encoding`。
    pub fn set_encoding(&self, v: i32) -> bool { self.put_i64("Encoding", v as i64) }

    /// 读取 COM 属性 `ProtectedTextAsCurves`。
    pub fn protected_text_as_curves(&self) -> Option<bool> {
        self.prop_bool("ProtectedTextAsCurves")
    }
    /// 设置 COM 属性 `ProtectedTextAsCurves`。
    pub fn set_protected_text_as_curves(&self, v: bool) -> bool {
        self.put_bool("ProtectedTextAsCurves", v)
    }

    /// `pdfTextExportMode`
    pub fn text_export_mode(&self) -> Option<i64> { self.prop_i64("TextExportMode") }
    /// 设置文本导出模式（`TextExportMode`）。
    pub fn set_text_export_mode(&self, v: i32) -> bool {
        self.put_i64("TextExportMode", v as i64)
    }

    // ---------------------------------------------------------
    // 降采样
    // ---------------------------------------------------------

    /// 读取降采样颜色（`DownsampleColor`）。
    pub fn downsample_color(&self) -> Option<bool> { self.prop_bool("DownsampleColor") }
    /// 设置降采样颜色（`DownsampleColor`）。
    pub fn set_downsample_color(&self, v: bool) -> bool {
        self.put_bool("DownsampleColor", v)
    }

    /// 读取降采样灰度（`DownsampleGray`）。
    pub fn downsample_gray(&self) -> Option<bool> { self.prop_bool("DownsampleGray") }
    /// 设置降采样灰度（`DownsampleGray`）。
    pub fn set_downsample_gray(&self, v: bool) -> bool {
        self.put_bool("DownsampleGray", v)
    }

    /// 读取降采样单色（`DownsampleMono`）。
    pub fn downsample_mono(&self) -> Option<bool> { self.prop_bool("DownsampleMono") }
    /// 设置降采样单色（`DownsampleMono`）。
    pub fn set_downsample_mono(&self, v: bool) -> bool {
        self.put_bool("DownsampleMono", v)
    }

    /// 读取颜色分辨率（`ColorResolution`）。
    pub fn color_resolution(&self) -> Option<i64> { self.prop_i64("ColorResolution") }
    /// 设置颜色分辨率（`ColorResolution`）。
    pub fn set_color_resolution(&self, v: i32) -> bool {
        self.put_i64("ColorResolution", v as i64)
    }

    /// 读取单色分辨率（`MonoResolution`）。
    pub fn mono_resolution(&self) -> Option<i64> { self.prop_i64("MonoResolution") }
    /// 设置单色分辨率（`MonoResolution`）。
    pub fn set_mono_resolution(&self, v: i32) -> bool {
        self.put_i64("MonoResolution", v as i64)
    }

    /// 读取灰度分辨率（`GrayResolution`）。
    pub fn gray_resolution(&self) -> Option<i64> { self.prop_i64("GrayResolution") }
    /// 设置灰度分辨率（`GrayResolution`）。
    pub fn set_gray_resolution(&self, v: i32) -> bool {
        self.put_i64("GrayResolution", v as i64)
    }

    // ---------------------------------------------------------
    // 超链接/ 书签 / 缩略图
    // ---------------------------------------------------------

    /// 读取超链接（`Hyperlinks`）。
    pub fn hyperlinks(&self) -> Option<bool> { self.prop_bool("Hyperlinks") }
    /// 设置超链接（`Hyperlinks`）。
    pub fn set_hyperlinks(&self, v: bool) -> bool { self.put_bool("Hyperlinks", v) }

    /// 读取书签（`Bookmarks`）。
    pub fn bookmarks(&self) -> Option<bool> { self.prop_bool("Bookmarks") }
    /// 设置书签（`Bookmarks`）。
    pub fn set_bookmarks(&self, v: bool) -> bool { self.put_bool("Bookmarks", v) }

    /// 读取 COM 属性 `Thumbnails`。
    pub fn thumbnails(&self) -> Option<bool> { self.prop_bool("Thumbnails") }
    /// 设置 COM 属性 `Thumbnails`。
    pub fn set_thumbnails(&self, v: bool) -> bool { self.put_bool("Thumbnails", v) }

    /// `pdfDisplayOnStart`
    pub fn startup(&self) -> Option<i64> { self.prop_i64("Startup") }
    /// 设置启动（`Startup`）。
    pub fn set_startup(&self, v: i32) -> bool { self.put_i64("Startup", v as i64) }

    // ---------------------------------------------------------
    // 复杂填充 / 叠印 / 半色调/ 专色
    // ---------------------------------------------------------

    /// 读取 COM 属性 `ComplexFillsAsBitmaps`。
    pub fn complex_fills_as_bitmaps(&self) -> Option<bool> {
        self.prop_bool("ComplexFillsAsBitmaps")
    }
    /// 设置 COM 属性 `ComplexFillsAsBitmaps`。
    pub fn set_complex_fills_as_bitmaps(&self, v: bool) -> bool {
        self.put_bool("ComplexFillsAsBitmaps", v)
    }

    /// 读取叠印（`Overprints`）。
    pub fn overprints(&self) -> Option<bool> { self.prop_bool("Overprints") }
    /// 设置叠印（`Overprints`）。
    pub fn set_overprints(&self, v: bool) -> bool { self.put_bool("Overprints", v) }

    /// 读取 COM 属性 `Halftones`。
    pub fn halftones(&self) -> Option<bool> { self.prop_bool("Halftones") }
    /// 设置 COM 属性 `Halftones`。
    pub fn set_halftones(&self, v: bool) -> bool { self.put_bool("Halftones", v) }

    /// 读取专色颜色集（`SpotColors`）。
    pub fn spot_colors(&self) -> Option<bool> { self.prop_bool("SpotColors") }
    /// 设置专色颜色集（`SpotColors`）。
    pub fn set_spot_colors(&self, v: bool) -> bool { self.put_bool("SpotColors", v) }

    /// 读取保持 OPI 链接（`MaintainOPILinks`）。
    pub fn maintain_opi_links(&self) -> Option<bool> { self.prop_bool("MaintainOPILinks") }
    /// 设置保持 OPI 链接（`MaintainOPILinks`）。
    pub fn set_maintain_opi_links(&self, v: bool) -> bool {
        self.put_bool("MaintainOPILinks", v)
    }

    /// 读取渐变步数（`FountainSteps`）。
    pub fn fountain_steps(&self) -> Option<i64> { self.prop_i64("FountainSteps") }
    /// 设置渐变步数（`FountainSteps`）。
    pub fn set_fountain_steps(&self, v: i32) -> bool {
        self.put_i64("FountainSteps", v as i64)
    }

    /// `pdfEPSAs`
    pub fn eps_as(&self) -> Option<i64> { self.prop_i64("EPSAs") }
    /// 设置 EPSas（`EPSAs`）。
    pub fn set_eps_as(&self, v: i32) -> bool { self.put_i64("EPSAs", v as i64) }

    /// `pdfVersion`
    pub fn pdf_version(&self) -> Option<i64> { self.prop_i64("pdfVersion") }
    /// 设置 PDF 版本（`pdfVersion`）。
    pub fn set_pdf_version(&self, v: i32) -> bool {
        self.put_i64("pdfVersion", v as i64)
    }

    // ---------------------------------------------------------
    // 出血 / 标记
    // ---------------------------------------------------------

    /// 读取包含出血（`IncludeBleed`）。
    pub fn include_bleed(&self) -> Option<bool> { self.prop_bool("IncludeBleed") }
    /// 设置包含出血（`IncludeBleed`）。
    pub fn set_include_bleed(&self, v: bool) -> bool { self.put_bool("IncludeBleed", v) }

    /// 读取出血（`Bleed`）。
    pub fn bleed(&self) -> Option<i64> { self.prop_i64("Bleed") }
    /// 设置出血（`Bleed`）。
    pub fn set_bleed(&self, v: i32) -> bool { self.put_i64("Bleed", v as i64) }

    /// 读取 COM 属性 `Linearize`。
    pub fn linearize(&self) -> Option<bool> { self.prop_bool("Linearize") }
    /// 设置 COM 属性 `Linearize`。
    pub fn set_linearize(&self, v: bool) -> bool { self.put_bool("Linearize", v) }

    /// 读取裁剪标记（`CropMarks`）。
    pub fn crop_marks(&self) -> Option<bool> { self.prop_bool("CropMarks") }
    /// 设置裁剪标记（`CropMarks`）。
    pub fn set_crop_marks(&self, v: bool) -> bool { self.put_bool("CropMarks", v) }

    /// 读取套准标记（`RegistrationMarks`）。
    pub fn registration_marks(&self) -> Option<bool> { self.prop_bool("RegistrationMarks") }
    /// 设置套准标记（`RegistrationMarks`）。
    pub fn set_registration_marks(&self, v: bool) -> bool {
        self.put_bool("RegistrationMarks", v)
    }

    /// 读取 COM 属性 `DensitometerScales`。
    pub fn densitometer_scales(&self) -> Option<bool> { self.prop_bool("DensitometerScales") }
    /// 设置 COM 属性 `DensitometerScales`。
    pub fn set_densitometer_scales(&self, v: bool) -> bool {
        self.put_bool("DensitometerScales", v)
    }

    /// 读取文件信息（`FileInformation`）。
    pub fn file_information(&self) -> Option<bool> { self.prop_bool("FileInformation") }
    /// 设置文件信息（`FileInformation`）。
    pub fn set_file_information(&self, v: bool) -> bool {
        self.put_bool("FileInformation", v)
    }

    // ---------------------------------------------------------
    // 颜色
    // ---------------------------------------------------------

    /// `pdfColorMode`
    pub fn color_mode(&self) -> Option<i64> { self.prop_i64("ColorMode") }
    /// 设置颜色模式（`ColorMode`）。
    pub fn set_color_mode(&self, v: i32) -> bool { self.put_i64("ColorMode", v as i64) }

    /// 读取使用颜色配置文件（`UseColorProfile`）。
    pub fn use_color_profile(&self) -> Option<bool> { self.prop_bool("UseColorProfile") }
    /// 设置使用颜色配置文件（`UseColorProfile`）。
    pub fn set_use_color_profile(&self, v: bool) -> bool {
        self.put_bool("UseColorProfile", v)
    }

    /// `pdfColorProfile`
    pub fn color_profile(&self) -> Option<i64> { self.prop_i64("ColorProfile") }
    /// 设置颜色配置文件（`ColorProfile`）。
    pub fn set_color_profile(&self, v: i32) -> bool {
        self.put_i64("ColorProfile", v as i64)
    }

    /// 读取转换专色颜色集（`ConvertSpotColors`）。
    pub fn convert_spot_colors(&self) -> Option<bool> { self.prop_bool("ConvertSpotColors") }
    /// 设置转换专色颜色集（`ConvertSpotColors`）。
    pub fn set_convert_spot_colors(&self, v: bool) -> bool {
        self.put_bool("ConvertSpotColors", v)
    }

    /// `pdfSpotType`
    pub fn output_spot_colors_as(&self) -> Option<i64> {
        self.prop_i64("OutputSpotColorsAs")
    }
    /// 设置 COM 属性 `OutputSpotColorsAs`。
    pub fn set_output_spot_colors_as(&self, v: i32) -> bool {
        self.put_i64("OutputSpotColorsAs", v as i64)
    }

    /// 读取叠印黑色限制（`OverprintBlackLimit`）。
    pub fn overprint_black_limit(&self) -> Option<i64> {
        self.prop_i64("OverprintBlackLimit")
    }
    /// 设置叠印黑色限制（`OverprintBlackLimit`）。
    pub fn set_overprint_black_limit(&self, v: i32) -> bool {
        self.put_i64("OverprintBlackLimit", v as i64)
    }

    // ---------------------------------------------------------
    // 嵌入文件
    // ---------------------------------------------------------

    /// 读取 COM 属性 `EmbedFilename`。
    pub fn embed_filename(&self) -> Option<String> { self.prop_string("EmbedFilename") }
    /// 设置 COM 属性 `EmbedFilename`。
    pub fn set_embed_filename(&self, v: impl Into<String>) -> bool {
        self.put_string("EmbedFilename", v)
    }

    /// 读取嵌入文件（`EmbedFile`）。
    pub fn embed_file(&self) -> Option<bool> { self.prop_bool("EmbedFile") }
    /// 设置嵌入文件（`EmbedFile`）。
    pub fn set_embed_file(&self, v: bool) -> bool { self.put_bool("EmbedFile", v) }

    // ---------------------------------------------------------
    // 加密 / 权限
    // ---------------------------------------------------------

    /// `pdfPrintPermissions`
    pub fn print_permissions(&self) -> Option<i64> { self.prop_i64("PrintPermissions") }
    /// 设置打印权限（`PrintPermissions`）。
    pub fn set_print_permissions(&self, v: i32) -> bool {
        self.put_i64("PrintPermissions", v as i64)
    }

    /// `pdfEditPermissions`
    pub fn edit_permissions(&self) -> Option<i64> { self.prop_i64("EditPermissions") }
    /// 设置编辑权限（`EditPermissions`）。
    pub fn set_edit_permissions(&self, v: i32) -> bool {
        self.put_i64("EditPermissions", v as i64)
    }

    /// 读取 COM 属性 `ContentCopyingAllowed`。
    pub fn content_copying_allowed(&self) -> Option<bool> {
        self.prop_bool("ContentCopyingAllowed")
    }
    /// 设置 COM 属性 `ContentCopyingAllowed`。
    pub fn set_content_copying_allowed(&self, v: bool) -> bool {
        self.put_bool("ContentCopyingAllowed", v)
    }

    /// 读取打开密码（`OpenPassword`）。
    pub fn open_password(&self) -> Option<String> { self.prop_string("OpenPassword") }
    /// 设置打开密码（`OpenPassword`）。
    pub fn set_open_password(&self, v: impl Into<String>) -> bool {
        self.put_string("OpenPassword", v)
    }

    /// 读取 COM 属性 `PermissionPassword`。
    pub fn permission_password(&self) -> Option<String> {
        self.prop_string("PermissionPassword")
    }
    /// 设置 COM 属性 `PermissionPassword`。
    pub fn set_permission_password(&self, v: impl Into<String>) -> bool {
        self.put_string("PermissionPassword", v)
    }

    /// `pdfEncryptionType`
    pub fn encrypt_type(&self) -> Option<i64> { self.prop_i64("EncryptType") }
    /// 设置 COM 属性 `EncryptType`。
    pub fn set_encrypt_type(&self, v: i32) -> bool {
        self.put_i64("EncryptType", v as i64)
    }
}
//! `IPDFVBASettings` 鈥斺€?PDF 瀵煎嚭璁剧疆
//!
//! 鐢?`document.pdf_settings()` 鍙栧緱銆?
//! **娉ㄦ剰**锛氳鎺ュ彛鐨勬柟娉曞懡鍚嶄笉鏄?`get_/put_` 鑰屾槸 `get_/put_`锛屽睘鎬т篃寰堝銆?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgPdfVbaSettings {
    disp: ComObject,
}

impl IvgPdfVbaSettings {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

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
    // 涓绘搷浣?
    // ---------------------------------------------------------

    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    pub fn load(&self, setting_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(setting_name.into())];
        self.disp.invoke_method("Load", args).ok()?.to_bool().ok()
    }

    pub fn save(&self, setting_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(setting_name.into())];
        self.disp.invoke_method("Save", args).ok()?.to_bool().ok()
    }

    pub fn show_dialog(&self) -> Option<bool> {
        self.disp.invoke_method("ShowDialog", vec![]).ok()?.to_bool().ok()
    }

    /// 瀵煎嚭涓?PDF銆?
    pub fn publish_to_pdf(&self, file_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp.invoke_method("PublishToPDF", args).is_ok()
    }

    // ---------------------------------------------------------
    // 鍙戝竷鑼冨洿
    // ---------------------------------------------------------

    /// `pdfExportRange`
    pub fn publish_range(&self) -> Option<i64> { self.prop_i64("PublishRange") }
    pub fn set_publish_range(&self, v: i32) -> bool { self.put_i64("PublishRange", v as i64) }

    pub fn page_range(&self) -> Option<String> { self.prop_string("PageRange") }
    pub fn set_page_range(&self, v: impl Into<String>) -> bool {
        self.put_string("PageRange", v)
    }

    // ---------------------------------------------------------
    // 鏂囨。淇℃伅
    // ---------------------------------------------------------

    pub fn author(&self) -> Option<String> { self.prop_string("Author") }
    pub fn set_author(&self, v: impl Into<String>) -> bool {
        self.put_string("Author", v)
    }

    pub fn subject(&self) -> Option<String> { self.prop_string("Subject") }
    pub fn set_subject(&self, v: impl Into<String>) -> bool {
        self.put_string("Subject", v)
    }

    pub fn keywords(&self) -> Option<String> { self.prop_string("Keywords") }
    pub fn set_keywords(&self, v: impl Into<String>) -> bool {
        self.put_string("Keywords", v)
    }

    // ---------------------------------------------------------
    // 浣嶅浘鍘嬬缉
    // ---------------------------------------------------------

    /// `pdfBitmapCompressionType`
    pub fn bitmap_compression(&self) -> Option<i64> { self.prop_i64("BitmapCompression") }
    pub fn set_bitmap_compression(&self, v: i32) -> bool {
        self.put_i64("BitmapCompression", v as i64)
    }

    pub fn jpeg_quality_factor(&self) -> Option<i64> { self.prop_i64("JPEGQualityFactor") }
    pub fn set_jpeg_quality_factor(&self, v: i32) -> bool {
        self.put_i64("JPEGQualityFactor", v as i64)
    }

    pub fn jp2_quality_factor(&self) -> Option<i64> { self.prop_i64("JP2QualityFactor") }
    pub fn set_jp2_quality_factor(&self, v: i32) -> bool {
        self.put_i64("JP2QualityFactor", v as i64)
    }

    // ---------------------------------------------------------
    // 鏂囨湰 / 瀛椾綋
    // ---------------------------------------------------------

    pub fn text_as_curves(&self) -> Option<bool> { self.prop_bool("TextAsCurves") }
    pub fn set_text_as_curves(&self, v: bool) -> bool {
        self.put_bool("TextAsCurves", v)
    }

    pub fn embed_fonts(&self) -> Option<bool> { self.prop_bool("EmbedFonts") }
    pub fn set_embed_fonts(&self, v: bool) -> bool { self.put_bool("EmbedFonts", v) }

    pub fn embed_base_fonts(&self) -> Option<bool> { self.prop_bool("EmbedBaseFonts") }
    pub fn set_embed_base_fonts(&self, v: bool) -> bool {
        self.put_bool("EmbedBaseFonts", v)
    }

    pub fn truetype_to_type1(&self) -> Option<bool> { self.prop_bool("TrueTypeToType1") }
    pub fn set_truetype_to_type1(&self, v: bool) -> bool {
        self.put_bool("TrueTypeToType1", v)
    }

    pub fn subset_fonts(&self) -> Option<bool> { self.prop_bool("SubsetFonts") }
    pub fn set_subset_fonts(&self, v: bool) -> bool { self.put_bool("SubsetFonts", v) }

    pub fn subset_pct(&self) -> Option<i64> { self.prop_i64("SubsetPct") }
    pub fn set_subset_pct(&self, v: i32) -> bool { self.put_i64("SubsetPct", v as i64) }

    pub fn compress_text(&self) -> Option<bool> { self.prop_bool("CompressText") }
    pub fn set_compress_text(&self, v: bool) -> bool { self.put_bool("CompressText", v) }

    /// `pdfEncodingType`
    pub fn encoding(&self) -> Option<i64> { self.prop_i64("Encoding") }
    pub fn set_encoding(&self, v: i32) -> bool { self.put_i64("Encoding", v as i64) }

    pub fn protected_text_as_curves(&self) -> Option<bool> {
        self.prop_bool("ProtectedTextAsCurves")
    }
    pub fn set_protected_text_as_curves(&self, v: bool) -> bool {
        self.put_bool("ProtectedTextAsCurves", v)
    }

    /// `pdfTextExportMode`
    pub fn text_export_mode(&self) -> Option<i64> { self.prop_i64("TextExportMode") }
    pub fn set_text_export_mode(&self, v: i32) -> bool {
        self.put_i64("TextExportMode", v as i64)
    }

    // ---------------------------------------------------------
    // 闄嶉噰鏍?
    // ---------------------------------------------------------

    pub fn downsample_color(&self) -> Option<bool> { self.prop_bool("DownsampleColor") }
    pub fn set_downsample_color(&self, v: bool) -> bool {
        self.put_bool("DownsampleColor", v)
    }

    pub fn downsample_gray(&self) -> Option<bool> { self.prop_bool("DownsampleGray") }
    pub fn set_downsample_gray(&self, v: bool) -> bool {
        self.put_bool("DownsampleGray", v)
    }

    pub fn downsample_mono(&self) -> Option<bool> { self.prop_bool("DownsampleMono") }
    pub fn set_downsample_mono(&self, v: bool) -> bool {
        self.put_bool("DownsampleMono", v)
    }

    pub fn color_resolution(&self) -> Option<i64> { self.prop_i64("ColorResolution") }
    pub fn set_color_resolution(&self, v: i32) -> bool {
        self.put_i64("ColorResolution", v as i64)
    }

    pub fn mono_resolution(&self) -> Option<i64> { self.prop_i64("MonoResolution") }
    pub fn set_mono_resolution(&self, v: i32) -> bool {
        self.put_i64("MonoResolution", v as i64)
    }

    pub fn gray_resolution(&self) -> Option<i64> { self.prop_i64("GrayResolution") }
    pub fn set_gray_resolution(&self, v: i32) -> bool {
        self.put_i64("GrayResolution", v as i64)
    }

    // ---------------------------------------------------------
    // 瓒呴摼鎺?/ 涔︾ / 缂╃暐鍥?
    // ---------------------------------------------------------

    pub fn hyperlinks(&self) -> Option<bool> { self.prop_bool("Hyperlinks") }
    pub fn set_hyperlinks(&self, v: bool) -> bool { self.put_bool("Hyperlinks", v) }

    pub fn bookmarks(&self) -> Option<bool> { self.prop_bool("Bookmarks") }
    pub fn set_bookmarks(&self, v: bool) -> bool { self.put_bool("Bookmarks", v) }

    pub fn thumbnails(&self) -> Option<bool> { self.prop_bool("Thumbnails") }
    pub fn set_thumbnails(&self, v: bool) -> bool { self.put_bool("Thumbnails", v) }

    /// `pdfDisplayOnStart`
    pub fn startup(&self) -> Option<i64> { self.prop_i64("Startup") }
    pub fn set_startup(&self, v: i32) -> bool { self.put_i64("Startup", v as i64) }

    // ---------------------------------------------------------
    // 澶嶆潅濉厖 / 鍙犲嵃 / 鍗婅壊璋?/ 涓撹壊
    // ---------------------------------------------------------

    pub fn complex_fills_as_bitmaps(&self) -> Option<bool> {
        self.prop_bool("ComplexFillsAsBitmaps")
    }
    pub fn set_complex_fills_as_bitmaps(&self, v: bool) -> bool {
        self.put_bool("ComplexFillsAsBitmaps", v)
    }

    pub fn overprints(&self) -> Option<bool> { self.prop_bool("Overprints") }
    pub fn set_overprints(&self, v: bool) -> bool { self.put_bool("Overprints", v) }

    pub fn halftones(&self) -> Option<bool> { self.prop_bool("Halftones") }
    pub fn set_halftones(&self, v: bool) -> bool { self.put_bool("Halftones", v) }

    pub fn spot_colors(&self) -> Option<bool> { self.prop_bool("SpotColors") }
    pub fn set_spot_colors(&self, v: bool) -> bool { self.put_bool("SpotColors", v) }

    pub fn maintain_opi_links(&self) -> Option<bool> { self.prop_bool("MaintainOPILinks") }
    pub fn set_maintain_opi_links(&self, v: bool) -> bool {
        self.put_bool("MaintainOPILinks", v)
    }

    pub fn fountain_steps(&self) -> Option<i64> { self.prop_i64("FountainSteps") }
    pub fn set_fountain_steps(&self, v: i32) -> bool {
        self.put_i64("FountainSteps", v as i64)
    }

    /// `pdfEPSAs`
    pub fn eps_as(&self) -> Option<i64> { self.prop_i64("EPSAs") }
    pub fn set_eps_as(&self, v: i32) -> bool { self.put_i64("EPSAs", v as i64) }

    /// `pdfVersion`
    pub fn pdf_version(&self) -> Option<i64> { self.prop_i64("pdfVersion") }
    pub fn set_pdf_version(&self, v: i32) -> bool {
        self.put_i64("pdfVersion", v as i64)
    }

    // ---------------------------------------------------------
    // 鍑鸿 / 鏍囪
    // ---------------------------------------------------------

    pub fn include_bleed(&self) -> Option<bool> { self.prop_bool("IncludeBleed") }
    pub fn set_include_bleed(&self, v: bool) -> bool { self.put_bool("IncludeBleed", v) }

    pub fn bleed(&self) -> Option<i64> { self.prop_i64("Bleed") }
    pub fn set_bleed(&self, v: i32) -> bool { self.put_i64("Bleed", v as i64) }

    pub fn linearize(&self) -> Option<bool> { self.prop_bool("Linearize") }
    pub fn set_linearize(&self, v: bool) -> bool { self.put_bool("Linearize", v) }

    pub fn crop_marks(&self) -> Option<bool> { self.prop_bool("CropMarks") }
    pub fn set_crop_marks(&self, v: bool) -> bool { self.put_bool("CropMarks", v) }

    pub fn registration_marks(&self) -> Option<bool> { self.prop_bool("RegistrationMarks") }
    pub fn set_registration_marks(&self, v: bool) -> bool {
        self.put_bool("RegistrationMarks", v)
    }

    pub fn densitometer_scales(&self) -> Option<bool> { self.prop_bool("DensitometerScales") }
    pub fn set_densitometer_scales(&self, v: bool) -> bool {
        self.put_bool("DensitometerScales", v)
    }

    pub fn file_information(&self) -> Option<bool> { self.prop_bool("FileInformation") }
    pub fn set_file_information(&self, v: bool) -> bool {
        self.put_bool("FileInformation", v)
    }

    // ---------------------------------------------------------
    // 棰滆壊
    // ---------------------------------------------------------

    /// `pdfColorMode`
    pub fn color_mode(&self) -> Option<i64> { self.prop_i64("ColorMode") }
    pub fn set_color_mode(&self, v: i32) -> bool { self.put_i64("ColorMode", v as i64) }

    pub fn use_color_profile(&self) -> Option<bool> { self.prop_bool("UseColorProfile") }
    pub fn set_use_color_profile(&self, v: bool) -> bool {
        self.put_bool("UseColorProfile", v)
    }

    /// `pdfColorProfile`
    pub fn color_profile(&self) -> Option<i64> { self.prop_i64("ColorProfile") }
    pub fn set_color_profile(&self, v: i32) -> bool {
        self.put_i64("ColorProfile", v as i64)
    }

    pub fn convert_spot_colors(&self) -> Option<bool> { self.prop_bool("ConvertSpotColors") }
    pub fn set_convert_spot_colors(&self, v: bool) -> bool {
        self.put_bool("ConvertSpotColors", v)
    }

    /// `pdfSpotType`
    pub fn output_spot_colors_as(&self) -> Option<i64> {
        self.prop_i64("OutputSpotColorsAs")
    }
    pub fn set_output_spot_colors_as(&self, v: i32) -> bool {
        self.put_i64("OutputSpotColorsAs", v as i64)
    }

    pub fn overprint_black_limit(&self) -> Option<i64> {
        self.prop_i64("OverprintBlackLimit")
    }
    pub fn set_overprint_black_limit(&self, v: i32) -> bool {
        self.put_i64("OverprintBlackLimit", v as i64)
    }

    // ---------------------------------------------------------
    // 宓屽叆鏂囦欢
    // ---------------------------------------------------------

    pub fn embed_filename(&self) -> Option<String> { self.prop_string("EmbedFilename") }
    pub fn set_embed_filename(&self, v: impl Into<String>) -> bool {
        self.put_string("EmbedFilename", v)
    }

    pub fn embed_file(&self) -> Option<bool> { self.prop_bool("EmbedFile") }
    pub fn set_embed_file(&self, v: bool) -> bool { self.put_bool("EmbedFile", v) }

    // ---------------------------------------------------------
    // 鍔犲瘑 / 鏉冮檺
    // ---------------------------------------------------------

    /// `pdfPrintPermissions`
    pub fn print_permissions(&self) -> Option<i64> { self.prop_i64("PrintPermissions") }
    pub fn set_print_permissions(&self, v: i32) -> bool {
        self.put_i64("PrintPermissions", v as i64)
    }

    /// `pdfEditPermissions`
    pub fn edit_permissions(&self) -> Option<i64> { self.prop_i64("EditPermissions") }
    pub fn set_edit_permissions(&self, v: i32) -> bool {
        self.put_i64("EditPermissions", v as i64)
    }

    pub fn content_copying_allowed(&self) -> Option<bool> {
        self.prop_bool("ContentCopyingAllowed")
    }
    pub fn set_content_copying_allowed(&self, v: bool) -> bool {
        self.put_bool("ContentCopyingAllowed", v)
    }

    pub fn open_password(&self) -> Option<String> { self.prop_string("OpenPassword") }
    pub fn set_open_password(&self, v: impl Into<String>) -> bool {
        self.put_string("OpenPassword", v)
    }

    pub fn permission_password(&self) -> Option<String> {
        self.prop_string("PermissionPassword")
    }
    pub fn set_permission_password(&self, v: impl Into<String>) -> bool {
        self.put_string("PermissionPassword", v)
    }

    /// `pdfEncryptionType`
    pub fn encrypt_type(&self) -> Option<i64> { self.prop_i64("EncryptType") }
    pub fn set_encrypt_type(&self, v: i32) -> bool {
        self.put_i64("EncryptType", v as i64)
    }
}
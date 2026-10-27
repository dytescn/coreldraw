//! `IPrnVBAPrintPostScript` —— PostScript 输出设置

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IPrnVBAPrintPostScript` 对象的后期绑定包装（PostScript）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnPostScript {
    disp: ComObject,
}

impl IvgPrnPostScript {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- PostScript 级别 ----

    /// `PrnPostScriptLevel`
    pub fn level(&self) -> Option<i64> { self.prop_i64("Level") }
    /// 设置级别（`Level`）。
    pub fn set_level(&self, v: i32) -> bool { self.put_i64("Level", v as i64) }

    /// 读取 COM 属性 `ConformToDSC`。
    pub fn conform_to_dsc(&self) -> Option<bool> { self.prop_bool("ConformToDSC") }
    /// 设置 COM 属性 `ConformToDSC`。
    pub fn set_conform_to_dsc(&self, v: bool) -> bool { self.put_bool("ConformToDSC", v) }

    // ---- JPEG 压缩 ----

    /// 读取 JPEG 压缩（`JPEGCompression`）。
    pub fn jpeg_compression(&self) -> Option<bool> { self.prop_bool("JPEGCompression") }
    /// 设置 JPEG 压缩（`JPEGCompression`）。
    pub fn set_jpeg_compression(&self, v: bool) -> bool {
        self.put_bool("JPEGCompression", v)
    }

    /// 读取 JPEG 质量（`JPEGQuality`）。
    pub fn jpeg_quality(&self) -> Option<i64> { self.prop_i64("JPEGQuality") }
    /// 设置 JPEG 质量（`JPEGQuality`）。
    pub fn set_jpeg_quality(&self, v: i32) -> bool { self.put_i64("JPEGQuality", v as i64) }

    // ---- 链接 ----

    /// 读取保持 OPI 链接（`MaintainOPILinks`）。
    pub fn maintain_opi_links(&self) -> Option<bool> { self.prop_bool("MaintainOPILinks") }
    /// 设置保持 OPI 链接（`MaintainOPILinks`）。
    pub fn set_maintain_opi_links(&self, v: bool) -> bool {
        self.put_bool("MaintainOPILinks", v)
    }

    /// 读取 COM 属性 `ResolveDCSLinks`。
    pub fn resolve_dcs_links(&self) -> Option<bool> { self.prop_bool("ResolveDCSLinks") }
    /// 设置 COM 属性 `ResolveDCSLinks`。
    pub fn set_resolve_dcs_links(&self, v: bool) -> bool {
        self.put_bool("ResolveDCSLinks", v)
    }

    // ---- 字体 ----

    /// 读取 COM 属性 `DownloadType1`。
    pub fn download_type1(&self) -> Option<bool> { self.prop_bool("DownloadType1") }
    /// 设置 COM 属性 `DownloadType1`。
    pub fn set_download_type1(&self, v: bool) -> bool {
        self.put_bool("DownloadType1", v)
    }

    /// 读取 TrueTypeType1（`TrueTypeToType1`）。
    pub fn truetype_to_type1(&self) -> Option<bool> { self.prop_bool("TrueTypeToType1") }
    /// 设置 TrueTypeType1（`TrueTypeToType1`）。
    pub fn set_truetype_to_type1(&self, v: bool) -> bool {
        self.put_bool("TrueTypeToType1", v)
    }

    // ---- PDF 选项 ----

    /// `PrnPDFStartup`
    pub fn pdf_startup(&self) -> Option<i64> { self.prop_i64("PDFStartup") }
    /// 设置 PDF 启动（`PDFStartup`）。
    pub fn set_pdf_startup(&self, v: i32) -> bool { self.put_i64("PDFStartup", v as i64) }

    /// 读取 PDF 超链接（`PDFHyperlinks`）。
    pub fn pdf_hyperlinks(&self) -> Option<bool> { self.prop_bool("PDFHyperlinks") }
    /// 设置 PDF 超链接（`PDFHyperlinks`）。
    pub fn set_pdf_hyperlinks(&self, v: bool) -> bool {
        self.put_bool("PDFHyperlinks", v)
    }

    /// 读取 PDF 书签（`pdfBookmarks`）。
    pub fn pdf_bookmarks(&self) -> Option<bool> { self.prop_bool("pdfBookmarks") }
    /// 设置 PDF 书签（`pdfBookmarks`）。
    pub fn set_pdf_bookmarks(&self, v: bool) -> bool {
        self.put_bool("pdfBookmarks", v)
    }

    // ---- 曲线 / 平滑 ----

    /// 读取最大点集曲线（`MaxPointsPerCurve`）。
    pub fn max_points_per_curve(&self) -> Option<i64> { self.prop_i64("MaxPointsPerCurve") }
    /// 设置最大点集曲线（`MaxPointsPerCurve`）。
    pub fn set_max_points_per_curve(&self, v: i32) -> bool {
        self.put_i64("MaxPointsPerCurve", v as i64)
    }

    /// 读取平坦度（`Flatness`）。
    pub fn flatness(&self) -> Option<i64> { self.prop_i64("Flatness") }
    /// 设置平坦度（`Flatness`）。
    pub fn set_flatness(&self, v: i32) -> bool { self.put_i64("Flatness", v as i64) }

    /// 读取自动增加平坦度（`AutoIncreaseFlatness`）。
    pub fn auto_increase_flatness(&self) -> Option<bool> {
        self.prop_bool("AutoIncreaseFlatness")
    }
    /// 设置自动增加平坦度（`AutoIncreaseFlatness`）。
    pub fn set_auto_increase_flatness(&self, v: bool) -> bool {
        self.put_bool("AutoIncreaseFlatness", v)
    }

    /// 读取自动增加渐变步数（`AutoIncreaseFountainSteps`）。
    pub fn auto_increase_fountain_steps(&self) -> Option<bool> {
        self.prop_bool("AutoIncreaseFountainSteps")
    }
    /// 设置自动增加渐变步数（`AutoIncreaseFountainSteps`）。
    pub fn set_auto_increase_fountain_steps(&self, v: bool) -> bool {
        self.put_bool("AutoIncreaseFountainSteps", v)
    }

    /// 读取 COM 属性 `OptimizeFountainFills`。
    pub fn optimize_fountain_fills(&self) -> Option<bool> {
        self.prop_bool("OptimizeFountainFills")
    }
    /// 设置 COM 属性 `OptimizeFountainFills`。
    pub fn set_optimize_fountain_fills(&self, v: bool) -> bool {
        self.put_bool("OptimizeFountainFills", v)
    }

    /// 读取屏幕频率（`ScreenFrequency`）。
    pub fn screen_frequency(&self) -> Option<i64> { self.prop_i64("ScreenFrequency") }
    /// 设置屏幕频率（`ScreenFrequency`）。
    pub fn set_screen_frequency(&self, v: i32) -> bool {
        self.put_i64("ScreenFrequency", v as i64)
    }
}
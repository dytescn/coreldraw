//! `IPrnVBAPrintPostScript` 鈥斺€?PostScript 杈撳嚭璁剧疆

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgPrnPostScript {
    disp: ComObject,
}

impl IvgPrnPostScript {
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

    // ---- PostScript 绾у埆 ----

    /// `PrnPostScriptLevel`
    pub fn level(&self) -> Option<i64> { self.prop_i64("Level") }
    pub fn set_level(&self, v: i32) -> bool { self.put_i64("Level", v as i64) }

    pub fn conform_to_dsc(&self) -> Option<bool> { self.prop_bool("ConformToDSC") }
    pub fn set_conform_to_dsc(&self, v: bool) -> bool { self.put_bool("ConformToDSC", v) }

    // ---- JPEG 鍘嬬缉 ----

    pub fn jpeg_compression(&self) -> Option<bool> { self.prop_bool("JPEGCompression") }
    pub fn set_jpeg_compression(&self, v: bool) -> bool {
        self.put_bool("JPEGCompression", v)
    }

    pub fn jpeg_quality(&self) -> Option<i64> { self.prop_i64("JPEGQuality") }
    pub fn set_jpeg_quality(&self, v: i32) -> bool { self.put_i64("JPEGQuality", v as i64) }

    // ---- 閾炬帴 ----

    pub fn maintain_opi_links(&self) -> Option<bool> { self.prop_bool("MaintainOPILinks") }
    pub fn set_maintain_opi_links(&self, v: bool) -> bool {
        self.put_bool("MaintainOPILinks", v)
    }

    pub fn resolve_dcs_links(&self) -> Option<bool> { self.prop_bool("ResolveDCSLinks") }
    pub fn set_resolve_dcs_links(&self, v: bool) -> bool {
        self.put_bool("ResolveDCSLinks", v)
    }

    // ---- 瀛椾綋 ----

    pub fn download_type1(&self) -> Option<bool> { self.prop_bool("DownloadType1") }
    pub fn set_download_type1(&self, v: bool) -> bool {
        self.put_bool("DownloadType1", v)
    }

    pub fn truetype_to_type1(&self) -> Option<bool> { self.prop_bool("TrueTypeToType1") }
    pub fn set_truetype_to_type1(&self, v: bool) -> bool {
        self.put_bool("TrueTypeToType1", v)
    }

    // ---- PDF 閫夐」 ----

    /// `PrnPDFStartup`
    pub fn pdf_startup(&self) -> Option<i64> { self.prop_i64("PDFStartup") }
    pub fn set_pdf_startup(&self, v: i32) -> bool { self.put_i64("PDFStartup", v as i64) }

    pub fn pdf_hyperlinks(&self) -> Option<bool> { self.prop_bool("PDFHyperlinks") }
    pub fn set_pdf_hyperlinks(&self, v: bool) -> bool {
        self.put_bool("PDFHyperlinks", v)
    }

    pub fn pdf_bookmarks(&self) -> Option<bool> { self.prop_bool("pdfBookmarks") }
    pub fn set_pdf_bookmarks(&self, v: bool) -> bool {
        self.put_bool("pdfBookmarks", v)
    }

    // ---- 鏇茬嚎 / 骞虫粦 ----

    pub fn max_points_per_curve(&self) -> Option<i64> { self.prop_i64("MaxPointsPerCurve") }
    pub fn set_max_points_per_curve(&self, v: i32) -> bool {
        self.put_i64("MaxPointsPerCurve", v as i64)
    }

    pub fn flatness(&self) -> Option<i64> { self.prop_i64("Flatness") }
    pub fn set_flatness(&self, v: i32) -> bool { self.put_i64("Flatness", v as i64) }

    pub fn auto_increase_flatness(&self) -> Option<bool> {
        self.prop_bool("AutoIncreaseFlatness")
    }
    pub fn set_auto_increase_flatness(&self, v: bool) -> bool {
        self.put_bool("AutoIncreaseFlatness", v)
    }

    pub fn auto_increase_fountain_steps(&self) -> Option<bool> {
        self.prop_bool("AutoIncreaseFountainSteps")
    }
    pub fn set_auto_increase_fountain_steps(&self, v: bool) -> bool {
        self.put_bool("AutoIncreaseFountainSteps", v)
    }

    pub fn optimize_fountain_fills(&self) -> Option<bool> {
        self.prop_bool("OptimizeFountainFills")
    }
    pub fn set_optimize_fountain_fills(&self, v: bool) -> bool {
        self.put_bool("OptimizeFountainFills", v)
    }

    pub fn screen_frequency(&self) -> Option<i64> { self.prop_i64("ScreenFrequency") }
    pub fn set_screen_frequency(&self, v: i32) -> bool {
        self.put_i64("ScreenFrequency", v as i64)
    }
}
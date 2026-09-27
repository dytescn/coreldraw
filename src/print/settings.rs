//! `IPrnVBAPrintSettings` 鈥斺€?鎵撳嵃璁剧疆锛堟渶鏍稿績鐨勬墦鍗板璞★級

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::print::{
    IvgPrnLayout, IvgPrnOptions, IvgPrnPostScript, IvgPrnPrepress, IvgPrnPrinter,
    IvgPrnSeparations, IvgPrnTrapping,
};

pub struct IvgPrnSettings {
    disp: ComObject,
}

impl IvgPrnSettings {
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

    // fn put_dispatch(&self, name: &str, v: Variant) -> bool {
    //     self.disp.set_property(name, vec![v]).is_ok()
    // }

    // ---------------------------------------------------------
    // 鎵撳嵃鏈?
    // ---------------------------------------------------------

    pub fn printer(&self) -> Option<IvgPrnPrinter> {
        self.prop_dispatch("Printer").map(IvgPrnPrinter::new)
    }

    // pub fn set_printer(&self, p: &IvgPrnPrinter) -> bool {
    //     self.put_dispatch("Printer", Variant::from_dispatch( p.as_variant()))
    // }

    pub fn select_printer(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("SelectPrinter", args).is_ok()
    }

    // ---------------------------------------------------------
    // PPD 鏂囦欢
    // ---------------------------------------------------------

    pub fn use_ppd(&self) -> Option<bool> { self.prop_bool("UsePPD") }
    pub fn set_use_ppd(&self, v: bool) -> bool { self.put_bool("UsePPD", v) }

    pub fn ppd_file(&self) -> Option<String> { self.prop_string("PPDFile") }
    pub fn set_ppd_file(&self, v: impl Into<String>) -> bool {
        self.put_string("PPDFile", v)
    }

    // ---------------------------------------------------------
    // 杈撳嚭鍒版枃浠?
    // ---------------------------------------------------------

    pub fn print_to_file(&self) -> Option<bool> { self.prop_bool("PrintToFile") }
    pub fn set_print_to_file(&self, v: bool) -> bool { self.put_bool("PrintToFile", v) }

    pub fn file_name(&self) -> Option<String> { self.prop_string("FileName") }
    pub fn set_file_name(&self, v: impl Into<String>) -> bool {
        self.put_string("FileName", v)
    }

    pub fn for_mac(&self) -> Option<bool> { self.prop_bool("ForMac") }
    pub fn set_for_mac(&self, v: bool) -> bool { self.put_bool("ForMac", v) }

    /// `PrnFileMode`
    pub fn file_mode(&self) -> Option<i64> { self.prop_i64("FileMode") }
    pub fn set_file_mode(&self, v: i32) -> bool { self.put_i64("FileMode", v as i64) }

    // ---------------------------------------------------------
    // 鎵撳嵃鑼冨洿 / 浠芥暟
    // ---------------------------------------------------------

    /// `PrnPrintRange`
    pub fn print_range(&self) -> Option<i64> { self.prop_i64("PrintRange") }
    pub fn set_print_range(&self, v: i32) -> bool {
        self.put_i64("PrintRange", v as i64)
    }

    /// `PrnPageSet`
    pub fn page_set(&self) -> Option<i64> { self.prop_i64("PageSet") }
    pub fn set_page_set(&self, v: i32) -> bool { self.put_i64("PageSet", v as i64) }

    pub fn page_range(&self) -> Option<String> { self.prop_string("PageRange") }
    pub fn set_page_range(&self, v: impl Into<String>) -> bool {
        self.put_string("PageRange", v)
    }

    pub fn copies(&self) -> Option<i64> { self.prop_i64("Copies") }
    pub fn set_copies(&self, v: i32) -> bool { self.put_i64("Copies", v as i64) }

    pub fn collate(&self) -> Option<bool> { self.prop_bool("Collate") }
    pub fn set_collate(&self, v: bool) -> bool { self.put_bool("Collate", v) }

    // ---------------------------------------------------------
    // 绾稿紶 / 鏂瑰悜
    // ---------------------------------------------------------

    /// `PrnPaperOrientation`
    pub fn paper_orientation(&self) -> Option<i64> { self.prop_i64("PaperOrientation") }
    pub fn set_paper_orientation(&self, v: i32) -> bool {
        self.put_i64("PaperOrientation", v as i64)
    }

    /// `PrnPaperSize`
    pub fn paper_size(&self) -> Option<i64> { self.prop_i64("PaperSize") }
    pub fn set_paper_size(&self, v: i32) -> bool {
        self.put_i64("PaperSize", v as i64)
    }

    pub fn set_paper_size_ex(&self, size: i32, orientation: i32) -> bool {
        let args = vec![
            Variant::from_i64(size as i64),
            Variant::from_i64(orientation as i64),
        ];
        self.disp.invoke_method("SetPaperSize", args).is_ok()
    }

    pub fn paper_width(&self) -> Option<f64> { self.prop_f64("PaperWidth") }
    pub fn paper_height(&self) -> Option<f64> { self.prop_f64("PaperHeight") }

    pub fn set_custom_paper_size(
        &self,
        width: f64,
        height: f64,
        orientation: i32,
    ) -> bool {
        let args = vec![
            Variant::from_f64(width),
            Variant::from_f64(height),
            Variant::from_i64(orientation as i64),
        ];
        self.disp.invoke_method("SetCustomPaperSize", args).is_ok()
    }

    // ---------------------------------------------------------
    // 椤甸潰鍖归厤
    // ---------------------------------------------------------

    /// `PrnPageMatchingMode`
    pub fn page_matching_mode(&self) -> Option<i64> {
        self.prop_i64("PageMatchingMode")
    }
    pub fn set_page_matching_mode(&self, v: i32) -> bool {
        self.put_i64("PageMatchingMode", v as i64)
    }

    // ---------------------------------------------------------
    // 瀛愬璞?
    // ---------------------------------------------------------

    pub fn separations(&self) -> Option<IvgPrnSeparations> {
        self.prop_dispatch("Separations").map(IvgPrnSeparations::new)
    }

    pub fn prepress(&self) -> Option<IvgPrnPrepress> {
        self.prop_dispatch("Prepress").map(IvgPrnPrepress::new)
    }

    pub fn postscript(&self) -> Option<IvgPrnPostScript> {
        self.prop_dispatch("PostScript").map(IvgPrnPostScript::new)
    }

    pub fn trapping(&self) -> Option<IvgPrnTrapping> {
        self.prop_dispatch("Trapping").map(IvgPrnTrapping::new)
    }

    pub fn options(&self) -> Option<IvgPrnOptions> {
        self.prop_dispatch("Options").map(IvgPrnOptions::new)
    }

    pub fn layout(&self) -> Option<IvgPrnLayout> {
        self.prop_dispatch("Layout").map(IvgPrnLayout::new)
    }

    // ---------------------------------------------------------
    // 鎿嶄綔
    // ---------------------------------------------------------

    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    pub fn load(&self, file_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp.invoke_method("Load", args).ok()?.to_bool().ok()
    }

    pub fn save(&self, file_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp.invoke_method("Save", args).ok()?.to_bool().ok()
    }

    pub fn show_dialog(&self) -> Option<bool> {
        self.disp
            .invoke_method("ShowDialog", vec![])
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn print_out(&self) -> bool {
        self.disp.invoke_method("PrintOut", vec![]).is_ok()
    }

    pub fn print_color_proof(&self, proof_settings: Variant) -> bool {
        let args = vec![proof_settings];
        self.disp.invoke_method("PrintColorProof", args).is_ok()
    }
}
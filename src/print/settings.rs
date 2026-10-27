//! `IPrnVBAPrintSettings` —— 打印设置（最核心的打印对象）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::print::{
    IvgPrnLayout, IvgPrnOptions, IvgPrnPostScript, IvgPrnPrepress, IvgPrnPrinter,
    IvgPrnSeparations, IvgPrnTrapping,
};

/// CorelDRAW `IPrnVBAPrintSettings` 对象的后期绑定包装（设置集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnSettings {
    disp: ComObject,
}

impl IvgPrnSettings {
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
    // 打印机
    // ---------------------------------------------------------

    /// 读取打印机（`Printer`）。
    pub fn printer(&self) -> Option<IvgPrnPrinter> {
        self.prop_dispatch("Printer").map(IvgPrnPrinter::new)
    }

    // pub fn set_printer(&self, p: &IvgPrnPrinter) -> bool {
    //     self.put_dispatch("Printer", Variant::from_dispatch( p.as_variant()))
    // }

    /// 选择打印机（COM 方法 `SelectPrinter`）。
    pub fn select_printer(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("SelectPrinter", args).is_ok()
    }

    // ---------------------------------------------------------
    // PPD 文件
    // ---------------------------------------------------------

    /// 读取使用 PPD（`UsePPD`）。
    pub fn use_ppd(&self) -> Option<bool> { self.prop_bool("UsePPD") }
    /// 设置使用 PPD（`UsePPD`）。
    pub fn set_use_ppd(&self, v: bool) -> bool { self.put_bool("UsePPD", v) }

    /// 读取 PPD 文件（`PPDFile`）。
    pub fn ppd_file(&self) -> Option<String> { self.prop_string("PPDFile") }
    /// 设置 PPD 文件（`PPDFile`）。
    pub fn set_ppd_file(&self, v: impl Into<String>) -> bool {
        self.put_string("PPDFile", v)
    }

    // ---------------------------------------------------------
    // 输出到文件
    // ---------------------------------------------------------

    /// 读取打印文件（`PrintToFile`）。
    pub fn print_to_file(&self) -> Option<bool> { self.prop_bool("PrintToFile") }
    /// 设置打印文件（`PrintToFile`）。
    pub fn set_print_to_file(&self, v: bool) -> bool { self.put_bool("PrintToFile", v) }

    /// 读取文件名称（`FileName`）。
    pub fn file_name(&self) -> Option<String> { self.prop_string("FileName") }
    /// 设置文件名称（`FileName`）。
    pub fn set_file_name(&self, v: impl Into<String>) -> bool {
        self.put_string("FileName", v)
    }

    /// 读取 COM 属性 `ForMac`。
    pub fn for_mac(&self) -> Option<bool> { self.prop_bool("ForMac") }
    /// 设置 COM 属性 `ForMac`。
    pub fn set_for_mac(&self, v: bool) -> bool { self.put_bool("ForMac", v) }

    /// `PrnFileMode`
    pub fn file_mode(&self) -> Option<i64> { self.prop_i64("FileMode") }
    /// 设置文件模式（`FileMode`）。
    pub fn set_file_mode(&self, v: i32) -> bool { self.put_i64("FileMode", v as i64) }

    // ---------------------------------------------------------
    // 打印范围 / 份数
    // ---------------------------------------------------------

    /// `PrnPrintRange`
    pub fn print_range(&self) -> Option<i64> { self.prop_i64("PrintRange") }
    /// 设置打印范围（`PrintRange`）。
    pub fn set_print_range(&self, v: i32) -> bool {
        self.put_i64("PrintRange", v as i64)
    }

    /// `PrnPageSet`
    pub fn page_set(&self) -> Option<i64> { self.prop_i64("PageSet") }
    /// 设置页面设置（`PageSet`）。
    pub fn set_page_set(&self, v: i32) -> bool { self.put_i64("PageSet", v as i64) }

    /// 读取页面范围（`PageRange`）。
    pub fn page_range(&self) -> Option<String> { self.prop_string("PageRange") }
    /// 设置页面范围（`PageRange`）。
    pub fn set_page_range(&self, v: impl Into<String>) -> bool {
        self.put_string("PageRange", v)
    }

    /// 读取 COM 属性 `Copies`。
    pub fn copies(&self) -> Option<i64> { self.prop_i64("Copies") }
    /// 设置 COM 属性 `Copies`。
    pub fn set_copies(&self, v: i32) -> bool { self.put_i64("Copies", v as i64) }

    /// 读取 COM 属性 `Collate`。
    pub fn collate(&self) -> Option<bool> { self.prop_bool("Collate") }
    /// 设置 COM 属性 `Collate`。
    pub fn set_collate(&self, v: bool) -> bool { self.put_bool("Collate", v) }

    // ---------------------------------------------------------
    // 纸张 / 方向
    // ---------------------------------------------------------

    /// `PrnPaperOrientation`
    pub fn paper_orientation(&self) -> Option<i64> { self.prop_i64("PaperOrientation") }
    /// 设置纸张方向（`PaperOrientation`）。
    pub fn set_paper_orientation(&self, v: i32) -> bool {
        self.put_i64("PaperOrientation", v as i64)
    }

    /// `PrnPaperSize`
    pub fn paper_size(&self) -> Option<i64> { self.prop_i64("PaperSize") }
    /// 设置纸张尺寸（`PaperSize`）。
    pub fn set_paper_size(&self, v: i32) -> bool {
        self.put_i64("PaperSize", v as i64)
    }

    /// 设置纸张尺寸扩展版（`SetPaperSize`）。
    pub fn set_paper_size_ex(&self, size: i32, orientation: i32) -> bool {
        let args = vec![
            Variant::from_i64(size as i64),
            Variant::from_i64(orientation as i64),
        ];
        self.disp.invoke_method("SetPaperSize", args).is_ok()
    }

    /// 读取纸张宽度（`PaperWidth`）。
    pub fn paper_width(&self) -> Option<f64> { self.prop_f64("PaperWidth") }
    /// 读取纸张高度（`PaperHeight`）。
    pub fn paper_height(&self) -> Option<f64> { self.prop_f64("PaperHeight") }

    /// 设置自定义纸张尺寸（`SetCustomPaperSize`）。
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
    // 页面匹配
    // ---------------------------------------------------------

    /// `PrnPageMatchingMode`
    pub fn page_matching_mode(&self) -> Option<i64> {
        self.prop_i64("PageMatchingMode")
    }
    /// 设置页面匹配模式（`PageMatchingMode`）。
    pub fn set_page_matching_mode(&self, v: i32) -> bool {
        self.put_i64("PageMatchingMode", v as i64)
    }

    // ---------------------------------------------------------
    // 子对象
    // ---------------------------------------------------------

    /// 读取分色（`Separations`）。
    pub fn separations(&self) -> Option<IvgPrnSeparations> {
        self.prop_dispatch("Separations").map(IvgPrnSeparations::new)
    }

    /// 读取印前（`Prepress`）。
    pub fn prepress(&self) -> Option<IvgPrnPrepress> {
        self.prop_dispatch("Prepress").map(IvgPrnPrepress::new)
    }

    /// 读取 PostScript（`PostScript`）。
    pub fn postscript(&self) -> Option<IvgPrnPostScript> {
        self.prop_dispatch("PostScript").map(IvgPrnPostScript::new)
    }

    /// 读取陷印（`Trapping`）。
    pub fn trapping(&self) -> Option<IvgPrnTrapping> {
        self.prop_dispatch("Trapping").map(IvgPrnTrapping::new)
    }

    /// 读取选项集（`Options`）。
    pub fn options(&self) -> Option<IvgPrnOptions> {
        self.prop_dispatch("Options").map(IvgPrnOptions::new)
    }

    /// 读取排版（`Layout`）。
    pub fn layout(&self) -> Option<IvgPrnLayout> {
        self.prop_dispatch("Layout").map(IvgPrnLayout::new)
    }

    // ---------------------------------------------------------
    // 操作
    // ---------------------------------------------------------

    /// 重置（COM 方法 `Reset`）。
    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    /// load（COM 方法 `Load`）。
    pub fn load(&self, file_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp.invoke_method("Load", args).ok()?.to_bool().ok()
    }

    /// 保存（COM 方法 `Save`）。
    pub fn save(&self, file_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp.invoke_method("Save", args).ok()?.to_bool().ok()
    }

    /// 显示对话框（COM 方法 `ShowDialog`）。
    pub fn show_dialog(&self) -> Option<bool> {
        self.disp
            .invoke_method("ShowDialog", vec![])
            .ok()?
            .to_bool()
            .ok()
    }

    /// 打印输出（COM 方法 `PrintOut`）。
    pub fn print_out(&self) -> bool {
        self.disp.invoke_method("PrintOut", vec![]).is_ok()
    }

    /// 打印颜色校样（COM 方法 `PrintColorProof`）。
    pub fn print_color_proof(&self, proof_settings: Variant) -> bool {
        let args = vec![proof_settings];
        self.disp.invoke_method("PrintColorProof", args).is_ok()
    }
}
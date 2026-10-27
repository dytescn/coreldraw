//! `IPrnVBAPrinter` / `IPrnVBAPrinters` —— 打印机

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPrnPrinters
// =============================================================

/// CorelDRAW `IPrnVBAPrinter` 对象的后期绑定包装（打印机集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnPrinters {
    disp: ComObject,
}

impl IvgPrnPrinters {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgPrnPrinter> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPrnPrinter::new)
    }

    /// 读取默认值（`Default`）。
    pub fn default(&self) -> Option<IvgPrnPrinter> {
        self.prop_dispatch("Default").map(IvgPrnPrinter::new)
    }

    /// 返回全部条目。
    pub fn all(&self) -> Vec<IvgPrnPrinter> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }
}

// =============================================================
// IvgPrnPrinter
// =============================================================

/// CorelDRAW `IPrnVBAPrinter` 对象的后期绑定包装（打印机）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnPrinter {
    disp: ComObject,
}

impl IvgPrnPrinter {
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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 属性----

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 读取打印机类型（`Type`）。
    pub fn printer_type(&self) -> Option<String> { self.prop_string("Type") }
    /// 是否默认值（`Default`）。
    pub fn is_default(&self) -> Option<bool> { self.prop_bool("Default") }
    /// 读取 COM 属性 `Ready`。
    pub fn ready(&self) -> Option<bool> { self.prop_bool("Ready") }
    /// 读取 COM 属性 `Port`。
    pub fn port(&self) -> Option<String> { self.prop_string("Port") }
    /// 读取描述（`Description`）。
    pub fn description(&self) -> Option<String> { self.prop_string("Description") }
    /// 读取 PostScript 启用状态（`PostScriptEnabled`）。
    pub fn postscript_enabled(&self) -> Option<bool> { self.prop_bool("PostScriptEnabled") }
    /// 读取颜色启用状态（`ColorEnabled`）。
    pub fn color_enabled(&self) -> Option<bool> { self.prop_bool("ColorEnabled") }

    /// 读取 COM 属性 `PageSizeMatchingSupported`。
    pub fn page_size_matching_supported(&self) -> Option<bool> {
        self.prop_bool("PageSizeMatchingSupported")
    }
    /// 设置 COM 属性 `PageSizeMatchingSupported`。
    pub fn set_page_size_matching_supported(&self, v: bool) -> bool {
        self.put_bool("PageSizeMatchingSupported", v)
    }

    /// 弹出打印机属性对话框。
    pub fn show_dialog(&self) -> bool {
        self.disp.invoke_method("ShowDialog", vec![]).is_ok()
    }
}
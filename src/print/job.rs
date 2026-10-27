//! `IPrnVBAPrintJob` —— 打印任务

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::print::{
    IvgPrnDocument, IvgPrnDocuments, IvgPrnPage, IvgPrnPages, IvgPrnSettings,
};

/// CorelDRAW `IPrnVBAPrintJob` 对象的后期绑定包装（任务）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnJob {
    disp: ComObject,
}

impl IvgPrnJob {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---- 子对象----

    /// 读取设置集（`Settings`）。
    pub fn settings(&self) -> Option<IvgPrnSettings> {
        self.prop_dispatch("Settings").map(IvgPrnSettings::new)
    }

    /// 读取文档集（`Documents`）。
    pub fn documents(&self) -> Option<IvgPrnDocuments> {
        self.prop_dispatch("Documents").map(IvgPrnDocuments::new)
    }

    /// 读取页面集（`Pages`）。
    pub fn pages(&self) -> Option<IvgPrnPages> {
        self.prop_dispatch("Pages").map(IvgPrnPages::new)
    }

    // ---- 操作 ----

    /// 清空全部内容（COM 方法 `Clear`）。
    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    /// 打印输出（COM 方法 `PrintOut`）。
    pub fn print_out(&self) -> bool {
        self.disp.invoke_method("PrintOut", vec![]).is_ok()
    }

    /// 添加文档（COM 方法 `AddDocument`）。
    pub fn add_document(
        &self,
        doc: &IvgPrnDocument,
        page_range: impl Into<String>,
    ) -> bool {
        let args = vec![
            doc.raw().as_variant(),
            Variant::from_str(page_range.into()),
        ];
        self.disp.invoke_method("AddDocument", args).is_ok()
    }

    /// 添加页面（COM 方法 `AddPage`）。
    pub fn add_page(&self, page: &IvgPrnPage) -> bool {
        let args = vec![page.raw().as_variant()];
        self.disp.invoke_method("AddPage", args).is_ok()
    }
}
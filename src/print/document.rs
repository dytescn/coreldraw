//! `IPrnVBAPrintDocument(s)` / `IPrnVBAPrintPage(s)` —— 打印文档 / 页面

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPrnDocuments
// =============================================================

/// CorelDRAW 对象的后期绑定包装（文档集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnDocuments {
    disp: ComObject,
}

impl IvgPrnDocuments {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgPrnDocument> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPrnDocument::new)
    }
}

// =============================================================
// IvgPrnDocument
// =============================================================

/// `IPrnVBAPrintDocument` ——只有 `_GetPrintDocument`（返回内部指针）。
pub struct IvgPrnDocument {
    disp: ComObject,
}

impl IvgPrnDocument {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    /// `_GetPrintDocument` 返回内部指针（`i64`），仅内部使用。
    pub fn internal_ptr(&self) -> Option<i64> {
        self.disp.invoke_method("_GetPrintDocument", vec![]).ok()?.to_i64().ok()
    }
}

// =============================================================
// IvgPrnPages
// =============================================================

/// CorelDRAW 对象的后期绑定包装（页面集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnPages {
    disp: ComObject,
}

impl IvgPrnPages {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgPrnPage> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPrnPage::new)
    }
}

// =============================================================
// IvgPrnPage
// =============================================================

/// CorelDRAW 对象的后期绑定包装（页面）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnPage {
    disp: ComObject,
}

impl IvgPrnPage {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    /// `_GetPrintDocument` ——内部指针。
    pub fn internal_document_ptr(&self) -> Option<i64> {
        self.disp.invoke_method("_GetPrintDocument", vec![]).ok()?.to_i64().ok()
    }

    /// `_GetPrintPage` ——页号。
    pub fn internal_page_index(&self) -> Option<i64> {
        self.disp.invoke_method("_GetPrintPage", vec![]).ok()?.to_i64().ok()
    }
}
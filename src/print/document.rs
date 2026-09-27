//! `IPrnVBAPrintDocument(s)` / `IPrnVBAPrintPage(s)` 鈥斺€?鎵撳嵃鏂囨。 / 椤甸潰

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPrnDocuments
// =============================================================

pub struct IvgPrnDocuments {
    disp: ComObject,
}

impl IvgPrnDocuments {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

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

/// `IPrnVBAPrintDocument` 鈥斺€?鍙湁 `_GetPrintDocument`锛堣繑鍥炲唴閮ㄦ寚閽堬級銆?
pub struct IvgPrnDocument {
    disp: ComObject,
}

impl IvgPrnDocument {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    /// `_GetPrintDocument` 杩斿洖鍐呴儴鎸囬拡锛坄i64`锛夛紝浠呭唴閮ㄤ娇鐢ㄣ€?
    pub fn internal_ptr(&self) -> Option<i64> {
        self.disp.invoke_method("_GetPrintDocument", vec![]).ok()?.to_i64().ok()
    }
}

// =============================================================
// IvgPrnPages
// =============================================================

pub struct IvgPrnPages {
    disp: ComObject,
}

impl IvgPrnPages {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

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

pub struct IvgPrnPage {
    disp: ComObject,
}

impl IvgPrnPage {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    /// `_GetPrintDocument` 鈥斺€?鍐呴儴鎸囬拡銆?
    pub fn internal_document_ptr(&self) -> Option<i64> {
        self.disp.invoke_method("_GetPrintDocument", vec![]).ok()?.to_i64().ok()
    }

    /// `_GetPrintPage` 鈥斺€?椤靛彿銆?
    pub fn internal_page_index(&self) -> Option<i64> {
        self.disp.invoke_method("_GetPrintPage", vec![]).ok()?.to_i64().ok()
    }
}
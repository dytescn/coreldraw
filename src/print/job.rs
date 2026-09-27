//! `IPrnVBAPrintJob` 鈥斺€?鎵撳嵃浠诲姟

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::print::{
    IvgPrnDocument, IvgPrnDocuments, IvgPrnPage, IvgPrnPages, IvgPrnSettings,
};

pub struct IvgPrnJob {
    disp: ComObject,
}

impl IvgPrnJob {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---- 瀛愬璞?----

    pub fn settings(&self) -> Option<IvgPrnSettings> {
        self.prop_dispatch("Settings").map(IvgPrnSettings::new)
    }

    pub fn documents(&self) -> Option<IvgPrnDocuments> {
        self.prop_dispatch("Documents").map(IvgPrnDocuments::new)
    }

    pub fn pages(&self) -> Option<IvgPrnPages> {
        self.prop_dispatch("Pages").map(IvgPrnPages::new)
    }

    // ---- 鎿嶄綔 ----

    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    pub fn print_out(&self) -> bool {
        self.disp.invoke_method("PrintOut", vec![]).is_ok()
    }

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

    pub fn add_page(&self, page: &IvgPrnPage) -> bool {
        let args = vec![page.raw().as_variant()];
        self.disp.invoke_method("AddPage", args).is_ok()
    }
}
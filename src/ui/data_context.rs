//! `ICUIDataContext` / `ICUIDataSourceFactory` / `ICUIDataSourceProxy`

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// ICuiDataContext
// =============================================================

pub struct ICuiDataContext {
    disp: ComObject,
}

impl ICuiDataContext {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    pub fn categories(&self) -> Option<String> { self.prop_string("Categories") }

    pub fn has_category(&self, category: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(category.into())];
        self.disp
            .invoke_method("HasCategory", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn create_child_data_context(
        &self,
        category_list: impl Into<String>,
    ) -> Option<ICuiDataContext> {
        let args = vec![Variant::from_str(category_list.into())];
        self.disp
            .invoke_method("CreateChildDataContext", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiDataContext::new)
    }

    pub fn add_data_source(
        &self,
        name: impl Into<String>,
        source: IDispatch,
    ) -> Option<ICuiDataSourceProxy> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_dispatch(&source),
        ];
        self.disp
            .invoke_method("AddDataSource", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiDataSourceProxy::new)
    }

    pub fn get_data_source(
        &self,
        name: impl Into<String>,
    ) -> Option<ICuiDataSourceProxy> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("GetDataSource", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiDataSourceProxy::new)
    }

    pub fn show_dialog(&self, dialog_id: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(dialog_id.into())];
        self.disp
            .invoke_method("ShowDialog", args)
            .ok()?
            .to_bool()
            .ok()
    }
}

// =============================================================
// ICuiDataSourceProxy
// =============================================================

pub struct ICuiDataSourceProxy {
    disp: ComObject,
}

impl ICuiDataSourceProxy {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    pub fn update_listeners(&self, names: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(names.into())];
        self.disp.invoke_method("UpdateListeners", args).is_ok()
    }

    pub fn invoke_method(&self, method_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(method_name.into())];
        self.disp.invoke_method("InvokeMethod", args).is_ok()
    }

    pub fn get_property(&self, name: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("GetProperty", args).ok()
    }

    pub fn set_property(&self, name: impl Into<String>, value: Variant) -> bool {
        let args = vec![
            Variant::from_str(name.into()),
            value,
        ];
        self.disp.invoke_method("SetProperty", args).is_ok()
    }
}

// =============================================================
// ICuiDataSourceFactory
// =============================================================

pub struct ICuiDataSourceFactory {
    disp: ComObject,
}

impl ICuiDataSourceFactory {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn create_data_source(
        &self,
        name: impl Into<String>,
        proxy: IDispatch,
    ) -> Option<IDispatch> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_dispatch(&proxy),
        ];
        self.disp
            .invoke_method("CreateDataSource", args)
            .ok()?
            .to_idispatch()
            .ok()
    }
}
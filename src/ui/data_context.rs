//! `ICUIDataContext` / `ICUIDataSourceFactory` / `ICUIDataSourceProxy`

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// ICuiDataContext
// =============================================================

/// CorelDRAW 对象的后期绑定包装（数据上下文）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiDataContext {
    disp: ComObject,
}

impl ICuiDataContext {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    /// 读取 COM 属性 `Categories`。
    pub fn categories(&self) -> Option<String> { self.prop_string("Categories") }

    /// 是否有类别（COM 方法 `HasCategory`）。
    pub fn has_category(&self, category: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(category.into())];
        self.disp
            .invoke_method("HasCategory", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 创建子对象数据上下文（COM 方法 `CreateChildDataContext`）。
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

    /// 添加数据源（COM 方法 `AddDataSource`）。
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

    /// 读取数据源（`GetDataSource`）。
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

    /// 显示对话框（COM 方法 `ShowDialog`）。
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

/// CorelDRAW 对象的后期绑定包装（数据源代理）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiDataSourceProxy {
    disp: ComObject,
}

impl ICuiDataSourceProxy {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    /// 调用 COM 方法 `UpdateListeners`。
    pub fn update_listeners(&self, names: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(names.into())];
        self.disp.invoke_method("UpdateListeners", args).is_ok()
    }

    /// 调用方法（COM 方法 `InvokeMethod`）。
    pub fn invoke_method(&self, method_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(method_name.into())];
        self.disp.invoke_method("InvokeMethod", args).is_ok()
    }

    /// 读取属性（`GetProperty`）。
    pub fn get_property(&self, name: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("GetProperty", args).ok()
    }

    /// 设置属性（`SetProperty`）。
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

/// CorelDRAW 对象的后期绑定包装（数据源工厂）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiDataSourceFactory {
    disp: ComObject,
}

impl ICuiDataSourceFactory {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 创建数据源（COM 方法 `CreateDataSource`）。
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
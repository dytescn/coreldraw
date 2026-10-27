//! GMS 宏系统（`IVGGMSManager` / `IVGGMSProject(s)` / `IVGGMSMacro(s)`）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgGmsManager
// =============================================================

/// CorelDRAW `IVGGMSManager` 对象的后期绑定包装（GMS 宏管理器）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgGmsManager {
    disp: ComObject,
}

impl IvgGmsManager {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    /// 读取 GMS 宏路径（`GMSPath`）。
    pub fn gms_path(&self) -> Option<String> { self.prop_string("GMSPath") }
    /// 读取用户 GMS 宏路径（`UserGMSPath`）。
    pub fn user_gms_path(&self) -> Option<String> { self.prop_string("UserGMSPath") }

    /// 读取项目集（`Projects`）。
    pub fn projects(&self) -> Option<IvgGmsProjects> {
        self.disp
            .get_property("Projects")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgGmsProjects::new)
    }

    /// 调用宏。`Parameters` 是 SAFEARRAY（VARIANT）。
    pub fn run_macro(
        &self,
        module_name: impl Into<String>,
        macro_name: impl Into<String>,
        parameters: Variant,
    ) -> Option<Variant> {
        let args = vec![
            Variant::from_str(module_name.into()),
            Variant::from_str(macro_name.into()),
            parameters,
        ];
        self.disp.invoke_method("RunMacro", args).ok()
    }
}

// =============================================================
// IvgGmsProjects
// =============================================================

/// CorelDRAW `IVGGMSManager` 对象的后期绑定包装（GMS 宏项目集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgGmsProjects {
    disp: ComObject,
}

impl IvgGmsProjects {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgGmsProject> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgGmsProject::new)
    }

    /// load（COM 方法 `Load`）。
    pub fn load(
        &self,
        file_name: impl Into<String>,
        copy_file: bool,
        for_all_users: bool,
    ) -> Option<IvgGmsProject> {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_bool(copy_file),
            Variant::from_bool(for_all_users),
        ];
        self.disp
            .invoke_method("Load", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgGmsProject::new)
    }

    /// 创建（COM 方法 `Create`）。
    pub fn create(
        &self,
        name: impl Into<String>,
        for_all_users: bool,
        file_name: impl Into<String>,
    ) -> Option<IvgGmsProject> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_bool(for_all_users),
            Variant::from_str(file_name.into()),
        ];
        self.disp
            .invoke_method("Create", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgGmsProject::new)
    }
}

// =============================================================
// IvgGmsProject
// =============================================================

/// CorelDRAW `IVGGMSManager` 对象的后期绑定包装（GMS 宏项目）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgGmsProject {
    disp: ComObject,
}

impl IvgGmsProject {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    /// 读取显示名称（`DisplayName`）。
    pub fn display_name(&self) -> Option<String> { self.prop_string("DisplayName") }
    /// 读取文件名称（`FileName`）。
    pub fn file_name(&self) -> Option<String> { self.prop_string("FileName") }
    /// 读取文件路径（`FilePath`）。
    pub fn file_path(&self) -> Option<String> { self.prop_string("FilePath") }
    /// 读取完整文件名称（`FullFileName`）。
    pub fn full_file_name(&self) -> Option<String> { self.prop_string("FullFileName") }

    /// 读取已修改（`Dirty`）。
    pub fn dirty(&self) -> Option<bool> { self.prop_bool("Dirty") }
    /// 设置已修改（`Dirty`）。
    pub fn set_dirty(&self, v: bool) -> bool { self.put_bool("Dirty", v) }

    /// 读取锁定状态（`Locked`）。
    pub fn locked(&self) -> Option<bool> { self.prop_bool("Locked") }
    /// 读取 COM 属性 `PasswordProtected`。
    pub fn password_protected(&self) -> Option<bool> { self.prop_bool("PasswordProtected") }

    /// 读取宏集（`Macros`）。
    pub fn macros(&self) -> Option<IvgGmsMacros> {
        self.disp
            .get_property("Macros")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgGmsMacros::new)
    }

    /// 调用 COM 方法 `Unload`。
    pub fn unload(&self) -> bool {
        self.disp.invoke_method("Unload", vec![]).is_ok()
    }
}

// =============================================================
// IvgGmsMacros
// =============================================================

/// CorelDRAW `IVGGMSManager` 对象的后期绑定包装（GMS 宏宏集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgGmsMacros {
    disp: ComObject,
}

impl IvgGmsMacros {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgGmsMacro> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgGmsMacro::new)
    }

    /// 创建（COM 方法 `Create`）。
    pub fn create(&self, name: impl Into<String>) -> Option<IvgGmsMacro> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("Create", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgGmsMacro::new)
    }
}

// =============================================================
// IvgGmsMacro
// =============================================================

/// CorelDRAW `IVGGMSManager` 对象的后期绑定包装（GMS 宏宏）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgGmsMacro {
    disp: ComObject,
}

impl IvgGmsMacro {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> {
        self.disp.get_property("Name").ok()?.to_string().ok()
    }

    /// 运行（COM 方法 `Run`）。
    pub fn run(&self) -> bool {
        self.disp.invoke_method("Run", vec![]).is_ok()
    }

    /// 编辑（COM 方法 `Edit`）。
    pub fn edit(&self) -> bool {
        self.disp.invoke_method("Edit", vec![]).is_ok()
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}
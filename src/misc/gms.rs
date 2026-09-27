//! GMS 瀹忕郴缁燂紙`IVGGMSManager` / `IVGGMSProject(s)` / `IVGGMSMacro(s)`锛?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgGmsManager
// =============================================================

pub struct IvgGmsManager {
    disp: ComObject,
}

impl IvgGmsManager {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    pub fn gms_path(&self) -> Option<String> { self.prop_string("GMSPath") }
    pub fn user_gms_path(&self) -> Option<String> { self.prop_string("UserGMSPath") }

    pub fn projects(&self) -> Option<IvgGmsProjects> {
        self.disp
            .get_property("Projects")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgGmsProjects::new)
    }

    /// 璋冪敤瀹忋€俙Parameters` 鏄?SAFEARRAY锛圴ARIANT锛夈€?
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

pub struct IvgGmsProjects {
    disp: ComObject,
}

impl IvgGmsProjects {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgGmsProject> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgGmsProject::new)
    }

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

pub struct IvgGmsProject {
    disp: ComObject,
}

impl IvgGmsProject {
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

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    pub fn display_name(&self) -> Option<String> { self.prop_string("DisplayName") }
    pub fn file_name(&self) -> Option<String> { self.prop_string("FileName") }
    pub fn file_path(&self) -> Option<String> { self.prop_string("FilePath") }
    pub fn full_file_name(&self) -> Option<String> { self.prop_string("FullFileName") }

    pub fn dirty(&self) -> Option<bool> { self.prop_bool("Dirty") }
    pub fn set_dirty(&self, v: bool) -> bool { self.put_bool("Dirty", v) }

    pub fn locked(&self) -> Option<bool> { self.prop_bool("Locked") }
    pub fn password_protected(&self) -> Option<bool> { self.prop_bool("PasswordProtected") }

    pub fn macros(&self) -> Option<IvgGmsMacros> {
        self.disp
            .get_property("Macros")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgGmsMacros::new)
    }

    pub fn unload(&self) -> bool {
        self.disp.invoke_method("Unload", vec![]).is_ok()
    }
}

// =============================================================
// IvgGmsMacros
// =============================================================

pub struct IvgGmsMacros {
    disp: ComObject,
}

impl IvgGmsMacros {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgGmsMacro> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgGmsMacro::new)
    }

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

pub struct IvgGmsMacro {
    disp: ComObject,
}

impl IvgGmsMacro {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn name(&self) -> Option<String> {
        self.disp.get_property("Name").ok()?.to_string().ok()
    }

    pub fn run(&self) -> bool {
        self.disp.invoke_method("Run", vec![]).is_ok()
    }

    pub fn edit(&self) -> bool {
        self.disp.invoke_method("Edit", vec![]).is_ok()
    }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}
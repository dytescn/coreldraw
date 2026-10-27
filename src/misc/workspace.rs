//! `IVGWorkspace(s)` —— 工作区

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgWorkspaces
// =============================================================

/// CorelDRAW 对象的后期绑定包装。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgWorkspaces {
    disp: ComObject,
}

impl IvgWorkspaces {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgWorkspace> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgWorkspace::new)
    }

    /// 条目索引。
    pub fn item_by_index(&self, i: i32) -> Option<IvgWorkspace> {
        self.item(Variant::from_i64(i as i64))
    }

    /// 条目名称。
    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgWorkspace> {
        self.item(Variant::from_str(name.into()))
    }
}

// =============================================================
// IvgWorkspace
// =============================================================

/// CorelDRAW 对象的后期绑定包装（工作区）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgWorkspace {
    disp: ComObject,
}

impl IvgWorkspace {
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

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 读取描述（`Description`）。
    pub fn description(&self) -> Option<String> { self.prop_string("Description") }
    /// 读取活动（`Active`）。
    pub fn active(&self) -> Option<bool> { self.prop_bool("Active") }

    /// activate（COM 方法 `Activate`）。
    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }
}
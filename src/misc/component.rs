//! `IVGComponent(s)` —— 组件（插件 / 功能模块）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgComponents
// =============================================================

/// CorelDRAW 对象的后期绑定包装。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgComponents {
    disp: ComObject,
}

impl IvgComponents {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从…构造分发。
    pub fn from_dispatch(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.prop_i64("Count")
    }

    /// `IndexOrName` 是 `VARIANT`（索引或名称）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgComponent> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgComponent::new)
    }

    /// 条目索引。
    pub fn item_by_index(&self, i: i32) -> Option<IvgComponent> {
        self.item(Variant::from_i64(i as i64))
    }

    /// 条目名称。
    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgComponent> {
        self.item(Variant::from_str(&name.into()))
    }

    /// 调用 COM 方法 `IsComponentInstalled`。
    pub fn is_component_installed(&self, component_id: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(&component_id.into())];
        self.disp
            .invoke_method("IsComponentInstalled", args)
            .ok()?
            .to_bool()
            .ok()
    }
}

// =============================================================
// IvgComponent
// =============================================================

/// CorelDRAW 对象的后期绑定包装。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgComponent {
    disp: ComObject,
}

impl IvgComponent {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从…构造分发。
    pub fn from_dispatch(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    /// 读取 COM 属性 `ComponentID`。
    pub fn component_id(&self) -> Option<String> {
        self.disp.get_property("ComponentID").ok()?.to_string().ok()
    }
}
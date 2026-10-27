//! `IVGSymbolDefinition(s)` —— 符号定义

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::IvgShapeRange;

// =============================================================
// IvgSymbolDefinitions
// =============================================================

/// CorelDRAW 对象的后期绑定包装（符号定义集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSymbolDefinitions {
    disp: ComObject,
}

impl IvgSymbolDefinitions {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgSymbolDefinition> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSymbolDefinition::new)
    }

    /// 条目索引。
    pub fn item_by_index(&self, i: i32) -> Option<IvgSymbolDefinition> {
        self.item(Variant::from_i64(i as i64))
    }

    /// 条目名称。
    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgSymbolDefinition> {
        self.item(Variant::from_str(name.into()))
    }

    /// 返回全部条目。
    pub fn all(&self) -> Vec<IvgSymbolDefinition> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item_by_index(i)).collect()
    }
}

// =============================================================
// IvgSymbolDefinition
// =============================================================

/// CorelDRAW 对象的后期绑定包装（符号定义）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSymbolDefinition {
    disp: ComObject,
}

impl IvgSymbolDefinition {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 基本信息
    // ---------------------------------------------------------

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    /// 读取描述（`Description`）。
    pub fn description(&self) -> Option<String> { self.prop_string("Description") }
    /// 设置描述（`Description`）。
    pub fn set_description(&self, v: impl Into<String>) -> bool {
        self.put_string("Description", v)
    }

    /// `cdrSymbolType`
    pub fn symbol_type(&self) -> Option<i64> { self.prop_i64("Type") }

    // ---------------------------------------------------------
    // 链接
    // ---------------------------------------------------------

    /// 读取已链接（`Linked`）。
    pub fn linked(&self) -> Option<bool> { self.prop_bool("Linked") }
    /// 读取链接库路径（`LinkLibraryPath`）。
    pub fn link_library_path(&self) -> Option<String> {
        self.prop_string("LinkLibraryPath")
    }
    /// 读取 COM 属性 `IsLinkBroken`。
    pub fn is_link_broken(&self) -> Option<bool> { self.prop_bool("IsLinkBroken") }
    /// 读取 COM 属性 `IsLinkUpdated`。
    pub fn is_link_updated(&self) -> Option<bool> { self.prop_bool("IsLinkUpdated") }
    /// 读取 COM 属性 `HasBrokenLinks`。
    pub fn has_broken_links(&self) -> Option<bool> { self.prop_bool("HasBrokenLinks") }
    /// 读取 COM 属性 `HasUpdatedLinks`。
    pub fn has_updated_links(&self) -> Option<bool> { self.prop_bool("HasUpdatedLinks") }

    // ---------------------------------------------------------
    // 嵌套
    // ---------------------------------------------------------

    /// 读取 COM 属性 `Nested`。
    pub fn nested(&self) -> Option<bool> { self.prop_bool("Nested") }
    /// 是否有链接（`HasLinks`）。
    pub fn has_links(&self) -> Option<bool> { self.prop_bool("HasLinks") }

    /// 嵌套的子符号定义。
    pub fn nested_symbols(&self) -> Option<IvgSymbolDefinitions> {
        self.prop_dispatch("NestedSymbols").map(IvgSymbolDefinitions::new)
    }

    // ---------------------------------------------------------
    // 可编辑/ 实例
    // ---------------------------------------------------------

    /// 读取可编辑（`Editable`）。
    pub fn editable(&self) -> Option<bool> { self.prop_bool("Editable") }

    /// 读取实例数量（`InstanceCount`）。
    pub fn instance_count(&self) -> Option<i64> { self.prop_i64("InstanceCount") }

    /// 所有实例（图形范围）。
    pub fn instances(&self) -> Option<IvgShapeRange> {
        self.prop_dispatch("Instances").map(IvgShapeRange::new)
    }

    // ---------------------------------------------------------
    // 编辑 / 复制 / 删除
    // ---------------------------------------------------------

    /// 输入编辑模式（COM 方法 `EnterEditMode`）。
    pub fn enter_edit_mode(&self) -> bool {
        self.disp.invoke_method("EnterEditMode", vec![]).is_ok()
    }

    /// 调用 COM 方法 `LeaveEditMode`。
    pub fn leave_edit_mode(&self) -> bool {
        self.disp.invoke_method("LeaveEditMode", vec![]).is_ok()
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    /// 复制（COM 方法 `Copy`）。
    pub fn copy(&self) -> bool {
        self.disp.invoke_method("Copy", vec![]).is_ok()
    }

    /// duplicate（COM 方法 `Duplicate`）。
    pub fn duplicate(&self, name: impl Into<String>) -> Option<IvgSymbolDefinition> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("Duplicate", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSymbolDefinition::new)
    }

    // ---------------------------------------------------------
    // 链接维护
    // ---------------------------------------------------------

    /// 断开链接（COM 方法 `BreakLink`）。
    pub fn break_link(&self) -> bool {
        self.disp.invoke_method("BreakLink", vec![]).is_ok()
    }

    /// 更新链接（COM 方法 `UpdateLinks`）。
    pub fn update_links(&self) -> bool {
        self.disp.invoke_method("UpdateLinks", vec![]).is_ok()
    }

    /// 调用 COM 方法 `FixLink`。
    pub fn fix_link(&self, library_path: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(library_path.into())];
        self.disp.invoke_method("FixLink", args).is_ok()
    }
}
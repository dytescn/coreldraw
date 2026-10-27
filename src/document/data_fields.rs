//! `DataField(s)` / `DataItem(s)` —— 数据字段与数据项

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgDataItems
// =============================================================

/// CorelDRAW 对象的后期绑定包装（数据条目集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgDataItems {
    disp: ComObject,
}

impl IvgDataItems {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgDataItem> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDataItem::new)
    }

    /// 条目索引。
    pub fn item_by_index(&self, i: i32) -> Option<IvgDataItem> {
        self.item(Variant::from_i64(i as i64))
    }

    /// 条目名称。
    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgDataItem> {
        self.item(Variant::from_str(name.into()))
    }

    /// 从其他图形复制数据项。
    pub fn copy_from(&self, shape: &crate::shape::IvgShape) -> bool {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("CopyFrom", args).is_ok()
    }

    /// 清空全部内容（COM 方法 `Clear`）。
    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }
}

// =============================================================
// IvgDataItem
// =============================================================

/// CorelDRAW 对象的后期绑定包装（数据条目）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgDataItem {
    disp: ComObject,
}

impl IvgDataItem {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 值（`VARIANT`）。
    pub fn value(&self) -> Option<Variant> {
        self.disp.get_property("Value").ok()
    }

    /// 设置值（`Value`）。
    pub fn set_value(&self, v: Variant) -> bool {
        self.disp.set_property("Value", vec![v]).is_ok()
    }

    /// 读取数据字段（`DataField`）。
    pub fn data_field(&self) -> Option<IvgDataField> {
        self.prop_dispatch("DataField").map(IvgDataField::new)
    }

    /// 读取 COM 属性 `FormattedValue`。
    pub fn formatted_value(&self) -> Option<String> {
        self.disp
            .get_property("FormattedValue")
            .ok()?
            .to_string()
            .ok()
    }

    /// 清空全部内容（COM 方法 `Clear`）。
    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }
}

// =============================================================
// IvgDataFields
// =============================================================

/// CorelDRAW 对象的后期绑定包装（数据字段）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgDataFields {
    disp: ComObject,
}

impl IvgDataFields {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgDataField> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDataField::new)
    }

    /// 条目索引。
    pub fn item_by_index(&self, i: i32) -> Option<IvgDataField> {
        self.item(Variant::from_i64(i as i64))
    }

    /// 条目名称。
    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgDataField> {
        self.item(Variant::from_str(name.into()))
    }

    /// 添加字段（简版）。
    pub fn add(
        &self,
        name: impl Into<String>,
        format: impl Into<String>,
        app_default: bool,
        doc_default: bool,
        summarize_group: bool,
    ) -> Option<IvgDataField> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_str(format.into()),
            Variant::from_bool(app_default),
            Variant::from_bool(doc_default),
            Variant::from_bool(summarize_group),
        ];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDataField::new)
    }

    /// 添加字段（扩展版）。
    #[allow(clippy::too_many_arguments)]
    pub fn add_ex(
        &self,
        name: impl Into<String>,
        data_type: i32,
        default_value: impl Into<String>,
        constraint: impl Into<String>,
        target: impl Into<String>,
        format: impl Into<String>,
        app_default: bool,
        doc_default: bool,
        summarize_group: bool,
        field_width: i32,
    ) -> Option<IvgDataField> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(data_type as i64),
            Variant::from_str(default_value.into()),
            Variant::from_str(constraint.into()),
            Variant::from_str(target.into()),
            Variant::from_str(format.into()),
            Variant::from_bool(app_default),
            Variant::from_bool(doc_default),
            Variant::from_bool(summarize_group),
            Variant::from_i64(field_width as i64),
        ];
        self.disp
            .invoke_method("AddEx", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDataField::new)
    }

    /// 是否存在（COM 方法 `IsPresent`）。
    pub fn is_present(&self, field_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(field_name.into())];
        self.disp
            .invoke_method("IsPresent", args)
            .ok()?
            .to_bool()
            .ok()
    }
}

// =============================================================
// IvgDataField
// =============================================================

/// CorelDRAW 对象的后期绑定包装（数据字段）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgDataField {
    disp: ComObject,
}

impl IvgDataField {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 字段定义 ----

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    /// 读取格式类型（`FormatType`）。
    pub fn format_type(&self) -> Option<i64> { self.prop_i64("FormatType") }

    /// 读取格式（`Format`）。
    pub fn format(&self) -> Option<String> { self.prop_string("Format") }
    /// 设置格式（`Format`）。
    pub fn set_format(&self, v: impl Into<String>) -> bool { self.put_string("Format", v) }

    /// 读取字段宽度（`FieldWidth`）。
    pub fn field_width(&self) -> Option<i64> { self.prop_i64("FieldWidth") }
    /// 设置字段宽度（`FieldWidth`）。
    pub fn set_field_width(&self, v: i32) -> bool { self.put_i64("FieldWidth", v as i64) }

    /// 读取数据类型（`DataType`）。
    pub fn data_type(&self) -> Option<i64> { self.prop_i64("DataType") }
    /// 设置数据类型（`DataType`）。
    pub fn set_data_type(&self, v: i32) -> bool { self.put_i64("DataType", v as i64) }

    /// 读取目标（`Target`）。
    pub fn target(&self) -> Option<String> { self.prop_string("Target") }
    /// 设置目标（`Target`）。
    pub fn set_target(&self, v: impl Into<String>) -> bool { self.put_string("Target", v) }

    /// 读取默认值值（`DefaultValue`）。
    pub fn default_value(&self) -> Option<String> { self.prop_string("DefaultValue") }
    /// 设置默认值值（`DefaultValue`）。
    pub fn set_default_value(&self, v: impl Into<String>) -> bool { self.put_string("DefaultValue", v) }

    /// 读取 COM 属性 `Constraint`。
    pub fn constraint(&self) -> Option<String> { self.prop_string("Constraint") }
    /// 设置 COM 属性 `Constraint`。
    pub fn set_constraint(&self, v: impl Into<String>) -> bool { self.put_string("Constraint", v) }

    /// 读取父对象名称（`ParentName`）。
    pub fn parent_name(&self) -> Option<String> { self.prop_string("ParentName") }
    /// 设置父对象名称（`ParentName`）。
    pub fn set_parent_name(&self, v: impl Into<String>) -> bool { self.put_string("ParentName", v) }

    /// 读取 COM 属性 `ElementName`。
    pub fn element_name(&self) -> Option<String> { self.prop_string("ElementName") }
    /// 设置 COM 属性 `ElementName`。
    pub fn set_element_name(&self, v: impl Into<String>) -> bool { self.put_string("ElementName", v) }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    // ---- 标志你----

    /// 读取应用默认值（`AppDefault`）。
    pub fn app_default(&self) -> Option<bool> { self.prop_bool("AppDefault") }
    /// 设置应用默认值（`AppDefault`）。
    pub fn set_app_default(&self, v: bool) -> bool { self.put_bool("AppDefault", v) }

    /// 读取文档默认值（`DocDefault`）。
    pub fn doc_default(&self) -> Option<bool> { self.prop_bool("DocDefault") }
    /// 设置文档默认值（`DocDefault`）。
    pub fn set_doc_default(&self, v: bool) -> bool { self.put_bool("DocDefault", v) }

    /// 读取 COM 属性 `SummarizeGroup`。
    pub fn summarize_group(&self) -> Option<bool> { self.prop_bool("SummarizeGroup") }
    /// 设置 COM 属性 `SummarizeGroup`。
    pub fn set_summarize_group(&self, v: bool) -> bool { self.put_bool("SummarizeGroup", v) }

    // ---- 操作 ----

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    /// 调用 COM 方法 `Reorder`。
    pub fn reorder(&self, new_index: i32) -> bool {
        let args = vec![Variant::from_i64(new_index as i64)];
        self.disp.invoke_method("Reorder", args).is_ok()
    }
}
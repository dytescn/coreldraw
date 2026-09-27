//! `DataField(s)` / `DataItem(s)` 鈥斺€?鏁版嵁瀛楁涓庢暟鎹」

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgDataItems
// =============================================================

pub struct IvgDataItems {
    disp: ComObject,
}

impl IvgDataItems {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgDataItem> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDataItem::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgDataItem> {
        self.item(Variant::from_i64(i as i64))
    }

    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgDataItem> {
        self.item(Variant::from_str(name.into()))
    }

    /// 浠庡叾浠栧浘褰㈠鍒舵暟鎹」銆?
    pub fn copy_from(&self, shape: &crate::shape::IvgShape) -> bool {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("CopyFrom", args).is_ok()
    }

    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }
}

// =============================================================
// IvgDataItem
// =============================================================

pub struct IvgDataItem {
    disp: ComObject,
}

impl IvgDataItem {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 鍊硷紙`VARIANT`锛夈€?
    pub fn value(&self) -> Option<Variant> {
        self.disp.get_property("Value").ok()
    }

    pub fn set_value(&self, v: Variant) -> bool {
        self.disp.set_property("Value", vec![v]).is_ok()
    }

    pub fn data_field(&self) -> Option<IvgDataField> {
        self.prop_dispatch("DataField").map(IvgDataField::new)
    }

    pub fn formatted_value(&self) -> Option<String> {
        self.disp
            .get_property("FormattedValue")
            .ok()?
            .to_string()
            .ok()
    }

    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }
}

// =============================================================
// IvgDataFields
// =============================================================

pub struct IvgDataFields {
    disp: ComObject,
}

impl IvgDataFields {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgDataField> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDataField::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgDataField> {
        self.item(Variant::from_i64(i as i64))
    }

    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgDataField> {
        self.item(Variant::from_str(name.into()))
    }

    /// 娣诲姞瀛楁锛堢畝鐗堬級銆?
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

    /// 娣诲姞瀛楁锛堟墿灞曠増锛夈€?
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

pub struct IvgDataField {
    disp: ComObject,
}

impl IvgDataField {
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

    // ---- 瀛楁瀹氫箟 ----

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    pub fn format_type(&self) -> Option<i64> { self.prop_i64("FormatType") }

    pub fn format(&self) -> Option<String> { self.prop_string("Format") }
    pub fn set_format(&self, v: impl Into<String>) -> bool { self.put_string("Format", v) }

    pub fn field_width(&self) -> Option<i64> { self.prop_i64("FieldWidth") }
    pub fn set_field_width(&self, v: i32) -> bool { self.put_i64("FieldWidth", v as i64) }

    pub fn data_type(&self) -> Option<i64> { self.prop_i64("DataType") }
    pub fn set_data_type(&self, v: i32) -> bool { self.put_i64("DataType", v as i64) }

    pub fn target(&self) -> Option<String> { self.prop_string("Target") }
    pub fn set_target(&self, v: impl Into<String>) -> bool { self.put_string("Target", v) }

    pub fn default_value(&self) -> Option<String> { self.prop_string("DefaultValue") }
    pub fn set_default_value(&self, v: impl Into<String>) -> bool { self.put_string("DefaultValue", v) }

    pub fn constraint(&self) -> Option<String> { self.prop_string("Constraint") }
    pub fn set_constraint(&self, v: impl Into<String>) -> bool { self.put_string("Constraint", v) }

    pub fn parent_name(&self) -> Option<String> { self.prop_string("ParentName") }
    pub fn set_parent_name(&self, v: impl Into<String>) -> bool { self.put_string("ParentName", v) }

    pub fn element_name(&self) -> Option<String> { self.prop_string("ElementName") }
    pub fn set_element_name(&self, v: impl Into<String>) -> bool { self.put_string("ElementName", v) }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    // ---- 鏍囧織浣?----

    pub fn app_default(&self) -> Option<bool> { self.prop_bool("AppDefault") }
    pub fn set_app_default(&self, v: bool) -> bool { self.put_bool("AppDefault", v) }

    pub fn doc_default(&self) -> Option<bool> { self.prop_bool("DocDefault") }
    pub fn set_doc_default(&self, v: bool) -> bool { self.put_bool("DocDefault", v) }

    pub fn summarize_group(&self) -> Option<bool> { self.prop_bool("SummarizeGroup") }
    pub fn set_summarize_group(&self, v: bool) -> bool { self.put_bool("SummarizeGroup", v) }

    // ---- 鎿嶄綔 ----

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    pub fn reorder(&self, new_index: i32) -> bool {
        let args = vec![Variant::from_i64(new_index as i64)];
        self.disp.invoke_method("Reorder", args).is_ok()
    }
}
//! `IVGProperties` —— 通用属性包（name/id 双键）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;
use crate::geometry::{IvgPoint, IvgVector};

/// CorelDRAW `IVGProperties` 对象的后期绑定包装（属性集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgProperties {
    disp: ComObject,
}

impl IvgProperties {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    // ---------------------------------------------------------
    // 计数
    // ---------------------------------------------------------

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    // ---------------------------------------------------------
    // 通用读写（name + id）
    // ---------------------------------------------------------

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, name: impl Into<String>, id: i32) -> Option<Variant> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
        ];
        self.disp.invoke_method("Item", args).ok()
    }

    /// 设置按索引或名称取条目（`put_Item`）。
    pub fn set_item(
        &self,
        name: impl Into<String>,
        id: i32,
        value: Variant,
    ) -> bool {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
            value,
        ];
        self.disp.invoke_method("put_Item", args).is_ok()
    }

    /// 条目索引（COM 方法 `ItemByIndex`）。
    pub fn item_by_index(&self, index: i32) -> Option<Variant> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("ItemByIndex", args).ok()
    }

    /// 索引（COM 方法 `Index`）。
    pub fn index_of(&self, name: impl Into<String>, id: i32) -> Option<i64> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
        ];
        self.disp.invoke_method("Index", args).ok()?.to_i64().ok()
    }

    /// exists（COM 方法 `Exists`）。
    pub fn exists(&self, name: impl Into<String>, id: i32) -> Option<bool> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
        ];
        self.disp.invoke_method("Exists", args).ok()?.to_bool().ok()
    }

    /// 描述（COM 方法 `Description`）。
    pub fn description(&self, index: i32) -> Option<(String, i32)> {
        // 双出参数，只取 name。
        let args = vec![Variant::from_i64(index as i64)];
        let name = self
            .disp
            .invoke_method("Description", args)
            .ok()?
            .to_string()
            .ok()?;
        Some((name, 0))
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self, name: impl Into<String>, id: i32) -> bool {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
        ];
        self.disp.invoke_method("Delete", args).is_ok()
    }

    /// 删除索引（COM 方法 `DeleteByIndex`）。
    pub fn delete_by_index(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("DeleteByIndex", args).is_ok()
    }

    /// 删除全部（COM 方法 `DeleteAll`）。
    pub fn delete_all(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("DeleteAll", args).ok()?.to_bool().ok()
    }

    // ---------------------------------------------------------
    // 文件操作
    // ---------------------------------------------------------

    /// 设置文件（`PutFile`）。
    pub fn put_file(
        &self,
        name: impl Into<String>,
        id: i32,
        file_name: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
            Variant::from_str(file_name.into()),
        ];
        self.disp.invoke_method("PutFile", args).is_ok()
    }

    /// 读取文件（`GetFile`）。
    pub fn get_file(
        &self,
        name: impl Into<String>,
        id: i32,
        file_name: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
            Variant::from_str(file_name.into()),
        ];
        self.disp.invoke_method("GetFile", args).is_ok()
    }

    // ---------------------------------------------------------
    // 类型化快捷方法（uuidName）
    // ---------------------------------------------------------

    /// 读取点（`GetPoint`）。
    pub fn get_point(&self, uuid_name: impl Into<String>) -> Option<IvgPoint> {
        let args = vec![Variant::from_str(uuid_name.into())];
        self.disp
            .invoke_method("GetPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPoint::new)
    }

    /// 设置点（`SetPoint`）。
    pub fn set_point(&self, uuid_name: impl Into<String>, p: &IvgPoint) -> bool {
        let args = vec![
            Variant::from_str(uuid_name.into()),
            p.as_variant(),
        ];
        self.disp.invoke_method("SetPoint", args).is_ok()
    }

    /// 读取向量（`GetVector`）。
    pub fn get_vector(&self, uuid_name: impl Into<String>) -> Option<IvgVector> {
        let args = vec![Variant::from_str(uuid_name.into())];
        self.disp
            .invoke_method("GetVector", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgVector::new)
    }

    /// 设置向量（`SetVector`）。
    pub fn set_vector(&self, uuid_name: impl Into<String>, v: &IvgVector) -> bool {
        let args = vec![
            Variant::from_str(uuid_name.into()),
            v.as_variant(),
        ];
        self.disp.invoke_method("SetVector", args).is_ok()
    }

    /// 读取曲线（`GetCurve`）。
    pub fn get_curve(&self, uuid_name: impl Into<String>) -> Option<IvgCurve> {
        let args = vec![Variant::from_str(uuid_name.into())];
        self.disp
            .invoke_method("GetCurve", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgCurve::new)
    }

    /// 设置曲线（`SetCurve`）。
    pub fn set_curve(&self, uuid_name: impl Into<String>, c: &IvgCurve) -> bool {
        let args = vec![
            Variant::from_str(uuid_name.into()),
            c.as_variant(),
        ];
        self.disp.invoke_method("SetCurve", args).is_ok()
    }
}
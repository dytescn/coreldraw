//! `IVGPageSize` / `IVGPageSizes` —— 命名页面尺寸

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPageSizes
// =============================================================

/// CorelDRAW `IVGPageSize` 对象的后期绑定包装（页面尺寸集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPageSizes {
    disp: ComObject,
}

impl IvgPageSizes {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgPageSize> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPageSize::new)
    }

    /// 条目索引。
    pub fn item_by_index(&self, i: i32) -> Option<IvgPageSize> {
        self.item(Variant::from_i64(i as i64))
    }

    /// 条目名称。
    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgPageSize> {
        self.item(Variant::from_str(name.into()))
    }

    /// 添加自定义页面尺寸。
    pub fn add(
        &self,
        name: impl Into<String>,
        width: f64,
        height: f64,
    ) -> Option<IvgPageSize> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_f64(width),
            Variant::from_f64(height),
        ];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPageSize::new)
    }

    /// 遍历所有。
    pub fn all(&self) -> Vec<IvgPageSize> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item_by_index(i)).collect()
    }
}

// =============================================================
// IvgPageSize
// =============================================================

/// CorelDRAW `IVGPageSize` 对象的后期绑定包装（页面尺寸）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPageSize {
    disp: ComObject,
}

impl IvgPageSize {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<f64> { self.prop_f64("Height") }
    /// 设置高度（`Height`）。
    pub fn set_height(&self, v: f64) -> bool { self.put_f64("Height", v) }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    /// 读取 COM 属性 `BuiltIn`。
    pub fn built_in(&self) -> Option<bool> { self.prop_bool("BuiltIn") }
    /// 读取固定方向（`FixedOrientation`）。
    pub fn fixed_orientation(&self) -> Option<bool> { self.prop_bool("FixedOrientation") }

    /// `cdrUnit` ——该尺寸所用单位。
    pub fn default_unit(&self) -> Option<i64> { self.prop_i64("DefaultUnit") }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}
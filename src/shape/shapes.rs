//! `IVGShapes` —— 图形集合

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::{IvgShape, IvgShapeRange};

/// CorelDRAW `IVGShapes` 对象的后期绑定包装（图形集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgShapes {
    disp: ComObject,
}

impl IvgShapes {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 读取按索引或名称取条目。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgShape> {
        // `Item` 是带参数的属性（parameterized property），必须用
        // DISPATCH_PROPERTYGET；用 DISPATCH_METHOD 会返回 DISP_E_MEMBERNOTFOUND。
        let args = vec![index_or_name];
        self.disp
            .get_property_with("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 条目索引。
    pub fn item_by_index(&self, i: i32) -> Option<IvgShape> {
        self.item(Variant::from_i64(i as i64))
    }

    /// 条目名称。
    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgShape> {
        self.item(Variant::from_str(name.into()))
    }


    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgShape> { self.prop_dispatch("First").map(IvgShape::new) }
    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgShape> { self.prop_dispatch("Last").map(IvgShape::new) }

    /// 返回全部条目。
    pub fn all(&self) -> Vec<IvgShape> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item_by_index(i)).collect()
    }

    /// 用索引数组取子范围。
    pub fn range(&self, indices: Variant) -> Option<IvgShapeRange> {
        let args = vec![indices];
        self.disp
            .invoke_method("Range", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 全部图形（返回范围）。
    pub fn all_range(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("All", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 调用 COM 方法 `AllExcluding`。
    pub fn all_excluding(&self, indices: Variant) -> Option<IvgShapeRange> {
        let args = vec![indices];
        self.disp
            .invoke_method("AllExcluding", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 查找图形（COM 方法 `FindShape`）。
    pub fn find_shape(
        &self,
        name: impl Into<String>,
        shape_type: i32,
        static_id: i32,
        recursive: bool,
        query: impl Into<String>,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(shape_type as i64),
            Variant::from_i64(static_id as i64),
            Variant::from_bool(recursive),
            Variant::from_str(query.into()),
        ];
        self.disp
            .invoke_method("FindShape", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 查找图形集（COM 方法 `FindShapes`）。
    pub fn find_shapes(
        &self,
        name: impl Into<String>,
        shape_type: i32,
        recursive: bool,
        query: impl Into<String>,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(shape_type as i64),
            Variant::from_bool(recursive),
            Variant::from_str(query.into()),
        ];
        self.disp
            .invoke_method("FindShapes", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }
}
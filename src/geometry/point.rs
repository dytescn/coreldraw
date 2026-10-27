//! `IVGPoint` / `IVGPointRange` —— 点/ 点集

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgVector;

// =============================================================
// IvgPoint
// =============================================================

/// CorelDRAW `IVGPoint` 对象的后期绑定包装（点）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPoint {
    disp: ComObject,
}

impl IvgPoint {
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

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 坐标 ----

    /// ⚠️ 注意属性名是小农 `x` / `y`（COM 层如此）。
    pub fn x(&self) -> Option<f64> { self.prop_f64("x") }
    /// 设置 X 坐标（`x`）。
    pub fn set_x(&self, v: f64) -> bool { self.put_f64("x", v) }

    /// 读取 Y 坐标（`y`）。
    pub fn y(&self) -> Option<f64> { self.prop_f64("y") }
    /// 设置 Y 坐标（`y`）。
    pub fn set_y(&self, v: f64) -> bool { self.put_f64("y", v) }

    /// 读取 XY。
    pub fn get_xy(&self) -> Option<(f64, f64)> {
        Some((self.x()?, self.y()?))
    }

    /// 设置 XY。
    pub fn set_xy(&self, x: f64, y: f64) -> bool {
        self.set_x(x) && self.set_y(y)
    }

    // ---- 运算 ----

    /// 添加（COM 方法 `Add`）。
    pub fn add(&self, v: &IvgVector) -> bool {
        let args = vec![v.as_variant()];
        self.disp.invoke_method("Add", args).is_ok()
    }

    /// 调用 COM 方法 `Subtract`。
    pub fn subtract(&self, v: &IvgVector) -> bool {
        let args = vec![v.as_variant()];
        self.disp.invoke_method("Subtract", args).is_ok()
    }

    /// 到另一点的距离。
    pub fn distance_to(&self, other: &IvgPoint) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("DistanceTo", args).ok()?.to_f64().ok()
    }

    // ---- 复制 / 绑定 ----

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgPoint> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPoint::new)
    }

    /// 绑定文档（COM 方法 `BindToDocument`）。
    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> bool {
        let args = vec![doc.as_variant()];
        self.disp.invoke_method("BindToDocument", args).is_ok()
    }
}

// =============================================================
// IvgPointRange
// =============================================================

/// CorelDRAW `IVGPoint` 对象的后期绑定包装（点范围）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPointRange {
    disp: ComObject,
}

impl IvgPointRange {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---- 基本信息 ----

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgPoint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPoint::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgPoint> {
        self.prop_dispatch("First").map(IvgPoint::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgPoint> {
        self.prop_dispatch("Last").map(IvgPoint::new)
    }

    /// 返回所有点（用索引迭代）。
    pub fn all(&self) -> Vec<IvgPoint> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }

    // ---- 添加 ----

    /// 添加点（COM 方法 `AddPoint`）。
    pub fn add_point(&self, p: &IvgPoint) -> bool {
        let args = vec![p.as_variant()];
        self.disp.invoke_method("AddPoint", args).is_ok()
    }

    /// 添加点 XY（COM 方法 `AddPointXY`）。
    pub fn add_point_xy(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("AddPointXY", args).is_ok()
    }

    /// 插入点（COM 方法 `InsertPoint`）。
    pub fn insert_point(&self, index: i32, p: &IvgPoint) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            p.as_variant(),
        ];
        self.disp.invoke_method("InsertPoint", args).is_ok()
    }

    /// 添加点集（COM 方法 `AddPoints`）。
    pub fn add_points(&self, other: &IvgPointRange) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("AddPoints", args).is_ok()
    }

    /// 插入点集（COM 方法 `InsertPoints`）。
    pub fn insert_points(&self, index: i32, other: &IvgPointRange) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            other.as_variant(),
        ];
        self.disp.invoke_method("InsertPoints", args).is_ok()
    }

    // ---- 移除 ----

    /// 移除（COM 方法 `Remove`）。
    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }

    /// 移除范围（COM 方法 `RemoveRange`）。
    pub fn remove_range(&self, start: i32, end: i32) -> bool {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(end as i64),
        ];
        self.disp.invoke_method("RemoveRange", args).is_ok()
    }

    /// 移除全部（COM 方法 `RemoveAll`）。
    pub fn remove_all(&self) -> bool {
        self.disp.invoke_method("RemoveAll", vec![]).is_ok()
    }

    /// 调用 COM 方法 `RemoveAdjacentDuplicates`。
    pub fn remove_adjacent_duplicates(&self) -> bool {
        self.disp
            .invoke_method("RemoveAdjacentDuplicates", vec![])
            .is_ok()
    }

    // ---- 其它操作 ----

    /// reverse（COM 方法 `Reverse`）。
    pub fn reverse(&self) -> bool {
        self.disp.invoke_method("Reverse", vec![]).is_ok()
    }

    /// 调用 COM 方法 `Smoothen`。
    pub fn smoothen(&self, n_points: f64, closed: bool) -> bool {
        let args = vec![
            Variant::from_f64(n_points),
            Variant::from_bool(closed),
        ];
        self.disp.invoke_method("Smoothen", args).is_ok()
    }

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgPointRange> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPointRange::new)
    }

    /// 绑定文档（COM 方法 `BindToDocument`）。
    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> bool {
        let args = vec![doc.as_variant()];
        self.disp.invoke_method("BindToDocument", args).is_ok()
    }
}
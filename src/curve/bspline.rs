//! `IVGBSpline` / `IVGBSplineControlPoint(s)` —— B 样条

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgBSplineControlPoints
// =============================================================

/// CorelDRAW `IVGBSpline` 对象的后期绑定包装（B 样条控制点集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgBSplineControlPoints {
    disp: ComObject,
}

impl IvgBSplineControlPoints {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgBSplineControlPoint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgBSplineControlPoint::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgBSplineControlPoint> {
        self.prop_dispatch("First").map(IvgBSplineControlPoint::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgBSplineControlPoint> {
        self.prop_dispatch("Last").map(IvgBSplineControlPoint::new)
    }

    /// resize（COM 方法 `Resize`）。
    pub fn resize(&self, n: i32) -> bool {
        let args = vec![Variant::from_i64(n as i64)];
        self.disp.invoke_method("Resize", args).is_ok()
    }
}

// =============================================================
// IvgBSplineControlPoint
// =============================================================

/// CorelDRAW `IVGBSpline` 对象的后期绑定包装（B 样条控制点）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgBSplineControlPoint {
    disp: ComObject,
}

impl IvgBSplineControlPoint {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取 X 坐标（`x`）。
    pub fn x(&self) -> Option<f64> { self.prop_f64("x") }
    /// 设置 X 坐标（`x`）。
    pub fn set_x(&self, v: f64) -> bool { self.put_f64("x", v) }

    /// 读取 Y 坐标（`y`）。
    pub fn y(&self) -> Option<f64> { self.prop_f64("y") }
    /// 设置 Y 坐标（`y`）。
    pub fn set_y(&self, v: f64) -> bool { self.put_f64("y", v) }

    /// 读取 COM 属性 `Clamped`。
    pub fn clamped(&self) -> Option<bool> { self.prop_bool("Clamped") }
    /// 设置 COM 属性 `Clamped`。
    pub fn set_clamped(&self, v: bool) -> bool { self.put_bool("Clamped", v) }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 移动（COM 方法 `Move`）。
    pub fn move_by(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool { self.disp.invoke_method("Delete", vec![]).is_ok() }

    /// 读取位置。
    pub fn get_position(&self) -> Option<(f64, f64)> {
        Some((self.x()?, self.y()?))
    }

    /// 设置位置。
    pub fn set_position(&self, x: f64, y: f64) -> bool {
        self.set_x(x) && self.set_y(y)
    }

    /// 设置属性集（`SetProperties`）。
    pub fn set_properties(&self, x: f64, y: f64, clamped: bool) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(clamped),
        ];
        self.disp.invoke_method("SetProperties", args).is_ok()
    }
}

// =============================================================
// IvgBSpline
// =============================================================

/// CorelDRAW `IVGBSpline` 对象的后期绑定包装。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgBSpline {
    disp: ComObject,
}

impl IvgBSpline {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }
    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取控件点集（`ControlPoints`）。
    pub fn control_points(&self) -> Option<IvgBSplineControlPoints> {
        self.prop_dispatch("ControlPoints").map(IvgBSplineControlPoints::new)
    }

    /// 是否闭合（`Closed`）。
    pub fn is_closed(&self) -> Option<bool> { self.prop_bool("Closed") }
    /// 设置闭合（`Closed`）。
    pub fn set_closed(&self, v: bool) -> bool { self.put_bool("Closed", v) }

    /// 插入控件点（COM 方法 `InsertControlPoint`）。
    pub fn insert_control_point(
        &self,
        index: i32,
        x: f64,
        y: f64,
        clamped: bool,
    ) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(clamped),
        ];
        self.disp.invoke_method("InsertControlPoint", args).is_ok()
    }

    /// 插入控件点集（COM 方法 `InsertControlPoints`）。
    pub fn insert_control_points(
        &self,
        index: i32,
        how_many: i32,
        x: f64,
        y: f64,
        clamped: bool,
    ) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(how_many as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(clamped),
        ];
        self.disp.invoke_method("InsertControlPoints", args).is_ok()
    }

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgBSpline> {
        self.disp.invoke_method("GetCopy", vec![]).ok()?.to_idispatch().ok().map(IvgBSpline::new)
    }

    /// 复制 assign（COM 方法 `CopyAssign`）。
    pub fn copy_assign(&self, other: &IvgBSpline) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }
}
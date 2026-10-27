//! `IVGCrossPoint` / `IVGCrossPoints` —— 交点

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgCrossPoints
// =============================================================

/// CorelDRAW `IVGCrossPoint` 对象的后期绑定包装（交叉点集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgCrossPoints {
    disp: ComObject,
}

impl IvgCrossPoints {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgCrossPoint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgCrossPoint::new)
    }
}

// =============================================================
// IvgCrossPoint
// =============================================================

/// CorelDRAW `IVGCrossPoint` 对象的后期绑定包装（交叉点）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgCrossPoint {
    disp: ComObject,
}

impl IvgCrossPoint {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    /// 读取位置 X 坐标（`PositionX`）。
    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    /// 读取位置 Y 坐标（`PositionY`）。
    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    /// 读取偏移（`Offset`）。
    pub fn offset(&self) -> Option<f64> { self.prop_f64("Offset") }
    /// 读取 COM 属性 `Offset2`。
    pub fn offset2(&self) -> Option<f64> { self.prop_f64("Offset2") }
}
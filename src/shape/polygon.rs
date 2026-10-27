//! `IVGPolygon` —— 多边形

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGPolygon` 对象的后期绑定包装（多边形）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPolygon {
    disp: ComObject,
}

impl IvgPolygon {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// `cdrPolygonType`
    pub fn polygon_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置多边形类型（`Type`）。
    pub fn set_polygon_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取 COM 属性 `Sides`。
    pub fn sides(&self) -> Option<i64> { self.prop_i64("Sides") }
    /// 设置 COM 属性 `Sides`。
    pub fn set_sides(&self, v: i32) -> bool { self.put_i64("Sides", v as i64) }

    /// 读取 COM 属性 `Sharpness`。
    pub fn sharpness(&self) -> Option<i64> { self.prop_i64("Sharpness") }
    /// 设置 COM 属性 `Sharpness`。
    pub fn set_sharpness(&self, v: i32) -> bool { self.put_i64("Sharpness", v as i64) }
}
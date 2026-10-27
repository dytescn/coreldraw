//! `IVGRectangle` —— 矩形

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGRectangle` 对象的后期绑定包装（矩形）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgRectangle {
    disp: ComObject,
}

impl IvgRectangle {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 圆角百分比 ----

    /// 读取左上圆角（`CornerUpperLeft`）。
    pub fn corner_upper_left(&self) -> Option<i64> { self.prop_i64("CornerUpperLeft") }
    /// 设置左上圆角（`CornerUpperLeft`）。
    pub fn set_corner_upper_left(&self, v: i32) -> bool { self.put_i64("CornerUpperLeft", v as i64) }

    /// 读取右上圆角（`CornerUpperRight`）。
    pub fn corner_upper_right(&self) -> Option<i64> { self.prop_i64("CornerUpperRight") }
    /// 设置右上圆角（`CornerUpperRight`）。
    pub fn set_corner_upper_right(&self, v: i32) -> bool { self.put_i64("CornerUpperRight", v as i64) }

    /// 读取左下圆角（`CornerLowerLeft`）。
    pub fn corner_lower_left(&self) -> Option<i64> { self.prop_i64("CornerLowerLeft") }
    /// 设置左下圆角（`CornerLowerLeft`）。
    pub fn set_corner_lower_left(&self, v: i32) -> bool { self.put_i64("CornerLowerLeft", v as i64) }

    /// 读取右下圆角（`CornerLowerRight`）。
    pub fn corner_lower_right(&self) -> Option<i64> { self.prop_i64("CornerLowerRight") }
    /// 设置右下圆角（`CornerLowerRight`）。
    pub fn set_corner_lower_right(&self, v: i32) -> bool { self.put_i64("CornerLowerRight", v as i64) }

    /// 读取相等圆角（`EqualCorners`）。
    pub fn equal_corners(&self) -> Option<bool> { self.prop_bool("EqualCorners") }
    /// 读取最大半径（`MaxRadius`）。
    pub fn max_radius(&self) -> Option<f64> { self.prop_f64("MaxRadius") }

    // ---- 圆角半径 ----

    /// 读取左上半径（`RadiusUpperLeft`）。
    pub fn radius_upper_left(&self) -> Option<f64> { self.prop_f64("RadiusUpperLeft") }
    /// 设置左上半径（`RadiusUpperLeft`）。
    pub fn set_radius_upper_left(&self, v: f64) -> bool { self.put_f64("RadiusUpperLeft", v) }

    /// 读取右上半径（`RadiusUpperRight`）。
    pub fn radius_upper_right(&self) -> Option<f64> { self.prop_f64("RadiusUpperRight") }
    /// 设置右上半径（`RadiusUpperRight`）。
    pub fn set_radius_upper_right(&self, v: f64) -> bool { self.put_f64("RadiusUpperRight", v) }

    /// 读取左下半径（`RadiusLowerLeft`）。
    pub fn radius_lower_left(&self) -> Option<f64> { self.prop_f64("RadiusLowerLeft") }
    /// 设置左下半径（`RadiusLowerLeft`）。
    pub fn set_radius_lower_left(&self, v: f64) -> bool { self.put_f64("RadiusLowerLeft", v) }

    /// 读取右下半径（`RadiusLowerRight`）。
    pub fn radius_lower_right(&self) -> Option<f64> { self.prop_f64("RadiusLowerRight") }
    /// 设置右下半径（`RadiusLowerRight`）。
    pub fn set_radius_lower_right(&self, v: f64) -> bool { self.put_f64("RadiusLowerRight", v) }

    // ---- 操作 ----

    /// 设置 COM 属性 `SetRoundness`。
    pub fn set_roundness(&self, roundness: i32) -> bool {
        let args = vec![Variant::from_i64(roundness as i64)];
        self.disp.invoke_method("SetRoundness", args).is_ok()
    }

    /// 设置半径（`SetRadius`）。
    pub fn set_radius(&self, radius: f64) -> bool {
        let args = vec![Variant::from_f64(radius)];
        self.disp.invoke_method("SetRadius", args).is_ok()
    }

    /// `cdrCornerType`
    pub fn corner_type(&self) -> Option<i64> { self.prop_i64("CornerType") }
    /// 设置圆角类型（`CornerType`）。
    pub fn set_corner_type(&self, v: i32) -> bool { self.put_i64("CornerType", v as i64) }

    /// 读取相对圆角缩放（`RelativeCornerScaling`）。
    pub fn relative_corner_scaling(&self) -> Option<bool> {
        self.prop_bool("RelativeCornerScaling")
    }
    /// 设置相对圆角缩放（`RelativeCornerScaling`）。
    pub fn set_relative_corner_scaling(&self, v: bool) -> bool {
        self.put_bool("RelativeCornerScaling", v)
    }
}
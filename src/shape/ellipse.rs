//! `IVGEllipse` —— 椭圆

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGEllipse` 对象的后期绑定包装（椭圆）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEllipse {
    disp: ComObject,
}

impl IvgEllipse {
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

    /// `cdrEllipseType`
    pub fn ellipse_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置椭圆类型（`Type`）。
    pub fn set_ellipse_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取启动角度（`StartAngle`）。
    pub fn start_angle(&self) -> Option<f64> { self.prop_f64("StartAngle") }
    /// 设置启动角度（`StartAngle`）。
    pub fn set_start_angle(&self, v: f64) -> bool { self.put_f64("StartAngle", v) }

    /// 读取结束角度（`EndAngle`）。
    pub fn end_angle(&self) -> Option<f64> { self.prop_f64("EndAngle") }
    /// 设置结束角度（`EndAngle`）。
    pub fn set_end_angle(&self, v: f64) -> bool { self.put_f64("EndAngle", v) }

    /// 读取顺时针（`Clockwise`）。
    pub fn clockwise(&self) -> Option<bool> { self.prop_bool("Clockwise") }
    /// 设置顺时针（`Clockwise`）。
    pub fn set_clockwise(&self, v: bool) -> bool { self.put_bool("Clockwise", v) }

    /// 读取中心 X 坐标（`CenterX`）。
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    /// 设置中心 X 坐标（`CenterX`）。
    pub fn set_center_x(&self, v: f64) -> bool { self.put_f64("CenterX", v) }

    /// 读取中心 Y 坐标（`CenterY`）。
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }
    /// 设置中心 Y 坐标（`CenterY`）。
    pub fn set_center_y(&self, v: f64) -> bool { self.put_f64("CenterY", v) }

    /// 读取高半径（`HRadius`）。
    pub fn h_radius(&self) -> Option<f64> { self.prop_f64("HRadius") }
    /// 设置高半径（`HRadius`）。
    pub fn set_h_radius(&self, v: f64) -> bool { self.put_f64("HRadius", v) }

    /// 读取半径（`VRadius`）。
    pub fn v_radius(&self) -> Option<f64> { self.prop_f64("VRadius") }
    /// 设置半径（`VRadius`）。
    pub fn set_v_radius(&self, v: f64) -> bool { self.put_f64("VRadius", v) }

    /// 设置中心位置（`SetCenterPosition`）。
    pub fn set_center_position(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetCenterPosition", args).is_ok()
    }

    /// 读取中心位置。
    pub fn get_center_position(&self) -> Option<(f64, f64)> {
        Some((self.center_x()?, self.center_y()?))
    }

    /// 设置半径（`SetRadius`）。
    pub fn set_radius(&self, h: f64, v: f64) -> bool {
        let args = vec![
            Variant::from_f64(h),
            Variant::from_f64(v),
        ];
        self.disp.invoke_method("SetRadius", args).is_ok()
    }

    /// 读取半径。
    pub fn get_radius(&self) -> Option<(f64, f64)> {
        Some((self.h_radius()?, self.v_radius()?))
    }
}
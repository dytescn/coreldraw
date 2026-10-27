//! `IVGGuide` —— 参考线

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGGuide` 对象的后期绑定包装（参考线）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgGuide {
    disp: ComObject,
}

impl IvgGuide {
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

    // ---- 类型 / 预设 ----

    /// `cdrGuideType`
    pub fn guide_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 读取 COM 属性 `Preset`。
    pub fn preset(&self) -> Option<bool> { self.prop_bool("Preset") }
    /// 生成可编辑（COM 方法 `MakeEditable`）。
    pub fn make_editable(&self) -> bool {
        self.disp.invoke_method("MakeEditable", vec![]).is_ok()
    }

    // ---- 点 / 角度 ----

    /// 读取 COM 属性 `Point1X`。
    pub fn point1_x(&self) -> Option<f64> { self.prop_f64("Point1X") }
    /// 读取 COM 属性 `Point1Y`。
    pub fn point1_y(&self) -> Option<f64> { self.prop_f64("Point1Y") }
    /// 读取 COM 属性 `Point2X`。
    pub fn point2_x(&self) -> Option<f64> { self.prop_f64("Point2X") }
    /// 读取 COM 属性 `Point2Y`。
    pub fn point2_y(&self) -> Option<f64> { self.prop_f64("Point2Y") }
    /// 读取角度（`Angle`）。
    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }

    /// 读取点集。
    pub fn get_points(&self) -> Option<(f64, f64, f64, f64)> {
        Some((self.point1_x()?, self.point1_y()?, self.point2_x()?, self.point2_y()?))
    }

    /// 读取点角度。
    pub fn get_point_and_angle(&self) -> Option<(f64, f64, f64)> {
        Some((self.point1_x()?, self.point1_y()?, self.angle()?))
    }

    /// 设置点集（`SetPoints`）。
    pub fn set_points(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
        ];
        self.disp.invoke_method("SetPoints", args).is_ok()
    }

    /// 设置点角度（`SetPointAndAngle`）。
    pub fn set_point_and_angle(&self, x: f64, y: f64, angle: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(angle),
        ];
        self.disp.invoke_method("SetPointAndAngle", args).is_ok()
    }

    // ---- 截距 / 中心 ----

    /// 读取 COM 属性 `InterceptX`。
    pub fn intercept_x(&self) -> Option<f64> { self.prop_f64("InterceptX") }
    /// 读取 COM 属性 `InterceptY`。
    pub fn intercept_y(&self) -> Option<f64> { self.prop_f64("InterceptY") }
    /// 读取中心 X 坐标（`CenterX`）。
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    /// 读取中心 Y 坐标（`CenterY`）。
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }
}
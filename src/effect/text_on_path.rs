//! `IVGEffectTextOnPath` —— 文本适配路径

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::IvgShape;

/// CorelDRAW `IVGEffectTextOnPath` 对象的后期绑定包装（路径上的效果文本）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectTextOnPath {
    disp: ComObject,
}

impl IvgEffectTextOnPath {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
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

    fn put_shape(&self, name: &str, s: &IvgShape) -> bool {
        let arg = s.as_variant();
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 图形 ----

    /// 读取文本（`Text`）。
    pub fn text(&self) -> Option<IvgShape> {
        self.prop_dispatch("Text").map(IvgShape::new)
    }
    /// 设置文本（`Text`）。
    pub fn set_text(&self, s: &IvgShape) -> bool {
        self.put_shape("Text", s)
    }

    /// 读取路径（`Path`）。
    pub fn path(&self) -> Option<IvgShape> {
        self.prop_dispatch("Path").map(IvgShape::new)
    }
    /// 设置路径（`Path`）。
    pub fn set_path(&self, s: &IvgShape) -> bool {
        self.put_shape("Path", s)
    }

    // ---- 位置 ----

    /// 读取距离路径（`DistanceFromPath`）。
    pub fn distance_from_path(&self) -> Option<f64> {
        self.prop_f64("DistanceFromPath")
    }
    /// 设置距离路径（`DistanceFromPath`）。
    pub fn set_distance_from_path(&self, v: f64) -> bool {
        self.put_f64("DistanceFromPath", v)
    }

    /// 读取偏移（`Offset`）。
    pub fn offset(&self) -> Option<f64> { self.prop_f64("Offset") }
    /// 设置偏移（`Offset`）。
    pub fn set_offset(&self, v: f64) -> bool { self.put_f64("Offset", v) }

    // ---- 朝向 / 放置 ----

    /// `cdrFittedOrientation`
    pub fn orientation(&self) -> Option<i64> { self.prop_i64("Orientation") }
    /// 设置方向（`Orientation`）。
    pub fn set_orientation(&self, v: i32) -> bool { self.put_i64("Orientation", v as i64) }

    /// `cdrFittedPlacement`
    pub fn placement(&self) -> Option<i64> { self.prop_i64("Placement") }
    /// 设置放置（`Placement`）。
    pub fn set_placement(&self, v: i32) -> bool { self.put_i64("Placement", v as i64) }

    /// 读取 COM 属性 `PlaceOnOtherSide`。
    pub fn place_on_other_side(&self) -> Option<bool> {
        self.prop_bool("PlaceOnOtherSide")
    }
    /// 设置 COM 属性 `PlaceOnOtherSide`。
    pub fn set_place_on_other_side(&self, v: bool) -> bool {
        self.put_bool("PlaceOnOtherSide", v)
    }

    /// `cdrFittedQuadrant`
    pub fn quadrant(&self) -> Option<i64> { self.prop_i64("Quadrant") }
    /// 设置 COM 属性 `Quadrant`。
    pub fn set_quadrant(&self, v: i32) -> bool { self.put_i64("Quadrant", v as i64) }

    /// `cdrFittedVertPlacement`
    pub fn vert_placement(&self) -> Option<i64> { self.prop_i64("VertPlacement") }
    /// 设置垂直放置（`VertPlacement`）。
    pub fn set_vert_placement(&self, v: i32) -> bool {
        self.put_i64("VertPlacement", v as i64)
    }
}
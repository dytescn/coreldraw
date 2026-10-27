//! `IVGEffectPerspective` —— 透视效果
use wincom::{ComObject, Variant};


/// CorelDRAW `IVGEffectPerspective` 对象的后期绑定包装（效果透视）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectPerspective {
    disp: ComObject,
}

impl IvgEffectPerspective {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: windows::Win32::System::Com::IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 水平消失点----

    /// 读取使用水平消失点（`UseHorizVanishingPoint`）。
    pub fn use_horiz_vanishing_point(&self) -> Option<bool> {
        self.prop_bool("UseHorizVanishingPoint")
    }
    /// 设置使用水平消失点（`UseHorizVanishingPoint`）。
    pub fn set_use_horiz_vanishing_point(&self, v: bool) -> bool {
        self.put_bool("UseHorizVanishingPoint", v)
    }

    /// 读取水平消失点 X 坐标（`HorizVanishingPointX`）。
    pub fn horiz_vanishing_point_x(&self) -> Option<f64> {
        self.prop_f64("HorizVanishingPointX")
    }
    /// 设置水平消失点 X 坐标（`HorizVanishingPointX`）。
    pub fn set_horiz_vanishing_point_x(&self, v: f64) -> bool {
        self.put_f64("HorizVanishingPointX", v)
    }

    /// 读取水平消失点 Y 坐标（`HorizVanishingPointY`）。
    pub fn horiz_vanishing_point_y(&self) -> Option<f64> {
        self.prop_f64("HorizVanishingPointY")
    }
    /// 设置水平消失点 Y 坐标（`HorizVanishingPointY`）。
    pub fn set_horiz_vanishing_point_y(&self, v: f64) -> bool {
        self.put_f64("HorizVanishingPointY", v)
    }

    // ---- 垂直消失点----

    /// 读取使用垂直消失点（`UseVertVanishingPoint`）。
    pub fn use_vert_vanishing_point(&self) -> Option<bool> {
        self.prop_bool("UseVertVanishingPoint")
    }
    /// 设置使用垂直消失点（`UseVertVanishingPoint`）。
    pub fn set_use_vert_vanishing_point(&self, v: bool) -> bool {
        self.put_bool("UseVertVanishingPoint", v)
    }

    /// 读取垂直消失点 X 坐标（`VertVanishingPointX`）。
    pub fn vert_vanishing_point_x(&self) -> Option<f64> {
        self.prop_f64("VertVanishingPointX")
    }
    /// 设置垂直消失点 X 坐标（`VertVanishingPointX`）。
    pub fn set_vert_vanishing_point_x(&self, v: f64) -> bool {
        self.put_f64("VertVanishingPointX", v)
    }

    /// 读取垂直消失点 Y 坐标（`VertVanishingPointY`）。
    pub fn vert_vanishing_point_y(&self) -> Option<f64> {
        self.prop_f64("VertVanishingPointY")
    }
    /// 设置垂直消失点 Y 坐标（`VertVanishingPointY`）。
    pub fn set_vert_vanishing_point_y(&self, v: f64) -> bool {
        self.put_f64("VertVanishingPointY", v)
    }
}
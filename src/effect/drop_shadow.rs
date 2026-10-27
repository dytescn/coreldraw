//! `IVGEffectDropShadow` —— 阴影效果

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::shape::IvgShape;

/// CorelDRAW `IVGEffectDropShadow` 对象的后期绑定包装（效果投影阴影）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectDropShadow {
    disp: ComObject,
}

impl IvgEffectDropShadow {
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

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
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

    fn put_color(&self, name: &str, c: &IvgColor) -> bool {
        let arg = c.as_variant();
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 位置 ----

    /// 读取偏移 X 坐标（`OffsetX`）。
    pub fn offset_x(&self) -> Option<f64> { self.prop_f64("OffsetX") }
    /// 设置偏移 X 坐标（`OffsetX`）。
    pub fn set_offset_x(&self, v: f64) -> bool { self.put_f64("OffsetX", v) }

    /// 读取偏移 Y 坐标（`OffsetY`）。
    pub fn offset_y(&self) -> Option<f64> { self.prop_f64("OffsetY") }
    /// 设置偏移 Y 坐标（`OffsetY`）。
    pub fn set_offset_y(&self, v: f64) -> bool { self.put_f64("OffsetY", v) }

    /// 设置偏移（`SetOffset`）。
    pub fn set_offset(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetOffset", args).is_ok()
    }

    // ---- 透明 / 羽化 ----

    /// 读取不透明度（`Opacity`）。
    pub fn opacity(&self) -> Option<i64> { self.prop_i64("Opacity") }
    /// 设置不透明度（`Opacity`）。
    pub fn set_opacity(&self, v: i32) -> bool { self.put_i64("Opacity", v as i64) }

    /// 读取羽化（`Feather`）。
    pub fn feather(&self) -> Option<i64> { self.prop_i64("Feather") }
    /// 设置羽化（`Feather`）。
    pub fn set_feather(&self, v: i32) -> bool { self.put_i64("Feather", v as i64) }

    /// `cdrFeatherType`
    pub fn feather_type(&self) -> Option<i64> { self.prop_i64("FeatherType") }
    /// 设置羽化类型（`FeatherType`）。
    pub fn set_feather_type(&self, v: i32) -> bool { self.put_i64("FeatherType", v as i64) }

    /// `cdrEdgeType`
    pub fn feather_edge(&self) -> Option<i64> { self.prop_i64("FeatherEdge") }
    /// 设置羽化边缘（`FeatherEdge`）。
    pub fn set_feather_edge(&self, v: i32) -> bool { self.put_i64("FeatherEdge", v as i64) }

    // ---- 类型 ----

    /// `cdrDropShadowType`
    pub fn shadow_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置阴影类型（`Type`）。
    pub fn set_shadow_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取透视角度（`PerspectiveAngle`）。
    pub fn perspective_angle(&self) -> Option<f64> {
        self.prop_f64("PerspectiveAngle")
    }
    /// 设置透视角度（`PerspectiveAngle`）。
    pub fn set_perspective_angle(&self, v: f64) -> bool {
        self.put_f64("PerspectiveAngle", v)
    }

    /// 读取透视拉伸（`PerspectiveStretch`）。
    pub fn perspective_stretch(&self) -> Option<f64> {
        self.prop_f64("PerspectiveStretch")
    }
    /// 设置透视拉伸（`PerspectiveStretch`）。
    pub fn set_perspective_stretch(&self, v: f64) -> bool {
        self.put_f64("PerspectiveStretch", v)
    }

    /// 读取 COM 属性 `Fade`。
    pub fn fade(&self) -> Option<i64> { self.prop_i64("Fade") }
    /// 设置 COM 属性 `Fade`。
    pub fn set_fade(&self, v: i32) -> bool { self.put_i64("Fade", v as i64) }

    // ---- 颜色 / 混合 ----

    /// 读取颜色（`Color`）。
    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    /// 设置颜色（`Color`）。
    pub fn set_color(&self, c: &IvgColor) -> bool {
        self.put_color("Color", c)
    }

    /// `cdrMergeMode`
    pub fn merge_mode(&self) -> Option<i64> { self.prop_i64("MergeMode") }
    /// 设置合并模式（`MergeMode`）。
    pub fn set_merge_mode(&self, v: i32) -> bool { self.put_i64("MergeMode", v as i64) }

    /// 阴影组（组合图形）。
    pub fn shadow_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ShadowGroup").map(IvgShape::new)
    }
}
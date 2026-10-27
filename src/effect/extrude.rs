//! `IVGEffectExtrude` —— 立体化效果
//! 以及 `IVGExtrudeVanishingPoint`（消失点）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::effect::IvgEffects;
use crate::shape::IvgShape;

// =============================================================
// IvgExtrudeVanishingPoint
// =============================================================

/// CorelDRAW `IVGEffectExtrude` 对象的后期绑定包装（立体化消失点）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgExtrudeVanishingPoint {
    disp: ComObject,
}

impl IvgExtrudeVanishingPoint {
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

    /// `cdrExtrudeVPType`
    pub fn vp_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置 COM 属性 `Type`。
    pub fn set_vp_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取位置 X 坐标（`PositionX`）。
    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    /// 设置位置 X 坐标（`PositionX`）。
    pub fn set_position_x(&self, v: f64) -> bool { self.put_f64("PositionX", v) }

    /// 读取位置 Y 坐标（`PositionY`）。
    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    /// 设置位置 Y 坐标（`PositionY`）。
    pub fn set_position_y(&self, v: f64) -> bool { self.put_f64("PositionY", v) }

    /// 读取效果集（`Effects`）。
    pub fn effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("Effects").map(IvgEffects::new)
    }

    /// 与另一个立体化效果共享消失点。
    pub fn share(&self, src: &IvgEffectExtrude) -> Option<bool> {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("Share", args).ok()?.to_bool().ok()
    }
}

// =============================================================
// IvgEffectExtrude
// =============================================================

/// CorelDRAW `IVGEffectExtrude` 对象的后期绑定包装（效果立体化）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectExtrude {
    disp: ComObject,
}

impl IvgEffectExtrude {
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

    fn put_color(&self, name: &str, c: &IvgColor) -> bool {
        let arg = c.as_variant();
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 类型 ----

    /// `cdrExtrudeType`
    pub fn extrude_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置立体化类型（`Type`）。
    pub fn set_extrude_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取消失点（`VanishingPoint`）。
    pub fn vanishing_point(&self) -> Option<IvgExtrudeVanishingPoint> {
        self.prop_dispatch("VanishingPoint").map(IvgExtrudeVanishingPoint::new)
    }

    // ---- 深度 / 角度 ----

    /// 读取深度（`Depth`）。
    pub fn depth(&self) -> Option<i64> { self.prop_i64("Depth") }
    /// 设置深度（`Depth`）。
    pub fn set_depth(&self, v: i32) -> bool { self.put_i64("Depth", v as i64) }

    /// 读取角度 X 坐标（`AngleX`）。
    pub fn angle_x(&self) -> Option<f64> { self.prop_f64("AngleX") }
    /// 设置角度 X 坐标（`AngleX`）。
    pub fn set_angle_x(&self, v: f64) -> bool { self.put_f64("AngleX", v) }

    /// 读取角度 Y 坐标（`AngleY`）。
    pub fn angle_y(&self) -> Option<f64> { self.prop_f64("AngleY") }
    /// 设置角度 Y 坐标（`AngleY`）。
    pub fn set_angle_y(&self, v: f64) -> bool { self.put_f64("AngleY", v) }

    /// 读取角度 Z 坐标（`AngleZ`）。
    pub fn angle_z(&self) -> Option<f64> { self.prop_f64("AngleZ") }
    /// 设置角度 Z 坐标（`AngleZ`）。
    pub fn set_angle_z(&self, v: f64) -> bool { self.put_f64("AngleZ", v) }

    // ---- 着色----

    /// `cdrExtrudeShading`
    pub fn shading(&self) -> Option<i64> { self.prop_i64("Shading") }
    /// 设置明暗（`Shading`）。
    pub fn set_shading(&self, v: i32) -> bool { self.put_i64("Shading", v as i64) }

    /// 读取基础颜色（`BaseColor`）。
    pub fn base_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("BaseColor").map(IvgColor::new)
    }
    /// 设置基础颜色（`BaseColor`）。
    pub fn set_base_color(&self, c: &IvgColor) -> bool {
        self.put_color("BaseColor", c)
    }

    /// 读取明暗颜色（`ShadingColor`）。
    pub fn shading_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("ShadingColor").map(IvgColor::new)
    }
    /// 设置明暗颜色（`ShadingColor`）。
    pub fn set_shading_color(&self, c: &IvgColor) -> bool {
        self.put_color("ShadingColor", c)
    }

    /// 读取使用完整颜色范围（`UseFullColorRange`）。
    pub fn use_full_color_range(&self) -> Option<bool> {
        self.prop_bool("UseFullColorRange")
    }
    /// 设置使用完整颜色范围（`UseFullColorRange`）。
    pub fn set_use_full_color_range(&self, v: bool) -> bool {
        self.put_bool("UseFullColorRange", v)
    }

    // ---- 斜面 ----

    /// 读取使用斜角（`UseBevel`）。
    pub fn use_bevel(&self) -> Option<bool> { self.prop_bool("UseBevel") }
    /// 设置使用斜角（`UseBevel`）。
    pub fn set_use_bevel(&self, v: bool) -> bool { self.put_bool("UseBevel", v) }

    /// 读取显示斜角仅（`ShowBevelOnly`）。
    pub fn show_bevel_only(&self) -> Option<bool> { self.prop_bool("ShowBevelOnly") }
    /// 设置显示斜角仅（`ShowBevelOnly`）。
    pub fn set_show_bevel_only(&self, v: bool) -> bool { self.put_bool("ShowBevelOnly", v) }

    /// 读取斜角深度（`BevelDepth`）。
    pub fn bevel_depth(&self) -> Option<f64> { self.prop_f64("BevelDepth") }
    /// 设置斜角深度（`BevelDepth`）。
    pub fn set_bevel_depth(&self, v: f64) -> bool { self.put_f64("BevelDepth", v) }

    /// 读取斜角角度（`BevelAngle`）。
    pub fn bevel_angle(&self) -> Option<f64> { self.prop_f64("BevelAngle") }
    /// 设置斜角角度（`BevelAngle`）。
    pub fn set_bevel_angle(&self, v: f64) -> bool { self.put_f64("BevelAngle", v) }

    /// 读取使用立体化颜色斜角（`UseExtrudeColorForBevel`）。
    pub fn use_extrude_color_for_bevel(&self) -> Option<bool> {
        self.prop_bool("UseExtrudeColorForBevel")
    }
    /// 设置使用立体化颜色斜角（`UseExtrudeColorForBevel`）。
    pub fn set_use_extrude_color_for_bevel(&self, v: bool) -> bool {
        self.put_bool("UseExtrudeColorForBevel", v)
    }

    /// 读取斜角颜色（`BevelColor`）。
    pub fn bevel_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("BevelColor").map(IvgColor::new)
    }
    /// 设置斜角颜色（`BevelColor`）。
    pub fn set_bevel_color(&self, c: &IvgColor) -> bool {
        self.put_color("BevelColor", c)
    }

    /// 设置斜角（`SetBevel`）。
    pub fn set_bevel(&self, depth: f64, angle: f64, show_only: bool) -> bool {
        let args = vec![
            Variant::from_f64(depth),
            Variant::from_f64(angle),
            Variant::from_bool(show_only),
        ];
        self.disp.invoke_method("SetBevel", args).is_ok()
    }

    // ---- 光源（3 个） ----

    /// 索引 1..3
    pub fn light_present(&self, index: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("LightPresent", args).ok()?.to_bool().ok()
    }

    /// 设置光源存在（`put_LightPresent`）。
    pub fn set_light_present(&self, index: i32, v: bool) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_bool(v),
        ];
        self.disp.invoke_method("put_LightPresent", args).is_ok()
    }

    /// `cdrExtrudeLightPosition`
    pub fn light_position(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("LightPosition", args).ok()?.to_i64().ok()
    }

    /// 设置光源位置（`put_LightPosition`）。
    pub fn set_light_position(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_LightPosition", args).is_ok()
    }

    /// 光源强度（COM 方法 `LightIntensity`）。
    pub fn light_intensity(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("LightIntensity", args).ok()?.to_i64().ok()
    }

    /// 设置光源强度（`put_LightIntensity`）。
    pub fn set_light_intensity(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_LightIntensity", args).is_ok()
    }

    /// 设置光源（`SetLight`）。
    pub fn set_light(
        &self,
        index: i32,
        position: i32,
        intensity: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(position as i64),
            Variant::from_i64(intensity as i64),
        ];
        self.disp.invoke_method("SetLight", args).is_ok()
    }

    // ---- 面/ 经----

    /// 读取字面可见性（`FaceVisible`）。
    pub fn face_visible(&self) -> Option<bool> { self.prop_bool("FaceVisible") }

    /// 读取字面图形（`FaceShape`）。
    pub fn face_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("FaceShape").map(IvgShape::new)
    }

    /// 读取斜角组（`BevelGroup`）。
    pub fn bevel_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("BevelGroup").map(IvgShape::new)
    }

    /// 读取立体化组（`ExtrudeGroup`）。
    pub fn extrude_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ExtrudeGroup").map(IvgShape::new)
    }

    // ---- 旋转 / 复制 ----

    /// rotate（COM 方法 `Rotate`）。
    pub fn rotate(&self, ax: f64, ay: f64, az: f64) -> bool {
        let args = vec![
            Variant::from_f64(ax),
            Variant::from_f64(ay),
            Variant::from_f64(az),
        ];
        self.disp.invoke_method("Rotate", args).is_ok()
    }

    /// 复制（COM 方法 `CopyFrom`）。
    pub fn copy_from(&self, src: &IvgEffectExtrude) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyFrom", args).is_ok()
    }
}
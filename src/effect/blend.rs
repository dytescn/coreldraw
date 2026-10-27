//! `IVGEffectBlend` —— 调和效果

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgSnapPoint;
use crate::shape::IvgShape;

/// CorelDRAW `IVGEffectBlend` 对象的后期绑定包装（效果调和）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectBlend {
    disp: ComObject,
}

impl IvgEffectBlend {
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

    fn put_dispatch(&self, name: &str, d: &IvgShape) -> bool {
        let arg = d.as_variant();
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 端点图形 ----

    /// 读取启动图形（`StartShape`）。
    pub fn start_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("StartShape").map(IvgShape::new)
    }
    /// 设置启动图形（`StartShape`）。
    pub fn set_start_shape(&self, s: &IvgShape) -> bool {
        self.put_dispatch("StartShape", s)
    }

    /// 读取结束图形（`EndShape`）。
    pub fn end_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("EndShape").map(IvgShape::new)
    }
    /// 设置结束图形（`EndShape`）。
    pub fn set_end_shape(&self, s: &IvgShape) -> bool {
        self.put_dispatch("EndShape", s)
    }

    /// 调和生成的组合图形。
    pub fn blend_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("BlendGroup").map(IvgShape::new)
    }

    // ---- 路径 ----

    /// 读取路径（`Path`）。
    pub fn path(&self) -> Option<IvgShape> {
        self.prop_dispatch("Path").map(IvgShape::new)
    }
    /// 设置路径（`Path`）。
    pub fn set_path(&self, s: &IvgShape) -> bool {
        self.put_dispatch("Path", s)
    }

    /// 读取启动图形偏移（`StartShapeOffset`）。
    pub fn start_shape_offset(&self) -> Option<f64> {
        self.prop_f64("StartShapeOffset")
    }
    /// 设置启动图形偏移（`StartShapeOffset`）。
    pub fn set_start_shape_offset(&self, v: f64) -> bool {
        self.put_f64("StartShapeOffset", v)
    }

    /// 读取结束图形偏移（`EndShapeOffset`）。
    pub fn end_shape_offset(&self) -> Option<f64> {
        self.prop_f64("EndShapeOffset")
    }
    /// 设置结束图形偏移（`EndShapeOffset`）。
    pub fn set_end_shape_offset(&self, v: f64) -> bool {
        self.put_f64("EndShapeOffset", v)
    }

    // ---- 步进 ----

    /// `cdrBlendMode` ——见 `enums/effect`
    pub fn mode(&self) -> Option<i64> { self.prop_i64("Mode") }
    /// 设置模式（`Mode`）。
    pub fn set_mode(&self, v: i32) -> bool { self.put_i64("Mode", v as i64) }

    /// 读取步数（`Steps`）。
    pub fn steps(&self) -> Option<i64> { self.prop_i64("Steps") }
    /// 设置步数（`Steps`）。
    pub fn set_steps(&self, v: i32) -> bool { self.put_i64("Steps", v as i64) }

    /// 读取间距（`Spacing`）。
    pub fn spacing(&self) -> Option<f64> { self.prop_f64("Spacing") }
    /// 设置间距（`Spacing`）。
    pub fn set_spacing(&self, v: f64) -> bool { self.put_f64("Spacing", v) }

    /// 读取角度（`Angle`）。
    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    /// 设置角度（`Angle`）。
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }

    // ---- 路径循环 ----

    /// 读取 COM 属性 `Loop`。
    pub fn looping(&self) -> Option<bool> { self.prop_bool("Loop") }
    /// 设置 COM 属性 `Loop`。
    pub fn set_looping(&self, v: bool) -> bool { self.put_bool("Loop", v) }

    /// 读取完整路径（`FullPath`）。
    pub fn full_path(&self) -> Option<bool> { self.prop_bool("FullPath") }
    /// 设置完整路径（`FullPath`）。
    pub fn set_full_path(&self, v: bool) -> bool { self.put_bool("FullPath", v) }

    /// 读取旋转图形集（`RotateShapes`）。
    pub fn rotate_shapes(&self) -> Option<bool> { self.prop_bool("RotateShapes") }
    /// 设置旋转图形集（`RotateShapes`）。
    pub fn set_rotate_shapes(&self, v: bool) -> bool { self.put_bool("RotateShapes", v) }

    // ---- 颜色混合 ----

    /// `cdrFountainFillBlendType`
    pub fn color_blend_type(&self) -> Option<i64> {
        self.prop_i64("ColorBlendType")
    }
    /// 设置颜色调和类型（`ColorBlendType`）。
    pub fn set_color_blend_type(&self, v: i32) -> bool {
        self.put_i64("ColorBlendType", v as i64)
    }

    /// 读取间距加速度（`SpacingAcceleration`）。
    pub fn spacing_acceleration(&self) -> Option<i64> {
        self.prop_i64("SpacingAcceleration")
    }
    /// 设置间距加速度（`SpacingAcceleration`）。
    pub fn set_spacing_acceleration(&self, v: i32) -> bool {
        self.put_i64("SpacingAcceleration", v as i64)
    }

    /// 读取颜色加速度（`ColorAcceleration`）。
    pub fn color_acceleration(&self) -> Option<i64> {
        self.prop_i64("ColorAcceleration")
    }
    /// 设置颜色加速度（`ColorAcceleration`）。
    pub fn set_color_acceleration(&self, v: i32) -> bool {
        self.put_i64("ColorAcceleration", v as i64)
    }

    /// 读取链接加速度（`LinkAcceleration`）。
    pub fn link_acceleration(&self) -> Option<bool> {
        self.prop_bool("LinkAcceleration")
    }
    /// 设置链接加速度（`LinkAcceleration`）。
    pub fn set_link_acceleration(&self, v: bool) -> bool {
        self.put_bool("LinkAcceleration", v)
    }

    /// 读取 COM 属性 `AccelerateSize`。
    pub fn accelerate_size(&self) -> Option<bool> {
        self.prop_bool("AccelerateSize")
    }
    /// 设置 COM 属性 `AccelerateSize`。
    pub fn set_accelerate_size(&self, v: bool) -> bool {
        self.put_bool("AccelerateSize", v)
    }

    /// 读取映射节点集（`MapNodes`）。
    pub fn map_nodes(&self) -> Option<bool> { self.prop_bool("MapNodes") }
    /// 设置映射节点集（`MapNodes`）。
    pub fn set_map_nodes(&self, v: bool) -> bool { self.put_bool("MapNodes", v) }

    // ---- 抓取点----

    /// 读取启动点（`StartPoint`）。
    pub fn start_point(&self) -> Option<IvgSnapPoint> {
        self.prop_dispatch("StartPoint").map(IvgSnapPoint::new)
    }

    /// 读取结束点（`EndPoint`）。
    pub fn end_point(&self) -> Option<IvgSnapPoint> {
        self.prop_dispatch("EndPoint").map(IvgSnapPoint::new)
    }

    // ---- 操作 ----

    /// 在第 `step_no` 步处拆分。
    pub fn split(&self, step_no: i32) -> Option<IvgShape> {
        let args = vec![Variant::from_i64(step_no as i64)];
        self.disp
            .invoke_method("Split", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 调用 COM 方法 `FuseStart`。
    pub fn fuse_start(&self) -> Option<bool> {
        self.disp.invoke_method("FuseStart", vec![]).ok()?.to_bool().ok()
    }

    /// 调用 COM 方法 `FuseEnd`。
    pub fn fuse_end(&self) -> Option<bool> {
        self.disp.invoke_method("FuseEnd", vec![]).ok()?.to_bool().ok()
    }

    /// 从其实 blend 复制属性。
    pub fn copy_from(&self, src: &IvgEffectBlend) -> Option<bool> {
        let args = vec![src.as_variant()];
        self.disp
            .invoke_method("CopyFrom", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
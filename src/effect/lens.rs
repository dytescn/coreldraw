//! `IVGEffectLens` —— 透镜效果

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::shape::{IvgShape, IvgShapeRange, IvgShapes};

/// CorelDRAW `IVGEffectLens` 对象的后期绑定包装（效果透镜）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectLens {
    disp: ComObject,
}

impl IvgEffectLens {
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

    fn put_color(&self, name: &str, c: &IvgColor) -> bool {
        let arg = c.as_variant();
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 冻结 ----

    /// 调用 COM 方法 `Freeze`。
    pub fn freeze(&self) -> bool { self.disp.invoke_method("Freeze", vec![]).is_ok() }
    /// 调用 COM 方法 `Unfreeze`。
    pub fn unfreeze(&self) -> bool { self.disp.invoke_method("Unfreeze", vec![]).is_ok() }

    /// 取消群组（COM 方法 `Ungroup`）。
    pub fn ungroup(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("Ungroup", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 读取 COM 属性 `Frozen`。
    pub fn frozen(&self) -> Option<bool> { self.prop_bool("Frozen") }

    /// 读取图形集（`Shapes`）。
    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    // ---- 类型 ----

    /// `cdrLensType`
    pub fn lens_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置透镜类型（`Type`）。
    pub fn set_lens_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取 COM 属性 `Rate`。
    pub fn rate(&self) -> Option<i64> { self.prop_i64("Rate") }
    /// 设置 COM 属性 `Rate`。
    pub fn set_rate(&self, v: i32) -> bool { self.put_i64("Rate", v as i64) }

    /// 读取 COM 属性 `Magnification`。
    pub fn magnification(&self) -> Option<f64> { self.prop_f64("Magnification") }
    /// 设置 COM 属性 `Magnification`。
    pub fn set_magnification(&self, v: f64) -> bool { self.put_f64("Magnification", v) }

    // ---- 颜色 ----

    /// 读取颜色（`Color`）。
    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    /// 设置颜色（`Color`）。
    pub fn set_color(&self, c: &IvgColor) -> bool { self.put_color("Color", c) }

    /// 读取轮廓颜色（`OutlineColor`）。
    pub fn outline_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("OutlineColor").map(IvgColor::new)
    }
    /// 设置轮廓颜色（`OutlineColor`）。
    pub fn set_outline_color(&self, c: &IvgColor) -> bool { self.put_color("OutlineColor", c) }

    /// 读取填充颜色（`FillColor`）。
    pub fn fill_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("FillColor").map(IvgColor::new)
    }
    /// 设置填充颜色（`FillColor`）。
    pub fn set_fill_color(&self, c: &IvgColor) -> bool { self.put_color("FillColor", c) }

    /// 读取从…构造颜色（`FromColor`）。
    pub fn from_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("FromColor").map(IvgColor::new)
    }
    /// 设置从…构造颜色（`FromColor`）。
    pub fn set_from_color(&self, c: &IvgColor) -> bool { self.put_color("FromColor", c) }

    /// 读取转换为颜色（`ToColor`）。
    pub fn to_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("ToColor").map(IvgColor::new)
    }
    /// 设置转换为颜色（`ToColor`）。
    pub fn set_to_color(&self, c: &IvgColor) -> bool { self.put_color("ToColor", c) }

    /// 读取使用轮廓颜色（`UseOutlineColor`）。
    pub fn use_outline_color(&self) -> Option<bool> {
        self.prop_bool("UseOutlineColor")
    }
    /// 设置使用轮廓颜色（`UseOutlineColor`）。
    pub fn set_use_outline_color(&self, v: bool) -> bool { self.put_bool("UseOutlineColor", v) }

    /// 读取使用填充颜色（`UseFillColor`）。
    pub fn use_fill_color(&self) -> Option<bool> {
        self.prop_bool("UseFillColor")
    }
    /// 设置使用填充颜色（`UseFillColor`）。
    pub fn set_use_fill_color(&self, v: bool) -> bool { self.put_bool("UseFillColor", v) }

    /// `cdrFountainFillBlendType`
    pub fn color_map_palette(&self) -> Option<i64> {
        self.prop_i64("ColorMapPalette")
    }
    /// 设置颜色映射调色板（`ColorMapPalette`）。
    pub fn set_color_map_palette(&self, v: i32) -> bool {
        self.put_i64("ColorMapPalette", v as i64)
    }

    // ---- 视点 ----

    /// 读取使用视图点（`UseViewPoint`）。
    pub fn use_view_point(&self) -> Option<bool> { self.prop_bool("UseViewPoint") }
    /// 设置使用视图点（`UseViewPoint`）。
    pub fn set_use_view_point(&self, v: bool) -> bool { self.put_bool("UseViewPoint", v) }

    /// 读取视图点 X 坐标（`ViewPointX`）。
    pub fn view_point_x(&self) -> Option<f64> { self.prop_f64("ViewPointX") }
    /// 设置视图点 X 坐标（`ViewPointX`）。
    pub fn set_view_point_x(&self, v: f64) -> bool { self.put_f64("ViewPointX", v) }

    /// 读取视图点 Y 坐标（`ViewPointY`）。
    pub fn view_point_y(&self) -> Option<f64> { self.prop_f64("ViewPointY") }
    /// 设置视图点 Y 坐标（`ViewPointY`）。
    pub fn set_view_point_y(&self, v: f64) -> bool { self.put_f64("ViewPointY", v) }

    // ---- 移除面/ 调色板旋转----

    /// 读取移除字面（`RemoveFace`）。
    pub fn remove_face(&self) -> Option<bool> { self.prop_bool("RemoveFace") }
    /// 设置移除字面（`RemoveFace`）。
    pub fn set_remove_face(&self, v: bool) -> bool { self.put_bool("RemoveFace", v) }

    /// 读取调色板旋转（`PaletteRotation`）。
    pub fn palette_rotation(&self) -> Option<i64> { self.prop_i64("PaletteRotation") }
    /// 设置调色板旋转（`PaletteRotation`）。
    pub fn set_palette_rotation(&self, v: i32) -> bool {
        self.put_i64("PaletteRotation", v as i64)
    }

    // ---- 便捷：直接拿到关联图形----

    /// 读取 COM 属性 `Shapes`。
    pub fn associated_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("Shapes")
            .and_then(|s| IvgShapes::new(s).item_by_index(1))
    }
}
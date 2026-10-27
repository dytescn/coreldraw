//! `IVGFountainFill` —— 渐变填充
//! `IVGFountainColor(s)` —— 渐变上的颜色停靠点

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;

// =============================================================
// IvgFountainFill
// =============================================================

/// CorelDRAW `IVGFountainFill` 对象的后期绑定包装（渐变填充）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgFountainFill {
    disp: ComObject,
}

impl IvgFountainFill {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
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
        self.disp.set_property(name, vec![c.as_variant()]).is_ok()
    }

    // ---- 类型 ----

    /// `cdrFountainFillType`
    pub fn fountain_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置渐变类型（`Type`）。
    pub fn set_fountain_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    // ---- 起止坐标 ----

    /// 读取启动 X 坐标（`StartX`）。
    pub fn start_x(&self) -> Option<f64> { self.prop_f64("StartX") }
    /// 设置启动 X 坐标（`StartX`）。
    pub fn set_start_x(&self, v: f64) -> bool { self.put_f64("StartX", v) }

    /// 读取启动 Y 坐标（`StartY`）。
    pub fn start_y(&self) -> Option<f64> { self.prop_f64("StartY") }
    /// 设置启动 Y 坐标（`StartY`）。
    pub fn set_start_y(&self, v: f64) -> bool { self.put_f64("StartY", v) }

    /// 读取结束 X 坐标（`EndX`）。
    pub fn end_x(&self) -> Option<f64> { self.prop_f64("EndX") }
    /// 设置结束 X 坐标（`EndX`）。
    pub fn set_end_x(&self, v: f64) -> bool { self.put_f64("EndX", v) }

    /// 读取结束 Y 坐标（`EndY`）。
    pub fn end_y(&self) -> Option<f64> { self.prop_f64("EndY") }
    /// 设置结束 Y 坐标（`EndY`）。
    pub fn set_end_y(&self, v: f64) -> bool { self.put_f64("EndY", v) }

    /// 读取末端 X 坐标（`End2X`）。
    pub fn end2_x(&self) -> Option<f64> { self.prop_f64("End2X") }
    /// 设置末端 X 坐标（`End2X`）。
    pub fn set_end2_x(&self, v: f64) -> bool { self.put_f64("End2X", v) }

    /// 读取末端 Y 坐标（`End2Y`）。
    pub fn end2_y(&self) -> Option<f64> { self.prop_f64("End2Y") }
    /// 设置末端 Y 坐标（`End2Y`）。
    pub fn set_end2_y(&self, v: f64) -> bool { self.put_f64("End2Y", v) }

    // ---- 角度 ----

    /// 读取角度（`Angle`）。
    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }

    /// 设置角度（`SetAngle`）。
    pub fn set_angle(&self, v: f64) -> bool {
        let args = vec![Variant::from_f64(v)];
        self.disp.invoke_method("SetAngle", args).is_ok()
    }

    // ---- 步进 / 混合 ----

    /// 读取步数（`Steps`）。
    pub fn steps(&self) -> Option<i64> { self.prop_i64("Steps") }
    /// 设置步数（`Steps`）。
    pub fn set_steps(&self, v: i32) -> bool { self.put_i64("Steps", v as i64) }

    /// 读取边缘边距（`EdgePad`）。
    pub fn edge_pad(&self) -> Option<i64> { self.prop_i64("EdgePad") }
    /// 设置边缘边距（`SetEdgePad`）。
    pub fn set_edge_pad(&self, v: i32) -> bool {
        let args = vec![Variant::from_i64(v as i64)];
        self.disp.invoke_method("SetEdgePad", args).is_ok()
    }

    /// `cdrFountainFillBlendType`
    pub fn blend_type(&self) -> Option<i64> { self.prop_i64("BlendType") }
    /// 设置调和类型（`BlendType`）。
    pub fn set_blend_type(&self, v: i32) -> bool { self.put_i64("BlendType", v as i64) }

    /// 读取中间点（`MidPoint`）。
    pub fn mid_point(&self) -> Option<i64> { self.prop_i64("MidPoint") }
    /// 设置中间点（`MidPoint`）。
    pub fn set_mid_point(&self, v: i32) -> bool { self.put_i64("MidPoint", v as i64) }

    /// 读取调和加速度（`BlendAcceleration`）。
    pub fn blend_acceleration(&self) -> Option<f64> {
        self.prop_f64("BlendAcceleration")
    }
    /// 设置调和加速度（`BlendAcceleration`）。
    pub fn set_blend_acceleration(&self, v: f64) -> bool {
        self.put_f64("BlendAcceleration", v)
    }

    // ---- 颜色 ----

    /// 读取颜色集（`Colors`）。
    pub fn colors(&self) -> Option<IvgFountainColors> {
        self.prop_dispatch("Colors").map(IvgFountainColors::new)
    }

    /// 设置颜色集（`Colors`）。
    pub fn set_colors(&self, colors: &IvgFountainColors) -> bool {
        self.disp
            .set_property("Colors", vec![colors.as_variant()])
            .is_ok()
    }

    /// 读取启动颜色（`StartColor`）。
    pub fn start_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("StartColor").map(IvgColor::new)
    }
    /// 设置启动颜色（`StartColor`）。
    pub fn set_start_color(&self, c: &IvgColor) -> bool {
        self.put_color("StartColor", c)
    }

    /// 读取结束颜色（`EndColor`）。
    pub fn end_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("EndColor").map(IvgColor::new)
    }
    /// 设置结束颜色（`EndColor`）。
    pub fn set_end_color(&self, c: &IvgColor) -> bool {
        self.put_color("EndColor", c)
    }

    // ---- 中心偏移 ----

    /// 读取中心偏移 X 坐标（`CenterOffsetX`）。
    pub fn center_offset_x(&self) -> Option<f64> {
        self.prop_f64("CenterOffsetX")
    }
    /// 设置中心偏移 X 坐标（`CenterOffsetX`）。
    pub fn set_center_offset_x(&self, v: f64) -> bool {
        self.put_f64("CenterOffsetX", v)
    }

    /// 读取中心偏移 Y 坐标（`CenterOffsetY`）。
    pub fn center_offset_y(&self) -> Option<f64> {
        self.prop_f64("CenterOffsetY")
    }
    /// 设置中心偏移 Y 坐标（`CenterOffsetY`）。
    pub fn set_center_offset_y(&self, v: f64) -> bool {
        self.put_f64("CenterOffsetY", v)
    }

    // ---- 平滑 / 扩散 ----

    /// 读取平滑调和（`SmoothBlend`）。
    pub fn smooth_blend(&self) -> Option<bool> { self.prop_bool("SmoothBlend") }
    /// 设置平滑调和（`SmoothBlend`）。
    pub fn set_smooth_blend(&self, v: bool) -> bool { self.put_bool("SmoothBlend", v) }

    /// `cdrFountainFillSpreadMethod`
    pub fn spread_method(&self) -> Option<i64> {
        self.prop_i64("SpreadMethod")
    }
    /// 设置跨页方法（`SpreadMethod`）。
    pub fn set_spread_method(&self, v: i32) -> bool {
        self.put_i64("SpreadMethod", v as i64)
    }

    /// 读取各向异性（`Anisotropic`）。
    pub fn anisotropic(&self) -> Option<bool> { self.prop_bool("Anisotropic") }
    /// 设置各向异性（`Anisotropic`）。
    pub fn set_anisotropic(&self, v: bool) -> bool { self.put_bool("Anisotropic", v) }

    // ---- 透明 / 混合模式 ----

    /// 是否透明（`IsTransparent`）。
    pub fn is_transparent(&self) -> Option<bool> { self.prop_bool("IsTransparent") }

    /// `cdrMergeMode`
    pub fn merge_mode(&self) -> Option<i64> { self.prop_i64("MergeMode") }
    /// 设置合并模式（`MergeMode`）。
    pub fn set_merge_mode(&self, v: i32) -> bool { self.put_i64("MergeMode", v as i64) }

    // ---- 缩放 / 倾斜 ----

    /// 读取缩放 X 坐标（`ScaleX`）。
    pub fn scale_x(&self) -> Option<f64> { self.prop_f64("ScaleX") }
    /// 设置缩放 X 坐标（`ScaleX`）。
    pub fn set_scale_x(&self, v: f64) -> bool { self.put_f64("ScaleX", v) }

    /// 读取缩放 Y 坐标（`ScaleY`）。
    pub fn scale_y(&self) -> Option<f64> { self.prop_f64("ScaleY") }
    /// 设置缩放 Y 坐标（`ScaleY`）。
    pub fn set_scale_y(&self, v: f64) -> bool { self.put_f64("ScaleY", v) }

    /// 读取倾斜（`Skew`）。
    pub fn skew(&self) -> Option<f64> { self.prop_f64("Skew") }
    /// 设置倾斜（`Skew`）。
    pub fn set_skew(&self, v: f64) -> bool { self.put_f64("Skew", v) }

    // ---- 变换矩阵 ----

    /// 返回 (d11, d12, d21, d22)
    pub fn get_transformations(&self) -> Option<(f64, f64, f64, f64)> {
        // 4 个 out 参数在 IDispatch 里拿不到，返回 None 占位：
        // 用 ScaleX / ScaleY / Skew 替代。
        None
    }

    /// 设置变换（`SetTransformations`）。
    pub fn set_transformations(&self, d11: f64, d12: f64, d21: f64, d22: f64) -> bool {
        let args = vec![
            Variant::from_f64(d11),
            Variant::from_f64(d12),
            Variant::from_f64(d21),
            Variant::from_f64(d22),
        ];
        self.disp.invoke_method("SetTransformations", args).is_ok()
    }

    // ---- 混合检测----

    /// 读取 COM 属性 `HasHSBBlends`。
    pub fn has_hsb_blends(&self) -> Option<bool> { self.prop_bool("HasHSBBlends") }
    /// 读取 COM 属性 `HasNonLinearBlends`。
    pub fn has_non_linear_blends(&self) -> Option<bool> {
        self.prop_bool("HasNonLinearBlends")
    }

    // ---- 操作 ----

    /// translate（COM 方法 `Translate`）。
    pub fn translate(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("Translate", args).is_ok()
    }

    /// 调用 COM 方法 `MakeOpaque`。
    pub fn make_opaque(&self) -> bool {
        self.disp.invoke_method("MakeOpaque", vec![]).is_ok()
    }
}

// =============================================================
// IvgFountainColors
// =============================================================

/// CorelDRAW `IVGFountainFill` 对象的后期绑定包装（渐变颜色集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgFountainColors {
    disp: ComObject,
}

impl IvgFountainColors {
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgFountainColor> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgFountainColor::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgFountainColor> {
        self.prop_dispatch("First").map(IvgFountainColor::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgFountainColor> {
        self.prop_dispatch("Last").map(IvgFountainColor::new)
    }

    /// 添加颜色停靠点。
    pub fn add(&self, color: &IvgColor, position: i32) -> bool {
        let args = vec![
            color.as_variant(),
            Variant::from_i64(position as i64),
        ];
        self.disp.invoke_method("Add", args).is_ok()
    }

    /// 添加灰度停靠点。
    pub fn add_gray_level(&self, gray_level: i32, position: i32) -> bool {
        let args = vec![
            Variant::from_i64(gray_level as i64),
            Variant::from_i64(position as i64),
        ];
        self.disp.invoke_method("AddGrayLevel", args).is_ok()
    }

    /// 获取/设置灰度级别。
    pub fn gray_level(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("GrayLevel", args).ok()?.to_i64().ok()
    }

    /// 设置灰度级别（`put_GrayLevel`）。
    pub fn set_gray_level(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_GrayLevel", args).is_ok()
    }

    /// 删除全部（通过 `set_colors` 覆盖）。
    pub fn clear(&self) -> bool {
        // 没有直接 Clear 方法，改用 `IvgFountainFill::set_colors` 传新集合。
        false
    }
}

// =============================================================
// IvgFountainColor
// =============================================================

/// CorelDRAW `IVGFountainFill` 对象的后期绑定包装（渐变颜色）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgFountainColor {
    disp: ComObject,
}

impl IvgFountainColor {
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

    fn prop_u8(&self, name: &str) -> Option<u8> {
        self.disp.get_property(name).ok()?.to_u8().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取位置（`Position`）。
    pub fn position(&self) -> Option<i64> { self.prop_i64("Position") }

    /// 读取颜色（`Color`）。
    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    /// 设置颜色（`Color`）。
    pub fn set_color(&self, c: &IvgColor) -> bool {
        self.disp.set_property("Color", vec![c.as_variant()]).is_ok()
    }

    /// 读取中间点（`MidPoint`）。
    pub fn mid_point(&self) -> Option<i64> { self.prop_i64("MidPoint") }
    /// 设置中间点（`MidPoint`）。
    pub fn set_mid_point(&self, v: i32) -> bool { self.put_i64("MidPoint", v as i64) }

    /// `cdrFountainFillBlendType`
    pub fn blend_type(&self) -> Option<i64> { self.prop_i64("BlendType") }
    /// 设置调和类型（`BlendType`）。
    pub fn set_blend_type(&self, v: i32) -> bool {
        self.put_i64("BlendType", v as i64)
    }

    /// 0..255
    pub fn opacity(&self) -> Option<u8> { self.prop_u8("Opacity") }
    /// 设置不透明度（`Opacity`）。
    pub fn set_opacity(&self, v: u8) -> bool {
        let arg = Variant::from_i64(v as i64);
        self.disp.set_property("Opacity", vec![arg]).is_ok()
    }

    /// 移动（COM 方法 `Move`）。
    pub fn move_to(&self, new_position: i32) -> bool {
        let args = vec![Variant::from_i64(new_position as i64)];
        self.disp.invoke_method("Move", args).is_ok()
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}
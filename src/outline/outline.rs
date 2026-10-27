//! `IVGOutline` —— 图形轮廓
//!
//! 由 `shape.outline()` 取得。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::outline::{IvgArrowHead, IvgArrowHeadOptions, IvgOutlineStyle};
use crate::shape::IvgShape;

/// CorelDRAW `IVGOutline` 对象的后期绑定包装（轮廓）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgOutline {
    disp: ComObject,
}

impl IvgOutline {
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

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 通用工具
    // ---------------------------------------------------------

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

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---------------------------------------------------------
    // 宽度 / 类型
    // ---------------------------------------------------------

    /// 轮廓宽度（世界单位）。`Width` 与 `Size` 是同一值的两个名字。
    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    /// 读取尺寸（`Size`）。
    pub fn size(&self) -> Option<f64> { self.prop_f64("Size") }
    /// 设置尺寸（`Size`）。
    pub fn set_size(&self, v: f64) -> bool { self.put_f64("Size", v) }

    /// `cdrOutlineType`
    pub fn outline_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置轮廓类型（`Type`）。
    pub fn set_outline_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    // ---------------------------------------------------------
    // 颜色
    // ---------------------------------------------------------

    /// 读取颜色（`Color`）。
    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    /// 设置颜色（`Color`）。
    pub fn set_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("Color", c.as_variant())
    }

    // ---------------------------------------------------------
    // 线型 / 样式
    // ---------------------------------------------------------

    /// 读取样式（`Style`）。
    pub fn style(&self) -> Option<IvgOutlineStyle> {
        self.prop_dispatch("Style").map(IvgOutlineStyle::new)
    }
    /// 设置样式（`Style`）。
    pub fn set_style(&self, s: &IvgOutlineStyle) -> bool {
        self.put_dispatch("Style", s.as_variant())
    }

    /// 虚线单位长度（`DashDotLength`）。
    pub fn dash_dot_length(&self) -> Option<f64> { self.prop_f64("DashDotLength") }
    /// 设置虚线点长度（`DashDotLength`）。
    pub fn set_dash_dot_length(&self, v: f64) -> bool { self.put_f64("DashDotLength", v) }

    /// 笔尖宽度（`PenWidth`），与 `Width` 不同：用于书法笔尖。
    pub fn pen_width(&self) -> Option<f64> { self.prop_f64("PenWidth") }
    /// 设置钢笔宽度（`PenWidth`）。
    pub fn set_pen_width(&self, v: f64) -> bool { self.put_f64("PenWidth", v) }

    /// 笔尖角度（度）。
    pub fn nib_angle(&self) -> Option<f64> { self.prop_f64("NibAngle") }
    /// 设置笔尖角度（`NibAngle`）。
    pub fn set_nib_angle(&self, v: f64) -> bool { self.put_f64("NibAngle", v) }

    /// 笔尖拉伸（百分比）。
    pub fn nib_stretch(&self) -> Option<i64> { self.prop_i64("NibStretch") }
    /// 设置笔尖拉伸（`NibStretch`）。
    pub fn set_nib_stretch(&self, v: i32) -> bool { self.put_i64("NibStretch", v as i64) }

    // ---------------------------------------------------------
    // 端点 / 连接
    // ---------------------------------------------------------

    /// `cdrOutlineLineCaps`
    pub fn line_caps(&self) -> Option<i64> { self.prop_i64("LineCaps") }
    /// 设置线端点（`LineCaps`）。
    pub fn set_line_caps(&self, v: i32) -> bool { self.put_i64("LineCaps", v as i64) }

    /// `cdrOutlineLineJoin`
    pub fn line_join(&self) -> Option<i64> { self.prop_i64("LineJoin") }
    /// 设置线连接（`LineJoin`）。
    pub fn set_line_join(&self, v: i32) -> bool { self.put_i64("LineJoin", v as i64) }

    /// 斜接限制。
    pub fn miter_limit(&self) -> Option<f64> { self.prop_f64("MiterLimit") }
    /// 设置斜接限制（`MiterLimit`）。
    pub fn set_miter_limit(&self, v: f64) -> bool { self.put_f64("MiterLimit", v) }

    /// `cdrOutlineJustification`
    pub fn justification(&self) -> Option<i64> { self.prop_i64("Justification") }
    /// 设置对齐方式（`Justification`）。
    pub fn set_justification(&self, v: i32) -> bool {
        self.put_i64("Justification", v as i64)
    }

    /// `cdrOutlineDashAdjust`
    pub fn adjust_dashes(&self) -> Option<i64> { self.prop_i64("AdjustDashes") }
    /// 设置调整虚线（`AdjustDashes`）。
    pub fn set_adjust_dashes(&self, v: i32) -> bool {
        self.put_i64("AdjustDashes", v as i64)
    }

    // ---------------------------------------------------------
    // 填充前/ 随图形缩放
    // ---------------------------------------------------------

    /// 读取后方填充（`BehindFill`）。
    pub fn behind_fill(&self) -> Option<bool> { self.prop_bool("BehindFill") }
    /// 设置后方填充（`BehindFill`）。
    pub fn set_behind_fill(&self, v: bool) -> bool { self.put_bool("BehindFill", v) }

    /// 读取缩放图形（`ScaleWithShape`）。
    pub fn scale_with_shape(&self) -> Option<bool> { self.prop_bool("ScaleWithShape") }
    /// 设置缩放图形（`ScaleWithShape`）。
    pub fn set_scale_with_shape(&self, v: bool) -> bool {
        self.put_bool("ScaleWithShape", v)
    }

    // ---------------------------------------------------------
    // 箭头
    // ---------------------------------------------------------

    /// 读取启动箭头（`StartArrow`）。
    pub fn start_arrow(&self) -> Option<IvgArrowHead> {
        self.prop_dispatch("StartArrow").map(IvgArrowHead::new)
    }
    /// 设置启动箭头（`StartArrow`）。
    pub fn set_start_arrow(&self, a: &IvgArrowHead) -> bool {
        self.put_dispatch("StartArrow", a.as_variant())
    }

    /// 读取结束箭头（`EndArrow`）。
    pub fn end_arrow(&self) -> Option<IvgArrowHead> {
        self.prop_dispatch("EndArrow").map(IvgArrowHead::new)
    }
    /// 设置结束箭头（`EndArrow`）。
    pub fn set_end_arrow(&self, a: &IvgArrowHead) -> bool {
        self.put_dispatch("EndArrow", a.as_variant())
    }

    /// 读取启动箭头选项集（`StartArrowOptions`）。
    pub fn start_arrow_options(&self) -> Option<IvgArrowHeadOptions> {
        self.prop_dispatch("StartArrowOptions").map(IvgArrowHeadOptions::new)
    }
    /// 设置启动箭头选项集（`StartArrowOptions`）。
    pub fn set_start_arrow_options(&self, o: &IvgArrowHeadOptions) -> bool {
        self.put_dispatch("StartArrowOptions", o.as_variant())
    }

    /// 读取结束箭头选项集（`EndArrowOptions`）。
    pub fn end_arrow_options(&self) -> Option<IvgArrowHeadOptions> {
        self.prop_dispatch("EndArrowOptions").map(IvgArrowHeadOptions::new)
    }
    /// 设置结束箭头选项集（`EndArrowOptions`）。
    pub fn set_end_arrow_options(&self, o: &IvgArrowHeadOptions) -> bool {
        self.put_dispatch("EndArrowOptions", o.as_variant())
    }

    // ---------------------------------------------------------
    // 屏幕选项（PostScript）
    // ---------------------------------------------------------

    /// 读取 COM 属性 `PSScreen`。
    pub fn ps_screen(&self) -> Option<crate::fill::IvgPSScreenOptions> {
        self.prop_dispatch("PSScreen").map(crate::fill::IvgPSScreenOptions::new)
    }

    // ---------------------------------------------------------
    // 批量设置
    // ---------------------------------------------------------

    /// 用一整套参数设置轮廓。
    #[allow(clippy::too_many_arguments)]
    pub fn set_properties(
        &self,
        width: f64,
        style: &IvgOutlineStyle,
        color: &IvgColor,
        start_arrow: &IvgArrowHead,
        end_arrow: &IvgArrowHead,
        behind_fill: i32,
        scale_with_shape: i32,
        line_caps: i32,
        line_join: i32,
        nib_angle: f64,
        nib_stretch: i32,
        dash_dot_length: f64,
        pen_width: f64,
        miter_limit: f64,
    ) -> bool {
        let args = vec![
            Variant::from_f64(width),
            style.as_variant(),
            color.as_variant(),
            start_arrow.as_variant(),
            end_arrow.as_variant(),
            Variant::from_i64(behind_fill as i64),
            Variant::from_i64(scale_with_shape as i64),
            Variant::from_i64(line_caps as i64),
            Variant::from_i64(line_join as i64),
            Variant::from_f64(nib_angle),
            Variant::from_i64(nib_stretch as i64),
            Variant::from_f64(dash_dot_length),
            Variant::from_f64(pen_width),
            Variant::from_f64(miter_limit),
        ];
        self.disp.invoke_method("SetProperties", args).is_ok()
    }

    /// 与 `set_properties` 类似，但多了 `Justification`。
    #[allow(clippy::too_many_arguments)]
    pub fn set_properties_ex(
        &self,
        width: f64,
        style: &IvgOutlineStyle,
        color: &IvgColor,
        start_arrow: &IvgArrowHead,
        end_arrow: &IvgArrowHead,
        behind_fill: i32,
        scale_with_shape: i32,
        line_caps: i32,
        line_join: i32,
        nib_angle: f64,
        nib_stretch: i32,
        dash_dot_length: f64,
        pen_width: f64,
        miter_limit: f64,
        justification: i32,
    ) -> bool {
        let args = vec![
            Variant::from_f64(width),
            style.as_variant(),
            color.as_variant(),
            start_arrow.as_variant(),
            end_arrow.as_variant(),
            Variant::from_i64(behind_fill as i64),
            Variant::from_i64(scale_with_shape as i64),
            Variant::from_i64(line_caps as i64),
            Variant::from_i64(line_join as i64),
            Variant::from_f64(nib_angle),
            Variant::from_i64(nib_stretch as i64),
            Variant::from_f64(dash_dot_length),
            Variant::from_f64(pen_width),
            Variant::from_f64(miter_limit),
            Variant::from_i64(justification as i64),
        ];
        self.disp.invoke_method("SetPropertiesEx", args).is_ok()
    }

    // ---------------------------------------------------------
    // 复制 / 比较
    // ---------------------------------------------------------

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgOutline> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgOutline::new)
    }

    /// 复制 assign（COM 方法 `CopyAssign`）。
    pub fn copy_assign(&self, other: &IvgOutline) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    /// 比较（COM 方法 `CompareWith`）。
    pub fn compare_with(&self, other: &IvgOutline) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("CompareWith", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 字符串
    // ---------------------------------------------------------

    /// 转换为轮廓字符串（COM 方法 `ToString`）。
    pub fn to_outline_string(&self) -> Option<String> {
        self.disp.invoke_method("ToString", vec![]).ok()?.to_string().ok()
    }

    /// 字符串 assign（COM 方法 `StringAssign`）。
    pub fn string_assign(&self, s: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(s.into())];
        self.disp
            .invoke_method("StringAssign", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 操作
    // ---------------------------------------------------------

    /// 转为图形（把轮廓变成独立图形）。
    pub fn convert_to_object(&self) -> Option<IvgShape> {
        self.disp
            .invoke_method("ConvertToObject", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 移除轮廓。
    pub fn set_no_outline(&self) -> bool {
        self.disp.invoke_method("SetNoOutline", vec![]).is_ok()
    }

    /// 弹出交互式轮廓属性对话框。
    pub fn user_assign(&self, parent_window_handle: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(parent_window_handle as i64)];
        self.disp
            .invoke_method("UserAssign", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
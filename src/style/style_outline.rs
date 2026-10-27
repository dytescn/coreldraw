//! `IVGStyleOutline` —— 样式里的轮廓部分

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGStyleOutline` 对象的后期绑定包装（样式轮廓）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStyleOutline {
    disp: ComObject,
}

impl IvgStyleOutline {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
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

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---- 主属性----

    /// `cdrOutlineType`
    pub fn outline_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置轮廓类型（`Type`）。
    pub fn set_outline_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    /// 读取颜色（`Color`）。
    pub fn color(&self) -> Option<crate::color::IvgColor> {
        self.prop_dispatch("Color").map(crate::color::IvgColor::new)
    }
    /// 设置颜色（`Color`）。
    pub fn set_color(&self, c: &crate::color::IvgColor) -> bool {
        self.put_dispatch("Color", c.as_variant())
    }

    /// 读取叠印（`Overprint`）。
    pub fn overprint(&self) -> Option<bool> { self.prop_bool("Overprint") }
    /// 设置叠印（`Overprint`）。
    pub fn set_overprint(&self, v: bool) -> bool { self.put_bool("Overprint", v) }

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

    // ---- 箭头 ----

    /// 读取重叠箭头（`OverlapArrow`）。
    pub fn overlap_arrow(&self) -> Option<bool> { self.prop_bool("OverlapArrow") }
    /// 设置重叠箭头（`OverlapArrow`）。
    pub fn set_overlap_arrow(&self, v: bool) -> bool { self.put_bool("OverlapArrow", v) }

    /// 读取共享箭头（`ShareArrow`）。
    pub fn share_arrow(&self) -> Option<bool> { self.prop_bool("ShareArrow") }
    /// 设置共享箭头（`ShareArrow`）。
    pub fn set_share_arrow(&self, v: bool) -> bool { self.put_bool("ShareArrow", v) }

    // ---- 斜接 / 笔尖 ----

    /// 读取斜接限制（`MiterLimit`）。
    pub fn miter_limit(&self) -> Option<f64> { self.prop_f64("MiterLimit") }
    /// 设置斜接限制（`MiterLimit`）。
    pub fn set_miter_limit(&self, v: f64) -> bool { self.put_f64("MiterLimit", v) }

    /// 读取笔尖拉伸（`NibStretch`）。
    pub fn nib_stretch(&self) -> Option<i64> { self.prop_i64("NibStretch") }
    /// 设置笔尖拉伸（`NibStretch`）。
    pub fn set_nib_stretch(&self, v: i32) -> bool { self.put_i64("NibStretch", v as i64) }

    /// 读取笔尖角度（`NibAngle`）。
    pub fn nib_angle(&self) -> Option<f64> { self.prop_f64("NibAngle") }
    /// 设置笔尖角度（`NibAngle`）。
    pub fn set_nib_angle(&self, v: f64) -> bool { self.put_f64("NibAngle", v) }

    /// 读取 COM 属性 `WidelineWidth`。
    pub fn wideline_width(&self) -> Option<f64> { self.prop_f64("WidelineWidth") }
    /// 设置 COM 属性 `WidelineWidth`。
    pub fn set_wideline_width(&self, v: f64) -> bool { self.put_f64("WidelineWidth", v) }

    // ---- 端点 / 连接 / 对齐 ----

    /// `cdrOutlineLineCaps`
    pub fn line_caps(&self) -> Option<i64> { self.prop_i64("LineCaps") }
    /// 设置线端点（`LineCaps`）。
    pub fn set_line_caps(&self, v: i32) -> bool { self.put_i64("LineCaps", v as i64) }

    /// `cdrOutlineLineJoin`
    pub fn line_join(&self) -> Option<i64> { self.prop_i64("LineJoin") }
    /// 设置线连接（`LineJoin`）。
    pub fn set_line_join(&self, v: i32) -> bool { self.put_i64("LineJoin", v as i64) }

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

    // ---- 关联的样式----

    /// 读取样式（`Style`）。
    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.prop_dispatch("Style").map(crate::style::IvgStyle::new)
    }
}
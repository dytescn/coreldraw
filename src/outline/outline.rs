//! `IVGOutline` 鈥斺€?鍥惧舰杞粨
//!
//! 鐢?`shape.outline()` 鍙栧緱銆?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::outline::{IvgArrowHead, IvgArrowHeadOptions, IvgOutlineStyle};
use crate::shape::IvgShape;

pub struct IvgOutline {
    disp: ComObject,
}

impl IvgOutline {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 閫氱敤宸ュ叿
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
    // 瀹藉害 / 绫诲瀷
    // ---------------------------------------------------------

    /// 杞粨瀹藉害锛堜笘鐣屽崟浣嶏級銆俙Width` 涓?`Size` 鏄悓涓€鍊肩殑涓や釜鍚嶅瓧銆?
    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    pub fn size(&self) -> Option<f64> { self.prop_f64("Size") }
    pub fn set_size(&self, v: f64) -> bool { self.put_f64("Size", v) }

    /// `cdrOutlineType`
    pub fn outline_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_outline_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    // ---------------------------------------------------------
    // 棰滆壊
    // ---------------------------------------------------------

    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    pub fn set_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("Color", c.as_variant())
    }

    // ---------------------------------------------------------
    // 绾垮瀷 / 鏍峰紡
    // ---------------------------------------------------------

    pub fn style(&self) -> Option<IvgOutlineStyle> {
        self.prop_dispatch("Style").map(IvgOutlineStyle::new)
    }
    pub fn set_style(&self, s: &IvgOutlineStyle) -> bool {
        self.put_dispatch("Style", s.as_variant())
    }

    /// 铏氱嚎鍗曚綅闀垮害锛坄DashDotLength`锛夈€?
    pub fn dash_dot_length(&self) -> Option<f64> { self.prop_f64("DashDotLength") }
    pub fn set_dash_dot_length(&self, v: f64) -> bool { self.put_f64("DashDotLength", v) }

    /// 绗斿皷瀹藉害锛坄PenWidth`锛夛紝涓?`Width` 涓嶅悓锛氱敤浜庝功娉曠瑪灏栥€?
    pub fn pen_width(&self) -> Option<f64> { self.prop_f64("PenWidth") }
    pub fn set_pen_width(&self, v: f64) -> bool { self.put_f64("PenWidth", v) }

    /// 绗斿皷瑙掑害锛堝害锛夈€?
    pub fn nib_angle(&self) -> Option<f64> { self.prop_f64("NibAngle") }
    pub fn set_nib_angle(&self, v: f64) -> bool { self.put_f64("NibAngle", v) }

    /// 绗斿皷鎷変几锛堢櫨鍒嗘瘮锛夈€?
    pub fn nib_stretch(&self) -> Option<i64> { self.prop_i64("NibStretch") }
    pub fn set_nib_stretch(&self, v: i32) -> bool { self.put_i64("NibStretch", v as i64) }

    // ---------------------------------------------------------
    // 绔偣 / 杩炴帴
    // ---------------------------------------------------------

    /// `cdrOutlineLineCaps`
    pub fn line_caps(&self) -> Option<i64> { self.prop_i64("LineCaps") }
    pub fn set_line_caps(&self, v: i32) -> bool { self.put_i64("LineCaps", v as i64) }

    /// `cdrOutlineLineJoin`
    pub fn line_join(&self) -> Option<i64> { self.prop_i64("LineJoin") }
    pub fn set_line_join(&self, v: i32) -> bool { self.put_i64("LineJoin", v as i64) }

    /// 鏂滄帴闄愬埗銆?
    pub fn miter_limit(&self) -> Option<f64> { self.prop_f64("MiterLimit") }
    pub fn set_miter_limit(&self, v: f64) -> bool { self.put_f64("MiterLimit", v) }

    /// `cdrOutlineJustification`
    pub fn justification(&self) -> Option<i64> { self.prop_i64("Justification") }
    pub fn set_justification(&self, v: i32) -> bool {
        self.put_i64("Justification", v as i64)
    }

    /// `cdrOutlineDashAdjust`
    pub fn adjust_dashes(&self) -> Option<i64> { self.prop_i64("AdjustDashes") }
    pub fn set_adjust_dashes(&self, v: i32) -> bool {
        self.put_i64("AdjustDashes", v as i64)
    }

    // ---------------------------------------------------------
    // 濉厖鍓?/ 闅忓浘褰㈢缉鏀?
    // ---------------------------------------------------------

    pub fn behind_fill(&self) -> Option<bool> { self.prop_bool("BehindFill") }
    pub fn set_behind_fill(&self, v: bool) -> bool { self.put_bool("BehindFill", v) }

    pub fn scale_with_shape(&self) -> Option<bool> { self.prop_bool("ScaleWithShape") }
    pub fn set_scale_with_shape(&self, v: bool) -> bool {
        self.put_bool("ScaleWithShape", v)
    }

    // ---------------------------------------------------------
    // 绠ご
    // ---------------------------------------------------------

    pub fn start_arrow(&self) -> Option<IvgArrowHead> {
        self.prop_dispatch("StartArrow").map(IvgArrowHead::new)
    }
    pub fn set_start_arrow(&self, a: &IvgArrowHead) -> bool {
        self.put_dispatch("StartArrow", a.as_variant())
    }

    pub fn end_arrow(&self) -> Option<IvgArrowHead> {
        self.prop_dispatch("EndArrow").map(IvgArrowHead::new)
    }
    pub fn set_end_arrow(&self, a: &IvgArrowHead) -> bool {
        self.put_dispatch("EndArrow", a.as_variant())
    }

    pub fn start_arrow_options(&self) -> Option<IvgArrowHeadOptions> {
        self.prop_dispatch("StartArrowOptions").map(IvgArrowHeadOptions::new)
    }
    pub fn set_start_arrow_options(&self, o: &IvgArrowHeadOptions) -> bool {
        self.put_dispatch("StartArrowOptions", o.as_variant())
    }

    pub fn end_arrow_options(&self) -> Option<IvgArrowHeadOptions> {
        self.prop_dispatch("EndArrowOptions").map(IvgArrowHeadOptions::new)
    }
    pub fn set_end_arrow_options(&self, o: &IvgArrowHeadOptions) -> bool {
        self.put_dispatch("EndArrowOptions", o.as_variant())
    }

    // ---------------------------------------------------------
    // 灞忓箷閫夐」锛圥ostScript锛?
    // ---------------------------------------------------------

    pub fn ps_screen(&self) -> Option<crate::fill::IvgPSScreenOptions> {
        self.prop_dispatch("PSScreen").map(crate::fill::IvgPSScreenOptions::new)
    }

    // ---------------------------------------------------------
    // 鎵归噺璁剧疆
    // ---------------------------------------------------------

    /// 鐢ㄤ竴鏁村鍙傛暟璁剧疆杞粨銆?
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

    /// 涓?`set_properties` 绫讳技锛屼絾澶氫簡 `Justification`銆?
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
    // 澶嶅埗 / 姣旇緝
    // ---------------------------------------------------------

    pub fn get_copy(&self) -> Option<IvgOutline> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgOutline::new)
    }

    pub fn copy_assign(&self, other: &IvgOutline) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    pub fn compare_with(&self, other: &IvgOutline) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("CompareWith", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 瀛楃涓?
    // ---------------------------------------------------------

    pub fn to_outline_string(&self) -> Option<String> {
        self.disp.invoke_method("ToString", vec![]).ok()?.to_string().ok()
    }

    pub fn string_assign(&self, s: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(s.into())];
        self.disp
            .invoke_method("StringAssign", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 鎿嶄綔
    // ---------------------------------------------------------

    /// 杞负鍥惧舰锛堟妸杞粨鍙樻垚鐙珛鍥惧舰锛夈€?
    pub fn convert_to_object(&self) -> Option<IvgShape> {
        self.disp
            .invoke_method("ConvertToObject", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 绉婚櫎杞粨銆?
    pub fn set_no_outline(&self) -> bool {
        self.disp.invoke_method("SetNoOutline", vec![]).is_ok()
    }

    /// 寮瑰嚭浜や簰寮忚疆寤撳睘鎬у璇濇銆?
    pub fn user_assign(&self, parent_window_handle: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(parent_window_handle as i64)];
        self.disp
            .invoke_method("UserAssign", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
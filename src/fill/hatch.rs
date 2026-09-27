//! `IVGHatchFill` 鈥斺€?鍓栭潰绾垮～鍏?
//! `IVGHatchPattern(s)` 鈥斺€?鍓栭潰绾垮浘妗?
//! `IVGHatchLibrary / Libraries` 鈥斺€?鍓栭潰绾垮簱
//! `IVGHatchFills` 鈥斺€?搴撻噷鐨勫～鍏呴泦鍚?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::outline::IvgOutlineStyle;

// =============================================================
// IvgHatchFill
// =============================================================

pub struct IvgHatchFill {
    disp: ComObject,
}

impl IvgHatchFill {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // fn put_i64(&self, name: &str, v: i64) -> bool {
    //     let arg = Variant::from_i64(v);
    //     self.disp.set_property(name, vec![arg]).is_ok()
    // }

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

    // ---- 鑳屾櫙鑹?----

    pub fn back_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("BackColor").map(IvgColor::new)
    }
    pub fn set_back_color(&self, c: &IvgColor) -> bool {
        self.put_color("BackColor", c)
    }

    pub fn has_background(&self) -> Option<bool> { self.prop_bool("HasBackground") }

    pub fn set_no_back_color(&self) -> bool {
        self.disp.invoke_method("SetNoBackColor", vec![]).is_ok()
    }

    // ---- 鍙樻崲 ----

    pub fn transform_with_shape(&self) -> Option<bool> {
        self.prop_bool("TransformWithShape")
    }
    pub fn set_transform_with_shape(&self, v: bool) -> bool {
        self.put_bool("TransformWithShape", v)
    }

    pub fn scale_lines_with_shape(&self) -> Option<bool> {
        self.prop_bool("ScaleLinesWithShape")
    }
    pub fn set_scale_lines_with_shape(&self, v: bool) -> bool {
        self.put_bool("ScaleLinesWithShape", v)
    }

    pub fn use_world_coordinates(&self) -> Option<bool> {
        self.prop_bool("UseWorldCoordinates")
    }
    pub fn set_use_world_coordinates(&self, v: bool) -> bool {
        self.put_bool("UseWorldCoordinates", v)
    }

    // ---- 搴?/ 鍚嶇О ----

    pub fn library_name(&self) -> Option<String> {
        self.prop_string("LibraryName")
    }

    pub fn hatch_name(&self) -> Option<String> {
        self.prop_string("HatchName")
    }

    pub fn is_from_library(&self) -> Option<bool> { self.prop_bool("IsFromLibrary") }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    pub fn library(&self) -> Option<IvgHatchLibrary> {
        self.prop_dispatch("Library").map(IvgHatchLibrary::new)
    }

    // ---- 缂╂斁 ----

    pub fn fill_scale_x(&self) -> Option<f64> { self.prop_f64("FillScaleX") }
    pub fn set_fill_scale_x(&self, v: f64) -> bool { self.put_f64("FillScaleX", v) }

    pub fn fill_scale_y(&self) -> Option<f64> { self.prop_f64("FillScaleY") }
    pub fn set_fill_scale_y(&self, v: f64) -> bool { self.put_f64("FillScaleY", v) }

    pub fn rotation_angle(&self) -> Option<f64> {
        self.prop_f64("RotationAngle")
    }
    pub fn set_rotation_angle(&self, v: f64) -> bool {
        self.put_f64("RotationAngle", v)
    }

    pub fn skew_angle(&self) -> Option<f64> { self.prop_f64("SkewAngle") }
    pub fn set_skew_angle(&self, v: f64) -> bool { self.put_f64("SkewAngle", v) }

    pub fn set_fill_scale(&self, v: f64) -> bool {
        let args = vec![Variant::from_f64(v)];
        self.disp.invoke_method("SetFillScale", args).is_ok()
    }

    pub fn get_fill_scale(&self) -> Option<f64> {
        self.disp.invoke_method("GetFillScale", vec![]).ok()?.to_f64().ok()
    }

    pub fn set_line_scale(&self, v: f64) -> bool {
        let args = vec![Variant::from_f64(v)];
        self.disp.invoke_method("SetLineScale", args).is_ok()
    }

    pub fn get_line_scale(&self) -> Option<f64> {
        self.disp.invoke_method("GetLineScale", vec![]).ok()?.to_f64().ok()
    }

    // ---- 鍥炬 ----

    pub fn patterns(&self) -> Option<IvgHatchPatterns> {
        self.prop_dispatch("Patterns").map(IvgHatchPatterns::new)
    }

    /// 娣诲姞鍥炬銆?
    #[allow(clippy::too_many_arguments)]
    pub fn add_pattern(
        &self,
        angle: f64,
        spacing: f64,
        shift: f64,
        origin_x: f64,
        origin_y: f64,
        width: f64,
        color: &IvgColor,
        style: &IvgOutlineStyle,
        dash_dot_length: f64,
        pen_width: f64,
    ) -> Option<IvgHatchPattern> {
        let args = vec![
            Variant::from_f64(angle),
            Variant::from_f64(spacing),
            Variant::from_f64(shift),
            Variant::from_f64(origin_x),
            Variant::from_f64(origin_y),
            Variant::from_f64(width),
            color.as_variant(),
            style.as_variant(),
            Variant::from_f64(dash_dot_length),
            Variant::from_f64(pen_width),
        ];
        self.disp
            .invoke_method("AddPattern", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgHatchPattern::new)
    }

    /// 淇濆瓨鍒板墫闈㈢嚎搴撱€?
    pub fn add_to_library(
        &self,
        library_name: impl Into<String>,
        hatch_name: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(library_name.into()),
            Variant::from_str(hatch_name.into()),
        ];
        self.disp.invoke_method("AddToLibrary", args).is_ok()
    }

    /// 浠庡墫闈㈢嚎搴撻€夋嫨銆?
    #[allow(clippy::too_many_arguments)]
    pub fn select(
        &self,
        library_name: impl Into<String>,
        hatch_name_or_index: Variant,
        back_color: &IvgColor,
        transform_with_shape: bool,
        scale_lines_with_shape: bool,
        use_world_coordinates: bool,
        fill_scale: f64,
        line_scale: f64,
        fill_angle: f64,
        fill_skew: f64,
    ) -> bool {
        let args = vec![
            Variant::from_str(library_name.into()),
            hatch_name_or_index,
            back_color.as_variant(),
            Variant::from_bool(transform_with_shape),
            Variant::from_bool(scale_lines_with_shape),
            Variant::from_bool(use_world_coordinates),
            Variant::from_f64(fill_scale),
            Variant::from_f64(line_scale),
            Variant::from_f64(fill_angle),
            Variant::from_f64(fill_skew),
        ];
        self.disp.invoke_method("Select", args).is_ok()
    }
}

// =============================================================
// IvgHatchPatterns
// =============================================================

pub struct IvgHatchPatterns {
    disp: ComObject,
}

impl IvgHatchPatterns {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgHatchPattern> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgHatchPattern::new)
    }

    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }
}

// =============================================================
// IvgHatchPattern
// =============================================================

pub struct IvgHatchPattern {
    disp: ComObject,
}

impl IvgHatchPattern {
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

    // fn put_i64(&self, name: &str, v: i64) -> bool {
    //     let arg = Variant::from_i64(v);
    //     self.disp.set_property(name, vec![arg]).is_ok()
    // }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 鍩烘湰灞炴€?----

    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }

    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    pub fn spacing(&self) -> Option<f64> { self.prop_f64("Spacing") }
    pub fn set_spacing(&self, v: f64) -> bool { self.put_f64("Spacing", v) }

    pub fn shift(&self) -> Option<f64> { self.prop_f64("Shift") }
    pub fn set_shift(&self, v: f64) -> bool { self.put_f64("Shift", v) }

    pub fn shift_percent(&self) -> Option<f64> { self.prop_f64("ShiftPercent") }
    pub fn set_shift_percent(&self, v: f64) -> bool {
        self.put_f64("ShiftPercent", v)
    }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 鍥炬鐨勮疆寤擄紙绾挎潯灞炴€э級銆?
    pub fn outline(&self) -> Option<crate::outline::IvgOutline> {
        self.prop_dispatch("Outline").map(crate::outline::IvgOutline::new)
    }

    // ---- 鎿嶄綔 ----

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    pub fn set_origin(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetOrigin", args).is_ok()
    }

    pub fn get_origin(&self) -> Option<(f64, f64)> {
        // 鍙?out 鍙傛暟锛岀敤灞炴€т唬鏇?
        Some((self.origin_x()?, self.origin_y()?))
    }

    /// 涓€娆℃€ц缃€?
    #[allow(clippy::too_many_arguments)]
    pub fn set_properties(
        &self,
        angle: Variant,
        spacing: f64,
        shift: Variant,
        origin_x: Variant,
        origin_y: Variant,
        width: f64,
        color: &IvgColor,
        style: &IvgOutlineStyle,
        dash_dot_length: f64,
        pen_width: f64,
    ) -> bool {
        let args = vec![
            angle,
            Variant::from_f64(spacing),
            shift,
            origin_x,
            origin_y,
            Variant::from_f64(width),
            color.as_variant(),
            style.as_variant(),
            Variant::from_f64(dash_dot_length),
            Variant::from_f64(pen_width),
        ];
        self.disp.invoke_method("SetProperties", args).is_ok()
    }

    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }
}

// =============================================================
// IvgHatchLibrary
// =============================================================

pub struct IvgHatchLibrary {
    disp: ComObject,
}

impl IvgHatchLibrary {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn display_name(&self) -> Option<String> {
        self.prop_string("DisplayName")
    }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    pub fn active(&self) -> Option<bool> { self.prop_bool("Active") }

    pub fn fills(&self) -> Option<IvgHatchFills> {
        self.prop_dispatch("Fills").map(IvgHatchFills::new)
    }

    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }
}

// =============================================================
// IvgHatchLibraries
// =============================================================

pub struct IvgHatchLibraries {
    disp: ComObject,
}

impl IvgHatchLibraries {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgHatchLibrary> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgHatchLibrary::new)
    }

    pub fn find(&self, name: impl Into<String>) -> Option<IvgHatchLibrary> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgHatchLibrary::new)
    }

    pub fn active_library(&self) -> Option<IvgHatchLibrary> {
        self.prop_dispatch("ActiveLibrary").map(IvgHatchLibrary::new)
    }

    pub fn default_library(&self) -> Option<IvgHatchLibrary> {
        self.prop_dispatch("DefaultLibrary").map(IvgHatchLibrary::new)
    }
}

// =============================================================
// IvgHatchFills
// =============================================================

pub struct IvgHatchFills {
    disp: ComObject,
}

impl IvgHatchFills {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index_or_name: Variant) -> Option<crate::fill::IvgFill> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgFill::new)
    }

    pub fn find(&self, name: impl Into<String>) -> Option<crate::fill::IvgFill> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgFill::new)
    }
}
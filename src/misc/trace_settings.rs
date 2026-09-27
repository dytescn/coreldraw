//! `IVGTraceSettings` 鈥斺€?浣嶅浘鎻忔懝璁剧疆

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::shape::IvgShapeRange;

pub struct IvgTraceSettings {
    disp: ComObject,
}

impl IvgTraceSettings {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_i16(&self, name: &str) -> Option<i16> {
        self.disp.get_property(name).ok()?.to_i64().ok().map(|v| v as i16)
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

    fn put_i16(&self, name: &str, v: i16) -> bool {
        let arg = Variant::from_i64(v as i64);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 绫诲瀷 / 骞虫粦 ----

    /// `cdrTraceType`
    pub fn trace_type(&self) -> Option<i64> { self.prop_i64("TraceType") }
    pub fn set_trace_type(&self, v: i32) -> bool { self.put_i64("TraceType", v as i64) }

    pub fn smoothing(&self) -> Option<i16> { self.prop_i16("Smoothing") }
    pub fn set_smoothing(&self, v: i16) -> bool { self.put_i16("Smoothing", v) }

    pub fn detail_level(&self) -> Option<i16> { self.prop_i16("DetailLevel") }
    pub fn set_detail_level(&self, v: i16) -> bool { self.put_i16("DetailLevel", v) }

    pub fn detail_level_percent(&self) -> Option<i16> {
        self.prop_i16("DetailLevelPercent")
    }
    pub fn set_detail_level_percent(&self, v: i16) -> bool {
        self.put_i16("DetailLevelPercent", v)
    }

    pub fn max_detail_level(&self) -> Option<i16> { self.prop_i16("MaxDetailLevel") }
    pub fn min_detail_level(&self) -> Option<i16> { self.prop_i16("MinDetailLevel") }

    pub fn corner_smoothness(&self) -> Option<i16> { self.prop_i16("CornerSmoothness") }
    pub fn set_corner_smoothness(&self, v: i16) -> bool {
        self.put_i16("CornerSmoothness", v)
    }

    // ---- 棰滆壊 ----

    /// `cdrColorType`
    pub fn color_mode(&self) -> Option<i64> { self.prop_i64("ColorMode") }

    /// `cdrPaletteID`
    pub fn palette_id(&self) -> Option<i64> { self.prop_i64("PaletteID") }

    pub fn color_count(&self) -> Option<i64> { self.prop_i64("ColorCount") }

    pub fn color(&self, index: i32) -> Option<IvgColor> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Color", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColor::new)
    }

    pub fn set_color_count(&self, color_count: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(color_count as i64)];
        self.disp.invoke_method("SetColorCount", args).ok()?.to_i64().ok()
    }

    pub fn set_color_mode(&self, color_mode: i32, palette_id: i32) -> bool {
        let args = vec![
            Variant::from_i64(color_mode as i64),
            Variant::from_i64(palette_id as i64),
        ];
        self.disp.invoke_method("SetColorMode", args).is_ok()
    }

    // ---- 鑳屾櫙 ----

    pub fn delete_original_object(&self) -> Option<bool> {
        self.prop_bool("DeleteOriginalObject")
    }
    pub fn set_delete_original_object(&self, v: bool) -> bool {
        self.put_bool("DeleteOriginalObject", v)
    }

    pub fn remove_background(&self) -> Option<bool> {
        self.prop_bool("RemoveBackground")
    }
    pub fn set_remove_background(&self, v: bool) -> bool {
        self.put_bool("RemoveBackground", v)
    }

    pub fn remove_entire_back_color(&self) -> Option<bool> {
        self.prop_bool("RemoveEntireBackColor")
    }
    pub fn set_remove_entire_back_color(&self, v: bool) -> bool {
        self.put_bool("RemoveEntireBackColor", v)
    }

    /// `cdrTraceBackgroundMode`
    pub fn background_removal_mode(&self) -> Option<i64> {
        self.prop_i64("BackgroundRemovalMode")
    }
    pub fn set_background_removal_mode(&self, v: i32) -> bool {
        self.put_i64("BackgroundRemovalMode", v as i64)
    }

    pub fn background_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("BackgroundColor").map(IvgColor::new)
    }

    // ---- 鍚堝苟 / 鍒嗙粍 ----

    pub fn merge_adjacent_objects(&self) -> Option<bool> {
        self.prop_bool("MergeAdjacentObjects")
    }
    pub fn set_merge_adjacent_objects(&self, v: bool) -> bool {
        self.put_bool("MergeAdjacentObjects", v)
    }

    pub fn remove_overlap(&self) -> Option<bool> { self.prop_bool("RemoveOverlap") }
    pub fn set_remove_overlap(&self, v: bool) -> bool {
        self.put_bool("RemoveOverlap", v)
    }

    pub fn group_objects_by_color(&self) -> Option<bool> {
        self.prop_bool("GroupObjectsByColor")
    }
    pub fn set_group_objects_by_color(&self, v: bool) -> bool {
        self.put_bool("GroupObjectsByColor", v)
    }

    // ---- 缁熻 ----

    pub fn curve_count(&self) -> Option<i64> { self.prop_i64("CurveCount") }
    pub fn node_count(&self) -> Option<i64> { self.prop_i64("NodeCount") }
    pub fn bitmap_width(&self) -> Option<i64> { self.prop_i64("BitmapWidth") }
    pub fn bitmap_height(&self) -> Option<i64> { self.prop_i64("BitmapHeight") }

    // ---- 鎿嶄綔 ----

    pub fn apply_changes(&self) -> bool {
        self.disp.invoke_method("ApplyChanges", vec![]).is_ok()
    }

    pub fn finish(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("Finish", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn show_dialog(&self, parent_window_handle: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(parent_window_handle as i64)];
        self.disp
            .invoke_method("ShowDialog", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
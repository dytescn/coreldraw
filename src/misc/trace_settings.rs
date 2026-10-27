//! `IVGTraceSettings` —— 位图描摹设置

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::shape::IvgShapeRange;

/// CorelDRAW `IVGTraceSettings` 对象的后期绑定包装（描摹设置集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTraceSettings {
    disp: ComObject,
}

impl IvgTraceSettings {
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

    // ---- 类型 / 平滑 ----

    /// `cdrTraceType`
    pub fn trace_type(&self) -> Option<i64> { self.prop_i64("TraceType") }
    /// 设置描摹类型（`TraceType`）。
    pub fn set_trace_type(&self, v: i32) -> bool { self.put_i64("TraceType", v as i64) }

    /// 读取平滑（`Smoothing`）。
    pub fn smoothing(&self) -> Option<i16> { self.prop_i16("Smoothing") }
    /// 设置平滑（`Smoothing`）。
    pub fn set_smoothing(&self, v: i16) -> bool { self.put_i16("Smoothing", v) }

    /// 读取细节级别（`DetailLevel`）。
    pub fn detail_level(&self) -> Option<i16> { self.prop_i16("DetailLevel") }
    /// 设置细节级别（`DetailLevel`）。
    pub fn set_detail_level(&self, v: i16) -> bool { self.put_i16("DetailLevel", v) }

    /// 读取细节级别百分比（`DetailLevelPercent`）。
    pub fn detail_level_percent(&self) -> Option<i16> {
        self.prop_i16("DetailLevelPercent")
    }
    /// 设置细节级别百分比（`DetailLevelPercent`）。
    pub fn set_detail_level_percent(&self, v: i16) -> bool {
        self.put_i16("DetailLevelPercent", v)
    }

    /// 读取最大细节级别（`MaxDetailLevel`）。
    pub fn max_detail_level(&self) -> Option<i16> { self.prop_i16("MaxDetailLevel") }
    /// 读取最小细节级别（`MinDetailLevel`）。
    pub fn min_detail_level(&self) -> Option<i16> { self.prop_i16("MinDetailLevel") }

    /// 读取 COM 属性 `CornerSmoothness`。
    pub fn corner_smoothness(&self) -> Option<i16> { self.prop_i16("CornerSmoothness") }
    /// 设置 COM 属性 `CornerSmoothness`。
    pub fn set_corner_smoothness(&self, v: i16) -> bool {
        self.put_i16("CornerSmoothness", v)
    }

    // ---- 颜色 ----

    /// `cdrColorType`
    pub fn color_mode(&self) -> Option<i64> { self.prop_i64("ColorMode") }

    /// `cdrPaletteID`
    pub fn palette_id(&self) -> Option<i64> { self.prop_i64("PaletteID") }

    /// 读取颜色数量（`ColorCount`）。
    pub fn color_count(&self) -> Option<i64> { self.prop_i64("ColorCount") }

    /// 颜色（COM 方法 `Color`）。
    pub fn color(&self, index: i32) -> Option<IvgColor> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Color", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColor::new)
    }

    /// 设置颜色数量（`SetColorCount`）。
    pub fn set_color_count(&self, color_count: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(color_count as i64)];
        self.disp.invoke_method("SetColorCount", args).ok()?.to_i64().ok()
    }

    /// 设置颜色模式（`SetColorMode`）。
    pub fn set_color_mode(&self, color_mode: i32, palette_id: i32) -> bool {
        let args = vec![
            Variant::from_i64(color_mode as i64),
            Variant::from_i64(palette_id as i64),
        ];
        self.disp.invoke_method("SetColorMode", args).is_ok()
    }

    // ---- 背景 ----

    /// 读取删除原始对象（`DeleteOriginalObject`）。
    pub fn delete_original_object(&self) -> Option<bool> {
        self.prop_bool("DeleteOriginalObject")
    }
    /// 设置删除原始对象（`DeleteOriginalObject`）。
    pub fn set_delete_original_object(&self, v: bool) -> bool {
        self.put_bool("DeleteOriginalObject", v)
    }

    /// 读取移除背景（`RemoveBackground`）。
    pub fn remove_background(&self) -> Option<bool> {
        self.prop_bool("RemoveBackground")
    }
    /// 设置移除背景（`RemoveBackground`）。
    pub fn set_remove_background(&self, v: bool) -> bool {
        self.put_bool("RemoveBackground", v)
    }

    /// 读取 COM 属性 `RemoveEntireBackColor`。
    pub fn remove_entire_back_color(&self) -> Option<bool> {
        self.prop_bool("RemoveEntireBackColor")
    }
    /// 设置 COM 属性 `RemoveEntireBackColor`。
    pub fn set_remove_entire_back_color(&self, v: bool) -> bool {
        self.put_bool("RemoveEntireBackColor", v)
    }

    /// `cdrTraceBackgroundMode`
    pub fn background_removal_mode(&self) -> Option<i64> {
        self.prop_i64("BackgroundRemovalMode")
    }
    /// 设置 COM 属性 `BackgroundRemovalMode`。
    pub fn set_background_removal_mode(&self, v: i32) -> bool {
        self.put_i64("BackgroundRemovalMode", v as i64)
    }

    /// 读取背景颜色（`BackgroundColor`）。
    pub fn background_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("BackgroundColor").map(IvgColor::new)
    }

    // ---- 合并 / 分组 ----

    /// 读取合并相邻对象集（`MergeAdjacentObjects`）。
    pub fn merge_adjacent_objects(&self) -> Option<bool> {
        self.prop_bool("MergeAdjacentObjects")
    }
    /// 设置合并相邻对象集（`MergeAdjacentObjects`）。
    pub fn set_merge_adjacent_objects(&self, v: bool) -> bool {
        self.put_bool("MergeAdjacentObjects", v)
    }

    /// 读取移除重叠（`RemoveOverlap`）。
    pub fn remove_overlap(&self) -> Option<bool> { self.prop_bool("RemoveOverlap") }
    /// 设置移除重叠（`RemoveOverlap`）。
    pub fn set_remove_overlap(&self, v: bool) -> bool {
        self.put_bool("RemoveOverlap", v)
    }

    /// 读取群组对象集颜色（`GroupObjectsByColor`）。
    pub fn group_objects_by_color(&self) -> Option<bool> {
        self.prop_bool("GroupObjectsByColor")
    }
    /// 设置群组对象集颜色（`GroupObjectsByColor`）。
    pub fn set_group_objects_by_color(&self, v: bool) -> bool {
        self.put_bool("GroupObjectsByColor", v)
    }

    // ---- 统计 ----

    /// 读取曲线数量（`CurveCount`）。
    pub fn curve_count(&self) -> Option<i64> { self.prop_i64("CurveCount") }
    /// 读取节点数量（`NodeCount`）。
    pub fn node_count(&self) -> Option<i64> { self.prop_i64("NodeCount") }
    /// 读取位图宽度（`BitmapWidth`）。
    pub fn bitmap_width(&self) -> Option<i64> { self.prop_i64("BitmapWidth") }
    /// 读取位图高度（`BitmapHeight`）。
    pub fn bitmap_height(&self) -> Option<i64> { self.prop_i64("BitmapHeight") }

    // ---- 操作 ----

    /// 调用 COM 方法 `ApplyChanges`。
    pub fn apply_changes(&self) -> bool {
        self.disp.invoke_method("ApplyChanges", vec![]).is_ok()
    }

    /// 完成（COM 方法 `Finish`）。
    pub fn finish(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("Finish", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 显示对话框（COM 方法 `ShowDialog`）。
    pub fn show_dialog(&self, parent_window_handle: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(parent_window_handle as i64)];
        self.disp
            .invoke_method("ShowDialog", args)
            .ok()?
            .to_bool()
            .ok()
    }
}
//! `IVGShapeRange` 鈥斺€?鍥惧舰鑼冨洿

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::geometry::IvgRect;
use crate::layer::IvgLayer;
use crate::outline::IvgArrowHead;
use crate::shape::{IvgShape, IvgShapes};

pub struct IvgShapeRange {
    disp: ComObject,
}

impl IvgShapeRange {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // fn put_i64(&self, name: &str, v: i64) -> bool {
    //     let arg = Variant::from_i64(v);
    //     self.disp.set_property(name, vec![arg]).is_ok()
    // }

    // ---- 鍩烘湰 ----

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }
    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }
    pub fn first_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("FirstShape").map(IvgShape::new)
    }
    pub fn last_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("LastShape").map(IvgShape::new)
    }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgShape> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgShape> {
        self.item(Variant::from_i64(i as i64))
    }

    // ---- 鍑犱綍 ----

    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    pub fn set_position_x(&self, v: f64) -> bool { self.put_f64("PositionX", v) }

    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    pub fn set_position_y(&self, v: f64) -> bool { self.put_f64("PositionY", v) }

    pub fn size_width(&self) -> Option<f64> { self.prop_f64("SizeWidth") }
    pub fn set_size_width(&self, v: f64) -> bool { self.put_f64("SizeWidth", v) }

    pub fn size_height(&self) -> Option<f64> { self.prop_f64("SizeHeight") }
    pub fn set_size_height(&self, v: f64) -> bool { self.put_f64("SizeHeight", v) }

    pub fn left_x(&self) -> Option<f64> { self.prop_f64("LeftX") }
    pub fn right_x(&self) -> Option<f64> { self.prop_f64("RightX") }
    pub fn top_y(&self) -> Option<f64> { self.prop_f64("TopY") }
    pub fn bottom_y(&self) -> Option<f64> { self.prop_f64("BottomY") }

    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    pub fn set_center_x(&self, v: f64) -> bool { self.put_f64("CenterX", v) }

    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }
    pub fn set_center_y(&self, v: f64) -> bool { self.put_f64("CenterY", v) }

    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    // ---- 鍙樻崲 ----

    pub fn move_by(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    pub fn rotate(&self, angle: f64) -> bool {
        let args = vec![Variant::from_f64(angle)];
        self.disp.invoke_method("Rotate", args).is_ok()
    }

    pub fn rotate_ex(&self, angle: f64, cx: f64, cy: f64) -> bool {
        let args = vec![
            Variant::from_f64(angle),
            Variant::from_f64(cx),
            Variant::from_f64(cy),
        ];
        self.disp.invoke_method("RotateEx", args).is_ok()
    }

    pub fn skew(&self, ax: f64, ay: f64) -> bool {
        let args = vec![
            Variant::from_f64(ax),
            Variant::from_f64(ay),
        ];
        self.disp.invoke_method("Skew", args).is_ok()
    }

    pub fn set_position(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetPosition", args).is_ok()
    }

    pub fn set_size(&self, w: f64, h: f64) -> bool {
        let args = vec![
            Variant::from_f64(w),
            Variant::from_f64(h),
        ];
        self.disp.invoke_method("SetSize", args).is_ok()
    }

    pub fn flip(&self, axes: i32) -> bool {
        let args = vec![Variant::from_i64(axes as i64)];
        self.disp.invoke_method("Flip", args).is_ok()
    }

    pub fn stretch(&self, sx: f64, sy: f64, stretch_chars: bool) -> bool {
        let args = vec![
            Variant::from_f64(sx),
            Variant::from_f64(sy),
            Variant::from_bool(stretch_chars),
        ];
        self.disp.invoke_method("Stretch", args).is_ok()
    }

    // ---- 澶嶅埗 / 鍒犻櫎 ----

    pub fn duplicate(&self, dx: f64, dy: f64) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("Duplicate", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn clone(&self, dx: f64, dy: f64) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("Clone", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    pub fn copy(&self) -> bool {
        self.disp.invoke_method("Copy", vec![]).is_ok()
    }

    pub fn cut(&self) -> bool {
        self.disp.invoke_method("Cut", vec![]).is_ok()
    }

    // ---- 缇ょ粍 ----

    pub fn group(&self) -> Option<IvgShape> {
        self.disp
            .invoke_method("Group", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn ungroup(&self) -> bool {
        self.disp.invoke_method("Ungroup", vec![]).is_ok()
    }

    pub fn ungroup_all(&self) -> bool {
        self.disp.invoke_method("UngroupAll", vec![]).is_ok()
    }

    pub fn ungroup_ex(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("UngroupEx", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn ungroup_all_ex(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("UngroupAllEx", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn combine(&self) -> Option<IvgShape> {
        self.disp
            .invoke_method("Combine", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn break_apart(&self) -> bool {
        self.disp.invoke_method("BreakApart", vec![]).is_ok()
    }

    // ---- Z 椤哄簭 ----

    pub fn order_to_front(&self) -> bool {
        self.disp.invoke_method("OrderToFront", vec![]).is_ok()
    }
    pub fn order_to_back(&self) -> bool {
        self.disp.invoke_method("OrderToBack", vec![]).is_ok()
    }
    pub fn order_reverse(&self) -> bool {
        self.disp.invoke_method("OrderReverse", vec![]).is_ok()
    }

    // ---- 闆嗗悎鎿嶄綔 ----

    pub fn add(&self, shape: &IvgShape) -> bool {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("Add", args).is_ok()
    }
    pub fn add_range(&self, other: &IvgShapeRange) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("AddRange", args).is_ok()
    }
    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }
    pub fn remove_range(&self, other: &IvgShapeRange) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("RemoveRange", args).is_ok()
    }
    pub fn remove_all(&self) -> bool {
        self.disp.invoke_method("RemoveAll", vec![]).is_ok()
    }

    pub fn index_of(&self, shape: &IvgShape) -> Option<i64> {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("IndexOf", args).ok()?.to_i64().ok()
    }

    pub fn exists(&self, shape: &IvgShape) -> Option<bool> {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("Exists", args).ok()?.to_bool().ok()
    }

    // ---- 閫夋嫨 ----

    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }
    pub fn add_to_selection(&self) -> bool {
        self.disp.invoke_method("AddToSelection", vec![]).is_ok()
    }
    pub fn remove_from_selection(&self) -> bool {
        self.disp.invoke_method("RemoveFromSelection", vec![]).is_ok()
    }

    // ---- 濉厖 / 杞粨 ----

    pub fn apply_no_fill(&self) -> bool {
        self.disp.invoke_method("ApplyNoFill", vec![]).is_ok()
    }
    pub fn apply_uniform_fill(&self, color: &IvgColor) -> bool {
        let args = vec![color.as_variant()];
        self.disp.invoke_method("ApplyUniformFill", args).is_ok()
    }

    pub fn apply_outline(&self, outline: &crate::outline::IvgOutline) -> bool {
        let args = vec![outline.as_variant()];
        self.disp.invoke_method("ApplyOutline", args).is_ok()
    }
    pub fn apply_fill(&self, fill: &crate::fill::IvgFill) -> bool {
        let args = vec![fill.as_variant()];
        self.disp.invoke_method("ApplyFill", args).is_ok()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn set_outline_properties(
        &self,
        width: f64,
        style: &crate::outline::IvgOutlineStyle,
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
        self.disp.invoke_method("SetOutlineProperties", args).is_ok()
    }

    // ---- 鍙樻崲 / 杞崲 ----

    pub fn convert_to_curves(&self) -> bool {
        self.disp.invoke_method("ConvertToCurves", vec![]).is_ok()
    }

    pub fn convert_to_bitmap(
        &self,
        bit_depth: i32, grayscale: bool, dithered: bool, transparent_bg: bool,
        resolution: i32, anti_aliasing: i32, use_color_profile: bool,
        multi_channel: bool, always_overprint_black: bool, overprint_black_limit: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_i64(bit_depth as i64),
            Variant::from_bool(grayscale),
            Variant::from_bool(dithered),
            Variant::from_bool(transparent_bg),
            Variant::from_i64(resolution as i64),
            Variant::from_i64(anti_aliasing as i64),
            Variant::from_bool(use_color_profile),
            Variant::from_bool(multi_channel),
            Variant::from_bool(always_overprint_black),
            Variant::from_i64(overprint_black_limit as i64),
        ];
        self.disp
            .invoke_method("ConvertToBitmap", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---- 鏍峰紡 ----

    pub fn apply_style(&self, style_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(style_name.into())];
        self.disp.invoke_method("ApplyStyle", args).is_ok()
    }

    // ---- 灞?----

    pub fn move_to_layer(&self, l: &IvgLayer) -> bool {
        let args = vec![l.as_variant()];
        self.disp.invoke_method("MoveToLayer", args).is_ok()
    }

    pub fn copy_to_layer(&self, l: &IvgLayer) -> Option<IvgShapeRange> {
        let args = vec![l.as_variant()];
        self.disp
            .invoke_method("CopyToLayer", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    // ---- 瀵归綈 / 鍒嗗竷 ----

    pub fn align_to_shape(&self, align_type: i32, shape: &IvgShape, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            shape.as_variant(),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToShape", args).is_ok()
    }

    pub fn align_to_page(&self, align_type: i32, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToPage", args).is_ok()
    }

    pub fn align_to_page_center(&self, align_type: i32, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToPageCenter", args).is_ok()
    }

    pub fn align_to_grid(&self, align_type: i32, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToGrid", args).is_ok()
    }

    pub fn align_to_point(&self, align_type: i32, x: f64, y: f64, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToPoint", args).is_ok()
    }

    pub fn distribute(&self, dist_type: i32, page_extent: bool) -> bool {
        let args = vec![
            Variant::from_i64(dist_type as i64),
            Variant::from_bool(page_extent),
        ];
        self.disp.invoke_method("Distribute", args).is_ok()
    }

    // ---- 鍏跺畠 ----

    pub fn set_bounding_box(
        &self,
        x: f64, y: f64, w: f64, h: f64,
        keep_aspect: bool, ref_point: i32,
    ) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(w),
            Variant::from_f64(h),
            Variant::from_bool(keep_aspect),
            Variant::from_i64(ref_point as i64),
        ];
        self.disp.invoke_method("SetBoundingBox", args).is_ok()
    }

    pub fn lock(&self) -> bool { self.disp.invoke_method("Lock", vec![]).is_ok() }
    pub fn unlock(&self) -> bool { self.disp.invoke_method("Unlock", vec![]).is_ok() }

    pub fn clear_transformations(&self) -> bool {
        self.disp.invoke_method("ClearTransformations", vec![]).is_ok()
    }

    pub fn flatten_effects(&self) -> bool {
        self.disp.invoke_method("FlattenEffects", vec![]).is_ok()
    }
}
//! `IVGSegment` / `IVGSegments` 鈥斺€?绾挎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{IvgCrossPoints, IvgNode, IvgSegmentRange, IvgSubPath};

// =============================================================
// IvgSegments
// =============================================================

pub struct IvgSegments {
    disp: ComObject,
}

impl IvgSegments {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgSegment> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    pub fn first(&self) -> Option<IvgSegment> {
        self.disp.get_property("First").ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    pub fn last(&self) -> Option<IvgSegment> {
        self.disp.get_property("Last").ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    pub fn range(&self, indices:Variant ) -> Option<IvgSegmentRange> {
        let args = vec![indices];
        self.disp.invoke_method("Range", args).ok()?.to_idispatch().ok().map(IvgSegmentRange::new)
    }

    pub fn all(&self) -> Option<IvgSegmentRange> {
        self.disp.invoke_method("All", vec![]).ok()?.to_idispatch().ok().map(IvgSegmentRange::new)
    }
}

// =============================================================
// IvgSegment
// =============================================================

pub struct IvgSegment {
    disp: ComObject,
}

impl IvgSegment {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

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

    // ---- 绫诲瀷 / 绱㈠紩 ----

    /// `cdrSegmentType` 瑙?`enums::curve`
    pub fn segment_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_segment_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    pub fn sub_path_index(&self) -> Option<i64> { self.prop_i64("SubPathIndex") }
    pub fn absolute_index(&self) -> Option<i64> { self.prop_i64("AbsoluteIndex") }
    pub fn length(&self) -> Option<f64> { self.prop_f64("Length") }

    pub fn is_selected(&self) -> Option<bool> { self.prop_bool("Selected") }
    pub fn set_selected(&self, v: bool) -> bool { self.put_bool("Selected", v) }

    // ---- 鍏宠仈 ----

    pub fn sub_path(&self) -> Option<IvgSubPath> {
        self.prop_dispatch("SubPath").map(IvgSubPath::new)
    }

    pub fn start_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("StartNode").map(IvgNode::new)
    }

    pub fn end_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("EndNode").map(IvgNode::new)
    }

    // ---- 鎺у埗鐐癸紙瑙掑害 + 闀垮害锛?----

    pub fn starting_control_point_length(&self) -> Option<f64> {
        self.prop_f64("StartingControlPointLength")
    }
    pub fn set_starting_control_point_length(&self, v: f64) -> bool {
        self.put_f64("StartingControlPointLength", v)
    }

    pub fn starting_control_point_angle(&self) -> Option<f64> {
        self.prop_f64("StartingControlPointAngle")
    }
    pub fn set_starting_control_point_angle(&self, v: f64) -> bool {
        self.put_f64("StartingControlPointAngle", v)
    }

    pub fn ending_control_point_length(&self) -> Option<f64> {
        self.prop_f64("EndingControlPointLength")
    }
    pub fn set_ending_control_point_length(&self, v: f64) -> bool {
        self.put_f64("EndingControlPointLength", v)
    }

    pub fn ending_control_point_angle(&self) -> Option<f64> {
        self.prop_f64("EndingControlPointAngle")
    }
    pub fn set_ending_control_point_angle(&self, v: f64) -> bool {
        self.put_f64("EndingControlPointAngle", v)
    }

    // ---- 鎺у埗鐐癸紙x + y锛?----

    pub fn starting_control_point_x(&self) -> Option<f64> {
        self.prop_f64("StartingControlPointX")
    }
    pub fn set_starting_control_point_x(&self, v: f64) -> bool {
        self.put_f64("StartingControlPointX", v)
    }

    pub fn starting_control_point_y(&self) -> Option<f64> {
        self.prop_f64("StartingControlPointY")
    }
    pub fn set_starting_control_point_y(&self, v: f64) -> bool {
        self.put_f64("StartingControlPointY", v)
    }

    pub fn ending_control_point_x(&self) -> Option<f64> {
        self.prop_f64("EndingControlPointX")
    }
    pub fn set_ending_control_point_x(&self, v: f64) -> bool {
        self.put_f64("EndingControlPointX", v)
    }

    pub fn ending_control_point_y(&self) -> Option<f64> {
        self.prop_f64("EndingControlPointY")
    }
    pub fn set_ending_control_point_y(&self, v: f64) -> bool {
        self.put_f64("EndingControlPointY", v)
    }

    // ---- 閬嶅巻 ----

    pub fn next(&self) -> Option<IvgSegment> {
        self.disp.invoke_method("Next", vec![]).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    pub fn previous(&self) -> Option<IvgSegment> {
        self.disp.invoke_method("Previous", vec![]).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    // ---- 缂栬緫 ----

    pub fn get_copy(&self) -> Option<crate::curve::IvgCurve> {
        self.disp.invoke_method("GetCopy", vec![]).ok()?.to_idispatch().ok()
            .map(crate::curve::IvgCurve::new)
    }

    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }

    pub fn add_node_at(&self, offset: f64, offset_type: i32) -> Option<IvgNode> {
        let args = vec![
            Variant::from_f64(offset),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("AddNodeAt", args).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    pub fn break_apart_at(&self, offset: f64, offset_type: i32) -> Option<IvgNode> {
        let args = vec![
            Variant::from_f64(offset),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("BreakApartAt", args).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    // ---- 鍑犱綍鏌ヨ ----

    pub fn get_perpendicular_at(&self, offset: f64, offset_type: i32) -> Option<f64> {
        let args = vec![
            Variant::from_f64(offset),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("GetPerpendicularAt", args).ok()?.to_f64().ok()
    }

    pub fn get_tangent_at(&self, offset: f64, offset_type: i32) -> Option<f64> {
        let args = vec![
            Variant::from_f64(offset),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("GetTangentAt", args).ok()?.to_f64().ok()
    }

    pub fn get_curvature_at(&self, offset: f64, offset_type: i32) -> Option<f64> {
        let args = vec![
            Variant::from_f64(offset),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("GetCurvatureAt", args).ok()?.to_f64().ok()
    }

    pub fn get_intersections(&self, target: &IvgSegment, offset_type: i32) -> Option<IvgCrossPoints> {
        let args = vec![
            target.as_variant(),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("GetIntersections", args).ok()?.to_idispatch().ok().map(IvgCrossPoints::new)
    }

    pub fn is_rect_on_edge(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
        ];
        self.disp.invoke_method("IsRectOnEdge", args).ok()?.to_bool().ok()
    }
}
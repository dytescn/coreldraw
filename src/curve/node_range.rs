//! `IVGNodeRange` 鈥斺€?鑺傜偣鑼冨洿

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{IvgNode, IvgSegmentRange};
use crate::geometry::IvgRect;

pub struct IvgNodeRange {
    disp: ComObject,
}

impl IvgNodeRange {
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---- 鍩烘湰淇℃伅 ----

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }
    pub fn node_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    pub fn size_width(&self) -> Option<f64> { self.prop_f64("SizeWidth") }
    pub fn size_height(&self) -> Option<f64> { self.prop_f64("SizeHeight") }

    pub fn item(&self, index: i32) -> Option<IvgNode> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    pub fn first_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("FirstNode").map(IvgNode::new)
    }

    pub fn last_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("LastNode").map(IvgNode::new)
    }

    pub fn segment_range(&self) -> Option<IvgSegmentRange> {
        self.prop_dispatch("SegmentRange").map(IvgSegmentRange::new)
    }

    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    // ---- 缂栬緫 ----

    pub fn add(&self, node: &IvgNode) -> bool {
        let args = vec![node.as_variant()];
        self.disp.invoke_method("Add", args).is_ok()
    }

    pub fn add_range(&self, range: &IvgNodeRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("AddRange", args).is_ok()
    }

    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }

    pub fn remove_range(&self, range: &IvgNodeRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("RemoveRange", args).is_ok()
    }

    pub fn remove_all(&self) -> bool { self.disp.invoke_method("RemoveAll", vec![]).is_ok() }
    pub fn delete(&self) -> bool { self.disp.invoke_method("Delete", vec![]).is_ok() }

    pub fn move_by(&self, dx: f64, dy: f64, anchor: i32, elastic: bool) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
            Variant::from_i64(anchor as i64),
            Variant::from_bool(elastic),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    pub fn stretch(&self, rx: f32, ry: f32, use_anchor: bool, ax: f64, ay: f64) -> bool {
        let args = vec![
            Variant::from_f64(rx as f64),
            Variant::from_f64(ry as f64),
            Variant::from_bool(use_anchor),
            Variant::from_f64(ax),
            Variant::from_f64(ay),
        ];
        self.disp.invoke_method("Stretch", args).is_ok()
    }

    pub fn rotate(&self, angle: f64, use_center: bool, cx: f64, cy: f64) -> bool {
        let args = vec![
            Variant::from_f64(angle),
            Variant::from_bool(use_center),
            Variant::from_f64(cx),
            Variant::from_f64(cy),
        ];
        self.disp.invoke_method("Rotate", args).is_ok()
    }

    pub fn skew(&self, ax: f64, ay: f64, use_anchor: bool, sx: f64, sy: f64) -> bool {
        let args = vec![
            Variant::from_f64(ax),
            Variant::from_f64(ay),
            Variant::from_bool(use_anchor),
            Variant::from_f64(sx),
            Variant::from_f64(sy),
        ];
        self.disp.invoke_method("Skew", args).is_ok()
    }

    pub fn set_type(&self, node_type: i32) -> bool {
        let args = vec![Variant::from_i64(node_type as i64)];
        self.disp.invoke_method("SetType", args).is_ok()
    }

    pub fn auto_reduce(&self, margin: f64) -> bool {
        let args = vec![Variant::from_f64(margin)];
        self.disp.invoke_method("AutoReduce", args).is_ok()
    }

    pub fn smoothen(&self, smoothness: i32) -> bool {
        let args = vec![Variant::from_i64(smoothness as i64)];
        self.disp.invoke_method("Smoothen", args).is_ok()
    }

    pub fn break_apart(&self) -> bool { self.disp.invoke_method("BreakApart", vec![]).is_ok() }

    pub fn fillet(&self, radius: f64, combine: bool) -> bool {
        let args = vec![
            Variant::from_f64(radius),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Fillet", args).is_ok()
    }

    pub fn chamfer(&self, a: f64, b: f64, combine: bool) -> bool {
        let args = vec![
            Variant::from_f64(a),
            Variant::from_f64(b),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Chamfer", args).is_ok()
    }

    pub fn scallop(&self, radius: f64, combine: bool) -> bool {
        let args = vec![
            Variant::from_f64(radius),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Scallop", args).is_ok()
    }

    // ---- 閫夋嫨 ----

    pub fn create_selection(&self) -> bool { self.disp.invoke_method("CreateSelection", vec![]).is_ok() }
    pub fn add_to_selection(&self) -> bool { self.disp.invoke_method("AddToSelection", vec![]).is_ok() }
    pub fn remove_from_selection(&self) -> bool { self.disp.invoke_method("RemoveFromSelection", vec![]).is_ok() }
}
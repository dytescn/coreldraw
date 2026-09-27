//! `IVGNode` / `IVGNodes` 鈥斺€?鏇茬嚎鑺傜偣

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{IvgNodeRange, IvgSegment, IvgSubPath};

// =============================================================
// IvgNodes
// =============================================================

pub struct IvgNodes {
    disp: ComObject,
}

impl IvgNodes {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgNode> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    pub fn first(&self) -> Option<IvgNode> {
        self.disp.get_property("First").ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    pub fn last(&self) -> Option<IvgNode> {
        self.disp.get_property("Last").ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    /// 鎸夌储寮曟暟缁勫彇鑺傜偣鑼冨洿
    pub fn range(&self, indices: Variant) -> Option<IvgNodeRange> {
        let args = vec![indices];
        self.disp.invoke_method("Range", args).ok()?.to_idispatch().ok().map(IvgNodeRange::new)
    }

    pub fn all(&self) -> Option<IvgNodeRange> {
        self.disp.invoke_method("All", vec![]).ok()?.to_idispatch().ok().map(IvgNodeRange::new)
    }
}

// =============================================================
// IvgNode
// =============================================================

pub struct IvgNode {
    disp: ComObject,
}

impl IvgNode {
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

    // ---- 浣嶇疆 ----

    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    pub fn set_position_x(&self, v: f64) -> bool { self.put_f64("PositionX", v) }

    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    pub fn set_position_y(&self, v: f64) -> bool { self.put_f64("PositionY", v) }

    pub fn get_position(&self) -> Option<(f64, f64)> {
        Some((self.position_x()?, self.position_y()?))
    }

    pub fn set_position(&self, x: f64, y: f64) -> bool {
        self.set_position_x(x) && self.set_position_y(y)
    }

    pub fn move_by(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    // ---- 绫诲瀷 / 绱㈠紩 ----

    /// `cdrNodeType` 瑙?`enums::curve`
    pub fn node_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_node_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    pub fn sub_path_index(&self) -> Option<i64> { self.prop_i64("SubPathIndex") }
    pub fn absolute_index(&self) -> Option<i64> { self.prop_i64("AbsoluteIndex") }

    pub fn is_ending(&self) -> Option<bool> { self.prop_bool("IsEnding") }
    pub fn is_selected(&self) -> Option<bool> { self.prop_bool("Selected") }
    pub fn set_selected(&self, v: bool) -> bool { self.put_bool("Selected", v) }

    // ---- 鍏崇郴 ----

    pub fn sub_path(&self) -> Option<IvgSubPath> {
        self.prop_dispatch("SubPath").map(IvgSubPath::new)
    }

    pub fn segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("Segment").map(IvgSegment::new)
    }

    pub fn prev_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("PrevSegment").map(IvgSegment::new)
    }

    pub fn next_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("NextSegment").map(IvgSegment::new)
    }

    pub fn next(&self) -> Option<IvgNode> {
        self.disp.invoke_method("Next", vec![]).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    pub fn previous(&self) -> Option<IvgNode> {
        self.disp.invoke_method("Previous", vec![]).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    // ---- 鎿嶄綔 ----

    pub fn join_with(&self, target: &IvgNode) -> bool {
        let args = vec![target.as_variant()];
        self.disp.invoke_method("JoinWith", args).is_ok()
    }

    pub fn connect_with(&self, target: &IvgNode) -> bool {
        let args = vec![target.as_variant()];
        self.disp.invoke_method("ConnectWith", args).is_ok()
    }

    pub fn break_apart(&self) -> bool { self.disp.invoke_method("BreakApart", vec![]).is_ok() }
    pub fn delete(&self) -> bool { self.disp.invoke_method("Delete", vec![]).is_ok() }

    pub fn fillet(&self, radius: f64, combine: bool) -> Option<bool> {
        let args = vec![
            Variant::from_f64(radius),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Fillet", args).ok()?.to_bool().ok()
    }

    pub fn chamfer(&self, a: f64, b: f64, combine: bool) -> Option<bool> {
        let args = vec![
            Variant::from_f64(a),
            Variant::from_f64(b),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Chamfer", args).ok()?.to_bool().ok()
    }

    pub fn scallop(&self, radius: f64, combine: bool) -> Option<bool> {
        let args = vec![
            Variant::from_f64(radius),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Scallop", args).ok()?.to_bool().ok()
    }

    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }

    pub fn get_distance_from(&self, other: &IvgNode) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("GetDistanceFrom", args).ok()?.to_f64().ok()
    }

    pub fn extend_sub_paths(&self, other: &IvgNode, join: bool) -> Option<bool> {
        let args = vec![
            other.as_variant(),
            Variant::from_bool(join),
        ];
        self.disp.invoke_method("ExtendSubPaths", args).ok()?.to_bool().ok()
    }

    pub fn average_position_with(&self, other: &IvgNode, join: bool) -> bool {
        let args = vec![
            other.as_variant(),
            Variant::from_bool(join),
        ];
        self.disp.invoke_method("AveragePositionWith", args).is_ok()
    }
}
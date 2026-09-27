//! `IVGSubPath` / `IVGSubPaths` 鈥斺€?瀛愯矾寰?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{
    IvgCurve, IvgNode, IvgNodeRange, IvgNodes,
    IvgSegment, IvgSegments,
};
use crate::geometry::IvgRect;

// =============================================================
// IvgSubPaths
// =============================================================

pub struct IvgSubPaths {
    disp: ComObject,
}

impl IvgSubPaths {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgSubPath> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgSubPath::new)
    }

    pub fn first(&self) -> Option<IvgSubPath> {
        self.disp.get_property("First").ok()?.to_idispatch().ok().map(IvgSubPath::new)
    }

    pub fn last(&self) -> Option<IvgSubPath> {
        self.disp.get_property("Last").ok()?.to_idispatch().ok().map(IvgSubPath::new)
    }
}

// =============================================================
// IvgSubPath
// =============================================================

pub struct IvgSubPath {
    disp: ComObject,
}

impl IvgSubPath {
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

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 灞炴€?----

    pub fn nodes(&self) -> Option<IvgNodes> { self.prop_dispatch("Nodes").map(IvgNodes::new) }
    pub fn segments(&self) -> Option<IvgSegments> { self.prop_dispatch("Segments").map(IvgSegments::new) }
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    pub fn length(&self) -> Option<f64> { self.prop_f64("Length") }

    pub fn is_closed(&self) -> Option<bool> { self.prop_bool("Closed") }
    pub fn set_closed(&self, v: bool) -> bool { self.put_bool("Closed", v) }

    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    pub fn set_position_x(&self, v: f64) -> bool { self.put_f64("PositionX", v) }

    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    pub fn set_position_y(&self, v: f64) -> bool { self.put_f64("PositionY", v) }

    pub fn size_width(&self) -> Option<f64> { self.prop_f64("SizeWidth") }
    pub fn size_height(&self) -> Option<f64> { self.prop_f64("SizeHeight") }

    pub fn start_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("StartNode").map(IvgNode::new)
    }

    pub fn end_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("EndNode").map(IvgNode::new)
    }

    pub fn first_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("FirstSegment").map(IvgSegment::new)
    }

    pub fn last_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("LastSegment").map(IvgSegment::new)
    }

    pub fn is_clockwise(&self) -> Option<bool> { self.prop_bool("IsClockwise") }
    pub fn area(&self) -> Option<f64> { self.prop_f64("Area") }
    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    // ---- 閫夋嫨 / 鏂瑰悜 ----

    pub fn selection(&self) -> Option<IvgNodeRange> {
        self.disp.invoke_method("Selection", vec![]).ok()?.to_idispatch().ok().map(IvgNodeRange::new)
    }

    pub fn reverse_direction(&self) -> bool {
        self.disp.invoke_method("ReverseDirection", vec![]).is_ok()
    }

    pub fn next(&self) -> Option<IvgSubPath> {
        self.disp.invoke_method("Next", vec![]).ok()?.to_idispatch().ok().map(IvgSubPath::new)
    }

    pub fn previous(&self) -> Option<IvgSubPath> {
        self.disp.invoke_method("Previous", vec![]).ok()?.to_idispatch().ok().map(IvgSubPath::new)
    }

    // ---- 杩藉姞娈?----

    pub fn append_line_segment(&self, x: f64, y: f64, at_beginning: bool) -> Option<IvgSegment> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(at_beginning),
        ];
        self.disp.invoke_method("AppendLineSegment", args).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    pub fn append_curve_segment(
        &self,
        x: f64, y: f64,
        sc_len: f64, sc_angle: f64,
        ec_len: f64, ec_angle: f64,
        at_beginning: bool,
    ) -> Option<IvgSegment> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(sc_len),
            Variant::from_f64(sc_angle),
            Variant::from_f64(ec_len),
            Variant::from_f64(ec_angle),
            Variant::from_bool(at_beginning),
        ];
        self.disp.invoke_method("AppendCurveSegment", args).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    // ---- 缂栬緫 ----

    pub fn delete(&self) -> bool { self.disp.invoke_method("Delete", vec![]).is_ok() }

    pub fn move_by(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    pub fn get_position(&self) -> Option<(f64, f64)> {
        // 鍙屽嚭鍙傜畝鍖栵細鍙栧睘鎬?
        Some((self.position_x()?, self.position_y()?))
    }

    pub fn set_position(&self, x: f64, y: f64) -> bool {
        self.set_position_x(x) && self.set_position_y(y)
    }

    // ---- 鎶界 ----

    pub fn extract(&self) -> Option<(crate::shape::IvgShape, crate::shape::IvgShape)> {
        // COM 鍙屽嚭鍙傦細`Extract(OldCurve, ppVal)`銆?
        // 閫氳繃 IDispatch 鍙兘鎷夸竴涓紱杩欓噷淇濈暀鍗犱綅锛屽缓璁洿鎺ユ敼鐢?
        // `shape.curve().sub_paths().item(n)` 鎻愬彇銆?
        None
    }

    // ---- 鍑犱綍鏌ヨ ----

    pub fn is_on_sub_path(&self, x: f64, y: f64, hot_area: f64) -> Option<i64> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(hot_area),
        ];
        self.disp.invoke_method("IsOnSubPath", args).ok()?.to_i64().ok()
    }

    pub fn is_point_inside(&self, x: f64, y: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("IsPointInside", args).ok()?.to_bool().ok()
    }

    pub fn get_copy(&self) -> Option<IvgCurve> {
        self.disp.invoke_method("GetCopy", vec![]).ok()?.to_idispatch().ok().map(IvgCurve::new)
    }
}
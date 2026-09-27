//! `IVGSegmentRange` 鈥斺€?绾挎鑼冨洿

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{IvgNodeRange, IvgSegment};

pub struct IvgSegmentRange {
    disp: ComObject,
}

impl IvgSegmentRange {
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

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }
    pub fn segment_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn length(&self) -> Option<f64> { self.prop_f64("Length") }

    pub fn item(&self, index: i32) -> Option<IvgSegment> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    pub fn first_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("FirstSegment").map(IvgSegment::new)
    }

    pub fn last_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("LastSegment").map(IvgSegment::new)
    }

    pub fn node_range(&self) -> Option<IvgNodeRange> {
        self.prop_dispatch("NodeRange").map(IvgNodeRange::new)
    }

    pub fn add(&self, segment: &IvgSegment) -> bool {
        let args = vec![segment.as_variant()];
        self.disp.invoke_method("Add", args).is_ok()
    }

    pub fn add_range(&self, range: &IvgSegmentRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("AddRange", args).is_ok()
    }

    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }

    pub fn remove_range(&self, range: &IvgSegmentRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("RemoveRange", args).is_ok()
    }

    pub fn remove_all(&self) -> bool { self.disp.invoke_method("RemoveAll", vec![]).is_ok() }
    pub fn add_node(&self) -> bool { self.disp.invoke_method("AddNode", vec![]).is_ok() }

    pub fn set_type(&self, segment_type: i32) -> bool {
        let args = vec![Variant::from_i64(segment_type as i64)];
        self.disp.invoke_method("SetType", args).is_ok()
    }

    pub fn create_selection(&self) -> bool { self.disp.invoke_method("CreateSelection", vec![]).is_ok() }
    pub fn add_to_selection(&self) -> bool { self.disp.invoke_method("AddToSelection", vec![]).is_ok() }
    pub fn remove_from_selection(&self) -> bool { self.disp.invoke_method("RemoveFromSelection", vec![]).is_ok() }
}
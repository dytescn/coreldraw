//! `IVGSnapPoint` 涓€鏃?鈥斺€?鎶撳彇鐐?
//!
//! - [`IvgSnapPoint`]          : 閫氱敤鎶撳彇鐐?
//! - [`IvgSnapPoints`]         : 闆嗗悎
//! - [`IvgSnapPointRange`]     : 鑼冨洿
//! - [`IvgUserSnapPoint`]      : 鐢ㄦ埛鑷畾涔夋姄鍙栫偣
//! - [`IvgObjectSnapPoint`]    : 瀵硅薄鎶撳彇鐐癸紙涓偣銆佺鐐广€佸渾蹇冣€︹€︼級
//! - [`IvgBBoxSnapPoint`]      : 鍖呭洿鐩掕鐐?
//! - [`IvgEdgeSnapPoint`]      : 杈圭紭鎶撳彇鐐?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgNode;
use crate::shape::IvgShape;

// =============================================================
// IvgSnapPoint
// =============================================================

pub struct IvgSnapPoint {
    disp: ComObject,
}

impl IvgSnapPoint {
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

    // ---- 绫诲瀷 / 鏂瑰悜 ----

    /// `cdrPointType`
    pub fn point_type(&self) -> Option<i64> { self.prop_i64("Type") }

    pub fn direction(&self) -> Option<f64> { self.prop_f64("Direction") }
    pub fn set_direction(&self, v: f64) -> bool { self.put_f64("Direction", v) }

    pub fn uses_direction(&self) -> Option<bool> { self.prop_bool("UsesDirection") }
    pub fn set_uses_direction(&self, v: bool) -> bool { self.put_bool("UsesDirection", v) }

    pub fn is_selectable(&self) -> Option<bool> { self.prop_bool("IsSelectable") }
    pub fn is_deletable(&self) -> Option<bool> { self.prop_bool("IsDeletable") }
    pub fn is_movable(&self) -> Option<bool> { self.prop_bool("IsMovable") }
    pub fn can_change_direction(&self) -> Option<bool> {
        self.prop_bool("CanChangeDirection")
    }

    pub fn selected(&self) -> Option<bool> { self.prop_bool("Selected") }
    pub fn set_selected(&self, v: bool) -> bool { self.put_bool("Selected", v) }

    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }

    // ---- 鍏宠仈 ----

    pub fn node(&self) -> Option<IvgNode> {
        self.prop_dispatch("Node").map(IvgNode::new)
    }

    pub fn shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("Shape").map(IvgShape::new)
    }

    /// 寮曠敤鏁版嵁锛圙UID 瀛楃涓诧紝鍞竴瀹氫綅涓€涓姄鍙栫偣锛夈€?
    pub fn reference_data(&self) -> Option<String> {
        self.prop_string("ReferenceData")
    }

    // ---- 瀛愮被瑙嗗浘锛堟寜绫诲瀷鍙栵級 ----

    pub fn user(&self) -> Option<IvgUserSnapPoint> {
        self.prop_dispatch("User").map(IvgUserSnapPoint::new)
    }

    pub fn object(&self) -> Option<IvgObjectSnapPoint> {
        self.prop_dispatch("Object").map(IvgObjectSnapPoint::new)
    }

    pub fn bbox(&self) -> Option<IvgBBoxSnapPoint> {
        self.prop_dispatch("BBox").map(IvgBBoxSnapPoint::new)
    }

    pub fn edge(&self) -> Option<IvgEdgeSnapPoint> {
        self.prop_dispatch("Edge").map(IvgEdgeSnapPoint::new)
    }

    // ---- 鎿嶄綔 ----

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}

// =============================================================
// IvgUserSnapPoint
// =============================================================

pub struct IvgUserSnapPoint {
    disp: ComObject,
}

impl IvgUserSnapPoint {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn id(&self) -> Option<String> { self.prop_string("ID") }

    pub fn auto_snap(&self) -> Option<bool> { self.prop_bool("AutoSnap") }
    pub fn set_auto_snap(&self, v: bool) -> bool { self.put_bool("AutoSnap", v) }
}

// =============================================================
// IvgObjectSnapPoint
// =============================================================

pub struct IvgObjectSnapPoint {
    disp: ComObject,
}

impl IvgObjectSnapPoint {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// `cdrObjectSnapPointType`
    pub fn snap_type(&self) -> Option<i64> {
        self.disp.get_property("Type").ok()?.to_i64().ok()
    }
}

// =============================================================
// IvgBBoxSnapPoint
// =============================================================

pub struct IvgBBoxSnapPoint {
    disp: ComObject,
}

impl IvgBBoxSnapPoint {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// `cdrReferencePoint`
    pub fn snap_type(&self) -> Option<i64> {
        self.disp.get_property("Type").ok()?.to_i64().ok()
    }
}

// =============================================================
// IvgEdgeSnapPoint
// =============================================================

pub struct IvgEdgeSnapPoint {
    disp: ComObject,
}

impl IvgEdgeSnapPoint {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    pub fn segment_index(&self) -> Option<i64> { self.prop_i64("SegmentIndex") }
    pub fn segment_offset(&self) -> Option<f64> { self.prop_f64("SegmentOffset") }
}

// =============================================================
// IvgSnapPoints
// =============================================================

pub struct IvgSnapPoints {
    disp: ComObject,
}

impl IvgSnapPoints {
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

    pub fn item(&self, index: i32) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    pub fn all(&self) -> Vec<IvgSnapPoint> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }

    // ---- 鍒嗙被鑾峰彇 ----

    pub fn user(&self, id: impl Into<String>) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("User", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    /// `cdrReferencePoint`
    pub fn bbox(&self, ref_point: i32) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_i64(ref_point as i64)];
        self.disp
            .invoke_method("BBox", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    /// `cdrObjectSnapPointType`
    pub fn object(&self, snap_type: i32) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_i64(snap_type as i64)];
        self.disp
            .invoke_method("Object", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    pub fn edge(&self, segment_index: i32, segment_offset: f64) -> Option<IvgSnapPoint> {
        let args = vec![
            Variant::from_i64(segment_index as i64),
            Variant::from_f64(segment_offset),
        ];
        self.disp
            .invoke_method("Edge", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    pub fn auto_snap_point(&self) -> Option<IvgSnapPoint> {
        self.disp
            .invoke_method("Auto", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    pub fn find_closest(
        &self,
        type_set: i32,
        x: f64,
        y: f64,
    ) -> Option<IvgSnapPoint> {
        let args = vec![
            Variant::from_i64(type_set as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp
            .invoke_method("FindClosest", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    // ---- 鑼冨洿 ----

    /// 鎸夊紩鐢ㄦ暟缁勫彇鑼冨洿銆?
    pub fn range(&self, references: Variant) -> Option<IvgSnapPointRange> {
        let args = vec![references];
        self.disp
            .invoke_method("Range", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPointRange::new)
    }

    pub fn selection(&self) -> Option<IvgSnapPointRange> {
        self.prop_dispatch("Selection").map(IvgSnapPointRange::new)
    }

    pub fn all_range(&self) -> Option<IvgSnapPointRange> {
        self.prop_dispatch("All").map(IvgSnapPointRange::new)
    }

    pub fn clear_selection(&self) -> bool {
        self.disp.invoke_method("ClearSelection", vec![]).is_ok()
    }

    // ---- 娣诲姞鐢ㄦ埛鎶撳彇鐐?----

    pub fn add_user_snap_point(
        &self,
        x: f64,
        y: f64,
        direction: f64,
        use_direction: bool,
    ) -> Option<IvgSnapPoint> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(direction),
            Variant::from_bool(use_direction),
        ];
        self.disp
            .invoke_method("AddUserSnapPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    pub fn add_user_snap_point_ex(
        &self,
        id: impl Into<String>,
        x: f64,
        y: f64,
        direction: f64,
        use_direction: bool,
    ) -> Option<IvgSnapPoint> {
        let args = vec![
            Variant::from_str(id.into()),
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(direction),
            Variant::from_bool(use_direction),
        ];
        self.disp
            .invoke_method("AddUserSnapPointEx", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }
}

// =============================================================
// IvgSnapPointRange
// =============================================================

pub struct IvgSnapPointRange {
    disp: ComObject,
}

impl IvgSnapPointRange {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    pub fn all(&self) -> Vec<IvgSnapPoint> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }

    pub fn move_by(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    pub fn add(&self, sp: &IvgSnapPoint) -> Option<bool> {
        let args = vec![sp.as_variant()];
        self.disp.invoke_method("Add", args).ok()?.to_bool().ok()
    }

    pub fn remove(&self, index: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).ok()?.to_bool().ok()
    }

    pub fn remove_by_reference(&self, reference_data: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(reference_data.into())];
        self.disp
            .invoke_method("RemoveByReference", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn find(&self, reference_data: impl Into<String>) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_str(reference_data.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }

    pub fn add_to_selection(&self) -> bool {
        self.disp.invoke_method("AddToSelection", vec![]).is_ok()
    }

    pub fn remove_from_selection(&self) -> bool {
        self.disp.invoke_method("RemoveFromSelection", vec![]).is_ok()
    }

    pub fn change_direction(&self, direction: f64, uses_direction: i32) -> bool {
        let args = vec![
            Variant::from_f64(direction),
            Variant::from_i64(uses_direction as i64),
        ];
        self.disp.invoke_method("ChangeDirection", args).is_ok()
    }

    pub fn set_auto_snap(&self, v: bool) -> bool {
        let args = vec![Variant::from_bool(v)];
        self.disp.invoke_method("SetAutoSnap", args).is_ok()
    }
}
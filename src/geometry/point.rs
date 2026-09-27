//! `IVGPoint` / `IVGPointRange` 鈥斺€?鐐?/ 鐐归泦

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgVector;

// =============================================================
// IvgPoint
// =============================================================

pub struct IvgPoint {
    disp: ComObject,
}

impl IvgPoint {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 鍧愭爣 ----

    /// 鈿狅笍 娉ㄦ剰灞炴€у悕鏄皬鍐?`x` / `y`锛圕OM 灞傚姝わ級銆?
    pub fn x(&self) -> Option<f64> { self.prop_f64("x") }
    pub fn set_x(&self, v: f64) -> bool { self.put_f64("x", v) }

    pub fn y(&self) -> Option<f64> { self.prop_f64("y") }
    pub fn set_y(&self, v: f64) -> bool { self.put_f64("y", v) }

    pub fn get_xy(&self) -> Option<(f64, f64)> {
        Some((self.x()?, self.y()?))
    }

    pub fn set_xy(&self, x: f64, y: f64) -> bool {
        self.set_x(x) && self.set_y(y)
    }

    // ---- 杩愮畻 ----

    pub fn add(&self, v: &IvgVector) -> bool {
        let args = vec![v.as_variant()];
        self.disp.invoke_method("Add", args).is_ok()
    }

    pub fn subtract(&self, v: &IvgVector) -> bool {
        let args = vec![v.as_variant()];
        self.disp.invoke_method("Subtract", args).is_ok()
    }

    /// 鍒板彟涓€鐐圭殑璺濈銆?
    pub fn distance_to(&self, other: &IvgPoint) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("DistanceTo", args).ok()?.to_f64().ok()
    }

    // ---- 澶嶅埗 / 缁戝畾 ----

    pub fn get_copy(&self) -> Option<IvgPoint> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPoint::new)
    }

    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> bool {
        let args = vec![doc.as_variant()];
        self.disp.invoke_method("BindToDocument", args).is_ok()
    }
}

// =============================================================
// IvgPointRange
// =============================================================

pub struct IvgPointRange {
    disp: ComObject,
}

impl IvgPointRange {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---- 鍩烘湰淇℃伅 ----

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgPoint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPoint::new)
    }

    pub fn first(&self) -> Option<IvgPoint> {
        self.prop_dispatch("First").map(IvgPoint::new)
    }

    pub fn last(&self) -> Option<IvgPoint> {
        self.prop_dispatch("Last").map(IvgPoint::new)
    }

    /// 杩斿洖鎵€鏈夌偣锛堢敤绱㈠紩杩唬锛夈€?
    pub fn all(&self) -> Vec<IvgPoint> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }

    // ---- 娣诲姞 ----

    pub fn add_point(&self, p: &IvgPoint) -> bool {
        let args = vec![p.as_variant()];
        self.disp.invoke_method("AddPoint", args).is_ok()
    }

    pub fn add_point_xy(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("AddPointXY", args).is_ok()
    }

    pub fn insert_point(&self, index: i32, p: &IvgPoint) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            p.as_variant(),
        ];
        self.disp.invoke_method("InsertPoint", args).is_ok()
    }

    pub fn add_points(&self, other: &IvgPointRange) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("AddPoints", args).is_ok()
    }

    pub fn insert_points(&self, index: i32, other: &IvgPointRange) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            other.as_variant(),
        ];
        self.disp.invoke_method("InsertPoints", args).is_ok()
    }

    // ---- 绉婚櫎 ----

    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }

    pub fn remove_range(&self, start: i32, end: i32) -> bool {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(end as i64),
        ];
        self.disp.invoke_method("RemoveRange", args).is_ok()
    }

    pub fn remove_all(&self) -> bool {
        self.disp.invoke_method("RemoveAll", vec![]).is_ok()
    }

    pub fn remove_adjacent_duplicates(&self) -> bool {
        self.disp
            .invoke_method("RemoveAdjacentDuplicates", vec![])
            .is_ok()
    }

    // ---- 鍏跺畠鎿嶄綔 ----

    pub fn reverse(&self) -> bool {
        self.disp.invoke_method("Reverse", vec![]).is_ok()
    }

    pub fn smoothen(&self, n_points: f64, closed: bool) -> bool {
        let args = vec![
            Variant::from_f64(n_points),
            Variant::from_bool(closed),
        ];
        self.disp.invoke_method("Smoothen", args).is_ok()
    }

    pub fn get_copy(&self) -> Option<IvgPointRange> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPointRange::new)
    }

    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> bool {
        let args = vec![doc.as_variant()];
        self.disp.invoke_method("BindToDocument", args).is_ok()
    }
}
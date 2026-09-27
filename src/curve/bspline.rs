//! `IVGBSpline` / `IVGBSplineControlPoint(s)` 鈥斺€?B 鏍锋潯

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgBSplineControlPoints
// =============================================================

pub struct IvgBSplineControlPoints {
    disp: ComObject,
}

impl IvgBSplineControlPoints {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgBSplineControlPoint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgBSplineControlPoint::new)
    }

    pub fn first(&self) -> Option<IvgBSplineControlPoint> {
        self.prop_dispatch("First").map(IvgBSplineControlPoint::new)
    }

    pub fn last(&self) -> Option<IvgBSplineControlPoint> {
        self.prop_dispatch("Last").map(IvgBSplineControlPoint::new)
    }

    pub fn resize(&self, n: i32) -> bool {
        let args = vec![Variant::from_i64(n as i64)];
        self.disp.invoke_method("Resize", args).is_ok()
    }
}

// =============================================================
// IvgBSplineControlPoint
// =============================================================

pub struct IvgBSplineControlPoint {
    disp: ComObject,
}

impl IvgBSplineControlPoint {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn x(&self) -> Option<f64> { self.prop_f64("x") }
    pub fn set_x(&self, v: f64) -> bool { self.put_f64("x", v) }

    pub fn y(&self) -> Option<f64> { self.prop_f64("y") }
    pub fn set_y(&self, v: f64) -> bool { self.put_f64("y", v) }

    pub fn clamped(&self) -> Option<bool> { self.prop_bool("Clamped") }
    pub fn set_clamped(&self, v: bool) -> bool { self.put_bool("Clamped", v) }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    pub fn move_by(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    pub fn delete(&self) -> bool { self.disp.invoke_method("Delete", vec![]).is_ok() }

    pub fn get_position(&self) -> Option<(f64, f64)> {
        Some((self.x()?, self.y()?))
    }

    pub fn set_position(&self, x: f64, y: f64) -> bool {
        self.set_x(x) && self.set_y(y)
    }

    pub fn set_properties(&self, x: f64, y: f64, clamped: bool) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(clamped),
        ];
        self.disp.invoke_method("SetProperties", args).is_ok()
    }
}

// =============================================================
// IvgBSpline
// =============================================================

pub struct IvgBSpline {
    disp: ComObject,
}

impl IvgBSpline {
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
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

    pub fn control_points(&self) -> Option<IvgBSplineControlPoints> {
        self.prop_dispatch("ControlPoints").map(IvgBSplineControlPoints::new)
    }

    pub fn is_closed(&self) -> Option<bool> { self.prop_bool("Closed") }
    pub fn set_closed(&self, v: bool) -> bool { self.put_bool("Closed", v) }

    pub fn insert_control_point(
        &self,
        index: i32,
        x: f64,
        y: f64,
        clamped: bool,
    ) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(clamped),
        ];
        self.disp.invoke_method("InsertControlPoint", args).is_ok()
    }

    pub fn insert_control_points(
        &self,
        index: i32,
        how_many: i32,
        x: f64,
        y: f64,
        clamped: bool,
    ) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(how_many as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(clamped),
        ];
        self.disp.invoke_method("InsertControlPoints", args).is_ok()
    }

    pub fn get_copy(&self) -> Option<IvgBSpline> {
        self.disp.invoke_method("GetCopy", vec![]).ok()?.to_idispatch().ok().map(IvgBSpline::new)
    }

    pub fn copy_assign(&self, other: &IvgBSpline) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }
}
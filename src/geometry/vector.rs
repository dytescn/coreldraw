//! `IVGVector` 鈥斺€?2D 鍚戦噺

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgPoint;

pub struct IvgVector {
    disp: ComObject,
}

impl IvgVector {
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

    // ---- 鍒嗛噺锛堝睘鎬у悕灏忓啓锛?----

    pub fn x(&self) -> Option<f64> { self.prop_f64("x") }
    pub fn set_x(&self, v: f64) -> bool { self.put_f64("x", v) }

    pub fn y(&self) -> Option<f64> { self.prop_f64("y") }
    pub fn set_y(&self, v: f64) -> bool { self.put_f64("y", v) }

    // ---- 鏋佸潗鏍?----

    /// 鍚戦噺闀垮害銆?
    pub fn length(&self) -> Option<f64> { self.prop_f64("Length") }
    pub fn set_length(&self, v: f64) -> bool { self.put_f64("Length", v) }

    /// 鍚戦噺瑙掑害锛堝姬搴︼級銆?
    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }

    // ---- 杩愮畻 ----

    /// 鎶婂悜閲忓綋浣滃亸绉婚噺锛屼粠鏌愪釜鍘熺偣鍙栫鐐广€?
    pub fn get_offsetted_point(&self, origin: &IvgPoint, distance: f64) -> Option<IvgPoint> {
        let args = vec![
            origin.as_variant(),
            Variant::from_f64(distance),
        ];
        self.disp
            .invoke_method("GetOffsettedPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPoint::new)
    }

    pub fn add(&self, other: &IvgVector) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("Add", args).is_ok()
    }

    pub fn subtract(&self, other: &IvgVector) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("Subtract", args).is_ok()
    }

    pub fn multiply_by(&self, k: f64) -> bool {
        let args = vec![Variant::from_f64(k)];
        self.disp.invoke_method("MultiplyBy", args).is_ok()
    }

    pub fn negate(&self) -> bool {
        self.disp.invoke_method("Negate", vec![]).is_ok()
    }

    pub fn normalize(&self) -> bool {
        self.disp.invoke_method("Normalize", vec![]).is_ok()
    }

    // ---- 鐐圭Н / 鍙夌Н / 澶硅 ----

    pub fn angle_between(&self, other: &IvgVector) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("AngleBetween", args).ok()?.to_f64().ok()
    }

    pub fn small_angle_between(&self, other: &IvgVector) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("SmallAngleBetween", args)
            .ok()?
            .to_f64()
            .ok()
    }

    pub fn dot_product(&self, other: &IvgVector) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("DotProduct", args).ok()?.to_f64().ok()
    }

    pub fn cross_product(&self, other: &IvgVector) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("CrossProduct", args)
            .ok()?
            .to_f64()
            .ok()
    }

    // ---- 鏋勯€?/ 鎶曞奖 ----

    pub fn set_from_points(&self, start: &IvgPoint, end: &IvgPoint) -> bool {
        let args = vec![start.as_variant(), end.as_variant()];
        self.disp.invoke_method("SetFromPoints", args).is_ok()
    }

    pub fn project_onto(&self, other: &IvgVector) -> Option<IvgVector> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("ProjectOnto", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgVector::new)
    }

    pub fn get_copy(&self) -> Option<IvgVector> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgVector::new)
    }

    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> bool {
        let args = vec![doc.as_variant()];
        self.disp.invoke_method("BindToDocument", args).is_ok()
    }
}
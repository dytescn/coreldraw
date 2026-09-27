//! `IVGRect` 鈥斺€?鐭╁舰

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgRect {
    disp: ComObject,
}

impl IvgRect {
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

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 浣嶇疆 + 灏哄锛坸 / y 灏忓啓锛?----

    pub fn x(&self) -> Option<f64> { self.prop_f64("x") }
    pub fn set_x(&self, v: f64) -> bool { self.put_f64("x", v) }

    pub fn y(&self) -> Option<f64> { self.prop_f64("y") }
    pub fn set_y(&self, v: f64) -> bool { self.put_f64("y", v) }

    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    pub fn height(&self) -> Option<f64> { self.prop_f64("Height") }
    pub fn set_height(&self, v: f64) -> bool { self.put_f64("Height", v) }

    // ---- 渚挎嵎锛氬洓杈?+ 涓績 ----

    pub fn left(&self) -> Option<f64> { self.prop_f64("Left") }
    pub fn right(&self) -> Option<f64> { self.prop_f64("Right") }
    pub fn top(&self) -> Option<f64> { self.prop_f64("Top") }
    pub fn bottom(&self) -> Option<f64> { self.prop_f64("Bottom") }
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }

    // ---- 鐘舵€?----

    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("IsEmpty") }

    // ---- 璁剧疆 / 鑾峰彇 ----

    pub fn set_rect(&self, x: f64, y: f64, w: f64, h: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(w),
            Variant::from_f64(h),
        ];
        self.disp.invoke_method("SetRect", args).is_ok()
    }

    pub fn get_rect(&self) -> Option<(f64, f64, f64, f64)> {
        // 4 out 鍙傛暟鐢ㄥ睘鎬ф浛浠?
        Some((self.x()?, self.y()?, self.width()?, self.height()?))
    }

    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    // ---- 澶嶅埗 ----

    pub fn copy_assign(&self, src: &IvgRect) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    pub fn get_copy(&self) -> Option<IvgRect> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRect::new)
    }

    // ---- 闆嗗悎杩愮畻 ----

    pub fn intersect(&self, other: &IvgRect) -> Option<IvgRect> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("Intersect", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRect::new)
    }

    pub fn union(&self, other: &IvgRect) -> Option<IvgRect> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("Union", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRect::new)
    }

    // ---- 骞崇Щ / 鑶ㄨ儉 ----

    pub fn offset(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Offset", args).is_ok()
    }

    pub fn inflate(&self, l: f64, t: f64, r: f64, b: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(l),
            Variant::from_f64(t),
            Variant::from_f64(r),
            Variant::from_f64(b),
        ];
        self.disp.invoke_method("Inflate", args).ok()?.to_bool().ok()
    }

    // ---- 鍒ゆ柇 ----

    pub fn is_point_inside(&self, x: f64, y: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp
            .invoke_method("IsPointInside", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---- 璺ㄦ枃妗ｈ浆鎹?----

    pub fn change_context(
        &self,
        src_doc: &crate::document::IvgDocument,
        dst_doc: &crate::document::IvgDocument,
    ) -> Option<IvgRect> {
        let args = vec![src_doc.as_variant(), dst_doc.as_variant()];
        self.disp
            .invoke_method("ChangeContext", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRect::new)
    }
}
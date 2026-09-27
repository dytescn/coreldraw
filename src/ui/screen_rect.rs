//! `ICUIScreenRect` 鈥斺€?灞忓箷鍧愭爣鐭╁舰

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct ICuiScreenRect {
    disp: ComObject,
}

impl ICuiScreenRect {
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

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 鍥涜竟 ----

    pub fn left(&self) -> Option<i64> { self.prop_i64("Left") }
    pub fn set_left(&self, v: i32) -> bool { self.put_i64("Left", v as i64) }

    pub fn right(&self) -> Option<i64> { self.prop_i64("Right") }
    pub fn set_right(&self, v: i32) -> bool { self.put_i64("Right", v as i64) }

    pub fn top(&self) -> Option<i64> { self.prop_i64("Top") }
    pub fn set_top(&self, v: i32) -> bool { self.put_i64("Top", v as i64) }

    pub fn bottom(&self) -> Option<i64> { self.prop_i64("Bottom") }
    pub fn set_bottom(&self, v: i32) -> bool { self.put_i64("Bottom", v as i64) }

    // ---- 灏哄 ----

    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    // ---- 涓績 ----

    pub fn center_x(&self) -> Option<i64> { self.prop_i64("CenterX") }
    pub fn set_center_x(&self, v: i32) -> bool { self.put_i64("CenterX", v as i64) }

    pub fn center_y(&self) -> Option<i64> { self.prop_i64("CenterY") }
    pub fn set_center_y(&self, v: i32) -> bool { self.put_i64("CenterY", v as i64) }

    // ---- 鍙 ----

    pub fn is_read_only(&self) -> Option<bool> { self.prop_bool("ReadOnly") }
    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("IsEmpty") }

    // ---- 鎿嶄綔 ----

    pub fn set_position(&self, left: i32, top: i32, w: i32, h: i32) -> bool {
        let args = vec![
            Variant::from_i64(left as i64),
            Variant::from_i64(top as i64),
            Variant::from_i64(w as i64),
            Variant::from_i64(h as i64),
        ];
        self.disp.invoke_method("SetPosition", args).is_ok()
    }

    pub fn resize(&self, w: i32, h: i32) -> bool {
        let args = vec![
            Variant::from_i64(w as i64),
            Variant::from_i64(h as i64),
        ];
        self.disp.invoke_method("Resize", args).is_ok()
    }

    pub fn move_by(&self, left: i32, top: i32) -> bool {
        let args = vec![
            Variant::from_i64(left as i64),
            Variant::from_i64(top as i64),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    pub fn get_copy(&self) -> Option<ICuiScreenRect> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiScreenRect::new)
    }

    pub fn copy_assign(&self, src: &ICuiScreenRect) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    pub fn offset(&self, dx: i32, dy: i32) -> bool {
        let args = vec![
            Variant::from_i64(dx as i64),
            Variant::from_i64(dy as i64),
        ];
        self.disp.invoke_method("Offset", args).is_ok()
    }

    pub fn inflate(&self, l: i32, t: i32, r: i32, b: i32) -> bool {
        let args = vec![
            Variant::from_i64(l as i64),
            Variant::from_i64(t as i64),
            Variant::from_i64(r as i64),
            Variant::from_i64(b as i64),
        ];
        self.disp.invoke_method("Inflate", args).is_ok()
    }

    pub fn is_point_inside(&self, x: i32, y: i32) -> Option<bool> {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
        ];
        self.disp
            .invoke_method("IsPointInside", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---- 闆嗗悎杩愮畻 ----

    pub fn union(&self, other: &ICuiScreenRect) -> Option<ICuiScreenRect> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("Union", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiScreenRect::new)
    }

    pub fn intersect(&self, other: &ICuiScreenRect) -> Option<ICuiScreenRect> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("Intersect", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiScreenRect::new)
    }
}
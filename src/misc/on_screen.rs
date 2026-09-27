//! 灞忓箷缁樺埗瀵硅薄锛坄IVGOnScreenCurve` / `Handle` / `Text`锛?
//!
//! 杩欎簺瀵硅薄鐢?`Application.CreateOnScreen*()` 鍒涘缓锛?
//! 鐢ㄤ簬宸ュ叿 / 鎻掍欢鍦ㄨ鍥句笂缁樺埗涓存椂鍥惧舰銆?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;
use crate::geometry::IvgPointRange;

// =============================================================
// IvgOnScreenCurve
// =============================================================

pub struct IvgOnScreenCurve {
    disp: ComObject,
}

impl IvgOnScreenCurve {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn show(&self) -> bool { self.disp.invoke_method("Show", vec![]).is_ok() }
    pub fn hide(&self) -> bool { self.disp.invoke_method("Hide", vec![]).is_ok() }

    /// `cdrOnScreenCurvePenStyle`
    pub fn set_pen(&self, color: i32, width_in_pixels: i32, style: i32) -> bool {
        let args = vec![
            Variant::from_i64(color as i64),
            Variant::from_i64(width_in_pixels as i64),
            Variant::from_i64(style as i64),
        ];
        self.disp.invoke_method("SetPen", args).is_ok()
    }

    pub fn set_no_pen(&self) -> bool { self.disp.invoke_method("SetNoPen", vec![]).is_ok() }

    pub fn set_brush(&self, color: i32) -> bool {
        let args = vec![Variant::from_i64(color as i64)];
        self.disp.invoke_method("SetBrush", args).is_ok()
    }

    pub fn set_no_brush(&self) -> bool { self.disp.invoke_method("SetNoBrush", vec![]).is_ok() }

    pub fn set_curve(&self, curve: &IvgCurve) -> bool {
        let args = vec![curve.as_variant()];
        self.disp.invoke_method("SetCurve", args).is_ok()
    }

    pub fn set_line(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
        ];
        self.disp.invoke_method("SetLine", args).is_ok()
    }

    pub fn set_rectangle(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> bool {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
        ];
        self.disp.invoke_method("SetRectangle", args).is_ok()
    }

    pub fn set_circle(&self, cx: f64, cy: f64, radius: f64) -> bool {
        let args = vec![
            Variant::from_f64(cx),
            Variant::from_f64(cy),
            Variant::from_f64(radius),
        ];
        self.disp.invoke_method("SetCircle", args).is_ok()
    }

    pub fn set_points(&self, points: &IvgPointRange) -> bool {
        let args = vec![points.as_variant()];
        self.disp.invoke_method("SetPoints", args).is_ok()
    }
}

// =============================================================
// IvgOnScreenHandle
// =============================================================

pub struct IvgOnScreenHandle {
    disp: ComObject,
}

impl IvgOnScreenHandle {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn show(&self) -> bool { self.disp.invoke_method("Show", vec![]).is_ok() }
    pub fn hide(&self) -> bool { self.disp.invoke_method("Hide", vec![]).is_ok() }

    pub fn set_handle_color(&self, color: i32) -> bool {
        let args = vec![Variant::from_i64(color as i64)];
        self.disp.invoke_method("SetHandleColor", args).is_ok()
    }

    pub fn set_position(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetPosition", args).is_ok()
    }

    pub fn update_hot_tracking(&self, mx: f64, my: f64) -> bool {
        let args = vec![
            Variant::from_f64(mx),
            Variant::from_f64(my),
        ];
        self.disp.invoke_method("UpdateHotTracking", args).is_ok()
    }

    pub fn is_hot_tracked(&self) -> Option<bool> {
        self.disp
            .get_property("IsHotTracked")
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn is_on_handle(&self, mx: f64, my: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(mx),
            Variant::from_f64(my),
        ];
        self.disp
            .invoke_method("IsOnHandle", args)
            .ok()?
            .to_bool()
            .ok()
    }
}

// =============================================================
// IvgOnScreenText
// =============================================================

pub struct IvgOnScreenText {
    disp: ComObject,
}

impl IvgOnScreenText {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn show(&self) -> bool { self.disp.invoke_method("Show", vec![]).is_ok() }
    pub fn hide(&self) -> bool { self.disp.invoke_method("Hide", vec![]).is_ok() }

    pub fn set_text_color(&self, color: i32) -> bool {
        let args = vec![Variant::from_i64(color as i64)];
        self.disp.invoke_method("SetTextColor", args).is_ok()
    }

    /// `cdrOnScreenTextAlign`
    #[allow(clippy::too_many_arguments)]
    pub fn set_text_and_position(
        &self,
        text: impl Into<String>,
        x: f64, y: f64,
        align: i32,
        x_ref: f64, y_ref: f64,
    ) -> bool {
        let args = vec![
            Variant::from_str(text.into()),
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_i64(align as i64),
            Variant::from_f64(x_ref),
            Variant::from_f64(y_ref),
        ];
        self.disp.invoke_method("SetTextAndPosition", args).is_ok()
    }

    pub fn set_text(&self, text: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(text.into())];
        self.disp.invoke_method("SetText", args).is_ok()
    }

    pub fn set_pixel_offset(&self, x: i32, y: i32) -> bool {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
        ];
        self.disp.invoke_method("SetPixelOffset", args).is_ok()
    }

    pub fn update_position(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("UpdatePosition", args).is_ok()
    }
}
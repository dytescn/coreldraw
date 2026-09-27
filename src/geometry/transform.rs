//! `IVGTransformMatrix` 鈥斺€?2脳3 浠垮皠鍙樻崲鐭╅樀
//!
//! ```
//! | d11 d21 tx |
//! | d12 d22 ty |
//! |  0   0  1  |
//! ```

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::{IvgPoint, IvgPointRange, IvgVector};

pub struct IvgTransformMatrix {
    disp: ComObject,
}

impl IvgTransformMatrix {
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

    // ---- 鐭╅樀鍒嗛噺 ----

    pub fn d11(&self) -> Option<f64> { self.prop_f64("d11") }
    pub fn set_d11(&self, v: f64) -> bool { self.put_f64("d11", v) }

    pub fn d12(&self) -> Option<f64> { self.prop_f64("d12") }
    pub fn set_d12(&self, v: f64) -> bool { self.put_f64("d12", v) }

    pub fn d21(&self) -> Option<f64> { self.prop_f64("d21") }
    pub fn set_d21(&self, v: f64) -> bool { self.put_f64("d21", v) }

    pub fn d22(&self) -> Option<f64> { self.prop_f64("d22") }
    pub fn set_d22(&self, v: f64) -> bool { self.put_f64("d22", v) }

    pub fn translation_x(&self) -> Option<f64> { self.prop_f64("TranslationX") }
    pub fn set_translation_x(&self, v: f64) -> bool { self.put_f64("TranslationX", v) }

    pub fn translation_y(&self) -> Option<f64> { self.prop_f64("TranslationY") }
    pub fn set_translation_y(&self, v: f64) -> bool { self.put_f64("TranslationY", v) }

    pub fn translation(&self) -> Option<IvgVector> {
        self.disp
            .get_property("Translation")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgVector::new)
    }

    pub fn set_translation(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetTranslation", args).is_ok()
    }

    // ---- 鐘舵€佸垽鏂?----

    pub fn is_identity(&self) -> Option<bool> { self.prop_bool("IsIdentity") }
    pub fn is_skewed_or_rotated_or_mirrored(&self) -> Option<bool> {
        self.prop_bool("IsSkewedOrRotatedOrMirrored")
    }
    pub fn contains_only_translation(&self) -> Option<bool> {
        self.prop_bool("ContainsOnlyTranslation")
    }
    pub fn is_skewed_or_rotated(&self) -> Option<bool> {
        self.prop_bool("IsSkewedOrRotated")
    }
    pub fn is_scaled_or_skewed_or_rotated(&self) -> Option<bool> {
        self.prop_bool("IsScaledOrSkewedOrRotated")
    }
    pub fn is_orthogonal(&self) -> Option<bool> { self.prop_bool("IsOrthogonal") }
    pub fn is_orthonormal_axis_aligned(&self) -> Option<bool> {
        self.prop_bool("IsOrthonormalAxisAligned")
    }
    pub fn is_orthonormal(&self) -> Option<bool> { self.prop_bool("IsOrthonormal") }
    pub fn is_mirrored(&self) -> Option<bool> { self.prop_bool("IsMirrored") }
    pub fn is_scaled(&self) -> Option<bool> { self.prop_bool("IsScaled") }
    pub fn is_translated(&self) -> Option<bool> { self.prop_bool("IsTranslated") }

    // ---- 鏋勯€?----

    pub fn set_to_identity(&self) -> bool {
        self.disp.invoke_method("SetToIdentity", vec![]).is_ok()
    }

    pub fn invert(&self) -> bool {
        self.disp.invoke_method("Invert", vec![]).is_ok()
    }

    // ---- 鍙樻崲锛堝彲绱Н锛?----

    pub fn translate_by(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("TranslateBy", args).is_ok()
    }

    pub fn translate_by_vector(&self, v: &IvgVector) -> bool {
        let args = vec![v.as_variant()];
        self.disp.invoke_method("TranslateByVector", args).is_ok()
    }

    pub fn rotate(&self, angle: f64) -> bool {
        let args = vec![Variant::from_f64(angle)];
        self.disp.invoke_method("Rotate", args).is_ok()
    }

    pub fn rotate_around(&self, angle: f64, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(angle),
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("RotateAround", args).is_ok()
    }

    pub fn scale(&self, sx: f64, sy: f64) -> bool {
        let args = vec![
            Variant::from_f64(sx),
            Variant::from_f64(sy),
        ];
        self.disp.invoke_method("Scale", args).is_ok()
    }

    pub fn scale_around(&self, sx: f64, sy: f64, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(sx),
            Variant::from_f64(sy),
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("ScaleAround", args).is_ok()
    }

    pub fn transform(&self, other: &IvgTransformMatrix) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("Transform", args).is_ok()
    }

    pub fn transform_around(&self, other: &IvgTransformMatrix, x: f64, y: f64) -> bool {
        let args = vec![
            other.as_variant(),
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("TransformAround", args).is_ok()
    }

    // ---- 浣滅敤鍒扮偣 / 鍚戦噺锛堝師鍦颁慨鏀癸級 ----

    pub fn transform_point(&self, p: &IvgPoint) -> bool {
        let args = vec![p.as_variant()];
        self.disp.invoke_method("TransformPoint", args).is_ok()
    }

    pub fn transform_points(&self, points: &IvgPointRange) -> bool {
        let args = vec![points.as_variant()];
        self.disp.invoke_method("TransformPoints", args).is_ok()
    }

    pub fn transform_vector(&self, v: &IvgVector) -> bool {
        let args = vec![v.as_variant()];
        self.disp.invoke_method("TransformVector", args).is_ok()
    }

    pub fn untransform_point(&self, p: &IvgPoint) -> bool {
        let args = vec![p.as_variant()];
        self.disp.invoke_method("UntransformPoint", args).is_ok()
    }

    pub fn untransform_points(&self, points: &IvgPointRange) -> bool {
        let args = vec![points.as_variant()];
        self.disp.invoke_method("UntransformPoints", args).is_ok()
    }

    pub fn untransform_vector(&self, v: &IvgVector) -> bool {
        let args = vec![v.as_variant()];
        self.disp.invoke_method("UntransformVector", args).is_ok()
    }

    // ---- 澶嶅埗 / 缁戝畾 ----

    pub fn get_copy(&self) -> Option<IvgTransformMatrix> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTransformMatrix::new)
    }

    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> bool {
        let args = vec![doc.as_variant()];
        self.disp.invoke_method("BindToDocument", args).is_ok()
    }
}
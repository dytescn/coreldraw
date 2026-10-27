//! `IVGTransformMatrix` 
//!
//! ```text
//! | d11 d21 tx |
//! | d12 d22 ty |
//! |  0   0  1  |
//! ```

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::{IvgPoint, IvgPointRange, IvgVector};

/// CorelDRAW `IVGTransformMatrix` 对象的后期绑定包装（变换矩阵）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTransformMatrix {
    disp: ComObject,
}

impl IvgTransformMatrix {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
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

    // ---- 矩阵分量 ----

    /// 读取 COM 属性 `d11`。
    pub fn d11(&self) -> Option<f64> { self.prop_f64("d11") }
    /// 设置 COM 属性 `d11`。
    pub fn set_d11(&self, v: f64) -> bool { self.put_f64("d11", v) }

    /// 读取 COM 属性 `d12`。
    pub fn d12(&self) -> Option<f64> { self.prop_f64("d12") }
    /// 设置 COM 属性 `d12`。
    pub fn set_d12(&self, v: f64) -> bool { self.put_f64("d12", v) }

    /// 读取 COM 属性 `d21`。
    pub fn d21(&self) -> Option<f64> { self.prop_f64("d21") }
    /// 设置 COM 属性 `d21`。
    pub fn set_d21(&self, v: f64) -> bool { self.put_f64("d21", v) }

    /// 读取 COM 属性 `d22`。
    pub fn d22(&self) -> Option<f64> { self.prop_f64("d22") }
    /// 设置 COM 属性 `d22`。
    pub fn set_d22(&self, v: f64) -> bool { self.put_f64("d22", v) }

    /// 读取平移 X 坐标（`TranslationX`）。
    pub fn translation_x(&self) -> Option<f64> { self.prop_f64("TranslationX") }
    /// 设置平移 X 坐标（`TranslationX`）。
    pub fn set_translation_x(&self, v: f64) -> bool { self.put_f64("TranslationX", v) }

    /// 读取平移 Y 坐标（`TranslationY`）。
    pub fn translation_y(&self) -> Option<f64> { self.prop_f64("TranslationY") }
    /// 设置平移 Y 坐标（`TranslationY`）。
    pub fn set_translation_y(&self, v: f64) -> bool { self.put_f64("TranslationY", v) }

    /// 读取平移（`Translation`）。
    pub fn translation(&self) -> Option<IvgVector> {
        self.disp
            .get_property("Translation")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgVector::new)
    }

    /// 设置平移（`SetTranslation`）。
    pub fn set_translation(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetTranslation", args).is_ok()
    }

    // ---- 状态判断----

    /// 是否单位（`IsIdentity`）。
    pub fn is_identity(&self) -> Option<bool> { self.prop_bool("IsIdentity") }
    /// 读取 COM 属性 `IsSkewedOrRotatedOrMirrored`。
    pub fn is_skewed_or_rotated_or_mirrored(&self) -> Option<bool> {
        self.prop_bool("IsSkewedOrRotatedOrMirrored")
    }
    /// 读取是否包含仅平移（`ContainsOnlyTranslation`）。
    pub fn contains_only_translation(&self) -> Option<bool> {
        self.prop_bool("ContainsOnlyTranslation")
    }
    /// 是否倾斜旋转（`IsSkewedOrRotated`）。
    pub fn is_skewed_or_rotated(&self) -> Option<bool> {
        self.prop_bool("IsSkewedOrRotated")
    }
    /// 读取 COM 属性 `IsScaledOrSkewedOrRotated`。
    pub fn is_scaled_or_skewed_or_rotated(&self) -> Option<bool> {
        self.prop_bool("IsScaledOrSkewedOrRotated")
    }
    /// 读取 COM 属性 `IsOrthogonal`。
    pub fn is_orthogonal(&self) -> Option<bool> { self.prop_bool("IsOrthogonal") }
    /// 读取 COM 属性 `IsOrthonormalAxisAligned`。
    pub fn is_orthonormal_axis_aligned(&self) -> Option<bool> {
        self.prop_bool("IsOrthonormalAxisAligned")
    }
    /// 读取 COM 属性 `IsOrthonormal`。
    pub fn is_orthonormal(&self) -> Option<bool> { self.prop_bool("IsOrthonormal") }
    /// 读取 COM 属性 `IsMirrored`。
    pub fn is_mirrored(&self) -> Option<bool> { self.prop_bool("IsMirrored") }
    /// 读取 COM 属性 `IsScaled`。
    pub fn is_scaled(&self) -> Option<bool> { self.prop_bool("IsScaled") }
    /// 读取 COM 属性 `IsTranslated`。
    pub fn is_translated(&self) -> Option<bool> { self.prop_bool("IsTranslated") }

    // ---- 构造----

    /// 设置转换为单位（`SetToIdentity`）。
    pub fn set_to_identity(&self) -> bool {
        self.disp.invoke_method("SetToIdentity", vec![]).is_ok()
    }

    /// 反转（COM 方法 `Invert`）。
    pub fn invert(&self) -> bool {
        self.disp.invoke_method("Invert", vec![]).is_ok()
    }

    // ---- 变换（可累积）----

    /// 平移（COM 方法 `TranslateBy`）。
    pub fn translate_by(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("TranslateBy", args).is_ok()
    }

    /// 平移向量（COM 方法 `TranslateByVector`）。
    pub fn translate_by_vector(&self, v: &IvgVector) -> bool {
        let args = vec![v.as_variant()];
        self.disp.invoke_method("TranslateByVector", args).is_ok()
    }

    /// rotate（COM 方法 `Rotate`）。
    pub fn rotate(&self, angle: f64) -> bool {
        let args = vec![Variant::from_f64(angle)];
        self.disp.invoke_method("Rotate", args).is_ok()
    }

    /// 旋转围绕（COM 方法 `RotateAround`）。
    pub fn rotate_around(&self, angle: f64, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(angle),
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("RotateAround", args).is_ok()
    }

    /// 缩放（COM 方法 `Scale`）。
    pub fn scale(&self, sx: f64, sy: f64) -> bool {
        let args = vec![
            Variant::from_f64(sx),
            Variant::from_f64(sy),
        ];
        self.disp.invoke_method("Scale", args).is_ok()
    }

    /// 缩放围绕（COM 方法 `ScaleAround`）。
    pub fn scale_around(&self, sx: f64, sy: f64, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(sx),
            Variant::from_f64(sy),
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("ScaleAround", args).is_ok()
    }

    /// 变换（COM 方法 `Transform`）。
    pub fn transform(&self, other: &IvgTransformMatrix) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("Transform", args).is_ok()
    }

    /// 变换围绕（COM 方法 `TransformAround`）。
    pub fn transform_around(&self, other: &IvgTransformMatrix, x: f64, y: f64) -> bool {
        let args = vec![
            other.as_variant(),
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("TransformAround", args).is_ok()
    }

    // ---- 作用到点 / 向量（原地修改） ----

    /// 变换点（COM 方法 `TransformPoint`）。
    pub fn transform_point(&self, p: &IvgPoint) -> bool {
        let args = vec![p.as_variant()];
        self.disp.invoke_method("TransformPoint", args).is_ok()
    }

    /// 变换点集（COM 方法 `TransformPoints`）。
    pub fn transform_points(&self, points: &IvgPointRange) -> bool {
        let args = vec![points.as_variant()];
        self.disp.invoke_method("TransformPoints", args).is_ok()
    }

    /// 变换向量（COM 方法 `TransformVector`）。
    pub fn transform_vector(&self, v: &IvgVector) -> bool {
        let args = vec![v.as_variant()];
        self.disp.invoke_method("TransformVector", args).is_ok()
    }

    /// 取消变换点（COM 方法 `UntransformPoint`）。
    pub fn untransform_point(&self, p: &IvgPoint) -> bool {
        let args = vec![p.as_variant()];
        self.disp.invoke_method("UntransformPoint", args).is_ok()
    }

    /// 取消变换点集（COM 方法 `UntransformPoints`）。
    pub fn untransform_points(&self, points: &IvgPointRange) -> bool {
        let args = vec![points.as_variant()];
        self.disp.invoke_method("UntransformPoints", args).is_ok()
    }

    /// 取消变换向量（COM 方法 `UntransformVector`）。
    pub fn untransform_vector(&self, v: &IvgVector) -> bool {
        let args = vec![v.as_variant()];
        self.disp.invoke_method("UntransformVector", args).is_ok()
    }

    // ---- 复制 / 绑定 ----

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgTransformMatrix> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTransformMatrix::new)
    }

    /// 绑定文档（COM 方法 `BindToDocument`）。
    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> bool {
        let args = vec![doc.as_variant()];
        self.disp.invoke_method("BindToDocument", args).is_ok()
    }
}
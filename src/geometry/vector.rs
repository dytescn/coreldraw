//! `IVGVector` —— 2D 向量

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgPoint;

/// CorelDRAW `IVGVector` 对象的后期绑定包装（向量）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgVector {
    disp: ComObject,
}

impl IvgVector {
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

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 分量（属性名小写）----

    /// 读取 X 坐标（`x`）。
    pub fn x(&self) -> Option<f64> { self.prop_f64("x") }
    /// 设置 X 坐标（`x`）。
    pub fn set_x(&self, v: f64) -> bool { self.put_f64("x", v) }

    /// 读取 Y 坐标（`y`）。
    pub fn y(&self) -> Option<f64> { self.prop_f64("y") }
    /// 设置 Y 坐标（`y`）。
    pub fn set_y(&self, v: f64) -> bool { self.put_f64("y", v) }

    // ---- 极坐标----

    /// 向量长度。
    pub fn length(&self) -> Option<f64> { self.prop_f64("Length") }
    /// 设置长度（`Length`）。
    pub fn set_length(&self, v: f64) -> bool { self.put_f64("Length", v) }

    /// 向量角度（弧度）。
    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    /// 设置角度（`Angle`）。
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }

    // ---- 运算 ----

    /// 把向量当作偏移量，从某个原点取端点。
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

    /// 添加（COM 方法 `Add`）。
    pub fn add(&self, other: &IvgVector) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("Add", args).is_ok()
    }

    /// 调用 COM 方法 `Subtract`。
    pub fn subtract(&self, other: &IvgVector) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("Subtract", args).is_ok()
    }

    /// 调用 COM 方法 `MultiplyBy`。
    pub fn multiply_by(&self, k: f64) -> bool {
        let args = vec![Variant::from_f64(k)];
        self.disp.invoke_method("MultiplyBy", args).is_ok()
    }

    /// 调用 COM 方法 `Negate`。
    pub fn negate(&self) -> bool {
        self.disp.invoke_method("Negate", vec![]).is_ok()
    }

    /// normalize（COM 方法 `Normalize`）。
    pub fn normalize(&self) -> bool {
        self.disp.invoke_method("Normalize", vec![]).is_ok()
    }

    // ---- 点积 / 叉积 / 夹角 ----

    /// 调用 COM 方法 `AngleBetween`。
    pub fn angle_between(&self, other: &IvgVector) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("AngleBetween", args).ok()?.to_f64().ok()
    }

    /// 调用 COM 方法 `SmallAngleBetween`。
    pub fn small_angle_between(&self, other: &IvgVector) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("SmallAngleBetween", args)
            .ok()?
            .to_f64()
            .ok()
    }

    /// 调用 COM 方法 `DotProduct`。
    pub fn dot_product(&self, other: &IvgVector) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("DotProduct", args).ok()?.to_f64().ok()
    }

    /// 调用 COM 方法 `CrossProduct`。
    pub fn cross_product(&self, other: &IvgVector) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("CrossProduct", args)
            .ok()?
            .to_f64()
            .ok()
    }

    // ---- 构造/ 投影 ----

    /// 设置从…构造点集（`SetFromPoints`）。
    pub fn set_from_points(&self, start: &IvgPoint, end: &IvgPoint) -> bool {
        let args = vec![start.as_variant(), end.as_variant()];
        self.disp.invoke_method("SetFromPoints", args).is_ok()
    }

    /// 调用 COM 方法 `ProjectOnto`。
    pub fn project_onto(&self, other: &IvgVector) -> Option<IvgVector> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("ProjectOnto", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgVector::new)
    }

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgVector> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgVector::new)
    }

    /// 绑定文档（COM 方法 `BindToDocument`）。
    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> bool {
        let args = vec![doc.as_variant()];
        self.disp.invoke_method("BindToDocument", args).is_ok()
    }
}
//! `IVGMathUtils` —— 数学工具
//!
//! 由 `Application.Math` 取得。提供常用几何运算，返回的是
//! **新对象*（`IvgPoint` / `IvgVector` / ...），不会绑定到文档。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::{
    IvgPoint, IvgPointRange, IvgTransformMatrix, IvgVector,
};

/// CorelDRAW `IVGMathUtils` 对象的后期绑定包装。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgMathUtils {
    disp: ComObject,
}

impl IvgMathUtils {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn invoke_dispatch(&self, method: &str, args: Vec<Variant>) -> Option<IDispatch> {
        self.disp.invoke_method(method, args).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 线段相关
    // ---------------------------------------------------------

    /// 在两点之间线性插值（t ∈[0, 1]）。
    pub fn interpolate(&self, s: &IvgPoint, e: &IvgPoint, t: f64) -> Option<IvgPoint> {
        self.invoke_dispatch("Interpolate", vec![
            s.as_variant(),
            e.as_variant(),
            Variant::from_f64(t),
        ])
        .map(IvgPoint::new)
    }

    /// 求两线段交点，返回 `(点, 是否相交)`。
    pub fn intersect_line_segments(
        &self,
        s1: &IvgPoint,
        e1: &IvgPoint,
        s2: &IvgPoint,
        e2: &IvgPoint,
    ) -> Option<IvgPoint> {
        // 多 out 参数用单 dispatch 简化（只取点）
        self.invoke_dispatch("IntersectLineSegments", vec![
            s1.as_variant(),
            e1.as_variant(),
            s2.as_variant(),
            e2.as_variant(),
        ])
        .map(IvgPoint::new)
    }

    /// 距离线线段（COM 方法 `DistanceToLineSegment`）。
    pub fn distance_to_line_segment(
        &self,
        s: &IvgPoint,
        e: &IvgPoint,
        p: &IvgPoint,
    ) -> Option<f64> {
        self.disp
            .invoke_method("DistanceToLineSegment", vec![
                s.as_variant(),
                e.as_variant(),
                p.as_variant(),
            ])
            .ok()?
            .to_f64()
            .ok()
    }

    /// 最近点线线段。
    pub fn closest_point_to_line_segment(
        &self,
        s: &IvgPoint,
        e: &IvgPoint,
        p: &IvgPoint,
    ) -> Option<IvgPoint> {
        self.invoke_dispatch("ClosestPointToLineSegment", vec![
            s.as_variant(),
            e.as_variant(),
            p.as_variant(),
        ])
        .map(IvgPoint::new)
    }

    /// 相交无限线。
    pub fn intersect_infinite_lines(
        &self,
        s1: &IvgPoint,
        e1: &IvgPoint,
        s2: &IvgPoint,
        e2: &IvgPoint,
    ) -> Option<IvgPoint> {
        self.invoke_dispatch("IntersectInfiniteLines", vec![
            s1.as_variant(),
            e1.as_variant(),
            s2.as_variant(),
            e2.as_variant(),
        ])
        .map(IvgPoint::new)
    }

    /// 距离无限线（COM 方法 `DistanceToInfiniteLine`）。
    pub fn distance_to_infinite_line(
        &self,
        s: &IvgPoint,
        e: &IvgPoint,
        p: &IvgPoint,
    ) -> Option<f64> {
        self.disp
            .invoke_method("DistanceToInfiniteLine", vec![
                s.as_variant(),
                e.as_variant(),
                p.as_variant(),
            ])
            .ok()?
            .to_f64()
            .ok()
    }

    /// 最近点无限线。
    pub fn closest_point_to_infinite_line(
        &self,
        s: &IvgPoint,
        e: &IvgPoint,
        p: &IvgPoint,
    ) -> Option<IvgPoint> {
        self.invoke_dispatch("ClosestPointToInfiniteLine", vec![
            s.as_variant(),
            e.as_variant(),
            p.as_variant(),
        ])
        .map(IvgPoint::new)
    }

    // ---------------------------------------------------------
    // 随机数
    // ---------------------------------------------------------

    /// 调用 COM 方法 `GetRandomReal`。
    pub fn random_real(&self, low: f64, high: f64) -> Option<f64> {
        self.disp
            .invoke_method("GetRandomReal", vec![
                Variant::from_f64(low),
                Variant::from_f64(high),
            ])
            .ok()?
            .to_f64()
            .ok()
    }

    /// 调用 COM 方法 `GetRandomInteger`。
    pub fn random_integer(&self, low: i32, high: i32) -> Option<i32> {
        self.disp
            .invoke_method("GetRandomInteger", vec![
                Variant::from_i64(low as i64),
                Variant::from_i64(high as i64),
            ])
            .ok()?
            .to_i64()
            .ok()
            .map(|v| v as i32)
    }

    // ---------------------------------------------------------
    // 拟合 / 中点
    // ---------------------------------------------------------

    /// 拟合直线到点集。
    /// 返回 `(原点, 方向)`。
   pub fn fit_line_to_points(&self, points: &IvgPointRange) -> Option<IvgPoint> {
            self.invoke_dispatch(
                "FitLineToPoints",
                vec![points.as_variant()],
            )
            .map(IvgPoint::new)
        }

    /// 调用 CorelDRAW COM 接口。
    pub fn midpoint(&self, s: &IvgPoint, e: &IvgPoint) -> Option<IvgPoint> {
        self.invoke_dispatch("MidPoint", vec![s.as_variant(), e.as_variant()])
            .map(IvgPoint::new)
    }

    // ---------------------------------------------------------
    // 工厂：临时几何对象
    // ---------------------------------------------------------

    /// 创建点。
    pub fn create_point(&self, x: f64, y: f64) -> Option<IvgPoint> {
        self.invoke_dispatch("CreatePoint", vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ])
        .map(IvgPoint::new)
    }

    /// 创建向量。
    pub fn create_vector(&self, x: f64, y: f64) -> Option<IvgVector> {
        self.invoke_dispatch("CreateVector", vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ])
        .map(IvgVector::new)
    }

    /// 创建点范围。
    pub fn create_point_range(&self) -> Option<IvgPointRange> {
        self.invoke_dispatch("CreatePointRange", vec![])
            .map(IvgPointRange::new)
    }

    // ---------------------------------------------------------
    // 工厂：变换矩阵
    // ---------------------------------------------------------

    /// 创建单位变换矩阵。
    pub fn create_identity_transform_matrix(&self) -> Option<IvgTransformMatrix> {
        self.invoke_dispatch("CreateIdentityTransformMatrix", vec![])
            .map(IvgTransformMatrix::new)
    }

    /// 创建旋转变换矩阵。
    pub fn create_rotation_transform_matrix(
        &self,
        angle: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> Option<IvgTransformMatrix> {
        self.invoke_dispatch("CreateRotationTransformMatrix", vec![
            Variant::from_f64(angle),
            Variant::from_f64(origin_x),
            Variant::from_f64(origin_y),
        ])
        .map(IvgTransformMatrix::new)
    }

    /// 创建平移变换矩阵。
    pub fn create_translation_transform_matrix(
        &self,
        tx: f64,
        ty: f64,
    ) -> Option<IvgTransformMatrix> {
        self.invoke_dispatch("CreateTranslationTransformMatrix", vec![
            Variant::from_f64(tx),
            Variant::from_f64(ty),
        ])
        .map(IvgTransformMatrix::new)
    }

    /// 创建缩放变换矩阵。
    pub fn create_scale_transform_matrix(
        &self,
        sx: f64,
        sy: f64,
        origin_x: f64,
        origin_y: f64,
    ) -> Option<IvgTransformMatrix> {
        self.invoke_dispatch("CreateScaleTransformMatrix", vec![
            Variant::from_f64(sx),
            Variant::from_f64(sy),
            Variant::from_f64(origin_x),
            Variant::from_f64(origin_y),
        ])
        .map(IvgTransformMatrix::new)
    }

    /// 创建线线段变换矩阵。
    pub fn create_line_segment_transform_matrix(
        &self,
        from_start: &IvgPoint,
        from_end: &IvgPoint,
        to_start: &IvgPoint,
        to_end: &IvgPoint,
    ) -> Option<IvgTransformMatrix> {
        self.invoke_dispatch("CreateLineSegmentTransformMatrix", vec![
            from_start.as_variant(),
            from_end.as_variant(),
            to_start.as_variant(),
            to_end.as_variant(),
        ])
        .map(IvgTransformMatrix::new)
    }
}
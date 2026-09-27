//! `IVGMathUtils` 鈥斺€?鏁板宸ュ叿
//!
//! 鐢?`Application.Math` 鍙栧緱銆傛彁渚涘父鐢ㄥ嚑浣曡繍绠楋紝杩斿洖鐨勬槸
//! **鏂板璞?*锛坄IvgPoint` / `IvgVector` / ...锛夛紝涓嶄細缁戝畾鍒版枃妗ｃ€?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::{
    IvgPoint, IvgPointRange, IvgTransformMatrix, IvgVector,
};

pub struct IvgMathUtils {
    disp: ComObject,
}

impl IvgMathUtils {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn invoke_dispatch(&self, method: &str, args: Vec<Variant>) -> Option<IDispatch> {
        self.disp.invoke_method(method, args).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 绾挎鐩稿叧
    // ---------------------------------------------------------

    /// 鍦ㄤ袱鐐逛箣闂寸嚎鎬ф彃鍊硷紙t 鈭?[0, 1]锛夈€?
    pub fn interpolate(&self, s: &IvgPoint, e: &IvgPoint, t: f64) -> Option<IvgPoint> {
        self.invoke_dispatch("Interpolate", vec![
            s.as_variant(),
            e.as_variant(),
            Variant::from_f64(t),
        ])
        .map(IvgPoint::new)
    }

    /// 姹備袱绾挎浜ょ偣锛岃繑鍥?`(鐐? 鏄惁鐩镐氦)`銆?
    pub fn intersect_line_segments(
        &self,
        s1: &IvgPoint,
        e1: &IvgPoint,
        s2: &IvgPoint,
        e2: &IvgPoint,
    ) -> Option<IvgPoint> {
        // 鍙?out 鍙傛暟鐢ㄥ崟 dispatch 绠€鍖栵紙鍙彇鐐癸級
        self.invoke_dispatch("IntersectLineSegments", vec![
            s1.as_variant(),
            e1.as_variant(),
            s2.as_variant(),
            e2.as_variant(),
        ])
        .map(IvgPoint::new)
    }

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
    // 闅忔満鏁?
    // ---------------------------------------------------------

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
    // 鎷熷悎 / 涓偣
    // ---------------------------------------------------------

    /// 鎷熷悎鐩寸嚎鍒扮偣闆嗐€?
    /// 杩斿洖 `(鍘熺偣, 鏂瑰悜)`銆?
   pub fn fit_line_to_points(&self, points: &IvgPointRange) -> Option<IvgPoint> {
            self.invoke_dispatch(
                "FitLineToPoints",
                vec![points.as_variant()],
            )
            .map(IvgPoint::new)
        }

    pub fn midpoint(&self, s: &IvgPoint, e: &IvgPoint) -> Option<IvgPoint> {
        self.invoke_dispatch("MidPoint", vec![s.as_variant(), e.as_variant()])
            .map(IvgPoint::new)
    }

    // ---------------------------------------------------------
    // 宸ュ巶锛氫复鏃跺嚑浣曞璞?
    // ---------------------------------------------------------

    pub fn create_point(&self, x: f64, y: f64) -> Option<IvgPoint> {
        self.invoke_dispatch("CreatePoint", vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ])
        .map(IvgPoint::new)
    }

    pub fn create_vector(&self, x: f64, y: f64) -> Option<IvgVector> {
        self.invoke_dispatch("CreateVector", vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ])
        .map(IvgVector::new)
    }

    pub fn create_point_range(&self) -> Option<IvgPointRange> {
        self.invoke_dispatch("CreatePointRange", vec![])
            .map(IvgPointRange::new)
    }

    // ---------------------------------------------------------
    // 宸ュ巶锛氬彉鎹㈢煩闃?
    // ---------------------------------------------------------

    pub fn create_identity_transform_matrix(&self) -> Option<IvgTransformMatrix> {
        self.invoke_dispatch("CreateIdentityTransformMatrix", vec![])
            .map(IvgTransformMatrix::new)
    }

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
//! 几何基础类型
//!
//! - [`IvgPoint`] / [`IvgPointRange`] : 点 / 点集
//! - [`IvgVector`]                     : 向量
//! - [`IvgRect`]                       : 矩形
//! - [`IvgTransformMatrix`]            : 2×3 变换矩阵
//! - [`IvgMathUtils`]                  : 数学工具
//! - [`IvgSnapPoint`] 一族             : 抓取点

pub mod point;
pub mod vector;
pub mod rect;
pub mod transform;
pub mod math_utils;
pub mod snap_point;

pub use point::{IvgPoint, IvgPointRange};
pub use vector::IvgVector;
pub use rect::IvgRect;
pub use transform::IvgTransformMatrix;
pub use math_utils::IvgMathUtils;
pub use snap_point::{
    IvgSnapPoint, IvgSnapPoints, IvgSnapPointRange,
    IvgUserSnapPoint, IvgObjectSnapPoint, IvgBBoxSnapPoint, IvgEdgeSnapPoint,
};
pub mod units;
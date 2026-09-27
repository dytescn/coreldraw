//! `IVGCurve` 一族
//!
//! - [`IvgCurve`]                 : 曲线（`IVGCurve`）
//! - [`IvgSubPath`] / [`IvgSubPaths`]   : 子路径
//! - [`IvgNode`] / [`IvgNodes`]         : 节点
//! - [`IvgNodeRange`]             : 节点范围
//! - [`IvgSegment`] / [`IvgSegments`]   : 线段
//! - [`IvgSegmentRange`]          : 线段范围
//! - [`IvgCrossPoint`] / [`IvgCrossPoints`] : 交点
//! - [`IvgBSpline`] / ...         : B 样条

pub mod curve;
pub mod sub_path;
pub mod node;
pub mod node_range;
pub mod segment;
pub mod segment_range;
pub mod cross_point;
pub mod bspline;

pub use curve::IvgCurve;
pub use sub_path::{IvgSubPath, IvgSubPaths};
pub use node::{IvgNode, IvgNodes};
pub use node_range::IvgNodeRange;
pub use segment::{IvgSegment, IvgSegments};
pub use segment_range::IvgSegmentRange;
pub use cross_point::{IvgCrossPoint, IvgCrossPoints};
pub use bspline::{IvgBSpline, IvgBSplineControlPoint, IvgBSplineControlPoints};
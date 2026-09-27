//! 图形相关
//!
//! - [`IvgShape`] / [`IvgShapes`] / [`IvgShapeRange`]  : 主入口
//! - 具体类型（按 `Shape.Type` 区分）：
//!   - [`IvgRectangle`] : 矩形
//!   - [`IvgEllipse`]   : 椭圆
//!   - [`IvgPolygon`]   : 多边形
//!   - [`IvgBitmap`]    : 位图
//!   - [`IvgEps`]       : EPS
//!   - [`IvgConnector`] : 连接器
//!   - [`IvgGuide`]     : 参考线
//!   - [`IvgPowerClip`] : 图框精确裁剪
//!   - [`IvgCustomShape`] : 自定义图形
//! - [`IvgImage`] / [`IvgImageTile`] / [`IvgImageTiles`] : 像素级图像数据
//! - [`IvgSelectionInformation`] : 选择信息

pub mod shape;
pub mod shapes;
pub mod shape_range;
pub mod rectangle;
pub mod ellipse;
pub mod polygon;
pub mod bitmap;
pub mod image;
pub mod eps;
pub mod connector;
pub mod guide;
pub mod power_clip;
pub mod custom_shape;
pub mod selection_info;

pub use shape::IvgShape;
pub use shapes::IvgShapes;
pub use shape_range::IvgShapeRange;
pub use rectangle::IvgRectangle;
pub use ellipse::IvgEllipse;
pub use polygon::IvgPolygon;
pub use bitmap::IvgBitmap;
pub use image::{IvgImage, IvgImageTile, IvgImageTiles};
pub use eps::IvgEps;
pub use connector::IvgConnector;
pub use guide::IvgGuide;
pub use power_clip::IvgPowerClip;
pub use custom_shape::IvgCustomShape;
pub use selection_info::IvgSelectionInformation;
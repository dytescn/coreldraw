//! 常用类型集中导出
//!
//! ```no_run
//! use coreldraw::prelude::*;
//! ```

// ---------- 基础层 ----------

pub use crate::types::*;
pub use crate::enums::*;

// ---------- 应用 / 文档 ----------

pub use crate::app::IvgApplication;
pub use crate::document::{IvgDocument, IvgDocuments};

// ---------- 页面 / 图层 ----------

pub use crate::page::{IvgPage, IvgPages};
pub use crate::layer::{IvgLayer, IvgLayers};

// ---------- 图形 / 曲线 ----------

pub use crate::shape::{IvgShape, IvgShapes, IvgShapeRange};
pub use crate::curve::IvgCurve;

// ---------- 几何 ----------

pub use crate::geometry::{IvgPoint, IvgVector, IvgRect};

// ---------- 颜色 ----------

pub use crate::color::{IvgColor, IvgColors, IvgColorContext};

// ---------- 填充 / 轮廓 ----------

pub use crate::fill::IvgFill;
pub use crate::outline::IvgOutline;

// ---------- 文本 / 效果 ----------

pub use crate::text::IvgText;
pub use crate::effect::IvgEffect;

// ---------- 视图 / 树 ----------

pub use crate::view::IvgView;
pub use crate::tree::{IvgTreeNode, IvgTreeNodes};
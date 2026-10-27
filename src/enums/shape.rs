//! 图形枚举

// =============================================================
// cdrShapeType —— 图形类型
// =============================================================

/// `Shape.Type` 的返回值。
///
/// 来源：`VGCore::cdrShapeType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrShapeType {
    /// 无图形。
    NoShape = 0,
    /// 矩形图形。
    RectangleShape = 1,
    /// 椭圆图形。
    EllipseShape = 2,
    /// 曲线图形。
    CurveShape = 3,
    /// 多边形图形。
    PolygonShape = 4,
    /// 位图图形。
    BitmapShape = 5,
    /// 文本图形。
    TextShape = 6,
    /// 组图形。
    GroupShape = 7,
    /// 选择图形。
    SelectionShape = 8,
    /// 参考线图形。
    GuidelineShape = 9,
    /// 调和组图形。
    BlendGroupShape = 10,
    /// 立体化组图形。
    ExtrudeGroupShape = 11,
    /// OLE 对象图形。
    OLEObjectShape = 12,
    /// 轮廓组图形。
    ContourGroupShape = 13,
    /// 线性尺寸图形。
    LinearDimensionShape = 14,
    /// 斜角组图形。
    BevelGroupShape = 15,
    /// 投影阴影组图形。
    DropShadowGroupShape = 16,
    /// 3d 对象图形。
    V_3DObjectShape = 17,
    /// 艺术媒体组图形。
    ArtisticMediaGroupShape = 18,
    /// 连接器图形。
    ConnectorShape = 19,
    /// 网格填充图形。
    MeshFillShape = 20,
    /// 自定义图形。
    CustomShape = 21,
    /// 自定义效果组图形。
    CustomEffectGroupShape = 22,
    /// 符号图形。
    SymbolShape = 23,
    /// HTML 表单对象图形。
    HTMLFormObjectShape = 24,
    /// HTML 活动对象图形。
    HTMLActiveObjectShape = 25,
    /// 完美图形。
    PerfectShape = 26,
    /// EPS 图形。
    EPSShape = 27,
}

impl cdrShapeType {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::NoShape),
            1 => Some(Self::RectangleShape),
            2 => Some(Self::EllipseShape),
            3 => Some(Self::CurveShape),
            4 => Some(Self::PolygonShape),
            5 => Some(Self::BitmapShape),
            6 => Some(Self::TextShape),
            7 => Some(Self::GroupShape),
            8 => Some(Self::SelectionShape),
            9 => Some(Self::GuidelineShape),
            10 => Some(Self::BlendGroupShape),
            11 => Some(Self::ExtrudeGroupShape),
            12 => Some(Self::OLEObjectShape),
            13 => Some(Self::ContourGroupShape),
            14 => Some(Self::LinearDimensionShape),
            15 => Some(Self::BevelGroupShape),
            16 => Some(Self::DropShadowGroupShape),
            17 => Some(Self::V_3DObjectShape),
            18 => Some(Self::ArtisticMediaGroupShape),
            19 => Some(Self::ConnectorShape),
            20 => Some(Self::MeshFillShape),
            21 => Some(Self::CustomShape),
            22 => Some(Self::CustomEffectGroupShape),
            23 => Some(Self::SymbolShape),
            24 => Some(Self::HTMLFormObjectShape),
            25 => Some(Self::HTMLActiveObjectShape),
            26 => Some(Self::PerfectShape),
            27 => Some(Self::EPSShape),
            _ => None,
        }
    }
}

// =============================================================
// cdrReferencePoint —— 参考点
// =============================================================

/// 用于对齐、定位、缩放等操作的参考点。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrReferencePoint {
    /// 左顶。
    TopLeft = 0,
    /// 顶部中心。
    TopCenter = 1,
    /// 右顶。
    TopRight = 2,
    /// 左 middle。
    MiddleLeft = 3,
    /// middle 中心。
    MiddleCenter = 4,
    /// 右 middle。
    MiddleRight = 5,
    /// 左底。
    BottomLeft = 6,
    /// 底部中心。
    BottomCenter = 7,
    /// 右底。
    BottomRight = 8,
}

impl cdrReferencePoint {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::TopLeft),
            1 => Some(Self::TopCenter),
            2 => Some(Self::TopRight),
            3 => Some(Self::MiddleLeft),
            4 => Some(Self::MiddleCenter),
            5 => Some(Self::MiddleRight),
            6 => Some(Self::BottomLeft),
            7 => Some(Self::BottomCenter),
            8 => Some(Self::BottomRight),
            _ => None,
        }
    }
}

// =============================================================
// cdrAlignType —— 对齐方式
// =============================================================

/// CorelDRAW 枚举 `cdrAlignType`：对齐方式。
///
/// 对应类型库中的 `VGCore::cdrAlignType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrAlignType {
    /// 左对齐。
    AlignLeft = 1,
    /// 右对齐。
    AlignRight = 2,
    /// 对齐高中心。
    AlignHCenter = 3,
    /// 顶对齐。
    AlignTop = 4,
    /// 底对齐。
    AlignBottom = 8,
    /// 对齐中心。
    AlignVCenter = 12,
}

// =============================================================
// cdrShapeLevel —— 图形层次（用于 Next/Previous 遍历）
// =============================================================

/// CorelDRAW 枚举 `cdrShapeLevel`：图形层次（用于 Next/Previous 遍历）。
///
/// 对应类型库中的 `VGCore::cdrShapeLevel`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrShapeLevel {
    /// 级别组。
    LevelGroup = 0,
    /// 级别容器。
    LevelContainer = 1,
    /// 级别图层。
    LevelLayer = 2,
    /// 级别页面。
    LevelPage = 3,
    /// 级别文档。
    LevelDocument = 4,
}

// =============================================================
// cdrWrapStyle —— 文本环绕方式
// =============================================================

/// CorelDRAW 枚举 `cdrWrapStyle`：文本环绕方式。
///
/// 对应类型库中的 `VGCore::cdrWrapStyle`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrWrapStyle {
    /// 绕排无。
    WrapNone = -1,
    /// 左绕排轮廓。
    WrapContourLeft = 0,
    /// 右绕排轮廓。
    WrapContourRight = 1,
    /// 绕排轮廓 straddle。
    WrapContourStraddle = 2,
    /// 左绕排 square。
    WrapSquareLeft = 3,
    /// 右绕排 square。
    WrapSquareRight = 4,
    /// 绕排 squarestraddle。
    WrapSquareStraddle = 5,
    /// 绕排 square 上方 below。
    WrapSquareAboveBelow = 6,
}

// =============================================================
// cdrOutlineType —— 轮廓类型
// =============================================================

/// CorelDRAW 枚举 `cdrOutlineType`：轮廓类型。
///
/// 对应类型库中的 `VGCore::cdrOutlineType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrOutlineType {
    /// 无轮廓。
    NoOutline = 0,
    /// 轮廓。
    Outline = 1,
    /// 增强轮廓。
    EnhancedOutline = 2,
}

impl cdrOutlineType {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::NoOutline),
            1 => Some(Self::Outline),
            2 => Some(Self::EnhancedOutline),
            _ => None,
        }
    }
}

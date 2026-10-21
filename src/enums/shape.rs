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
    NoShape = 0,
    RectangleShape = 1,
    EllipseShape = 2,
    CurveShape = 3,
    PolygonShape = 4,
    BitmapShape = 5,
    TextShape = 6,
    GroupShape = 7,
    SelectionShape = 8,
    GuidelineShape = 9,
    BlendGroupShape = 10,
    ExtrudeGroupShape = 11,
    OLEObjectShape = 12,
    ContourGroupShape = 13,
    LinearDimensionShape = 14,
    BevelGroupShape = 15,
    DropShadowGroupShape = 16,
    V_3DObjectShape = 17,
    ArtisticMediaGroupShape = 18,
    ConnectorShape = 19,
    MeshFillShape = 20,
    CustomShape = 21,
    CustomEffectGroupShape = 22,
    SymbolShape = 23,
    HTMLFormObjectShape = 24,
    HTMLActiveObjectShape = 25,
    PerfectShape = 26,
    EPSShape = 27,
}

impl cdrShapeType {
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
    TopLeft = 0,
    TopCenter = 1,
    TopRight = 2,
    MiddleLeft = 3,
    MiddleCenter = 4,
    MiddleRight = 5,
    BottomLeft = 6,
    BottomCenter = 7,
    BottomRight = 8,
}

impl cdrReferencePoint {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrAlignType {
    AlignLeft = 1,
    AlignRight = 2,
    AlignHCenter = 3,
    AlignTop = 4,
    AlignBottom = 8,
    AlignVCenter = 12,
}

// =============================================================
// cdrShapeLevel —— 图形层次（用于 Next/Previous 遍历）
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrShapeLevel {
    LevelGroup = 0,
    LevelContainer = 1,
    LevelLayer = 2,
    LevelPage = 3,
    LevelDocument = 4,
}

// =============================================================
// cdrWrapStyle —— 文本环绕方式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrWrapStyle {
    WrapNone = -1,
    WrapContourLeft = 0,
    WrapContourRight = 1,
    WrapContourStraddle = 2,
    WrapSquareLeft = 3,
    WrapSquareRight = 4,
    WrapSquareStraddle = 5,
    WrapSquareAboveBelow = 6,
}

// =============================================================
// cdrOutlineType —— 轮廓类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrOutlineType {
    NoOutline = 0,
    Outline = 1,
    EnhancedOutline = 2,
}

impl cdrOutlineType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::NoOutline),
            1 => Some(Self::Outline),
            2 => Some(Self::EnhancedOutline),
            _ => None,
        }
    }
}

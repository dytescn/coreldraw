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
    /// 无。
    None       = 0,
    /// 矩形。
    Rectangle  = 1,
    /// 椭圆。
    Ellipse    = 2,
    /// 曲线。
    Curve      = 3,
    /// 多边形。
    Polygon    = 4,
    /// 位图。
    Bitmap     = 5,
    /// 文本。
    Text       = 6,
    /// 群组。
    Group      = 7,
    /// 图层。
    Layer      = 8,
    /// 立体化。
    Extrude    = 9,
    /// 封套。
    Envelope   = 10,
    /// 调和。
    Blend      = 11,
    /// 轮廓。
    Contour    = 12,
    /// 阴影。
    DropShadow = 13,
    /// 透镜。
    Lens       = 14,
    /// 透视。
    Perspective = 15,
    /// 图框精确裁剪。
    PowerClip  = 16,
    /// 裁剪群组。
    Crop       = 17,
    /// 自定义。
    Custom     = 18,
    /// 符号定义。
    SymbolDefinition = 19,
    /// 符号实例。
    SymbolInstance   = 20,
    /// 连接线。
    Connector  = 21,
    /// 艺术笔。
    ArtisticMedia = 22,
    /// 标注。
    Dimension  = 23,
    /// OLE 对象。
    OLE        = 24,
    /// 参考线。
    Guide      = 25,
    /// 卷帘。
    RollOver   = 26,
    /// 卷帘组。
    RollOverGroup = 27,
    /// Web 对象。
    WebObject  = 28,
    /// 条形码。
    BarCode    = 29,
    // TODO: 更完整的列表待补
}

impl cdrShapeType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0  => Some(Self::None),
            1  => Some(Self::Rectangle),
            2  => Some(Self::Ellipse),
            3  => Some(Self::Curve),
            4  => Some(Self::Polygon),
            5  => Some(Self::Bitmap),
            6  => Some(Self::Text),
            7  => Some(Self::Group),
            8  => Some(Self::Layer),
            9  => Some(Self::Extrude),
            10 => Some(Self::Envelope),
            11 => Some(Self::Blend),
            12 => Some(Self::Contour),
            13 => Some(Self::DropShadow),
            14 => Some(Self::Lens),
            15 => Some(Self::Perspective),
            16 => Some(Self::PowerClip),
            17 => Some(Self::Crop),
            18 => Some(Self::Custom),
            19 => Some(Self::SymbolDefinition),
            20 => Some(Self::SymbolInstance),
            21 => Some(Self::Connector),
            22 => Some(Self::ArtisticMedia),
            23 => Some(Self::Dimension),
            24 => Some(Self::OLE),
            25 => Some(Self::Guide),
            26 => Some(Self::RollOver),
            27 => Some(Self::RollOverGroup),
            28 => Some(Self::WebObject),
            29 => Some(Self::BarCode),
            _  => None,
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
    TopLeft      = 0,
    TopCenter    = 1,
    TopRight     = 2,
    MiddleLeft   = 3,
    MiddleCenter = 4,
    MiddleRight  = 5,
    BottomLeft   = 6,
    BottomCenter = 7,
    BottomRight  = 8,
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
    Left        = 0,
    Right       = 1,
    Top         = 2,
    Bottom      = 3,
    CenterHoriz = 4,
    CenterVert  = 5,
    // TODO: 待补
}

// =============================================================
// cdrShapeLevel —— 图形层次（用于 Next/Previous 遍历）
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrShapeLevel {
    SameLevel      = 0,
    AllLevels      = 1,
    // TODO: 待补
}

// =============================================================
// cdrWrapStyle —— 文本环绕方式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrWrapStyle {
    None         = 0,
    Square       = 1,
    Contour      = 2,
    // TODO: 待补
}

// =============================================================
// cdrOutlineType —— 轮廓类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrOutlineType {
    NoOutline    = 0,
    // TODO: 待补
}
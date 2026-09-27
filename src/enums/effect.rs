//! 效果枚举

// =============================================================
// cdrEffectType —— 效果类型
// =============================================================

/// `Effect.Type` 的取值。
///
/// 来源：`VGCore::cdrEffectType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrEffectType {
    Blend       = 0,
    Extrude     = 1,
    Envelope    = 2,
    TextOnPath  = 3,
    ControlPath = 4,
    DropShadow  = 5,
    Contour     = 6,
    Distortion  = 7,
    Perspective = 8,
    // 9 缺失
    CustomEffect = 10,
    Lens        = 11,
}

impl cdrEffectType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0  => Some(Self::Blend),
            1  => Some(Self::Extrude),
            2  => Some(Self::Envelope),
            3  => Some(Self::TextOnPath),
            4  => Some(Self::ControlPath),
            5  => Some(Self::DropShadow),
            6  => Some(Self::Contour),
            7  => Some(Self::Distortion),
            8  => Some(Self::Perspective),
            10 => Some(Self::CustomEffect),
            11 => Some(Self::Lens),
            _  => None,
        }
    }
}

// =============================================================
// cdrDropShadowType —— 阴影类型
// =============================================================

/// 来源：`VGCore::cdrDropShadowType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDropShadowType {
    /// 平面阴影。
    Flat        = 0,
    /// 底部阴影。
    Bottom      = 1,
    /// 右上阴影。
    RightTop    = 2,
    // TODO: 待补
}

impl cdrDropShadowType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Flat),
            1 => Some(Self::Bottom),
            2 => Some(Self::RightTop),
            _ => None,
        }
    }
}

// =============================================================
// cdrDistortionType —— 变形类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDistortionType {
    PushPull = 0,
    Zipper   = 1,
    Twister  = 2,
    Custom   = 3,
}

// =============================================================
// cdrEnvelopeMode —— 封套模式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrEnvelopeMode {
    StraightLine  = 0,
    SingleArc     = 1,
    DoubleArc     = 2,
    // TODO: 待补
}

// =============================================================
// cdrExtrudeType —— 立体化类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeType {
    SmallBack   = 0,
    SmallFront  = 1,
    BigBack     = 2,
    BigFront    = 3,
    // TODO: 待补
}

// =============================================================
// cdrExtrudeShading —— 立体化着色
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeShading {
    Solid       = 0,
    ObjectColor = 1,
    // TODO: 待补
}

// =============================================================
// cdrExtrudeLightPosition —— 立体化光源位置
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeLightPosition {
    TopLeft     = 0,
    TopRight    = 1,
    BottomLeft  = 2,
    BottomRight = 3,
    Top         = 4,
    Bottom      = 5,
    Left        = 6,
    Right       = 7,
    Front       = 8,
}

// =============================================================
// cdrExtrudeVPType —— 立体化消失点类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeVPType {
    Shared      = 0,
    CopyFrom    = 1,
    // TODO: 待补
}

// =============================================================
// cdrLensType —— 透镜类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrLensType {
    NoLens               = 0,
    Magnify              = 1,
    Brighten             = 2,
    Invert               = 3,
    ColorAdd             = 4,
    ColorLimit           = 5,
    ColorMultiply        = 6,
    Grayscale            = 7,
    HeatMap              = 8,
    CustomColorMap       = 9,
    FishEye              = 10,
    Wireframe            = 11,
    Transparency         = 12,
    // TODO: 待补
}

// =============================================================
// cdrContourDirection —— 轮廓方向
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrContourDirection {
    ToCenter  = 0,
    Outside   = 1,
    Inside    = 2,
}

// =============================================================
// cdrContourEndCapType —— 轮廓端点类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrContourEndCapType {
    // TODO: 待补
    Flat = 0,
    // ...
}

// =============================================================
// cdrContourCornerType —— 轮廓角类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrContourCornerType {
    // TODO: 待补
    Miter = 0,
    // ...
}

// =============================================================
// cdrBlendMode —— 调和模式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrBlendMode {
    Steps        = 0,
    Spacing      = 1,
    // TODO: 待补
}

// =============================================================
// cdrFeatherType —— 羽化类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFeatherType {
    Linear  = 0,
    Radial  = 1,
    // TODO: 待补
}

// =============================================================
// cdrEdgeType —— 边缘类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrEdgeType {
    Linear  = 0,
    Curved  = 1,
    // TODO: 待补
}

// =============================================================
// cdrMergeMode —— 混合模式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrMergeMode {
    Normal       = 0,
    Additive     = 1,
    Subtract     = 2,
    Multiply     = 3,
    Divide       = 4,
    // TODO: 待补
}

// =============================================================
// cdrFittedOrientation —— 适配路径朝向
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedOrientation {
    Rotate   = 0,
    Vertical = 1,
    Skew     = 2,
    // TODO: 待补
}

// =============================================================
// cdrFittedPlacement —— 适配路径放置
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedPlacement {
    Baseline = 0,
    Top      = 1,
    Center   = 2,
    Bottom   = 3,
    // TODO: 待补
}

// =============================================================
// cdrFittedQuadrant —— 适配路径象限
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedQuadrant {
    TopLeft     = 0,
    TopRight    = 1,
    BottomLeft  = 2,
    BottomRight = 3,
    // TODO: 待补
}

// =============================================================
// cdrFittedVertPlacement —— 适配路径垂直放置
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedVertPlacement {
    // TODO: 待补
    Top = 0,
    // ...
}
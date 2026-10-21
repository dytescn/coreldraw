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
    Blend = 0,
    Extrude = 1,
    Envelope = 2,
    TextOnPath = 3,
    ControlPath = 4,
    DropShadow = 5,
    Contour = 6,
    Distortion = 7,
    Perspective = 8,
    // 9 缺失
    CustomEffect = 10,
    Lens = 11,
}

impl cdrEffectType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Blend),
            1 => Some(Self::Extrude),
            2 => Some(Self::Envelope),
            3 => Some(Self::TextOnPath),
            4 => Some(Self::ControlPath),
            5 => Some(Self::DropShadow),
            6 => Some(Self::Contour),
            7 => Some(Self::Distortion),
            8 => Some(Self::Perspective),
            10 => Some(Self::CustomEffect),
            11 => Some(Self::Lens),
            _ => None,
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
    DropShadowFlat = 0,
    DropShadowBottom = 1,
    DropShadowTop = 2,
    DropShadowLeft = 3,
    DropShadowRight = 4,
}

impl cdrDropShadowType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::DropShadowFlat),
            1 => Some(Self::DropShadowBottom),
            2 => Some(Self::DropShadowTop),
            3 => Some(Self::DropShadowLeft),
            4 => Some(Self::DropShadowRight),
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
    Zipper = 1,
    Twister = 2,
    Custom = 3,
}

// =============================================================
// cdrEnvelopeMode —— 封套模式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrEnvelopeMode {
    EnvelopeHorizontal = 0,
    EnvelopeOriginal = 1,
    EnvelopePutty = 2,
    EnvelopeVertical = 3,
}

// =============================================================
// cdrExtrudeType —— 立体化类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeType {
    ExtrudeSmallBack = 0,
    ExtrudeSmallFront = 1,
    ExtrudeBigBack = 2,
    ExtrudeBigFront = 3,
    ExtrudeBackParallel = 4,
    ExtrudeFrontParallel = 5,
}

// =============================================================
// cdrExtrudeShading —— 立体化着色
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeShading {
    ExtrudeObjectFill = 0,
    ExtrudeSolidFill = 1,
    ExtrudeColorShading = 2,
}

// =============================================================
// cdrExtrudeLightPosition —— 立体化光源位置
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeLightPosition {
    TopLeft = 0,
    TopRight = 1,
    BottomLeft = 2,
    BottomRight = 3,
    Top = 4,
    Bottom = 5,
    Left = 6,
    Right = 7,
    Front = 8,
}

// =============================================================
// cdrExtrudeVPType —— 立体化消失点类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeVPType {
    VPLockedToShape = 0,
    VPLockedToPage = 1,
    VPShared = 2,
}

// =============================================================
// cdrLensType —— 透镜类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrLensType {
    LensMagnify = 0,
    LensFishEye = 1,
    LensWireframe = 2,
    LensColorLimit = 3,
    LensColorAdd = 4,
    LensInvert = 5,
    LensBrighten = 6,
    LensTintedGrayscale = 7,
    LensHeatMap = 8,
    LensTransparency = 9,
    LensCustomColorMap = 10,
}

// =============================================================
// cdrContourDirection —— 轮廓方向
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrContourDirection {
    ToCenter = 0,
    Outside = 1,
    Inside = 2,
}

// =============================================================
// cdrContourEndCapType —— 轮廓端点类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrContourEndCapType {
    ContourButtCap = 0,
    ContourRoundCap = 1,
    ContourSquareCap = 2,
}

// =============================================================
// cdrContourCornerType —— 轮廓角类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrContourCornerType {
    ContourCornerBevel = 0,
    ContourCornerOffsetBevel = 1,
    ContourCornerRound = 2,
    ContourCornerMiteredBevel = 3,
    ContourCornerMiteredOffsetBevel = 4,
    ContourCornerMiteredRound = 5,
}

// =============================================================
// cdrBlendMode —— 调和模式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrBlendMode {
    BlendSteps = 0,
    BlendSpacing = 1,
}

// =============================================================
// cdrFeatherType —— 羽化类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFeatherType {
    FeatherInside = 0,
    FeatherMiddle = 1,
    FeatherOutside = 2,
    FeatherAverage = 3,
    FeatherGaussianBlur = 4,
}

// =============================================================
// cdrEdgeType —— 边缘类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrEdgeType {
    EdgeLinear = 0,
    EdgeSquared = 1,
    EdgeFlat = 2,
    EdgeInverseSquared = 3,
    EdgeMesa = 4,
    EdgeGaussian = 5,
}

// =============================================================
// cdrMergeMode —— 混合模式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrMergeMode {
    MergeNormal = 0,
    MergeAND = 1,
    MergeOR = 2,
    MergeXOR = 3,
    MergeInvert = 6,
    MergeAdd = 7,
    MergeSubtract = 8,
    MergeMultiply = 9,
    MergeDivide = 10,
    MergeIfLighter = 11,
    MergeIfDarker = 12,
    MergeTexturize = 13,
    MergeColor = 14,
    MergeHue = 15,
    MergeSaturation = 16,
    MergeLightness = 17,
    MergeRed = 18,
    MergeGreen = 19,
    MergeBlue = 20,
    MergeDifference = 24,
    MergeBehind = 27,
    MergeScreen = 28,
    MergeOverlay = 29,
    MergeSoftlight = 30,
    MergeHardlight = 31,
    MergeDodge = 33,
    MergeBurn = 34,
    MergeExclusion = 36,
}

// =============================================================
// cdrFittedOrientation —— 适配路径朝向
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedOrientation {
    RotateOrientation = 0,
    VerticalSkewOrientation = 1,
    HorizontalSkewOrientation = 2,
    UprightOrientation = 3,
    TransformOrientation = 4,
}

// =============================================================
// cdrFittedPlacement —— 适配路径放置
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedPlacement {
    LeftPlacement = 0,
    RightPlacement = 1,
    CenterPlacement = 2,
}

// =============================================================
// cdrFittedQuadrant —— 适配路径象限
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedQuadrant {
    LeftQuadrant = 0,
    RightQuadrant = 1,
    TopQuadrant = 2,
    BottomQuadrant = 3,
}

// =============================================================
// cdrFittedVertPlacement —— 适配路径垂直放置
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedVertPlacement {
    CustomVertPlacement = 0,
    BaselineVertPlacement = 1,
    AscenderVertPlacement = 2,
    DescenderVertPlacement = 3,
    CenterVertPlacement = 4,
}

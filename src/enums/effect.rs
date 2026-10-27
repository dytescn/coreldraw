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
    /// 调和。
    Blend = 0,
    /// 立体化。
    Extrude = 1,
    /// 封套。
    Envelope = 2,
    /// 路径上的文本。
    TextOnPath = 3,
    /// 控件路径。
    ControlPath = 4,
    /// 投影阴影。
    DropShadow = 5,
    /// 轮廓。
    Contour = 6,
    /// 变形。
    Distortion = 7,
    /// 透视。
    Perspective = 8,
    // 9 缺失
    /// 自定义效果。
    CustomEffect = 10,
    /// 透镜。
    Lens = 11,
}

impl cdrEffectType {
    /// 由整数值构造枚举。
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
    /// 投影阴影 flat。
    DropShadowFlat = 0,
    /// 底投影阴影。
    DropShadowBottom = 1,
    /// 顶投影阴影。
    DropShadowTop = 2,
    /// 左投影阴影。
    DropShadowLeft = 3,
    /// 右投影阴影。
    DropShadowRight = 4,
}

impl cdrDropShadowType {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrDistortionType`：变形类型。
///
/// 对应类型库中的 `VGCore::cdrDistortionType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDistortionType {
    /// pushpull。
    PushPull = 0,
    /// zipper。
    Zipper = 1,
    /// twister。
    Twister = 2,
    /// 自定义。
    Custom = 3,
}

// =============================================================
// cdrEnvelopeMode —— 封套模式
// =============================================================

/// CorelDRAW 枚举 `cdrEnvelopeMode`：封套模式。
///
/// 对应类型库中的 `VGCore::cdrEnvelopeMode`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrEnvelopeMode {
    /// 水平封套。
    EnvelopeHorizontal = 0,
    /// 封套原始。
    EnvelopeOriginal = 1,
    /// 封套 putty。
    EnvelopePutty = 2,
    /// 垂直封套。
    EnvelopeVertical = 3,
}

// =============================================================
// cdrExtrudeType —— 立体化类型
// =============================================================

/// CorelDRAW 枚举 `cdrExtrudeType`：立体化类型。
///
/// 对应类型库中的 `VGCore::cdrExtrudeType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeType {
    /// 后立体化 small。
    ExtrudeSmallBack = 0,
    /// 前立体化 small。
    ExtrudeSmallFront = 1,
    /// 后立体化 big。
    ExtrudeBigBack = 2,
    /// 前立体化 big。
    ExtrudeBigFront = 3,
    /// 立体化后 parallel。
    ExtrudeBackParallel = 4,
    /// 立体化前 parallel。
    ExtrudeFrontParallel = 5,
}

// =============================================================
// cdrExtrudeShading —— 立体化着色
// =============================================================

/// CorelDRAW 枚举 `cdrExtrudeShading`：立体化着色。
///
/// 对应类型库中的 `VGCore::cdrExtrudeShading`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeShading {
    /// 立体化对象填充。
    ExtrudeObjectFill = 0,
    /// 立体化 solid 填充。
    ExtrudeSolidFill = 1,
    /// 立体化颜色明暗。
    ExtrudeColorShading = 2,
}

// =============================================================
// cdrExtrudeLightPosition —— 立体化光源位置
// =============================================================

/// CorelDRAW 枚举 `cdrExtrudeLightPosition`：立体化光源位置。
///
/// 对应类型库中的 `VGCore::cdrExtrudeLightPosition`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeLightPosition {
    /// 左顶。
    TopLeft = 0,
    /// 右顶。
    TopRight = 1,
    /// 左底。
    BottomLeft = 2,
    /// 右底。
    BottomRight = 3,
    /// 顶。
    Top = 4,
    /// 底。
    Bottom = 5,
    /// 左。
    Left = 6,
    /// 右。
    Right = 7,
    /// 前。
    Front = 8,
}

// =============================================================
// cdrExtrudeVPType —— 立体化消失点类型
// =============================================================

/// CorelDRAW 枚举 `cdrExtrudeVPType`：立体化消失点类型。
///
/// 对应类型库中的 `VGCore::cdrExtrudeVPType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExtrudeVPType {
    /// vp 锁定状态图形。
    VPLockedToShape = 0,
    /// vp 锁定状态页面。
    VPLockedToPage = 1,
    /// vpshared。
    VPShared = 2,
}

// =============================================================
// cdrLensType —— 透镜类型
// =============================================================

/// CorelDRAW 枚举 `cdrLensType`：透镜类型。
///
/// 对应类型库中的 `VGCore::cdrLensType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrLensType {
    /// 透镜 magnify。
    LensMagnify = 0,
    /// 透镜 fisheye。
    LensFishEye = 1,
    /// 透镜 wireframe。
    LensWireframe = 2,
    /// 透镜颜色限制。
    LensColorLimit = 3,
    /// 透镜颜色添加。
    LensColorAdd = 4,
    /// 透镜反转。
    LensInvert = 5,
    /// 透镜 brighten。
    LensBrighten = 6,
    /// 透镜 tinted 灰度。
    LensTintedGrayscale = 7,
    /// 透镜 heat 映射。
    LensHeatMap = 8,
    /// 透镜透明度。
    LensTransparency = 9,
    /// 透镜自定义颜色映射。
    LensCustomColorMap = 10,
}

// =============================================================
// cdrContourDirection —— 轮廓方向
// =============================================================

/// CorelDRAW 枚举 `cdrContourDirection`：轮廓方向。
///
/// 对应类型库中的 `VGCore::cdrContourDirection`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrContourDirection {
    /// 中心。
    ToCenter = 0,
    /// outside。
    Outside = 1,
    /// 内部。
    Inside = 2,
}

// =============================================================
// cdrContourEndCapType —— 轮廓端点类型
// =============================================================

/// CorelDRAW 枚举 `cdrContourEndCapType`：轮廓端点类型。
///
/// 对应类型库中的 `VGCore::cdrContourEndCapType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrContourEndCapType {
    /// 轮廓 butt 端点。
    ContourButtCap = 0,
    /// 轮廓 round 端点。
    ContourRoundCap = 1,
    /// 轮廓 square 端点。
    ContourSquareCap = 2,
}

// =============================================================
// cdrContourCornerType —— 轮廓角类型
// =============================================================

/// CorelDRAW 枚举 `cdrContourCornerType`：轮廓角类型。
///
/// 对应类型库中的 `VGCore::cdrContourCornerType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrContourCornerType {
    /// 轮廓圆角斜角。
    ContourCornerBevel = 0,
    /// 轮廓圆角偏移斜角。
    ContourCornerOffsetBevel = 1,
    /// 轮廓圆角 round。
    ContourCornerRound = 2,
    /// 轮廓圆角 mitered 斜角。
    ContourCornerMiteredBevel = 3,
    /// 轮廓圆角 mitered 偏移斜角。
    ContourCornerMiteredOffsetBevel = 4,
    /// 轮廓圆角 miteredround。
    ContourCornerMiteredRound = 5,
}

// =============================================================
// cdrBlendMode —— 调和模式
// =============================================================

/// CorelDRAW 枚举 `cdrBlendMode`：调和模式。
///
/// 对应类型库中的 `VGCore::cdrBlendMode`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrBlendMode {
    /// 调和步数。
    BlendSteps = 0,
    /// 调和间距。
    BlendSpacing = 1,
}

// =============================================================
// cdrFeatherType —— 羽化类型
// =============================================================

/// CorelDRAW 枚举 `cdrFeatherType`：羽化类型。
///
/// 对应类型库中的 `VGCore::cdrFeatherType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFeatherType {
    /// 羽化内部。
    FeatherInside = 0,
    /// 羽化 middle。
    FeatherMiddle = 1,
    /// 羽化 outside。
    FeatherOutside = 2,
    /// 羽化 average。
    FeatherAverage = 3,
    /// 羽化 gaussianblur。
    FeatherGaussianBlur = 4,
}

// =============================================================
// cdrEdgeType —— 边缘类型
// =============================================================

/// CorelDRAW 枚举 `cdrEdgeType`：边缘类型。
///
/// 对应类型库中的 `VGCore::cdrEdgeType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrEdgeType {
    /// 边缘线性。
    EdgeLinear = 0,
    /// 边缘 squared。
    EdgeSquared = 1,
    /// 边缘 flat。
    EdgeFlat = 2,
    /// 边缘 inversesquared。
    EdgeInverseSquared = 3,
    /// 边缘 mesa。
    EdgeMesa = 4,
    /// 边缘 gaussian。
    EdgeGaussian = 5,
}

// =============================================================
// cdrMergeMode —— 混合模式
// =============================================================

/// CorelDRAW 枚举 `cdrMergeMode`：混合模式。
///
/// 对应类型库中的 `VGCore::cdrMergeMode`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrMergeMode {
    /// 合并常规。
    MergeNormal = 0,
    /// 合并。
    MergeAND = 1,
    /// 合并。
    MergeOR = 2,
    /// 合并 xor。
    MergeXOR = 3,
    /// 合并反转。
    MergeInvert = 6,
    /// 合并添加。
    MergeAdd = 7,
    /// 合并 subtract。
    MergeSubtract = 8,
    /// 合并 multiply。
    MergeMultiply = 9,
    /// 合并 divide。
    MergeDivide = 10,
    /// 合并 iflighter。
    MergeIfLighter = 11,
    /// 合并 ifdarker。
    MergeIfDarker = 12,
    /// 合并 texturize。
    MergeTexturize = 13,
    /// 合并颜色。
    MergeColor = 14,
    /// 合并色相。
    MergeHue = 15,
    /// 合并饱和度。
    MergeSaturation = 16,
    /// 合并亮度。
    MergeLightness = 17,
    /// 合并红。
    MergeRed = 18,
    /// 合并绿。
    MergeGreen = 19,
    /// 合并蓝。
    MergeBlue = 20,
    /// 合并 difference。
    MergeDifference = 24,
    /// 合并后方。
    MergeBehind = 27,
    /// 合并屏幕。
    MergeScreen = 28,
    /// 合并 overlay。
    MergeOverlay = 29,
    /// 合并 softlight。
    MergeSoftlight = 30,
    /// 合并 hardlight。
    MergeHardlight = 31,
    /// 合并 dodge。
    MergeDodge = 33,
    /// 合并 burn。
    MergeBurn = 34,
    /// 合并 exclusion。
    MergeExclusion = 36,
}

// =============================================================
// cdrFittedOrientation —— 适配路径朝向
// =============================================================

/// CorelDRAW 枚举 `cdrFittedOrientation`：适配路径朝向。
///
/// 对应类型库中的 `VGCore::cdrFittedOrientation`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedOrientation {
    /// rotate 方向。
    RotateOrientation = 0,
    /// 垂直倾斜方向。
    VerticalSkewOrientation = 1,
    /// 水平倾斜方向。
    HorizontalSkewOrientation = 2,
    /// upright 方向。
    UprightOrientation = 3,
    /// 变换方向。
    TransformOrientation = 4,
}

// =============================================================
// cdrFittedPlacement —— 适配路径放置
// =============================================================

/// CorelDRAW 枚举 `cdrFittedPlacement`：适配路径放置。
///
/// 对应类型库中的 `VGCore::cdrFittedPlacement`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedPlacement {
    /// 左放置。
    LeftPlacement = 0,
    /// 右放置。
    RightPlacement = 1,
    /// 中心放置。
    CenterPlacement = 2,
}

// =============================================================
// cdrFittedQuadrant —— 适配路径象限
// =============================================================

/// CorelDRAW 枚举 `cdrFittedQuadrant`：适配路径象限。
///
/// 对应类型库中的 `VGCore::cdrFittedQuadrant`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedQuadrant {
    /// 左 quadrant。
    LeftQuadrant = 0,
    /// 右 quadrant。
    RightQuadrant = 1,
    /// 顶部 quadrant。
    TopQuadrant = 2,
    /// 底部 quadrant。
    BottomQuadrant = 3,
}

// =============================================================
// cdrFittedVertPlacement —— 适配路径垂直放置
// =============================================================

/// CorelDRAW 枚举 `cdrFittedVertPlacement`：适配路径垂直放置。
///
/// 对应类型库中的 `VGCore::cdrFittedVertPlacement`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFittedVertPlacement {
    /// 自定义垂直放置。
    CustomVertPlacement = 0,
    /// baseline 垂直放置。
    BaselineVertPlacement = 1,
    /// ascender 垂直放置。
    AscenderVertPlacement = 2,
    /// descender 垂直放置。
    DescenderVertPlacement = 3,
    /// 中心垂直放置。
    CenterVertPlacement = 4,
}

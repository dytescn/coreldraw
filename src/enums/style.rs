//! 样式枚举

// =============================================================
// cdrFillStyleType —— 填充样式类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFillStyleType {
    NoFillStyle = 0,
    UniformFillStyle = 1,
    FountainFillStyle = 2,
    PostscriptFillStyle = 3,
    TwoColorPatternFillStyle = 4,
    BitmapPatternFillStyle = 6,
    TextureFillStyle = 8,
    FullColorPatternFillStyle = 9,
    HatchFillStyle = 10,
}

// =============================================================
// cdrOutlineStyleType —— 轮廓样式类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrOutlineStyleType {
    NoOutline = 0,
    Solid = 1,
    Dashed = 2,
    // TODO: 待补
}

// =============================================================
// cdrTransparencyAppliedTo —— 透明应用对象
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTransparencyAppliedTo {
    Fill = 0,
    Outline = 1,
    Both = 2,
}

// =============================================================
// cdrTransparencyType —— 透明类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTransparencyType {
    NoTransparency = 0,
    UniformTransparency = 1,
    FountainTransparency = 2,
    PatternTransparency = 3,
    TextureTransparency = 4,
}

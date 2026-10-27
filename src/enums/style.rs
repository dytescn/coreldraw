//! 样式枚举

// =============================================================
// cdrFillStyleType —— 填充样式类型
// =============================================================

/// CorelDRAW 枚举 `cdrFillStyleType`：填充样式类型。
///
/// 对应类型库中的 `VGCore::cdrFillStyleType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFillStyleType {
    /// 无填充样式。
    NoFillStyle = 0,
    /// 均匀填充样式。
    UniformFillStyle = 1,
    /// 渐变填充样式。
    FountainFillStyle = 2,
    /// PostScript 填充样式。
    PostscriptFillStyle = 3,
    /// two 颜色图案填充样式。
    TwoColorPatternFillStyle = 4,
    /// 位图图案填充样式。
    BitmapPatternFillStyle = 6,
    /// 纹理填充样式。
    TextureFillStyle = 8,
    /// 完整颜色图案填充样式。
    FullColorPatternFillStyle = 9,
    /// 剖面线填充样式。
    HatchFillStyle = 10,
}

// =============================================================
// cdrOutlineStyleType —— 轮廓样式类型
// =============================================================

/// CorelDRAW 枚举 `cdrOutlineStyleType`：轮廓样式类型。
///
/// 对应类型库中的 `VGCore::cdrOutlineStyleType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrOutlineStyleType {
    /// 无轮廓。
    NoOutline = 0,
    /// solid。
    Solid = 1,
    /// dashed。
    Dashed = 2,
    // TODO: 待补
}

// =============================================================
// cdrTransparencyAppliedTo —— 透明应用对象
// =============================================================

/// CorelDRAW 枚举 `cdrTransparencyAppliedTo`：透明应用对象。
///
/// 对应类型库中的 `VGCore::cdrTransparencyAppliedTo`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTransparencyAppliedTo {
    /// 填充。
    Fill = 0,
    /// 轮廓。
    Outline = 1,
    /// 双向。
    Both = 2,
}

// =============================================================
// cdrTransparencyType —— 透明类型
// =============================================================

/// CorelDRAW 枚举 `cdrTransparencyType`：透明类型。
///
/// 对应类型库中的 `VGCore::cdrTransparencyType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTransparencyType {
    /// 无透明度。
    NoTransparency = 0,
    /// 均匀透明度。
    UniformTransparency = 1,
    /// 渐变透明度。
    FountainTransparency = 2,
    /// 图案透明度。
    PatternTransparency = 3,
    /// 纹理透明度。
    TextureTransparency = 4,
}

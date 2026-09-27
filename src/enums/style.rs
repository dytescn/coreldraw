//! 样式枚举

// =============================================================
// cdrFillStyleType —— 填充样式类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFillStyleType {
    Uniform  = 0,
    Fountain = 1,
    Pattern  = 2,
    Texture  = 3,
    PostScript = 4,
    Hatch    = 5,
    // TODO: 待补
}

// =============================================================
// cdrOutlineStyleType —— 轮廓样式类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrOutlineStyleType {
    NoOutline = 0,
    Solid     = 1,
    Dashed    = 2,
    // TODO: 待补
}

// =============================================================
// cdrTransparencyAppliedTo —— 透明应用对象
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTransparencyAppliedTo {
    Fill    = 0,
    Outline = 1,
    Both    = 2,
}

// =============================================================
// cdrTransparencyType —— 透明类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTransparencyType {
    NoTransparency     = 0,
    Uniform            = 1,
    Fountain           = 2,
    Pattern            = 3,
    Texture            = 4,
    // TODO: 待补
}
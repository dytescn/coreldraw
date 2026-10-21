//! 填充枚举

// =============================================================
// cdrFillType —— 填充类型
// =============================================================

/// `Fill.Type` 的取值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFillType {
    NoFill = 0,
    UniformFill = 1,
    FountainFill = 2,
    PostscriptFill = 3,
    TextureFill = 8,
    PatternFill = 9,
    HatchFill = 10,
}

impl cdrFillType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::NoFill),
            1 => Some(Self::UniformFill),
            2 => Some(Self::FountainFill),
            3 => Some(Self::PostscriptFill),
            8 => Some(Self::TextureFill),
            9 => Some(Self::PatternFill),
            10 => Some(Self::HatchFill),
            _ => None,
        }
    }
}

// =============================================================
// cdrFountainFillType —— 渐变填充类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFountainFillType {
    LinearFountainFill = 1,
    RadialFountainFill = 2,
    ConicalFountainFill = 3,
    SquareFountainFill = 4,
}

impl cdrFountainFillType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            1 => Some(Self::LinearFountainFill),
            2 => Some(Self::RadialFountainFill),
            3 => Some(Self::ConicalFountainFill),
            4 => Some(Self::SquareFountainFill),
            _ => None,
        }
    }
}

// =============================================================
// cdrFountainFillBlendType —— 渐变混合类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFountainFillBlendType {
    DirectFountainFillBlend = 0,
    RainbowCWFountainFillBlend = 1,
    RainbowCCWFountainFillBlend = 2,
    CustomFountainFillBlend = 3,
}

// =============================================================
// cdrFountainFillSpreadMethod —— 渐变扩散方式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFountainFillSpreadMethod {
    Pad = 0,
    Reflect = 1,
    Repeat = 2,
}

// =============================================================
// cdrPatternFillType —— 图案填充类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPatternFillType {
    TwoColorPattern = 0,
    FullColorPattern = 1,
    BitmapPattern = 2,
}

// =============================================================
// cdrHatchFillType —— 剖面线填充类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrHatchFillType {
    // TODO: 待补
    Custom = 0,
}

// =============================================================
// cdrFillMode —— 填充模式（用于复合路径）
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFillMode {
    /// 交替填充。
    Alternate = 0,
    /// 缠绕填充。
    Winding = 1,
}

// =============================================================
// cdrTileOffsetType —— 图块偏移类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTileOffsetType {
    TileOffsetRow = 0,
    TileOffsetColumn = 1,
}

// =============================================================
// cdrTexturePropertyType —— 纹理属性类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTexturePropertyType {
    TexturePropertyNumeric = 0,
    TexturePropertyColorRGB = 1,
    TexturePropertyColorHSB = 2,
    TexturePropertyColorCMYK = 3,
}

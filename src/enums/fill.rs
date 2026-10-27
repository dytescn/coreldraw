//! 填充枚举

// =============================================================
// cdrFillType —— 填充类型
// =============================================================

/// `Fill.Type` 的取值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFillType {
    /// 无填充。
    NoFill = 0,
    /// 均匀填充。
    UniformFill = 1,
    /// 渐变填充。
    FountainFill = 2,
    /// PostScript 填充。
    PostscriptFill = 3,
    /// 纹理填充。
    TextureFill = 8,
    /// 图案填充。
    PatternFill = 9,
    /// 剖面线填充。
    HatchFill = 10,
}

impl cdrFillType {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrFountainFillType`：渐变填充类型。
///
/// 对应类型库中的 `VGCore::cdrFountainFillType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFountainFillType {
    /// 线性渐变填充。
    LinearFountainFill = 1,
    /// radial 渐变填充。
    RadialFountainFill = 2,
    /// conical 渐变填充。
    ConicalFountainFill = 3,
    /// square 渐变填充。
    SquareFountainFill = 4,
}

impl cdrFountainFillType {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrFountainFillBlendType`：渐变混合类型。
///
/// 对应类型库中的 `VGCore::cdrFountainFillBlendType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFountainFillBlendType {
    /// direct 渐变填充调和。
    DirectFountainFillBlend = 0,
    /// rainbowcw 渐变填充调和。
    RainbowCWFountainFillBlend = 1,
    /// rainbowccw 渐变填充调和。
    RainbowCCWFountainFillBlend = 2,
    /// 自定义渐变填充调和。
    CustomFountainFillBlend = 3,
}

// =============================================================
// cdrFountainFillSpreadMethod —— 渐变扩散方式
// =============================================================

/// CorelDRAW 枚举 `cdrFountainFillSpreadMethod`：渐变扩散方式。
///
/// 对应类型库中的 `VGCore::cdrFountainFillSpreadMethod`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFountainFillSpreadMethod {
    /// 边距。
    Pad = 0,
    /// reflect。
    Reflect = 1,
    /// repeat。
    Repeat = 2,
}

// =============================================================
// cdrPatternFillType —— 图案填充类型
// =============================================================

/// CorelDRAW 枚举 `cdrPatternFillType`：图案填充类型。
///
/// 对应类型库中的 `VGCore::cdrPatternFillType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPatternFillType {
    /// two 颜色图案。
    TwoColorPattern = 0,
    /// 完整颜色图案。
    FullColorPattern = 1,
    /// 位图图案。
    BitmapPattern = 2,
}

// =============================================================
// cdrHatchFillType —— 剖面线填充类型
// =============================================================

/// CorelDRAW 枚举 `cdrHatchFillType`：剖面线填充类型。
///
/// 对应类型库中的 `VGCore::cdrHatchFillType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrHatchFillType {
    // TODO: 待补
    /// 自定义。
    Custom = 0,
}

// =============================================================
// cdrFillMode —— 填充模式（用于复合路径）
// =============================================================

/// CorelDRAW 枚举 `cdrFillMode`：填充模式（用于复合路径）。
///
/// 对应类型库中的 `VGCore::cdrFillMode`，用 `#[repr(i32)]` 映射实际整数值。
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

/// CorelDRAW 枚举 `cdrTileOffsetType`：图块偏移类型。
///
/// 对应类型库中的 `VGCore::cdrTileOffsetType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTileOffsetType {
    /// 平铺偏移 row。
    TileOffsetRow = 0,
    /// 平铺偏移分栏。
    TileOffsetColumn = 1,
}

// =============================================================
// cdrTexturePropertyType —— 纹理属性类型
// =============================================================

/// CorelDRAW 枚举 `cdrTexturePropertyType`：纹理属性类型。
///
/// 对应类型库中的 `VGCore::cdrTexturePropertyType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTexturePropertyType {
    /// 纹理属性 numeric。
    TexturePropertyNumeric = 0,
    /// 纹理属性颜色 RGB。
    TexturePropertyColorRGB = 1,
    /// 纹理属性颜色 HSB。
    TexturePropertyColorHSB = 2,
    /// 纹理属性颜色 CMYK。
    TexturePropertyColorCMYK = 3,
}

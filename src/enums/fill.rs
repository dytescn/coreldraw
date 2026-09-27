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
    None       = 0,
    /// 单色填充。
    Uniform    = 1,
    /// 渐变（喷泉）填充。
    Fountain   = 2,
    /// 图案填充。
    Pattern    = 3,
    /// 纹理填充。
    Texture    = 4,
    /// PostScript 填充。
    PostScript = 5,
    /// 网状填充。
    Mesh       = 6,
    /// 填充继承。
    Hatch      = 7,
    // TODO: 更完整的列表待补
}

impl cdrFillType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::None),
            1 => Some(Self::Uniform),
            2 => Some(Self::Fountain),
            3 => Some(Self::Pattern),
            4 => Some(Self::Texture),
            5 => Some(Self::PostScript),
            6 => Some(Self::Mesh),
            7 => Some(Self::Hatch),
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
    /// 线性。
    Linear       = 1,
    /// 射线。
    Radial       = 2,
    /// 圆锥。
    Conical      = 3,
    /// 方形。
    Square       = 4,
    /// 矩形。
    Rectangular  = 5,
    // TODO: 待补
}

impl cdrFountainFillType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            1 => Some(Self::Linear),
            2 => Some(Self::Radial),
            3 => Some(Self::Conical),
            4 => Some(Self::Square),
            5 => Some(Self::Rectangular),
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
    /// RGB 直接混合。
    Direct      = 0,
    /// RGB 顺时针混合。
    Clockwise   = 1,
    /// RGB 逆时针混合。
    CounterClockwise = 2,
    // TODO: 待补
}

// =============================================================
// cdrFountainFillSpreadMethod —— 渐变扩散方式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFountainFillSpreadMethod {
    Pad     = 0,
    Reflect = 1,
    Repeat  = 2,
}

// =============================================================
// cdrPatternFillType —— 图案填充类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPatternFillType {
    /// 双色位图。
    Monochrome = 0,
    /// 全彩位图。
    FullColor  = 1,
    // TODO: 待补
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
    Alternate   = 0,
    /// 缠绕填充。
    Winding     = 1,
}

// =============================================================
// cdrTileOffsetType —— 图块偏移类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTileOffsetType {
    // TODO: 待补
    Percent = 0,
    // ...
}

// =============================================================
// cdrTexturePropertyType —— 纹理属性类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTexturePropertyType {
    // TODO: 待补
    Integer = 0,
    // ...
}
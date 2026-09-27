//! 图像枚举

// =============================================================
// cdrImageType —— 图像色彩模式
// =============================================================

/// `Bitmap.Mode` 的取值。
///
/// 来源：`VGCore::cdrImageType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrImageType {
    /// 黑白（1 位）。
    BlackAndWhite = 0,
    /// 16 色。
    Colors16      = 1,
    /// 灰度。
    Grayscale     = 2,
    /// 调色板。
    Paletted      = 3,
    /// RGB 彩色。
    RGBColor      = 4,
    /// CMYK 彩色。
    CMYKColor     = 5,
    /// 双色调。
    Duotone       = 6,
}

impl cdrImageType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::BlackAndWhite),
            1 => Some(Self::Colors16),
            2 => Some(Self::Grayscale),
            3 => Some(Self::Paletted),
            4 => Some(Self::RGBColor),
            5 => Some(Self::CMYKColor),
            6 => Some(Self::Duotone),
            _ => None,
        }
    }
}

// =============================================================
// cdrAntiAliasingType —— 抗锯齿类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrAntiAliasingType {
    NoAntiAliasing     = 0,
    NormalAntiAliasing = 1,
    Supersampling      = 2,
}

impl cdrAntiAliasingType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::NoAntiAliasing),
            1 => Some(Self::NormalAntiAliasing),
            2 => Some(Self::Supersampling),
            _ => None,
        }
    }
}

// =============================================================
// cdrRenderType —— 黑白渲染类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrRenderType {
    Threshold  = 0,
    Halftone   = 1,
    // TODO: 待补
}

// =============================================================
// cdrHalftoneType —— 半色调类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrHalftoneType {
    // TODO: 待补
    Square = 0,
    // ...
}

// =============================================================
// cdrDitherType —— 抖动类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDitherType {
    None       = 0,
    Ordered    = 1,
    Diffusion  = 2,
    // TODO: 待补
}

// =============================================================
// cdrImagePaletteType —— 调色板类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrImagePaletteType {
    // TODO: 待补
    Optimized = 0,
    // ...
}

// =============================================================
// cdrTraceType —— 描摹类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTraceType {
    LineArt    = 0,
    // TODO: 待补
}

// =============================================================
// cdrTraceBackgroundMode —— 描摹背景模式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTraceBackgroundMode {
    // TODO: 待补
    Auto = 0,
    // ...
}